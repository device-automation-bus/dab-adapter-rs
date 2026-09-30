use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::process::Command;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

use crate::dab::structs::DabError;
use crate::dab::structs::StartSystemLogCollectionRequest;
use crate::dab::structs::StartSystemLogCollectionResponse;
use crate::device::rdk::interface::is_local_device;
use crate::device::rdk::system::logs::{
    append_range, crash_files, live_logs, Collection, COLLECTION, LOG_DIR, STAGING_DIR,
};

// Free space needed in /tmp (tmpfs) to stage the logs, in KiB.
const MIN_FREE_KB: u64 = 50 * 1024;
// logrotate runs every minute with copytruncate, which empties the live file
// in place. Polling often keeps the part only found in the rotated copy small.
const POLL_INTERVAL: Duration = Duration::from_millis(250);

#[allow(non_snake_case)]
#[allow(dead_code)]
#[allow(unused_mut)]
pub fn process(_dab_request: StartSystemLogCollectionRequest) -> Result<String, DabError> {
    let mut ResponseOperator = StartSystemLogCollectionResponse::default();
    // *** Fill in the fields of the struct StartSystemLogCollectionResponse here ***

    if !is_local_device() {
        return Err(DabError::Err501(
            "Log collection is only supported when the adapter runs on the device".to_string(),
        ));
    }

    let mut collection = COLLECTION.lock().unwrap();
    // The spec does not define a start while a collection is running. Start
    // over, so a client that lost track of a collection is never stuck.
    if let Some(previous) = collection.take() {
        previous.stop.store(true, Ordering::SeqCst);
        let _ = previous.reader.join();
    }

    // busybox `df -k` prints: Filesystem 1K-blocks Used Available Use% Mounted on
    let df = Command::new("df")
        .args(["-k", "/tmp"])
        .output()
        .map_err(|e| DabError::Err500(format!("Cannot check free disk space: {}", e)))?;
    let available_kb = String::from_utf8_lossy(&df.stdout)
        .lines()
        .last()
        .and_then(|line| line.split_whitespace().nth(3))
        .and_then(|available| available.parse::<u64>().ok())
        .ok_or_else(|| DabError::Err500("Cannot check free disk space".to_string()))?;
    if available_kb < MIN_FREE_KB {
        return Err(DabError::Err500(
            "No more disk space, cannot begin system log collection.".to_string(),
        ));
    }

    let _ = fs::remove_dir_all(STAGING_DIR);
    let system_dir = Path::new(STAGING_DIR).join("system");
    fs::create_dir_all(&system_dir)
        .map_err(|e| DabError::Err500(format!("Cannot create {}: {}", system_dir.display(), e)))?;

    // Only what is written after this point is collected. Logs created later
    // are not in the map and are read from the start.
    let mut offsets: HashMap<String, u64> = live_logs().into_iter().collect();
    let stop = Arc::new(AtomicBool::new(false));
    let reader_stop = stop.clone();

    let reader = thread::spawn(move || loop {
        // Read once more after stop is requested, to catch the last lines.
        let stopping = reader_stop.load(Ordering::SeqCst);
        for (name, len) in live_logs() {
            let src = Path::new(LOG_DIR).join(&name);
            let dest = system_dir.join(&name);
            let offset = offsets.entry(name.clone()).or_insert(0);

            if len < *offset {
                // The file was rotated with copytruncate. The lines written
                // since the last poll are at the end of the rotated copy, the
                // newest file named `<name>.N` or `<name>-<date>`.
                let rotated = fs::read_dir(LOG_DIR)
                    .into_iter()
                    .flatten()
                    .flatten()
                    .filter(|entry| {
                        let entry_name = entry.file_name().to_string_lossy().into_owned();
                        entry_name.starts_with(&format!("{}.", name))
                            || entry_name.starts_with(&format!("{}-", name))
                    })
                    .filter_map(|entry| Some((entry.path(), entry.metadata().ok()?)))
                    .max_by_key(|(_, metadata)| metadata.modified().ok());
                if let Some((path, metadata)) = rotated {
                    if let Err(e) = append_range(&path, *offset, metadata.len(), &dest) {
                        println!("Error reading {}: {}", path.display(), e);
                    }
                }
                *offset = 0;
            }

            if len > *offset {
                match append_range(&src, *offset, len, &dest) {
                    Ok(_) => *offset = len,
                    Err(e) => println!("Error reading {}: {}", src.display(), e),
                }
            }
        }
        if stopping {
            break;
        }
        thread::sleep(POLL_INTERVAL);
    });

    *collection = Some(Collection {
        stop,
        reader,
        crash_files: crash_files(),
    });

    // *******************************************************************
    Ok(serde_json::to_string(&ResponseOperator).unwrap())
}

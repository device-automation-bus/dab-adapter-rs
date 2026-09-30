use std::fs;
use std::path::Path;
use std::process::Command;
use std::sync::atomic::Ordering;

use crate::dab::structs::DabError;
use crate::dab::structs::StopSystemLogCollectionRequest;
use crate::dab::structs::StopSystemLogCollectionResponse;
use crate::device::rdk::system::logs::{crash_files, COLLECTION, STAGING_DIR};

use base64::{engine::general_purpose, Engine as _};

// Size of the base64 `logArchive` in each response.
const CHUNK_SIZE: usize = 1024 * 1024;
// RDK has no per-app log files: the apps run as Thunder plugins and log into
// wpeframework.log, with lines tagged by plugin (e.g. `[Cobalt]:[YouTube]`,
// `[Cobalt/24259:...]`). Lines are split by DAB appId on these tags.
const APP_LOG_TAGS: [(&str, &[&str]); 3] = [
    ("YouTube", &["[YouTube", "[Cobalt"]),
    ("PrimeVideo", &["[PrimeVideo", "[Amazon"]),
    ("Netflix", &["[Netflix"]),
];

#[allow(non_snake_case)]
#[allow(dead_code)]
#[allow(unused_mut)]
pub fn process(_dab_request: StopSystemLogCollectionRequest) -> Result<String, DabError> {
    let collection = COLLECTION
        .lock()
        .unwrap()
        .take()
        .ok_or_else(|| DabError::Err400("System log collection is not started".to_string()))?;
    collection.stop.store(true, Ordering::SeqCst);
    collection
        .reader
        .join()
        .map_err(|_| DabError::Err500("Log reader thread panicked".to_string()))?;

    let staging = Path::new(STAGING_DIR);
    let create_dir = |dir: &Path| {
        fs::create_dir_all(dir)
            .map_err(|e| DabError::Err500(format!("Cannot create {}: {}", dir.display(), e)))
    };

    let wpeframework_log = fs::read(staging.join("system").join("wpeframework.log"))
        .map(|log| String::from_utf8_lossy(&log).into_owned())
        .unwrap_or_default();
    for (app_id, tags) in APP_LOG_TAGS {
        let app_dir = staging.join("application").join(app_id);
        create_dir(&app_dir)?;
        let app_log: String = wpeframework_log
            .lines()
            .filter(|line| tags.iter().any(|tag| line.contains(tag)))
            .map(|line| format!("{}\n", line))
            .collect();
        if !app_log.is_empty() {
            fs::write(app_dir.join("wpeframework.log"), app_log)
                .map_err(|e| DabError::Err500(e.to_string()))?;
        }
    }

    let crash_dir = staging.join("crash");
    create_dir(&crash_dir)?;
    for path in crash_files().difference(&collection.crash_files) {
        if let Some(name) = path.file_name() {
            if let Err(e) = fs::copy(path, crash_dir.join(name)) {
                println!("Error copying {}: {}", path.display(), e);
            }
        }
    }

    let archive = format!("{}.tar.gz", STAGING_DIR);
    let tar = Command::new("tar")
        .args([
            "-czf",
            &archive,
            "-C",
            STAGING_DIR,
            "system",
            "application",
            "crash",
        ])
        .status()
        .map_err(|e| DabError::Err500(format!("Cannot run tar: {}", e)))?;
    let data = if tar.success() {
        fs::read(&archive).map_err(|e| DabError::Err500(e.to_string()))
    } else {
        Err(DabError::Err500(format!("tar failed: {}", tar)))
    };
    let _ = fs::remove_dir_all(STAGING_DIR);
    let _ = fs::remove_file(&archive);
    let log_archive = general_purpose::STANDARD.encode(data?);

    // base64 is ASCII, so splitting on bytes gives valid strings.
    let chunks: Vec<&[u8]> = log_archive.as_bytes().chunks(CHUNK_SIZE).collect();
    let ResponseOperator: Vec<StopSystemLogCollectionResponse> = chunks
        .iter()
        .enumerate()
        .map(|(index, chunk)| StopSystemLogCollectionResponse {
            logArchive: String::from_utf8_lossy(chunk).into_owned(),
            remainingChunks: (chunks.len() - 1 - index) as u32,
        })
        .collect();

    // *******************************************************************
    // A JSON array: dab.rs publishes each chunk as its own response.
    Ok(serde_json::to_string(&ResponseOperator).unwrap())
}

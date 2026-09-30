pub mod start_collection;
pub mod stop_collection;

use lazy_static::lazy_static;
use std::collections::HashSet;
use std::fs::{self, File, OpenOptions};
use std::io::{self, Read, Seek, SeekFrom};
use std::path::{Path, PathBuf};
use std::sync::atomic::AtomicBool;
use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;

// Log collection reads the device filesystem directly (there is no Thunder
// API that returns logs), so it only works when the adapter runs on the device.
pub const LOG_DIR: &str = "/opt/logs";
// Logs collected between start and stop are staged here, in the folder
// structure the spec asks for (system/, application/<appId>/, crash/).
pub const STAGING_DIR: &str = "/tmp/dab-logs";
// Where /lib/rdk/core_shell.sh writes crash dumps, with and without secure dump.
pub const CRASH_DIRS: [&str; 4] = [
    "/opt/secure/corefiles",
    "/opt/secure/minidumps",
    "/opt/minidumps",
    "/var/lib/systemd/coredump",
];

pub struct Collection {
    pub stop: Arc<AtomicBool>,
    pub reader: JoinHandle<()>,
    // Crash dumps already present at start, left out of the archive.
    pub crash_files: HashSet<PathBuf>,
}

lazy_static! {
    pub static ref COLLECTION: Mutex<Option<Collection>> = Mutex::new(None);
}

// Live logs in /opt/logs and their current size. Rotated copies (`*.log.1`,
// `*.log-<date>`) and the PreviousLogs folders are left out; the names match
// the patterns in /etc/logrotatemax.conf.
pub fn live_logs() -> Vec<(String, u64)> {
    let Ok(entries) = fs::read_dir(LOG_DIR) else {
        return Vec::new();
    };
    entries
        .flatten()
        .filter_map(|entry| {
            let name = entry.file_name().into_string().ok()?;
            if !(name.ends_with(".log") || name.ends_with(".txt") || name.ends_with(".txt.0")) {
                return None;
            }
            // fs::metadata follows symlinks such as rdk_shell.log.
            let metadata = fs::metadata(entry.path()).ok()?;
            metadata.is_file().then(|| (name, metadata.len()))
        })
        .collect()
}

pub fn crash_files() -> HashSet<PathBuf> {
    CRASH_DIRS
        .iter()
        .filter_map(|dir| fs::read_dir(dir).ok())
        .flatten()
        .flatten()
        .map(|entry| entry.path())
        .filter(|path| path.is_file())
        .collect()
}

// Appends bytes [from, to) of `src` to `dest`.
pub fn append_range(src: &Path, from: u64, to: u64, dest: &Path) -> io::Result<u64> {
    let mut src = File::open(src)?;
    src.seek(SeekFrom::Start(from))?;
    let mut dest = OpenOptions::new().create(true).append(true).open(dest)?;
    io::copy(&mut src.take(to.saturating_sub(from)), &mut dest)
}

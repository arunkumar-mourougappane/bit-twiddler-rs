use std::sync::Mutex;
use serde::Serialize;
use tauri::Emitter;
use digest::Digest;
use md5::Md5;
use sha1::Sha1;
use sha2::{Sha256, Sha512};
use hex;
use notify::{RecommendedWatcher, RecursiveMode, Watcher, Config, Event, EventKind};
use std::time::Duration;
use std::io::Read;

#[derive(Serialize, Clone)]
pub struct HashResult {
    pub md5: String,
    pub sha1: String,
    pub sha256: String,
    pub sha512: String,
}

static FILE_WATCHER: Mutex<Option<RecommendedWatcher>> = Mutex::new(None);

fn compute_hashes_from_bytes(data: &[u8]) -> HashResult {
    let md5_hash = hex::encode(Md5::digest(data));
    let sha1_hash = hex::encode(Sha1::digest(data));
    let sha256_hash = hex::encode(Sha256::digest(data));
    let sha512_hash = hex::encode(Sha512::digest(data));
    HashResult {
        md5: md5_hash,
        sha1: sha1_hash,
        sha256: sha256_hash,
        sha512: sha512_hash,
    }
}

fn compute_hashes_from_file(path: &str) -> Result<HashResult, String> {
    let mut file = std::fs::File::open(path).map_err(|e| e.to_string())?;
    let mut md5_h = Md5::new();
    let mut sha1_h = Sha1::new();
    let mut sha256_h = Sha256::new();
    let mut sha512_h = Sha512::new();
    let mut buf = [0u8; 65536];
    loop {
        let n = file.read(&mut buf).map_err(|e| e.to_string())?;
        if n == 0 {
            break;
        }
        digest::Update::update(&mut md5_h, &buf[..n]);
        digest::Update::update(&mut sha1_h, &buf[..n]);
        digest::Update::update(&mut sha256_h, &buf[..n]);
        digest::Update::update(&mut sha512_h, &buf[..n]);
    }
    Ok(HashResult {
        md5: hex::encode(md5_h.finalize()),
        sha1: hex::encode(sha1_h.finalize()),
        sha256: hex::encode(sha256_h.finalize()),
        sha512: hex::encode(sha512_h.finalize()),
    })
}

#[tauri::command]
pub fn generate_hashes(text: String) -> Result<HashResult, String> {
    Ok(compute_hashes_from_bytes(text.as_bytes()))
}

#[tauri::command]
pub fn hash_file(file_path: String, app_handle: tauri::AppHandle) -> Result<HashResult, String> {
    // Stop existing watcher
    {
        let mut guard = FILE_WATCHER.lock().map_err(|e| e.to_string())?;
        *guard = None;
    }

    // Compute initial hashes
    let hashes = compute_hashes_from_file(&file_path)?;

    // Start file watcher
    let watch_path = file_path.clone();
    let app = app_handle.clone();

    let (tx, rx) = std::sync::mpsc::channel::<notify::Result<Event>>();
    let mut watcher = RecommendedWatcher::new(tx, Config::default())
        .map_err(|e| e.to_string())?;
    watcher
        .watch(
            std::path::Path::new(&file_path),
            RecursiveMode::NonRecursive,
        )
        .map_err(|e| e.to_string())?;

    std::thread::spawn(move || {
        let mut last_emit = std::time::Instant::now();
        let debounce = Duration::from_millis(300);
        for result in rx {
            if let Ok(event) = result {
                if matches!(event.kind, EventKind::Modify(_)) {
                    let now = std::time::Instant::now();
                    if now.duration_since(last_emit) >= debounce {
                        last_emit = now;
                        if let Ok(h) = compute_hashes_from_file(&watch_path) {
                            let _ = app.emit("file-hash-update", &h);
                        }
                    }
                }
            }
        }
    });

    let mut guard = FILE_WATCHER.lock().map_err(|e| e.to_string())?;
    *guard = Some(watcher);
    Ok(hashes)
}

#[tauri::command]
pub fn stop_file_watch() -> Result<(), String> {
    let mut guard = FILE_WATCHER.lock().map_err(|e| e.to_string())?;
    *guard = None;
    Ok(())
}

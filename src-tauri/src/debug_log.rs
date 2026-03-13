// #region agent log
use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;

fn log_path() -> PathBuf {
    let base = if let Ok(manifest) = std::env::var("CARGO_MANIFEST_DIR") {
        let p = PathBuf::from(&manifest);
        p.parent().unwrap_or(&p).to_path_buf()
    } else if let Ok(cwd) = std::env::current_dir() {
        if cwd.ends_with("src-tauri") {
            cwd.parent().unwrap_or(&cwd).to_path_buf()
        } else {
            cwd
        }
    } else {
        PathBuf::from(".")
    };
    let log_dir = base.join(".cursor");
    let _ = std::fs::create_dir_all(&log_dir);
    log_dir.join("debug-1221d1.log")
}

pub fn debug_log(location: &str, message: &str, data: serde_json::Value, hypothesis_id: &str) {
    let payload = serde_json::json!({
        "sessionId": "1221d1",
        "location": location,
        "message": message,
        "data": data,
        "hypothesisId": hypothesis_id,
        "timestamp": std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_millis()
    });
    if let Ok(mut f) = OpenOptions::new().create(true).append(true).open(log_path()) {
        let _ = writeln!(f, "{}", payload);
    }
}
// #endregion

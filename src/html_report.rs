// Write KaTeX HTML reports to Documents\StormSewer and open in the default browser.

use std::fs;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

/// Folder under the user's Documents directory for storm-sewer reports.
pub fn output_folder() -> PathBuf {
    let docs = std::env::var("USERPROFILE")
        .ok()
        .map(|h| PathBuf::from(h).join("Documents"))
        .or_else(|| std::env::var("HOME").ok().map(PathBuf::from))
        .unwrap_or_else(|| PathBuf::from("."));
    docs.join("StormSewer")
}

fn sanitize_filename(name: &str) -> String {
    let s = name.trim();
    if s.is_empty() {
        return "drawing".into();
    }
    let bad = ['<', '>', ':', '"', '/', '\\', '|', '?', '*'];
    let mut out = String::with_capacity(s.len());
    for ch in s.chars() {
        if bad.contains(&ch) {
            out.push('_');
        } else {
            out.push(ch);
        }
    }
    if out.is_empty() {
        "drawing".into()
    } else {
        out
    }
}

fn stamp() -> String {
    let secs = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    format!("{secs}")
}

/// Write `html` to Documents\StormSewer and return the absolute path.
pub fn write_report(html: &str, drawing_label: &str) -> Result<PathBuf, String> {
    let folder = output_folder();
    fs::create_dir_all(&folder).map_err(|e| format!("create report folder: {e}"))?;
    let safe = sanitize_filename(drawing_label);
    let path = folder.join(format!("report-{safe}-{}.html", stamp()));
    fs::write(&path, html).map_err(|e| format!("write report: {e}"))?;
    Ok(path)
}

/// Open a file with the system default handler (browser for .html).
pub fn open_in_browser(path: &Path) {
    #[cfg(windows)]
    {
        let _ = std::process::Command::new("cmd")
            .args(["/C", "start", "", &path.display().to_string()])
            .spawn();
    }
    #[cfg(target_os = "macos")]
    {
        let _ = std::process::Command::new("open").arg(path).spawn();
    }
    #[cfg(all(unix, not(target_os = "macos")))]
    {
        let _ = std::process::Command::new("xdg-open").arg(path).spawn();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sanitize_strips_invalid_chars() {
        assert_eq!(sanitize_filename("a/b:c"), "a_b_c");
    }
}
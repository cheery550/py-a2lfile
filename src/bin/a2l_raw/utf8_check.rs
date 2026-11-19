//! Strict UTF-8 gate in front of `a2lfile`.
//!
//! `a2lfile` falls back to latin-1 when a file is not valid UTF-8, which turns
//! GBK or other legacy-encoded text into silent mojibake. The bridge rejects such
//! input instead, for the root file and every file it pulls in via `/include`.

use std::collections::HashSet;
use std::path::{MAIN_SEPARATOR_STR, Path, PathBuf};

/// Validate `root` and its include tree; `Err` is a `path:line: message` string.
pub fn check_tree(root: &Path) -> Result<(), String> {
    let mut seen = HashSet::new();
    check_file(root, &mut seen)
}

fn check_file(path: &Path, seen: &mut HashSet<PathBuf>) -> Result<(), String> {
    let key = std::fs::canonicalize(path).unwrap_or_else(|_| path.to_path_buf());
    if !seen.insert(key) {
        return Ok(()); // a2lfile itself reports include cycles
    }
    // Unreadable files are left to a2lfile, which reports them with context.
    let Ok(bytes) = std::fs::read(path) else {
        return Ok(());
    };
    let text = std::str::from_utf8(&bytes).map_err(|error| {
        let offset = error.valid_up_to();
        let line = bytes[..offset].iter().filter(|&&b| b == b'\n').count() + 1;
        format!(
            "{}:{line}: invalid UTF-8 byte sequence at offset {offset}; A2L text must be UTF-8",
            path.display()
        )
    })?;
    for include in includes(text) {
        check_file(&resolve(&include, path), seen)?;
    }
    Ok(())
}

/// Same resolution rule as `a2lfile::loader::make_include_filename`.
fn resolve(incname: &str, base: &Path) -> PathBuf {
    let normalized = incname.replace(['\\', '/'], MAIN_SEPARATOR_STR);
    let inc = Path::new(&normalized);
    if inc.is_absolute() {
        return inc.to_path_buf();
    }
    if let Some(dir) = base.parent() {
        let joined = dir.join(inc);
        if joined.exists() {
            return joined;
        }
    }
    PathBuf::from(incname)
}

/// `/include` targets outside comments and string literals.
fn includes(text: &str) -> Vec<String> {
    let bytes = text.as_bytes();
    let mut found = Vec::new();
    let mut i = 0;
    let mut expect_target = false;
    while i < bytes.len() {
        let c = bytes[i];
        if c.is_ascii_whitespace() {
            i += 1;
        } else if bytes[i..].starts_with(b"/*") {
            i = text[i + 2..].find("*/").map_or(bytes.len(), |end| i + 2 + end + 2);
        } else if bytes[i..].starts_with(b"//") {
            i = text[i..].find('\n').map_or(bytes.len(), |end| i + end + 1);
        } else if c == b'"' {
            let start = i + 1;
            let mut j = start;
            while j < bytes.len() && bytes[j] != b'"' {
                j += if bytes[j] == b'\\' { 2 } else { 1 };
            }
            let end = j.min(bytes.len());
            if expect_target {
                found.push(text[start..end].to_string());
                expect_target = false;
            }
            i = end + 1;
        } else {
            let start = i;
            while i < bytes.len() && !bytes[i].is_ascii_whitespace() && bytes[i] != b'"' {
                if i > start && (bytes[i..].starts_with(b"/*") || bytes[i..].starts_with(b"//")) {
                    break;
                }
                i += 1;
            }
            let word = &text[start..i];
            if expect_target {
                found.push(word.to_string());
                expect_target = false;
            } else if word == "/include" {
                expect_target = true;
            }
        }
    }
    found
}

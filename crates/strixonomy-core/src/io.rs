use crate::error::{Result, StrixonomyError};
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

/// Read at most `max_bytes` from `path`, rejecting files larger than the cap.
pub fn read_file_capped(path: &Path, max_bytes: u64) -> Result<Vec<u8>> {
    let mut file = File::open(path).map_err(|e| StrixonomyError::Scanner(e.to_string()))?;
    let metadata = file.metadata().map_err(|e| StrixonomyError::Scanner(e.to_string()))?;
    if metadata.len() > max_bytes {
        return Err(StrixonomyError::Scanner(format!(
            "file exceeds size limit ({} bytes > {max_bytes}): {}",
            metadata.len(),
            path.display()
        )));
    }

    let mut buf = Vec::with_capacity(metadata.len() as usize);
    let mut chunk = [0u8; 8192];
    loop {
        let n = file.read(&mut chunk).map_err(|e| StrixonomyError::Scanner(e.to_string()))?;
        if n == 0 {
            break;
        }
        if buf.len() + n > max_bytes as usize {
            return Err(StrixonomyError::Scanner(format!(
                "file exceeds size limit (> {max_bytes} bytes): {}",
                path.display()
            )));
        }
        buf.extend_from_slice(&chunk[..n]);
    }
    Ok(buf)
}

/// UTF-8 decode a capped file read. Uses [`MAX_FILE_BYTES`] when `max_bytes` is not needed elsewhere.
pub fn read_to_string_capped(path: &Path, max_bytes: u64) -> Result<String> {
    let bytes = read_file_capped(path, max_bytes)?;
    String::from_utf8(bytes)
        .map_err(|e| StrixonomyError::Scanner(format!("invalid UTF-8 in {}: {e}", path.display())))
}

/// Atomically replace `path` with `contents`.
///
/// Writes to a sibling temp file, preserves the destination's permission bits when it
/// already exists (#422), then renames into place (with a Windows-safe replace fallback).
/// Best-effort removes the temp file on mid-write failure (#343).
///
/// ACL / extended attributes are not preserved (platform-specific; mode bits only).
pub fn atomic_write(path: &Path, contents: &str) -> std::io::Result<()> {
    let parent =
        path.parent().filter(|p| !p.as_os_str().is_empty()).unwrap_or_else(|| Path::new("."));
    fs::create_dir_all(parent)?;
    let nanos = SystemTime::now().duration_since(UNIX_EPOCH).map(|d| d.as_nanos()).unwrap_or(0);
    let stem = path.file_name().and_then(|s| s.to_str()).unwrap_or("file");
    let tmp_path = parent.join(format!(".strixonomy-{stem}-{nanos}.tmp"));
    // Best-effort remove temp on mid-write failure (#343).
    let write_result = (|| -> std::io::Result<()> {
        let mut file = fs::File::create(&tmp_path)?;
        file.write_all(contents.as_bytes())?;
        file.sync_all()?;
        // Re-read destination mode immediately before chmod so a file that
        // appears (or whose mode changes) during the temp write is still honored (#422).
        if let Ok(meta) = fs::metadata(path) {
            fs::set_permissions(&tmp_path, meta.permissions())?;
        }
        Ok(())
    })();
    if let Err(e) = write_result {
        let _ = fs::remove_file(&tmp_path);
        return Err(e);
    }
    replace_file(&tmp_path, path)
}

/// Replace `path` with `tmp_path` (tmp is consumed). Works on Windows where `rename` cannot
/// overwrite an existing destination.
pub fn replace_file(tmp_path: &Path, path: &Path) -> std::io::Result<()> {
    match fs::rename(tmp_path, path) {
        Ok(()) => Ok(()),
        Err(_) if path.exists() => {
            // Windows (and some network FS): rename refuses to replace. Move the existing
            // file aside, then rename; restore on failure.
            let bak_path = tmp_path.with_extension("bak");
            fs::rename(path, &bak_path)?;
            match fs::rename(tmp_path, path) {
                Ok(()) => {
                    let _ = fs::remove_file(&bak_path);
                    Ok(())
                }
                Err(rename_err) => {
                    let _ = fs::rename(&bak_path, path);
                    let _ = fs::remove_file(tmp_path);
                    Err(rename_err)
                }
            }
        }
        Err(e) => {
            let _ = fs::remove_file(tmp_path);
            Err(e)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::io::Write;
    #[cfg(unix)]
    use std::os::unix::fs::PermissionsExt;

    #[test]
    fn rejects_oversized_file_even_if_metadata_was_small() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("small.ttl");
        {
            let mut f = fs::File::create(&path).unwrap();
            writeln!(f, "@prefix ex: <http://ex/> .").unwrap();
        }
        let content = read_file_capped(&path, 64).unwrap();
        assert!(!content.is_empty());

        let over = dir.path().join("over.ttl");
        fs::write(&over, vec![b'x'; 20]).unwrap();
        let err = read_file_capped(&over, 10).unwrap_err().to_string();
        assert!(err.contains("size limit"));
    }

    #[test]
    fn atomic_write_overwrites_existing_file() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("onto.ttl");
        fs::write(&path, "old\n").unwrap();
        atomic_write(&path, "new\n").expect("atomic write");
        assert_eq!(fs::read_to_string(&path).unwrap(), "new\n");
        let leftovers: Vec<_> = fs::read_dir(dir.path())
            .unwrap()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_name().to_string_lossy().contains(".tmp"))
            .collect();
        assert!(leftovers.is_empty(), "temp files left behind: {leftovers:?}");
    }

    #[cfg(unix)]
    #[test]
    fn atomic_write_preserves_unix_mode() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("secret.ttl");
        fs::write(&path, "private\n").unwrap();
        let mut perms = fs::metadata(&path).unwrap().permissions();
        perms.set_mode(0o600);
        fs::set_permissions(&path, perms).unwrap();
        assert_eq!(fs::metadata(&path).unwrap().permissions().mode() & 0o777, 0o600);

        atomic_write(&path, "still private\n").expect("atomic write");
        assert_eq!(fs::read_to_string(&path).unwrap(), "still private\n");
        assert_eq!(
            fs::metadata(&path).unwrap().permissions().mode() & 0o777,
            0o600,
            "mode must survive atomic replace (#422)"
        );
    }

    #[test]
    fn replace_file_removes_tmp_when_rename_fails_and_dest_missing() {
        let dir = tempfile::tempdir().unwrap();
        let tmp = dir.path().join(".strixonomy-missing.tmp");
        let dest = dir.path().join("out.ttl");
        let err = replace_file(&tmp, &dest);
        assert!(err.is_err());
        assert!(!tmp.exists());
    }

    #[test]
    fn replace_file_restores_dest_and_cleans_tmp_when_second_rename_fails() {
        let dir = tempfile::tempdir().unwrap();
        let dest = dir.path().join("out.ttl");
        fs::write(&dest, "original\n").unwrap();
        let tmp = dir.path().join(".strixonomy-out.tmp");
        let err = replace_file(&tmp, &dest);
        assert!(err.is_err());
        assert_eq!(fs::read_to_string(&dest).unwrap(), "original\n");
        assert!(!tmp.exists());
        let bak = tmp.with_extension("bak");
        assert!(!bak.exists(), "backup must not be left behind");
    }
}

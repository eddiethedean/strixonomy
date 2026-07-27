use crate::error::{Result, StrixonomyError};
use std::fs::File;
use std::io::Read;
use std::path::Path;

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

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::io::Write;

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
}

//! Whole-file replacement shared by the engine and studio.

use std::fs::File;
use std::io::{self, Write};
use std::path::Path;

/// Write and sync a unique sibling, then atomically replace the destination.
///
/// Errors before replacement leave the original intact. A parent-directory sync
/// error happens after commit: the new bytes are visible but durability is not
/// confirmed. Directory syncing is unavailable on Windows.
pub fn replace_file_atomic(
    destination: &Path,
    write: impl FnOnce(&mut File) -> io::Result<()>,
) -> io::Result<()> {
    let parent = destination
        .parent()
        .filter(|p| !p.as_os_str().is_empty())
        .unwrap_or(Path::new("."));
    let mut temporary = tempfile::Builder::new()
        .prefix(".zigzag-")
        .tempfile_in(parent)?;
    write(temporary.as_file_mut())?;
    temporary.flush()?;
    temporary.as_file().sync_all()?;
    // PersistError owns the temporary: dropping it also cleans up a failed rename.
    temporary
        .persist(destination)
        .map_err(|error| error.error)?;
    #[cfg(unix)]
    match File::open(parent)?.sync_all() {
        Err(error)
            if matches!(
                error.kind(),
                io::ErrorKind::Unsupported | io::ErrorKind::InvalidInput
            ) => {}
        result => result?,
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::ensure;

    #[test]
    fn replacement_commits_complete_bytes_and_cleans_failures() -> anyhow::Result<()> {
        let dir = tempfile::tempdir()?;
        let target = dir.path().join("design.svg");
        std::fs::write(&target, b"original")?;
        let failed = replace_file_atomic(&target, |file| {
            file.write_all(b"partial")?;
            Err(io::Error::other("injected write failure"))
        });
        ensure!(failed.is_err(), "write failure was swallowed");
        ensure!(std::fs::read(&target)? == b"original", "original changed");
        ensure!(
            std::fs::read_dir(dir.path())?.count() == 1,
            "temporary leaked"
        );
        replace_file_atomic(&target, |file| file.write_all(b"complete replacement"))?;
        ensure!(
            std::fs::read(&target)? == b"complete replacement",
            "incomplete commit"
        );
        ensure!(
            std::fs::read_dir(dir.path())?.count() == 1,
            "temporary leaked"
        );
        let directory = dir.path().join("directory");
        std::fs::create_dir(&directory)?;
        std::fs::write(directory.join("keep"), b"original")?;
        ensure!(
            replace_file_atomic(&directory, |file| file.write_all(b"new")).is_err(),
            "replaced directory"
        );
        ensure!(
            std::fs::read(directory.join("keep"))? == b"original",
            "replacement failure changed target"
        );
        ensure!(
            std::fs::read_dir(dir.path())?.count() == 2,
            "temporary leaked"
        );
        Ok(())
    }
}

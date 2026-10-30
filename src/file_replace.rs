//! Whole-file replacement shared by the engine and studio.

use std::fs::File;
use std::io::{self, Write};
use std::path::Path;

/// Write and sync a unique sibling, then atomically replace the destination.
///
/// Errors before replacement leave the original intact. A parent-directory sync
/// error happens after commit: the new bytes are visible but durability is not
/// confirmed. Directory syncing is unavailable on Windows.
/// Existing symlinks are followed (dangling links fail); basic permissions are
/// copied. Replacement creates a new inode/file: ownership, ACLs, hard links,
/// extended metadata and unusual/network filesystem guarantees are not preserved
/// or qualified here.
pub fn replace_file_atomic(
    destination: &Path,
    write: impl FnOnce(&mut File) -> io::Result<()>,
) -> io::Result<()> {
    replace_with_sync(destination, write, sync_parent)
}

fn replace_with_sync(
    destination: &Path,
    write: impl FnOnce(&mut File) -> io::Result<()>,
    sync: impl FnOnce(&Path) -> io::Result<()>,
) -> io::Result<()> {
    // Preserve the old write-through-symlink behavior, rather than replacing
    // the link itself. A dangling link is rejected before any file is written.
    let resolved;
    let destination = match std::fs::symlink_metadata(destination) {
        Ok(metadata) if metadata.file_type().is_symlink() => {
            resolved = std::fs::canonicalize(destination)?;
            resolved.as_path()
        }
        Ok(_) => destination,
        Err(error) if error.kind() == io::ErrorKind::NotFound => destination,
        Err(error) => return Err(error),
    };
    let permissions = match std::fs::metadata(destination) {
        Ok(metadata) => Some(metadata.permissions()),
        Err(error) if error.kind() == io::ErrorKind::NotFound => None,
        Err(error) => return Err(error),
    };
    let parent = destination
        .parent()
        .filter(|p| !p.as_os_str().is_empty())
        .unwrap_or(Path::new("."));
    let mut temporary = tempfile::Builder::new()
        .prefix(".zigzag-")
        .tempfile_in(parent)?;
    write(temporary.as_file_mut())?;
    temporary.flush()?;
    if let Some(permissions) = permissions {
        temporary.as_file().set_permissions(permissions)?;
    }
    temporary.as_file().sync_all()?;
    // PersistError owns the temporary: dropping it also cleans up a failed rename.
    temporary
        .persist(destination)
        .map_err(|error| error.error)?;
    sync(parent).map_err(|error| io::Error::other(format!(
        "replacement committed, but parent directory sync failed; durability unconfirmed: {error}"
    )))?;
    Ok(())
}

fn sync_parent(parent: &Path) -> io::Result<()> {
    #[cfg(unix)]
    match File::open(parent)?.sync_all() {
        Err(error)
            if matches!(
                error.kind(),
                io::ErrorKind::Unsupported | io::ErrorKind::InvalidInput
            ) => {}
        result => result?,
    }
    #[cfg(not(unix))]
    let _ = parent;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::ensure;

    #[cfg(windows)]
    #[test]
    fn failed_windows_replacement_preserves_existing_file() -> anyhow::Result<()> {
        use std::os::windows::fs::OpenOptionsExt;
        let dir = tempfile::tempdir()?;
        let path = dir.path().join("locked.svg");
        std::fs::write(&path, b"original")?;
        let mut held = None;
        let result = replace_file_atomic(&path, |file| {
            file.write_all(b"complete new bytes")?;
            // Lock after preparation, so the commit itself fails.
            held = Some(
                std::fs::OpenOptions::new()
                    .read(true)
                    .share_mode(0)
                    .open(&path)?,
            );
            Ok(())
        });
        drop(held);
        ensure!(
            result.is_err(),
            "replacement of locked destination succeeded"
        );
        ensure!(
            std::fs::read(&path)? == b"original",
            "old destination changed"
        );
        ensure!(
            std::fs::read_dir(dir.path())?.count() == 1,
            "temporary leaked"
        );
        Ok(())
    }

    #[test]
    fn post_commit_failure_reports_visible_replacement() -> anyhow::Result<()> {
        let dir = tempfile::tempdir()?;
        let path = dir.path().join("design.svg");
        std::fs::write(&path, b"old")?;
        let result = replace_with_sync(
            &path,
            |file| file.write_all(b"new"),
            |_| Err(io::Error::other("injected sync failure")),
        );
        let error = result
            .err()
            .ok_or_else(|| anyhow::anyhow!("sync failure swallowed"))?;
        ensure!(
            error.to_string().contains("replacement committed"),
            "ambiguous outcome"
        );
        ensure!(std::fs::read(&path)? == b"new", "commit was not visible");
        ensure!(
            std::fs::read_dir(dir.path())?.count() == 1,
            "temporary leaked"
        );
        Ok(())
    }

    #[cfg(unix)]
    #[test]
    fn replacement_preserves_mode_and_symlink_target() -> anyhow::Result<()> {
        use std::os::unix::fs::{PermissionsExt, symlink};
        let dir = tempfile::tempdir()?;
        let path = dir.path().join("target.svg");
        let link = dir.path().join("link.svg");
        std::fs::write(&path, b"old")?;
        std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o640))?;
        symlink("target.svg", &link)?;
        replace_file_atomic(&link, |file| file.write_all(b"new"))?;
        ensure!(
            std::fs::symlink_metadata(&link)?.file_type().is_symlink(),
            "link replaced"
        );
        ensure!(std::fs::read(&path)? == b"new", "target not replaced");
        ensure!(
            std::fs::metadata(&path)?.permissions().mode() & 0o777 == 0o640,
            "mode changed"
        );
        Ok(())
    }

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

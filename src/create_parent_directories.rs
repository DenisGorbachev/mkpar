use core::ops::Not;
use std::{fs, io, path::Path};

/// Creates every missing parent directory for the provided path.
pub fn create_parent_directories(path: &impl AsRef<Path>) -> io::Result<()> {
    let path = path.as_ref();
    path.parent()
        .and_then(non_empty_path)
        .map_or(Ok(()), fs::create_dir_all)
}

fn non_empty_path(path: &Path) -> Option<&Path> {
    path.as_os_str().is_empty().not().then_some(path)
}

pub mod models;

use std::{
    fs, io,
    path::{Path, PathBuf},
};

fn copy_dir(src: impl AsRef<Path>, dest: impl AsRef<Path>) -> io::Result<()> {
    fs::create_dir_all(&dest)?;

    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;

        if ty.is_dir() {
            copy_dir(entry.path(), &dest.as_ref().join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), &dest.as_ref().join(entry.file_name()))?;
        }
    }

    Ok(())
}

// Guaranteed that these directories exist - if not, panicking is okay
pub fn build_md_files(content_dir: &str, dist_dir: &str) -> io::Result<()> {
    let target_dir = PathBuf::from(dist_dir).join("content");
    // Copy all files from content_dir to dist/content
    copy_dir(PathBuf::from(content_dir), target_dir)?;

    Ok(())
}

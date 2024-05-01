use std::io;

use markdown::build_md_files;

static CONTENT_DIR: &str = "content";
static DIST_DIR: &str = "dist";

fn main() -> io::Result<()> {
    build_md_files(CONTENT_DIR, DIST_DIR)?;
    Ok(())
}

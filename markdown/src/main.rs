use std::io;

use markdown::build_md_files;

static BLOG_DIR: &str = "content/blog";
static DIST_DIR: &str = "dist";

fn main() -> io::Result<()> {
    build_md_files(BLOG_DIR, DIST_DIR)?;
    Ok(())
}

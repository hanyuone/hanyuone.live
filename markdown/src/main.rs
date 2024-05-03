use std::io;

use markdown::build_md_files;

static BLOG_DIR: &str = "content/blog";
// Seems pretty hacky to build the MD files directly into the website folder
static TARGET_DIR: &str = "website/public/blog";

fn main() -> io::Result<()> {
    build_md_files(BLOG_DIR, TARGET_DIR)?;
    Ok(())
}

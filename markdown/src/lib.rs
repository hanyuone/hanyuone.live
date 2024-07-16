pub mod render;
pub mod structs;

use gray_matter::{engine::YAML, Matter, ParsedEntityStruct};
use render::to_bytestring;
use std::{
    fs, io,
    path::{Path, PathBuf},
    str::FromStr,
};
use structs::{
    blog::{BlogCard, BlogId},
    front_matter::FrontMatter,
};

struct BlogFile {
    id: BlogId,
    front_matter: FrontMatter,
    content: Vec<u8>,
}

fn create_blog_files(content_dir: &str) -> io::Result<Vec<BlogFile>> {
    let mut results = vec![];

    let matter = Matter::<YAML>::new();

    for entry in fs::read_dir(PathBuf::from(content_dir))? {
        let entry = entry?;

        let raw_content = fs::read_to_string(entry.path())?;

        let ParsedEntityStruct { data, content, .. } = matter
            .parse_with_struct::<FrontMatter>(&raw_content)
            .unwrap();

        let render_info = to_bytestring(&content);

        // TODO: add reading time from render_info to frontmatter

        let filename = entry
            .path()
            .file_stem()
            .expect("file name")
            .to_str()
            .expect("valid file name")
            .to_string();

        results.push(BlogFile {
            id: BlogId::from_str(&filename).expect("valid MD name"),
            front_matter: data,
            content: render_info.bytes,
        });
    }

    Ok(results)
}

fn write_blog_files(target_dir: impl AsRef<Path>, files: Vec<BlogFile>) -> io::Result<()> {
    // Create dist/public directory, for copying in frontmatter and MD files
    fs::create_dir_all(&target_dir)?;

    // Mapping between blog IDs and frontmatter
    let mut blog_cards = vec![];

    for file in files {
        // Insert frontmatter into mapping
        blog_cards.push(BlogCard {
            id: file.id,
            front_matter: file.front_matter,
        });

        // Write content to file
        let filename = target_dir.as_ref().join(file.id.to_string());

        fs::write(filename, file.content)?;
    }

    // Write list of blog cards to target dir
    let blog_cards_filename = target_dir.as_ref().join("blog_cards");
    let bytestring = postcard::to_stdvec(&blog_cards).expect("valid utf-8");
    fs::write(blog_cards_filename, bytestring)?;

    Ok(())
}

// Guaranteed that these directories exist - if not, panicking is okay
pub fn build_md_files(blog_dir: &str, target_dir: &str) -> io::Result<()> {
    let target_dir = PathBuf::from(target_dir);
    let files = create_blog_files(blog_dir)?;
    write_blog_files(target_dir, files)?;

    Ok(())
}

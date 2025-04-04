use gray_matter::{engine::YAML, Matter, ParsedEntityStruct};
use markdown::{
    structs::{
        blog::BlogId,
        metadata::{BlogMetadata, RawFrontMatter},
    },
    translate::{to_bytestring, TranslateOutputBytes},
};
use std::{
    collections::HashMap,
    fs, io,
    path::{Path, PathBuf},
    str::FromStr,
};

static BLOG_DIR: &str = "public/blog";
// Seems pretty hacky to build the MD files directly into the website folder
static TARGET_DIR: &str = "website/public/blog";

struct BlogFile {
    id: BlogId,
    metadata: BlogMetadata,
    content: String,
}

fn create_blog_files(content_dir: &str) -> io::Result<Vec<BlogFile>> {
    let mut results = vec![];

    let matter = Matter::<YAML>::new();

    for entry in fs::read_dir(PathBuf::from(content_dir))? {
        let entry = entry?;

        let raw_content = fs::read_to_string(entry.path())?;

        let ParsedEntityStruct {
            data: front_matter,
            content,
            ..
        } = matter
            .parse_with_struct::<RawFrontMatter>(&raw_content)
            .unwrap();

        let TranslateOutputBytes {
            bytes,
            post_translate,
        } = match to_bytestring(&content) {
            Ok(output) => output,
            Err(err) => panic!("{}", err),
        };

        let filename = entry
            .path()
            .file_stem()
            .expect("file name")
            .to_str()
            .expect("valid file name")
            .to_string();

        results.push(BlogFile {
            id: BlogId::from_str(&filename).expect("valid MD name"),
            metadata: BlogMetadata {
                front_matter: front_matter.into(),
                post_translate,
            },
            content: bytes,
        });
    }

    Ok(results)
}

fn write_blog_files(target_dir: impl AsRef<Path>, files: Vec<BlogFile>) -> io::Result<()> {
    // Create dist/public directory, for copying in frontmatter and MD files
    fs::create_dir_all(&target_dir)?;

    // Mapping between blog IDs and frontmatter
    let mut blog_map: HashMap<BlogId, BlogMetadata> = HashMap::new();

    for file in files {
        // Insert frontmatter into mapping
        blog_map.insert(file.id, file.metadata);

        // Write content to file
        let filename = format!("{}.ron", file.id);
        let file_path = target_dir.as_ref().join(filename);

        // Commenting this line stops the "double build" bug.
        // However, Trunk only builds *twice* no matter how many MD files
        // exist, and commenting the `blog_map` writing line still causes
        // Trunk to build twice.
        // Having only one MD file causes fs::write to only build once
        fs::write(file_path, file.content)?;
    }

    // Write list of blog cards to target dir
    let blog_map_filename = target_dir.as_ref().join("blog_map.ron");
    let bytestring = ron::to_string(&blog_map).expect("Written as bytes");

    fs::write(blog_map_filename, bytestring.clone())?;

    Ok(())
}

// Guaranteed that these directories exist - if not, panicking is okay
pub fn build_md_files(blog_dir: &str, target_dir: &str) -> io::Result<()> {
    let target_dir = PathBuf::from(target_dir);
    let files = create_blog_files(blog_dir)?;
    write_blog_files(target_dir, files)?;

    Ok(())
}

fn main() -> io::Result<()> {
    build_md_files(BLOG_DIR, TARGET_DIR)?;
    Ok(())
}

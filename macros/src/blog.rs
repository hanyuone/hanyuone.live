use std::{fs, io, path::PathBuf};

use proc_macro::TokenStream;
use quote::{quote, TokenStreamExt};
use syn::{
    parse::{Parse, ParseStream},
    parse_str, Ident, LitStr,
};

use crate::util::to_title_case;

#[derive(Default)]
struct BlogGenerator {
    enum_items: proc_macro2::TokenStream,
    display: proc_macro2::TokenStream,
    from_str: proc_macro2::TokenStream,
}

impl BlogGenerator {
    pub fn add_blog(&mut self, blog_name: String) {
        let enum_name = to_title_case(&blog_name);
        let enum_ident = parse_str::<Ident>(&enum_name).expect("enum name");

        self.enum_items.append_all(quote! {
            #enum_ident,
        });

        self.display.append_all(quote! {
            Self::#enum_ident => #blog_name,
        });

        self.from_str.append_all(quote! {
            #blog_name => Ok(Self::#enum_ident),
        });
    }
}

pub struct BlogDirInput {
    directory: LitStr,
}

impl Parse for BlogDirInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            directory: input.parse()?,
        })
    }
}

/// Loads all blogs as a list of strings.
fn load_blogs(dir: &str) -> io::Result<Vec<String>> {
    let mut blog_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    blog_dir.pop();
    blog_dir.push(dir);

    let mut blog_names = Vec::new();

    for entry in fs::read_dir(blog_dir)? {
        let entry = entry?;
        blog_names.push(
            entry
                .path()
                .file_stem()
                .expect("file name")
                .to_str()
                .expect("valid file name")
                .to_string(),
        );
    }

    Ok(blog_names)
}

/// Generates the `BlogId` enum dynamically, given a list of blogs.
///
/// `BlogId` serves as a "bridge" between Yew's routing and the (dynamic) list
/// of blogs themselves - it contains all blogs as individual enum members that
/// can be enumerated into proper routes.
///
/// # Errors
///
/// This function will return an error if the directory specified in `input`
/// does not exist.
pub fn generate_blog_enum(input: BlogDirInput) -> io::Result<TokenStream> {
    let BlogGenerator {
        enum_items,
        display,
        from_str,
    } = {
        let mut generator = BlogGenerator::default();

        for blog_name in load_blogs(&input.directory.value())? {
            generator.add_blog(blog_name);
        }

        generator
    };

    Ok(TokenStream::from(quote! {
        #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize, enum_iterator::Sequence)]
        pub enum BlogId {
            #enum_items
        }

        impl std::fmt::Display for BlogId {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(
                    f,
                    "{}",
                    match self {
                        #display
                    }
                )
            }
        }

        impl std::str::FromStr for BlogId {
            type Err = String;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                match s {
                    #from_str
                    _ => Err(format!("Unknown document '{}'", s)),
                }
            }
        }
    }))
}

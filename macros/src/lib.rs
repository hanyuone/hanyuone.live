mod generator;

use std::{fs, io, path::PathBuf};

use generator::Generator;
use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input, LitStr,
};

struct BlogModelsInput {
    directory: LitStr,
}

impl Parse for BlogModelsInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            directory: input.parse()?,
        })
    }
}

/// Loads all blogs as a list of strings.
fn load_blogs(dir: &str) -> Result<Vec<String>, io::Error> {
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
fn generate(input: BlogModelsInput) -> Result<TokenStream, io::Error> {
    let Generator {
        enumerators,
        display,
        from_str,
    } = {
        let mut generator = Generator::default();

        for blog_name in load_blogs(&input.directory.value())? {
            generator.add_blog(blog_name);
        }

        generator
    };

    Ok(TokenStream::from(quote! {
        #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, enum_iterator::Sequence)]
        #[archive(check_bytes)]
        pub enum BlogId {
            #enumerators
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

#[proc_macro]
pub fn generate_blog_id(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as BlogModelsInput);
    generate(input).expect("Generating blog models")
}

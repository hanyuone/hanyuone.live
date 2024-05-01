use std::{fs, io, path::PathBuf};

use proc_macro::TokenStream;
use quote::{quote, TokenStreamExt};
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input, parse_str, Ident, LitStr,
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

#[derive(Default)]
struct Generator {
    enumerators: proc_macro2::TokenStream,
    display: proc_macro2::TokenStream,
    from_str: proc_macro2::TokenStream,
}

// Loads all blogs as a list of strings.
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

fn slug_constr(slug: &str) -> String {
    slug.split('-').map(titlecase).collect::<Vec<_>>().join("")
}

fn titlecase(word: &str) -> String {
    let mut chars = word.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
    }
}

fn generate_blog(generator: &mut Generator, blog_name: String) {
    let enum_name = slug_constr(&blog_name);
    let enum_ident = parse_str::<Ident>(&enum_name).expect("enum name");

    generator.enumerators.append_all(quote! {
        #enum_ident,
    });

    generator.display.append_all(quote! {
        Self::#enum_ident => #blog_name,
    });

    generator.from_str.append_all(quote! {
        #blog_name => Ok(Self::#enum_ident),
    });
}

fn generate(input: BlogModelsInput) -> Result<TokenStream, io::Error> {
    let Generator {
        enumerators,
        display,
        from_str,
    } = {
        let mut generator = Generator::default();

        for blog_name in load_blogs(&input.directory.value())? {
            generate_blog(&mut generator, blog_name);
        }

        generator
    };

    Ok(TokenStream::from(quote! {
        #[derive(Copy, Clone, PartialEq, enum_iterator::Sequence)]
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
pub fn generate_blog_models(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as BlogModelsInput);
    generate(input).expect("Generating blog models")
}

use std::{fs, path::PathBuf};

use proc_macro::TokenStream;
use quote::{quote, TokenStreamExt};
use serde::Deserialize;
use syn::{
    parse::{Parse, ParseStream},
    parse_str, Ident, LitStr,
};

use crate::util::to_title_case;

#[derive(Deserialize)]
struct YamlTag {
    name: String,
    colour: String,
    description: String,
}

#[derive(Default)]
struct TagGenerator {
    enum_items: proc_macro2::TokenStream,
    display: proc_macro2::TokenStream,
    from_str: proc_macro2::TokenStream,
    from_tag_id: proc_macro2::TokenStream,
}

impl TagGenerator {
    fn add_tag(&mut self, tag: YamlTag) {
        let YamlTag { name, colour, description } = tag;
        let description = description.split("\n").collect::<Vec<_>>().join(" ");

        let enum_name = to_title_case(&name);
        let enum_ident = parse_str::<Ident>(&enum_name).expect("enum name");

        self.enum_items.append_all(quote! {
            #enum_ident,
        });

        self.display.append_all(quote! {
            Self::#enum_ident => #name,
        });

        self.from_str.append_all(quote! {
            #name => Ok(Self::#enum_ident),
        });

        self.from_tag_id.append_all(quote! {
            TagId::#enum_ident => Self {
                colour: #colour.to_string(),
                description: #description.to_string(),
            },
        });
    }
}

pub struct TagInput {
    filename: LitStr,
}

impl Parse for TagInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            filename: input.parse()?,
        })
    }
}

fn load_tags(yaml: &str) -> Vec<YamlTag> {
    serde_yml::from_str::<Vec<YamlTag>>(yaml).expect("tags.yaml properly formatted")
}

pub fn generate_tag_enum(input: TagInput) -> TokenStream {
    let mut tag_location = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    tag_location.pop();
    tag_location.push(input.filename.value());

    let TagGenerator {
        enum_items,
        display,
        from_str,
        from_tag_id,
    } = {
        let mut generator = TagGenerator::default();

        let text = fs::read_to_string(tag_location).expect("file exists");

        for tag in load_tags(&text) {
            generator.add_tag(tag);
        }

        generator
    };

    TokenStream::from(quote! {
        #[derive(Clone, PartialEq, enum_iterator::Sequence)]
        pub enum TagId {
            #enum_items
        }

        impl std::fmt::Display for TagId {
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

        impl std::str::FromStr for TagId {
            type Err = String;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                match s {
                    #from_str
                    _ => Err(format!("Unknown tag '{}'", s)),
                }
            }
        }

        impl From<TagId> for TagMetadata {
            fn from(value: TagId) -> Self {
                match value {
                    #from_tag_id
                }
            }
        }
    })
}

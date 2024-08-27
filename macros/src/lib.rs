mod blog;
mod tag;
mod util;

use blog::{generate_blog_enum, BlogDirInput};
use proc_macro::TokenStream;
use syn::parse_macro_input;
use tag::{generate_tag_enum, TagInput};

#[proc_macro]
pub fn generate_blog_id(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as BlogDirInput);
    generate_blog_enum(input).expect("Generating blog models")
}

#[proc_macro]
pub fn generate_tag_id(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as TagInput);
    generate_tag_enum(input)
}

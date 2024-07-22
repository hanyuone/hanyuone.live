use quote::{quote, TokenStreamExt};
use syn::{parse_str, Ident};

fn capitalise(word: &str) -> String {
    let mut chars = word.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
    }
}

fn to_title_case(kebab_case: &str) -> String {
    kebab_case
        .split('-')
        .map(capitalise)
        .collect::<Vec<_>>()
        .join("")
}

#[derive(Default)]
pub struct Generator {
    pub enumerators: proc_macro2::TokenStream,
    pub display: proc_macro2::TokenStream,
    pub from_str: proc_macro2::TokenStream,
}

impl Generator {
    pub fn add_blog(&mut self, blog_name: String) {
        let enum_name = to_title_case(&blog_name);
        let enum_ident = parse_str::<Ident>(&enum_name).expect("enum name");

        self.enumerators.append_all(quote! {
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

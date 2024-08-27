pub fn capitalise(word: &str) -> String {
    let mut chars = word.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
    }
}

pub fn to_title_case(snake_case: &str) -> String {
    snake_case
        .split('_')
        .map(capitalise)
        .collect::<Vec<_>>()
        .join("")
}

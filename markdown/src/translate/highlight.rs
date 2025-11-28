use std::sync::LazyLock;

use syntect::{highlighting::ThemeSet, parsing::SyntaxSet};

pub static SYNTAX_SET: LazyLock<SyntaxSet> = LazyLock::new(SyntaxSet::load_defaults_newlines);
pub static THEME_SET: LazyLock<ThemeSet> = LazyLock::new(ThemeSet::load_defaults);

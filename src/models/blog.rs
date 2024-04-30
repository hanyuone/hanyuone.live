use std::{fmt::Display, str::FromStr};

use enum_iterator::Sequence;

// PLAN: Have two ways of fetching/rendering documents:
// - For CSR (local), fetch Markdown files using HTTP
// - For SSR (deployed), fetch Markdown files locally

// Steps:
// 1. Read Markdown files as raw strings (HTTP or local), and have website display
// blogs with pre-rendered MD
// 2. Build MD files, *making sure* not to include the rendered output in the codebase
//   - CSR: build MD, fetch build files with HTTP, have code to "decode"
//     compressed simplified virtual DOM
//   - SSR: build MD, fetch build files using macros

// TODO: generate BlogId using metaprogramming
#[derive(Copy, Clone, PartialEq, Sequence)]
pub enum BlogId {
    HelloWorld,
    SecondBlog,
}

impl Display for BlogId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                BlogId::HelloWorld => "hello-world",
                BlogId::SecondBlog => "second-blog",
            }
        )
    }
}

impl FromStr for BlogId {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "hello-world" => Ok(BlogId::HelloWorld),
            "second-blog" => Ok(BlogId::SecondBlog),
            _ => Err(format!("Blog does not exist: {}", s)),
        }
    }
}

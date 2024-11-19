use std::fmt::Display;

use clap::{Args, ValueEnum};

use crate::commands::version::npm_get_version;

#[derive(Args, Debug)]
pub(crate) struct EcosystemArgs {
    #[clap(long)]
    pub(crate) ecosystem: Option<Ecosystem>,
}

#[derive(Debug, Copy, Clone, ValueEnum)]
pub(crate) enum Ecosystem {
    #[clap(name = "javascript")]
    JavaScript,
    Rust,
}

impl Display for Ecosystem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Ecosystem::JavaScript => "javascript",
                Ecosystem::Rust => "rust",
            }
        )
    }
}

impl Default for Ecosystem {
    fn default() -> Self {
        if npm_get_version().is_ok() {
            Self::JavaScript
        } else {
            Self::Rust
        }
    }
}

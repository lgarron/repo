use std::fmt::Display;

use clap::{Args, ValueEnum};

#[derive(Args, Debug)]
pub(crate) struct EcosystemArgs {
    #[clap(long)]
    pub(crate) ecosystem: Option<Ecosystem>,
}

#[derive(Debug, Copy, Clone, ValueEnum, PartialEq, Eq)]
pub(crate) enum Ecosystem {
    #[clap(name = "javascript")]
    JavaScript,
    Rust,
    // Python,
}

impl Display for Ecosystem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Ecosystem::JavaScript => "javascript",
                Ecosystem::Rust => "rust",
                // Ecosystem::Python => "python",
            }
        )
    }
}

use std::fmt::Display;

use clap::{Args, ValueEnum};

use crate::version::npm_get_version;

#[derive(Args, Debug)]
pub(crate) struct EcosystemArgs {
    #[clap(long)]
    pub(crate) ecosystem: Option<Ecosystem>,
}

#[derive(Debug, Clone, ValueEnum)]
pub(crate) enum Ecosystem {
    Npm,
    Cargo,
}

impl Display for Ecosystem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Ecosystem::Npm => "npm",
                Ecosystem::Cargo => "cargo",
            }
        )
    }
}

impl Default for Ecosystem {
    fn default() -> Self {
        if npm_get_version().is_ok() {
            Self::Npm
        } else {
            Self::Cargo
        }
    }
}

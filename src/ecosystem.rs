use std::fmt::Display;

use clap::{Args, ValueEnum};

#[derive(Args, Debug)]
pub(crate) struct EcosystemArgs {
    // TODO: flatten?
    #[clap(long, default_value = "auto")]
    pub(crate) ecosystem: RepoEcosystem,
}

#[derive(Debug, Clone, ValueEnum)]
pub(crate) enum RepoEcosystem {
    Auto,
    Npm,
    Cargo,
}

impl Display for RepoEcosystem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                RepoEcosystem::Auto => "auto",
                RepoEcosystem::Npm => "npm",
                RepoEcosystem::Cargo => "cargo",
            }
        )
    }
}

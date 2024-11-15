use std::fmt::Display;
use std::process::Command;

use clap::{Args, Subcommand};

use cargo_metadata::MetadataCommand;

#[derive(Args, Debug)]
pub(crate) struct VersionArgs {
    #[command(subcommand)]
    pub command: VersionCommand,
}

#[derive(Debug, Subcommand)]
pub(crate) enum VersionCommand {
    /// Get the current version
    Get(VersionGetArgs),
    /// Bump the current version
    Bump(VersionBumpArgs),
}

#[derive(Args, Debug)]
pub(crate) struct VersionGetArgs {
    /// Do not print the `v` prefix (e.g. print `0.1.3` instead of `v0.1.3`)
    #[clap(long)]
    pub no_prefix: bool,
}

#[derive(Args, Debug)]
pub(crate) struct VersionBumpArgs {
    #[command(subcommand)]
    pub command: VersionBumpCommand,
}

#[derive(Debug, Subcommand)]
pub(crate) enum VersionBumpCommand {
    Major,
    Minor,
    Patch,
}

impl Display for VersionBumpCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                VersionBumpCommand::Major => "major",
                VersionBumpCommand::Minor => "minor",
                VersionBumpCommand::Patch => "patch",
            }
        )
    }
}

pub(crate) fn version_command(version_args: VersionArgs) {
    match version_args.command {
        VersionCommand::Get(version_get_args) => {
            let prefix = if version_get_args.no_prefix { "" } else { "v" };
            println!(
                "{}{}",
                prefix,
                MetadataCommand::new()
                    .manifest_path("./Cargo.toml")
                    .current_dir(".")
                    .exec()
                    .unwrap()
                    .root_package()
                    .unwrap()
                    .version
            );
        }
        VersionCommand::Bump(version_bump_args) => {
            println!("Assuming `cargo-bump` is installed…");
            Command::new("cargo")
                .args(["bump", &version_bump_args.command.to_string()])
                .status()
                .expect("Could not bump version");
        }
    }
}

use std::fmt::Display;
use std::fs::File;
use std::process::Command;

use clap::{Args, Subcommand};

use cargo_metadata::MetadataCommand;
use serde::Deserialize;

use crate::common::{
    ecosystem::{Ecosystem, EcosystemArgs},
    package_manager::PACKAGE_JSON_PATH,
};

#[derive(Args, Debug)]
pub(crate) struct VersionArgs {
    #[command(subcommand)]
    command: VersionCommand,
    #[command(flatten)]
    ecosystem_args: EcosystemArgs,
}

#[derive(Debug, Subcommand)]
enum VersionCommand {
    /// Get the current version
    Get(VersionGetArgs),
    /// Bump the current version
    Bump(VersionBumpArgs),
}

#[derive(Args, Debug)]
struct VersionGetArgs {
    /// Do not print the `v` prefix (e.g. print `0.1.3` instead of `v0.1.3`)
    #[clap(long)]
    pub no_prefix: bool,
}

#[derive(Args, Debug)]
struct VersionBumpArgs {
    #[command(subcommand)]
    pub command: VersionBumpCommand,
}

#[derive(Debug, Subcommand)]
enum VersionBumpCommand {
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

#[derive(Deserialize)]
struct PackageJSONWithVersion {
    version: String,
}

pub(crate) fn npm_get_version() -> Result<String, String> {
    // TODO: semantically parse version
    let Ok(file) = File::open(PACKAGE_JSON_PATH) else {
        return Err("Could not read `package.json`".to_owned());
    };
    let Ok(package_json) = serde_json::from_reader::<_, PackageJSONWithVersion>(file) else {
        return Err("Could not read `package.json`".to_owned());
    };
    Ok(package_json.version)
}

fn cargo_get_version() -> String {
    MetadataCommand::new()
        .manifest_path("./Cargo.toml")
        .current_dir(".")
        .exec()
        .unwrap()
        .root_package()
        .unwrap()
        .version
        .to_string()
}

fn print_version(version: &str, version_get_args: &VersionGetArgs) {
    let prefix = if version_get_args.no_prefix { "" } else { "v" };
    println!("{}{}", prefix, version);
}

fn npm_bump_version(version_bump_command: VersionBumpCommand) {
    Command::new("npm")
        .args([
            "version",
            "--no-git-tag-version",
            &version_bump_command.to_string(),
        ])
        .status()
        .expect("Could not bump version using `npm`");
}

fn cargo_bump_version(version_bump_command: VersionBumpCommand) {
    println!("Assuming `cargo-bump` is installed…");
    Command::new("cargo")
        .args(["bump", &version_bump_command.to_string()])
        .status()
        .expect("Could not bump version using `cargo-bump`");
}

fn version_get_and_print(ecosystem_args: &EcosystemArgs, version_get_args: VersionGetArgs) {
    let version: String = match ecosystem_args.ecosystem {
        None => match npm_get_version() {
            Ok(version) => version,
            Err(_) => cargo_get_version(),
        },
        Some(Ecosystem::Npm) => npm_get_version().expect("Could not get `npm` package version."),
        Some(Ecosystem::Cargo) => cargo_get_version(),
    };
    print_version(&version, &version_get_args);
}

// TODO: get version from output of the bump commands themselves?
fn version_bump(ecosystem_args: &EcosystemArgs, version_bump_args: VersionBumpArgs) {
    let auto_print_version = |repo_ecosystem: Ecosystem| {
        println!("Bumped version using ecosystem: {}", repo_ecosystem);
        print!("Bumped to version: ");
        version_get_and_print(
            ecosystem_args,
            VersionGetArgs {
                no_prefix: false, // TODO
            },
        )
    };
    match ecosystem_args.ecosystem {
        None => {
            if npm_get_version().is_ok() {
                npm_bump_version(version_bump_args.command);
                auto_print_version(Ecosystem::Npm);
            } else {
                cargo_bump_version(version_bump_args.command);
                auto_print_version(Ecosystem::Cargo);
            }
        }
        Some(Ecosystem::Npm) => {
            npm_bump_version(version_bump_args.command);
            auto_print_version(Ecosystem::Npm);
        }
        Some(Ecosystem::Cargo) => {
            cargo_bump_version(version_bump_args.command);
            auto_print_version(Ecosystem::Cargo);
        }
    }
}

// TODO: use traits to abstract across ecosystems
// TODO: support cross-checking versions across ecosystems
pub(crate) fn version_command(version_args: VersionArgs) {
    match version_args.command {
        VersionCommand::Get(version_get_args) => {
            version_get_and_print(&version_args.ecosystem_args, version_get_args);
        }
        VersionCommand::Bump(version_bump_args) => {
            version_bump(&version_args.ecosystem_args, version_bump_args);
        }
    };
}

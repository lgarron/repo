use std::env::current_dir;
use std::fs::File;
use std::process::Command;
use std::{fmt::Display, process::exit};

use cargo_metadata::semver::Prerelease;
use clap::{Args, Subcommand};

use cargo_metadata::{semver::Version, MetadataCommand};
use serde::Deserialize;

use crate::common::commit_wrapped_operation::CommitWrappedOperation;
use crate::common::vcs::{auto_detect_preferred_vcs_and_repo_root_for_ecosystem, VcsKind};
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
    /// Set the current version
    Set(VersionSetArgs),
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
struct VersionSetArgs {
    #[clap()]
    pub version: String,
    #[command(flatten)]
    commit_args: CommitArgs,
}

#[derive(Args, Debug)]
struct VersionBumpArgs {
    #[command(subcommand)]
    pub command: VersionBumpMagnitude,
    #[command(flatten)]
    commit_args: CommitArgs,
}

#[derive(Debug, Subcommand, PartialEq, Eq, Clone)]
enum VersionBumpMagnitude {
    Major,
    Minor,
    Patch,
    Dev,
}

impl Display for VersionBumpMagnitude {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                VersionBumpMagnitude::Major => "major",
                VersionBumpMagnitude::Minor => "minor",
                VersionBumpMagnitude::Patch => "patch",
                VersionBumpMagnitude::Dev => "dev",
            }
        )
    }
}

#[derive(Args, Debug)]
pub(crate) struct CommitArgs {
    #[clap(long)]
    pub commit: bool,
    #[clap(long)]
    pub commit_using: Option<VcsKind>,
}

impl CommitArgs {
    pub fn vcs(&self) -> Result<VcsKind, String> {
        Ok(match &self.commit_using {
            Some(vcs_kind) => vcs_kind.clone(),
            None => {
                let Some((vcs_kind, _)) =
                    auto_detect_preferred_vcs_and_repo_root_for_ecosystem(&current_dir().unwrap())
                else {
                    return Err("No VCS specified or found.".to_owned());
                };
                vcs_kind
            }
        })
    }
}

#[derive(Deserialize)]
struct PackageJSONWithVersion {
    version: Option<String>,
}

pub(crate) fn npm_get_version() -> Result<String, String> {
    // TODO: semantically parse version
    let Ok(file) = File::open(PACKAGE_JSON_PATH) else {
        return Err("Could not file `package.json`".to_owned());
    };
    let Ok(package_json) = serde_json::from_reader::<_, PackageJSONWithVersion>(file) else {
        return Err("Could not read `package.json`".to_owned());
    };
    match package_json.version {
        Some(version) => Ok(match version.strip_prefix("v") {
            Some(version) => version.to_owned(),
            None => version,
        }),
        None => Err("No version field found in `package.json`".to_owned()),
    }
}

pub(crate) fn cargo_get_version() -> Result<String, String> {
    let mut command = MetadataCommand::new();
    let Ok(metadata) = command
        .manifest_path("./Cargo.toml")
        .current_dir(".")
        .exec()
    else {
        return Err("Could not file `Cargo.toml`".to_owned());
    };
    let Some(root_package) = metadata.root_package() else {
        return Err("Could not file `Cargo.toml` root package.".to_owned());
    };
    Ok(root_package.version.to_string())
}

fn print_version(version: &str, version_get_args: &VersionGetArgs) {
    let prefix = if version_get_args.no_prefix { "" } else { "v" };
    print!("{}{}", prefix, version);
}

fn dev_bump(version: Version) -> Version {
    let mut version = version.clone();
    version.patch += 1;
    version.pre = Prerelease::new("dev").unwrap();
    version
}

fn npm_bump_version(version_bump_command: VersionBumpMagnitude) {
    if version_bump_command == VersionBumpMagnitude::Dev {
        let version = npm_get_version().expect("Could not get current version.");
        let version = Version::parse(&version).expect("Could not parse current version.");
        npm_set_version(dev_bump(version));
        return;
    }
    Command::new("npm")
        .args([
            "version",
            "--no-git-tag-version",
            &version_bump_command.to_string(),
        ])
        .status()
        .expect("Could not bump version using `npm`");
}

fn cargo_bump_version(version_bump_command: VersionBumpMagnitude) {
    if version_bump_command == VersionBumpMagnitude::Dev {
        let version = cargo_get_version().expect("Could not get current version.");
        let version = Version::parse(&version).expect("Could not parse current version.");
        cargo_set_version(dev_bump(version));
        return;
    }
    eprintln!("Assuming `cargo-bump` is installed…");
    Command::new("cargo")
        .args(["bump", &version_bump_command.to_string()])
        .status()
        .expect("Could not bump version using `cargo-bump`");
}

pub(crate) fn detect_ecosystem_by_getting_version(
    ecosystem_args: &EcosystemArgs,
) -> Option<(Ecosystem, String)> {
    for (ecosystem, get_version) in [
        (
            Ecosystem::JavaScript,
            npm_get_version as fn() -> Result<String, String>,
        ),
        (
            Ecosystem::Rust,
            cargo_get_version as fn() -> Result<String, String>,
        ),
    ] {
        if let Some(required_ecosystem) = ecosystem_args.ecosystem {
            if required_ecosystem != ecosystem {
                // TODO: make this neater
                continue;
            }
        }
        if let Ok(version) = get_version() {
            return Some((ecosystem, version));
        }
    }
    None
}

pub(crate) fn must_detect_ecosystem_by_getting_version(
    ecosystem_args: &EcosystemArgs,
) -> (Ecosystem, String) {
    detect_ecosystem_by_getting_version(ecosystem_args)
        .expect("Could not detect an ecosystem for this repo.")
}

fn version_get_and_print(ecosystem_args: &EcosystemArgs, version_get_args: &VersionGetArgs) {
    let Some((_, version)) = detect_ecosystem_by_getting_version(ecosystem_args) else {
        eprintln!("No version found.");
        exit(1);
    };
    print_version(&version, version_get_args);
}

// TODO: get version from output of the bump commands themselves?
// TODO: return `Result<Version, …>`.
fn version_bump(
    ecosystem_args: &EcosystemArgs,
    version_bump_magniture: VersionBumpMagnitude,
) -> Result<String, String> {
    let auto_print_version = |repo_ecosystem: Ecosystem| {
        eprintln!("Bumped version using ecosystem: {}", repo_ecosystem);
    };
    match must_detect_ecosystem_by_getting_version(ecosystem_args) {
        (Ecosystem::JavaScript, _) => {
            npm_bump_version(version_bump_magniture);
            auto_print_version(Ecosystem::JavaScript);
            npm_get_version()
        }
        (Ecosystem::Rust, _) => {
            cargo_bump_version(version_bump_magniture);
            auto_print_version(Ecosystem::Rust);
            cargo_get_version()
        }
    }
}

fn npm_set_version(version: Version) {
    Command::new("npm")
        .args(["version", "--no-git-tag-version", &version.to_string()])
        .status()
        .expect("Could not bump version using `npm`");
}

fn cargo_set_version(version: Version) {
    Command::new("cargo")
        .args(["bump", &version.to_string()])
        .status()
        .expect("Could not bump version using `npm`");
}

fn version_set(ecosystem_args: &EcosystemArgs, version: Version) {
    eprintln!("Setting version to: v{}", version);

    match must_detect_ecosystem_by_getting_version(ecosystem_args) {
        (Ecosystem::JavaScript, _) => {
            npm_set_version(version);
        }
        (Ecosystem::Rust, _) => {
            cargo_set_version(version);
        }
    }
}

// TODO: use traits to abstract across ecosystems
// TODO: support cross-checking versions across ecosystems
pub(crate) fn version_command(version_args: VersionArgs) {
    match version_args.command {
        VersionCommand::Get(version_get_args) => {
            version_get_and_print(&version_args.ecosystem_args, &version_get_args);
        }
        VersionCommand::Set(version_set_args) => {
            let commit_wrapped_operation =
                CommitWrappedOperation::try_from(version_set_args.commit_args).unwrap();
            commit_wrapped_operation
                .perform_operation(&|| {
                    let version = version_set_args
                        .version
                        .strip_prefix("v")
                        .unwrap_or(&version_set_args.version);
                    let version = Version::parse(version).expect("Invalid version specified");
                    version_set(&version_args.ecosystem_args, version.clone());
                    Ok(format!("Set version to: `v{}`", version))
                })
                .unwrap();
        }
        VersionCommand::Bump(version_bump_args) => {
            let commit_wrapped_operation =
                CommitWrappedOperation::try_from(version_bump_args.commit_args).unwrap();
            commit_wrapped_operation
                .perform_operation(&|| {
                    let version_bump_magnitude: &VersionBumpMagnitude = &version_bump_args.command;
                    let new_version =
                        version_bump(&version_args.ecosystem_args, version_bump_magnitude.clone())
                            .unwrap();

                    Ok(format!(
                        "Bump to next {} version: `v{}`",
                        version_bump_magnitude, new_version
                    ))
                })
                .unwrap();
        }
    };
}

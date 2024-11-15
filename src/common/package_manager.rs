use std::{fmt::Display, fs::exists};

use clap::{Args, ValueEnum};

use self::ecosystem::Ecosystem;

use super::ecosystem;

#[derive(Args, Debug)]
pub(crate) struct PackageManagerArgs {
    #[clap(long)]
    pub(crate) package_manager: Option<PackageManager>,
}

#[derive(Debug, Clone, ValueEnum)]
pub(crate) enum PackageManager {
    Npm,
    Bun,
    Yarn,
    Pnpm,
    Cargo,
}

impl Display for PackageManager {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Npm => "npm",
                Self::Bun => "bun",
                Self::Yarn => "yarn",
                Self::Pnpm => "pnpm",
                Self::Cargo => "cargo",
            }
        )
    }
}

pub(crate) const PACKAGE_JSON_PATH: &str = "./package.json";
pub(crate) const PACKAGE_LOCK_JSON_PATH: &str = "./package-lock.json";
pub(crate) const YARN_LOCK_PATH: &str = "./yarn.lock";
pub(crate) const YARN_PNPM_LOCK_YAML_PATH: &str = "./pnpm-lock.yaml";
const BUN_LOCKB_PATH: &str = "./bun.lockb";
const CARGO_TOML: &str = "./Cargo.toml";

impl PackageManager {
    pub(crate) fn preferred_detected_package_manager_for_ecosystem(
        ecosystem: Ecosystem,
    ) -> Option<Self> {
        match ecosystem {
            Ecosystem::JavaScript => {
                if exists(BUN_LOCKB_PATH).unwrap() {
                    Some(Self::Bun)
                } else if exists(YARN_LOCK_PATH).unwrap() {
                    Some(Self::Yarn)
                } else if exists(YARN_PNPM_LOCK_YAML_PATH).unwrap() {
                    Some(Self::Pnpm)
                } else if exists(PACKAGE_JSON_PATH).unwrap() {
                    Some(Self::Npm)
                } else {
                    None
                }
            }
            Ecosystem::Rust => {
                if exists(CARGO_TOML).unwrap() {
                    Some(Self::Cargo)
                } else {
                    None
                }
            }
        }
    }
}

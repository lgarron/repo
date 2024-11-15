use std::{fs::exists, process::Command};

use clap::{Args, Subcommand};

use crate::common::{
    ecosystem::Ecosystem,
    package_manager::{PackageManager, PackageManagerArgs, PACKAGE_LOCK_JSON_PATH},
};

#[derive(Args, Debug)]
pub(crate) struct SetupArgs {
    #[command(subcommand)]
    command: SetupCommand,
    #[command(flatten)]
    package_manager_args: PackageManagerArgs,
}

#[derive(Debug, Subcommand)]
enum SetupCommand {
    /// Install dependencies
    Dependencies,
}

// TODO: skip empty deps?
fn npm_install() {
    println!("Installing dependencies using: `npm`");
    let install_commmand = match exists(PACKAGE_LOCK_JSON_PATH).unwrap() {
        true => "ci",
        false => {
            println!(
                "Using `npm install` instead of `npm ci` because the lockfile was not found at: {}\nThis may create a new lockfile.",
                PACKAGE_LOCK_JSON_PATH
            );
            "install"
        }
    };
    Command::new("npm")
        .args([install_commmand])
        .status()
        .expect("Could not install dependencies using `npm`");
}

fn bun_install() {
    println!("Installing dependencies using: `bun`");
    Command::new("bun")
        .args(["install", "--no-save"])
        .status()
        .expect("Could not install dependencies using `bun`");
}

fn yarn_install() {
    println!("Installing dependencies using: `npx yarn`");
    Command::new("npx")
        .args(["yarn", "install", "--frozen-lockfile"])
        .status()
        .expect("Could not install dependencies using `npx yarn`");
}

fn pnpm_install() {
    println!("Installing dependencies using: `npx pnpm`");
    Command::new("npx")
        .args(["pnpm", "install", "--frozen-lockfile"])
        .status()
        .expect("Could not install dependencies using `npx pnpm`");
}

fn cargo_install() {
    println!("Installing dependencies using: `cargo`");
    println!("Installing dependencies by building the default target. For more information, see: https://github.com/rust-lang/cargo/issues/2644");
    // TODO: https://github.com/rust-lang/cargo/issues/2644
    Command::new("cargo")
        .args(["build"])
        .status()
        .expect("Could not install dependencies using `cargo`");
}

// TODO: multiple package managers in a single repo
fn setup_dependencies(package_manager_args: PackageManagerArgs) {
    let package_manager = package_manager_args.package_manager;
    match package_manager {
        Some(crate::common::package_manager::PackageManager::Npm) => npm_install(),
        Some(crate::common::package_manager::PackageManager::Bun) => bun_install(),
        Some(crate::common::package_manager::PackageManager::Yarn) => yarn_install(),
        Some(crate::common::package_manager::PackageManager::Pnpm) => pnpm_install(),
        Some(crate::common::package_manager::PackageManager::Cargo) => cargo_install(),
        None => {
            if let Some(package_manager) =
                PackageManager::auto_detect_preferred_package_manager_for_ecosystem(
                    Ecosystem::JavaScript,
                )
            {
                // TODO: encode this in the type system
                match package_manager {
                    PackageManager::Npm => npm_install(),
                    PackageManager::Bun => bun_install(),
                    PackageManager::Yarn => yarn_install(),
                    PackageManager::Pnpm => pnpm_install(),
                    PackageManager::Cargo => panic!("unrechachable"),
                }
            }
            if let Some(package_manager) =
                PackageManager::auto_detect_preferred_package_manager_for_ecosystem(Ecosystem::Rust)
            {
                // TODO: encode this in the type system
                match package_manager {
                    PackageManager::Npm => panic!("unrechachable"),
                    PackageManager::Bun => panic!("unrechachable"),
                    PackageManager::Yarn => panic!("unrechachable"),
                    PackageManager::Pnpm => panic!("unrechachable"),
                    PackageManager::Cargo => cargo_install(),
                }
            }
        }
    }
}

pub(crate) fn setup_command(setup_args: SetupArgs) {
    match setup_args.command {
        SetupCommand::Dependencies => {
            setup_dependencies(setup_args.package_manager_args);
        }
    };
}

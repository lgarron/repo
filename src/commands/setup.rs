use std::{fs::exists, process::Stdio};

use clap::{Args, Subcommand};
use printable_shell_command::PrintableShellCommand;

use crate::common::{
    debug::DebugPrintable,
    ecosystem::Ecosystem,
    package_manager::{PackageManager, PackageManagerArgs, PACKAGE_LOCK_JSON_PATH},
};

#[derive(Args, Debug)]
pub(crate) struct SetupArgs {
    /// Run a specific setup command, or infer the ssetup command to run.
    #[command(subcommand)]
    command: Option<SetupCommand>,
}

#[derive(Debug, Subcommand)]
enum SetupCommand {
    /// Install dependencies
    Dependencies(PackageManagerArgs),
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
    PrintableShellCommand::new("npm")
        .arg_each([install_commmand])
        .debug_print()
        .status()
        .expect("Could not install dependencies using `npm`");
}

fn bun_install() {
    println!("Installing dependencies using: `bun`");
    PrintableShellCommand::new("bun")
        .arg_each(["install", "--no-save"])
        .debug_print()
        .status()
        .expect("Could not install dependencies using `bun`");
}

fn yarn_install() {
    println!("Installing dependencies using: `npx yarn`");
    PrintableShellCommand::new("npx")
        .arg_each(["yarn", "install", "--frozen-lockfile"])
        .debug_print()
        .status()
        .expect("Could not install dependencies using `npx yarn`");
}

fn pnpm_install() {
    println!("Installing dependencies using: `npx pnpm`");
    PrintableShellCommand::new("npx")
        .arg_each(["pnpm", "install", "--frozen-lockfile"])
        .debug_print()
        .status()
        .expect("Could not install dependencies using `npx pnpm`");
}

fn cargo_install() {
    println!("Installing dependencies using: `cargo`");
    println!("Installing dependencies by building the default target. For more information, see: https://github.com/rust-lang/cargo/issues/2644");
    // TODO: https://github.com/rust-lang/cargo/issues/2644
    PrintableShellCommand::new("cargo")
        .arg("build")
        .debug_print()
        .status()
        .expect("Could not install dependencies using `cargo`");
}

// TODO: multiple package managers in a single repo
fn setup_dependencies(package_manager_args: PackageManagerArgs) {
    // TODO: multiple ecosystems
    let package_manager = package_manager_args.package_manager;
    match package_manager {
        Some(PackageManager::Npm) => npm_install(),
        Some(PackageManager::Bun) => bun_install(),
        Some(PackageManager::Yarn) => yarn_install(),
        Some(PackageManager::Pnpm) => pnpm_install(),
        Some(PackageManager::Cargo) => cargo_install(),
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

pub(crate) fn make_setup_exists() -> bool {
    PrintableShellCommand::new("make")
        .arg_each(["-n", "setup"])
        .debug_print()
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .expect("Could not detect whether `make setup` exists")
        .success()
}

pub(crate) fn make_setup() {
    PrintableShellCommand::new("make")
        .arg("setup")
        .debug_print()
        .status()
        .expect("Could not run `make setup` exists`");
}

pub(crate) fn setup_command(setup_args: SetupArgs) {
    match setup_args.command {
        None => {
            if make_setup_exists() {
                eprintln!("Running: make setup");
                make_setup();
            } else {
                setup_dependencies(PackageManagerArgs {
                    package_manager: None,
                });
            }
        }
        Some(SetupCommand::Dependencies(package_manager_args)) => {
            setup_dependencies(package_manager_args);
        }
    };
}

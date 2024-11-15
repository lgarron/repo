use clap::{Args, Subcommand, ValueEnum};
use edit::edit_file;
use std::{
    fs::{create_dir_all, exists, write},
    process::exit,
};

use opener::reveal;

#[derive(Args, Debug)]
pub(crate) struct CIArgs {
    #[command(subcommand)]
    command: CICommand,
}

#[derive(Debug, Subcommand)]
enum CICommand {
    /// Alias for `repo setup ci`
    Setup(CISetupArgs),
    // TODO: support `Open` as a version of `Edit` that doesn't wait.
    /// Open the CI file.
    Edit,
}

#[derive(Args, Debug)]
pub(crate) struct CISetupArgs {
    #[clap(long)]
    followup: Option<CISetupFollowup>,
    #[clap(long)]
    overwrite: bool,
}

#[derive(Debug, Clone, ValueEnum)]
enum CISetupFollowup {
    // TODO: support `Open` as a version of `Edit` that doesn't wait.
    Edit,
    Reveal,
    None,
}

const CI_YAML_DIR: &str = "./.github/workflows";
const CI_YAML_PATH: &str = "./.github/workflows/CI.yaml";
const CI_YAML_TEMPLATE: &str = "name: CI

on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest

    steps:
      - uses: actions/checkout@v4
      #   with:
      #       lfs: 'true'
      - uses: actions/setup-node@v4
        with:
          node-version: 22
      # - uses: oven-sh/setup-bun@v1
      # - uses: Swatinem/rust-cache@v2
      # - run: make setup
      - run: make lint
";

// pub fn open_in_vs_code() {
//     Command::new("code")
//         .args(["--", CI_YAML_PATH])
//         .status()
//         .expect("Unable to open CI template in VS Code");
// }

// TODO: `open` as implemented by `opener` does not properly open the file in the current workspace when invoked from a VS Code terminal. https://github.com/Seeker14491/opener/issues/34
// pub fn open_file() {
//     open(CI_YAML_PATH).expect("Unable to open CI template")
// }

fn open_ci_template_for_editing() {
    edit_file(CI_YAML_PATH).expect("Could not open CI file for editing.");
}

pub(crate) fn setup_ci(ci_setup_args: CISetupArgs) {
    if exists(CI_YAML_PATH).expect("Could not access file system.") {
        if ci_setup_args.overwrite {
            eprintln!("Overwriting CI file due to `--overwrite` flag.");
        } else {
            eprintln!("CI file already exists. Pass `--overwrite` to overwrite.");
            exit(1);
        }
    }
    create_dir_all(CI_YAML_DIR).expect("Unable to write CI template");
    write(CI_YAML_PATH, CI_YAML_TEMPLATE).expect("Unable to write CI template");
    match ci_setup_args.followup {
        Some(CISetupFollowup::Edit) => open_ci_template_for_editing(),
        Some(CISetupFollowup::Reveal) => {
            reveal(CI_YAML_PATH).expect("Unable to reveal CI template")
        }
        Some(CISetupFollowup::None) => {}
        None => {
            open_ci_template_for_editing();
        }
    };
}

pub(crate) fn ci_command(ci_args: CIArgs) {
    match ci_args.command {
        CICommand::Edit => {
            open_ci_template_for_editing();
        }
        CICommand::Setup(ci_setup_args) => setup_ci(ci_setup_args),
    }
}

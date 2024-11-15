use std::fs::write;

use clap::{Args, Subcommand};

#[derive(Args, Debug)]
pub(crate) struct CIArgs {
    #[command(subcommand)]
    pub command: CICommand,
}

#[derive(Debug, Subcommand)]
pub(crate) enum CICommand {
    /// Get the current version
    Setup,
}

const CI_YAML_TEMPLATE: &str = "name: CI

on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest

    steps:
      - uses: actions/checkout@v4
      #   with:
      #       lfs: 'true'
      # - uses: oven-sh/setup-bun@v1
      - uses: actions/setup-node@v4
        with:
          node-version: 22
      # - uses: Swatinem/rust-cache@v2
      # - run: make setup
      - run: make lint
";

pub(crate) fn ci_command(ci_args: CIArgs) {
    match ci_args.command {
        CICommand::Setup => {
            write("./github/workflows/CI.yaml", CI_YAML_TEMPLATE)
                .expect("Unable to write CI template");
        }
    }
}

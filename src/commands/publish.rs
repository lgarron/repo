use std::process::Command;

use clap::Args;

use crate::common::ecosystem::{Ecosystem, EcosystemArgs};

#[derive(Args, Debug)]
pub(crate) struct PublishArgs {
    #[command(flatten)]
    ecosystem_args: EcosystemArgs,
}

// TODO: use traits to abstract across ecosystems
// TODO: support cross-checking versions across ecosystems
pub(crate) fn publish_command(publish_args: PublishArgs) {
    match publish_args.ecosystem_args.ecosystem.unwrap_or_default() {
        Ecosystem::Npm => {
            Command::new("npm")
                .args(["publish"])
                .spawn()
                .expect("Could not publish using `npm`");
        }
        Ecosystem::Cargo => {
            Command::new("cargo")
                .args(["publish"])
                .spawn()
                .expect("Could not publish using `npm`");
        }
    }
}

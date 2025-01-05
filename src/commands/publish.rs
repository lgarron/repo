use std::process::Command;

use clap::Args;

use crate::common::ecosystem::{Ecosystem, EcosystemArgs};

use super::version::must_detect_ecosystem_by_getting_version;

#[derive(Args, Debug)]
pub(crate) struct PublishArgs {
    #[command(flatten)]
    ecosystem_args: EcosystemArgs,
}

// TODO: use traits to abstract across ecosystems
// TODO: support cross-checking versions across ecosystems
pub(crate) fn publish_command(publish_args: PublishArgs) {
    match must_detect_ecosystem_by_getting_version(&publish_args.ecosystem_args) {
        (Ecosystem::JavaScript, _) => {
            Command::new("npm")
                .args(["publish"])
                .spawn()
                .expect("Could not publish using `npm`")
                .wait()
                .unwrap();
        }
        (Ecosystem::Rust, _) => {
            Command::new("cargo")
                .args(["publish"])
                .spawn()
                .expect("Could not publish using `cargo`")
                .wait()
                .unwrap();
        }
    }
}

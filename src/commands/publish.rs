use clap::Args;
use printable_shell_command::PrintableShellCommand;

use crate::common::{
    debug::DebugPrintable,
    ecosystem::{Ecosystem, EcosystemArgs},
};

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
            PrintableShellCommand::new("npm")
                .arg("publish")
                .debug_print()
                .spawn()
                .expect("Could not publish using `npm`")
                .wait()
                .unwrap();
        }
        (Ecosystem::Rust, _) => {
            PrintableShellCommand::new("cargo")
                .arg("publish")
                .debug_print()
                .spawn()
                .expect("Could not publish using `cargo`")
                .wait()
                .unwrap();
        }
    }
}

use clap::{Args, Subcommand};

use crate::ci::{setup_ci, CISetupArgs};

#[derive(Args, Debug)]
pub(crate) struct SetupArgs {
    #[command(subcommand)]
    command: SetupCommand,
}

#[derive(Debug, Subcommand)]
enum SetupCommand {
    // Set up a CI template for GitHub and open for editing.
    CI(CISetupArgs),
    // Set up a CI template for auto-publishing releases from tags pushed to GitHub.
    AutoPublishGithubRelease,
}

// TODO: use traits to abstract across ecosystems
// TODO: support cross-checking Setups across ecosystems
pub(crate) fn setup_command(setup_args: SetupArgs) {
    match setup_args.command {
        SetupCommand::CI(ci_setup_args) => setup_ci(ci_setup_args),
        SetupCommand::AutoPublishGithubRelease => todo!(),
    };
}

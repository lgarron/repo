mod args;
mod commands;
mod common;

use args::get_args;
use commands::boilerplate::boilerplate as boilerplate_command;
use commands::publish::publish_command;
use commands::setup::setup_command;
use commands::vcs::vcs_command;
use commands::version::version_command;
use commands::workspace::workspace_command;
use shadow_rs::shadow;

use crate::commands::dependencies::dependencies_command;

shadow!(build);

fn main() {
    let args = get_args();

    match args.command {
        args::RepoCommand::Version(version_args) => version_command(version_args),
        args::RepoCommand::Publish(publish_args) => publish_command(publish_args),
        args::RepoCommand::Boilerplate(boilerplate_args) => boilerplate_command(boilerplate_args),
        args::RepoCommand::Setup(setup_args) => setup_command(setup_args),
        args::RepoCommand::Vcs(vcs_args) => vcs_command(vcs_args).unwrap(),
        args::RepoCommand::Workspace(workspace_args) => workspace_command(workspace_args),
        args::RepoCommand::Dependencies(dependencies_args) => {
            dependencies_command(dependencies_args).unwrap()
        }
        args::RepoCommand::Completions(_) => panic!("We should have exited earlier."),
    }
}

mod args;
mod commands;
mod common;

use args::get_args;
use commands::boilerplate::boilerplate as boilerplate_command;
use commands::publish::publish_command;
use commands::setup::setup_command;
use commands::version::version_command;

fn main() {
    let args = get_args();

    match args.command {
        args::RepoCommand::Version(version_args) => version_command(version_args),
        args::RepoCommand::Publish(publish_args) => publish_command(publish_args),
        args::RepoCommand::Boilerplate(boilerplate_args) => boilerplate_command(boilerplate_args),
        args::RepoCommand::Setup(setup_args) => setup_command(setup_args),
        args::RepoCommand::Completions(_) => panic!("We should have exited earlier."),
    }
}

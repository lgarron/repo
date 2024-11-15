use std::io::stdout;
use std::process::exit;

use clap::{Args, CommandFactory, Parser, Subcommand};
use clap_complete::generator::generate;
use clap_complete::{Generator, Shell};

use crate::commands::ci::CIArgs;
use crate::commands::publish::PublishArgs;
use crate::commands::setup::SetupArgs;
use crate::commands::version::VersionArgs;

/// repo — a tool for repo management
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[clap(name = "repo")]
pub(crate) struct RepoArgs {
    #[command(subcommand)]
    pub command: RepoCommand,
}

#[derive(Debug, Subcommand)]
pub(crate) enum RepoCommand {
    /// Perform operations on the repo version.
    Version(VersionArgs),
    /// Publish.
    Publish(PublishArgs),
    /// Set up the repo.
    Setup(SetupArgs),
    /// Manage CI (continuous integration) at `.github/workflows/CI.yaml`
    CI(CIArgs),
    /// Print completions for the given shell.
    Completions(CompletionsArgs),
}

#[derive(Args, Debug)]
pub(crate) struct CompletionsArgs {
    /// Print completions for the given shell.
    /// These can be loaded/stored permanently (e.g. when using Homebrew), but they can also be sourced directly, e.g.:
    ///
    ///  repo completions fish | source # fish
    ///  source <(repo completions zsh) # zsh
    #[clap(verbatim_doc_comment, id = "SHELL")]
    shell: Shell,
}

fn completions_for_shell(cmd: &mut clap::Command, generator: impl Generator) {
    generate(generator, cmd, "repo", &mut stdout());
}

pub(crate) fn get_args() -> RepoArgs {
    let mut command = RepoArgs::command();

    let args = RepoArgs::parse();
    if let RepoCommand::Completions(completions_args) = args.command {
        completions_for_shell(&mut command, completions_args.shell);
        exit(0);
    };

    args
}

use std::io::stdout;
use std::process::exit;

use clap::{Args, CommandFactory, Parser, Subcommand};
use clap_complete::generator::generate;
use clap_complete::{Generator, Shell};

use crate::build::CLAP_LONG_VERSION;
use crate::commands::boilerplate::BoilerplateArgs;
use crate::commands::dependencies::DependenciesArgs;
use crate::commands::publish::PublishArgs;
use crate::commands::setup::SetupArgs;
use crate::commands::vcs::VcsArgs;
use crate::commands::version::VersionArgs;
use crate::commands::workspace::WorkspaceArgs;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[clap(name = "repo", long_version = CLAP_LONG_VERSION)]
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
    /// Set up boilerplate for the repo.
    Boilerplate(BoilerplateArgs),
    /// Set up a repository checkout.
    Setup(SetupArgs),
    /// Get information about the current VCS.
    Vcs(VcsArgs),
    /// Get information about the current workspace.
    Workspace(WorkspaceArgs),
    /// Operate on dependencies.
    Dependencies(DependenciesArgs),
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

#[cfg(test)]
mod tests {
    use crate::args::RepoArgs;

    // https://docs.rs/clap/latest/clap/_derive/_tutorial/index.html#testing
    #[test]
    fn test_clap_args() {
        use clap::CommandFactory;

        RepoArgs::command().debug_assert();
    }
}

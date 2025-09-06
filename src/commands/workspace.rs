use std::process::exit;

use clap::{Args, Subcommand};

use crate::common::{args::PathArgs, workspace::auto_detect_workspace_root};

#[derive(Args, Debug)]
pub(crate) struct WorkspaceArgs {
    #[command(subcommand)]
    command: WorkspaceCommand,
}

#[derive(Debug, Subcommand)]
enum WorkspaceCommand {
    /// Get the workspace root folder based on VCS or other litmus files (e.g. `package.json`, `Cargo.toml`)
    /// If the folder is part of multiple repositories, at most one will be returned (consistent with the `kind` subcommand).
    ///
    /// Also consider `repo vcs root` if you are only looking for VCS roots.
    Root(PathArgs),
}

pub(crate) fn workspace_command(workspace_args: WorkspaceArgs) {
    match workspace_args.command {
        WorkspaceCommand::Root(workspace_root_args) => {
            if let Some(path) = auto_detect_workspace_root(&workspace_root_args.path()) {
                print!("{}", path)
            } else {
                exit(1)
            }
        }
    };
}

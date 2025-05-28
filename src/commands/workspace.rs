use std::{env::current_dir, process::exit};

use clap::{Args, Subcommand};

use crate::common::{package_manager::PackageManagerArgs, workspace::auto_detect_workspace_root};

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
    Root,
}

#[derive(Args, Debug)]
pub(crate) struct DependenciesArgs {
    #[command(flatten)]
    package_manager_args: PackageManagerArgs,
}

pub(crate) fn workspace_command(workspace_args: WorkspaceArgs) {
    match workspace_args.command {
        WorkspaceCommand::Root => {
            if let Some(path) = auto_detect_workspace_root(&current_dir().unwrap()) {
                println!("{}", path)
            } else {
                exit(1)
            }
        }
    };
}

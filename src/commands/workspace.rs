use std::process::exit;

use clap::{Args, Subcommand, ValueEnum};

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
    Root(WorkspaceRootArgs),
}

#[derive(Args, Debug)]
pub(crate) struct WorkspaceRootArgs {
    #[clap(long)]
    fallback: Option<RootFallback>,

    #[command(flatten)]
    path_args: PathArgs,
}

#[derive(Debug, Clone, ValueEnum, PartialEq, Eq)]
enum RootFallback {
    #[clap(name = "closest-dir")]
    ClosestDir,
}

pub(crate) fn workspace_command(workspace_args: WorkspaceArgs) {
    match workspace_args.command {
        WorkspaceCommand::Root(workspace_root_args) => {
            let path = &workspace_root_args.path_args.path();
            if let Some(path) = auto_detect_workspace_root(path) {
                print!("{}", path)
            } else {
                match workspace_root_args.fallback {
                    Some(RootFallback::ClosestDir) => {
                        if path.is_dir() {
                            print!("{}", path.to_string_lossy())
                        } else if let Some(parent_path) = path.parent() {
                            print!("{}", parent_path.to_string_lossy())
                        }
                    }
                    None => {}
                }
                exit(1)
            }
        }
    };
}

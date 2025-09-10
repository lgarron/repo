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
    /// Use either the path itself (if it's an existing directory) or its parent (if it's not).
    /// Note: due to Rust parsing quirks, non-existent paths are always treated as non-directories (even if they have a trailing slash), i.e. their parent will be returned.
    // TODO: always treat paths with a trailing slash as dirs.
    #[clap(name = "closest-dir")]
    ClosestDir,
}

pub(crate) fn workspace_command(workspace_args: WorkspaceArgs) {
    match workspace_args.command {
        WorkspaceCommand::Root(workspace_root_args) => {
            let path = &workspace_root_args.path_args.path();
            let root_path = if let Some(path) = auto_detect_workspace_root(path) {
                path
            } else {
                match workspace_root_args.fallback {
                    Some(RootFallback::ClosestDir) => {
                        // TODO: wire things up so that we can tell if the argument had a trailing slash. Probably requires asking `clap` to keep parse into a `String` an `PathBuf` at the same time.
                        if path.is_dir() {
                            path.to_string_lossy().to_string()
                        } else if let Some(parent_path) = path.parent() {
                            parent_path.to_string_lossy().to_string()
                        } else {
                            eprintln!("Could not get parent path");
                            exit(1);
                        }
                    }
                    None => {
                        eprintln!("No workspace found. Consider passing: `--fallback closest-dir`");
                        exit(1)
                    }
                }
            };
            print!("{}", root_path)
        }
    };
}

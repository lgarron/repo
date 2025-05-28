use std::{env::current_dir, process::exit};

use clap::{Args, Subcommand};

use crate::common::{package_manager::PackageManagerArgs, vcs::VcsKind};

#[derive(Args, Debug)]
pub(crate) struct VcsArgs {
    /// Run a specific setup command, or infer the ssetup command to run.
    #[command(subcommand)]
    command: VcsCommand,
}

#[derive(Debug, Subcommand)]
enum VcsCommand {
    /// Get the kind of VCS.
    /// If there are multiple in the same project (e.g. `jj` + `git`), at most one will be returned (consistent with the `root` subcommand).
    Kind,
    /// Get the repository root folder
    /// If the folder is part of multiple repositories, at most one will be returned (consistent with the `kind` subcommand).
    Root,
}

#[derive(Args, Debug)]
pub(crate) struct DependenciesArgs {
    #[command(flatten)]
    package_manager_args: PackageManagerArgs,
}

pub(crate) fn vcs_command(vcs_args: VcsArgs) {
    match vcs_args.command {
        VcsCommand::Kind => {
            match VcsKind::auto_detect_preferred_vcs_and_repo_root_for_ecosystem(
                &current_dir().unwrap(),
            ) {
                Some((vcs, _)) => println!("{}", vcs),
                None => exit(1),
            }
        }
        VcsCommand::Root => {
            match VcsKind::auto_detect_preferred_vcs_and_repo_root_for_ecosystem(
                &current_dir().unwrap(),
            ) {
                Some((_, path)) => println!("{}", path),
                None => exit(1),
            }
        }
    };
}

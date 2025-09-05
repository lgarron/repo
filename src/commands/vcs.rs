use std::{env::current_dir, process::Command};

use clap::{Args, Subcommand};

use crate::common::{
    inference::get_stdout,
    package_manager::PackageManagerArgs,
    vcs::{auto_detect_preferred_vcs_and_repo_root_for_ecosystem, VcsKind},
};

#[derive(Args, Debug)]
pub(crate) struct VcsArgs {
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
    ///
    /// Also consider `repo worktree root` if you are only looking for a project root folder and don't specifically need it to have a VCS.
    Root,
    /// Operate on the latest commit. This does not include the working copy (or a non-merge `@` if it is empty or has an empty description, in case of `jj`).
    LatestCommit(LatestCommitArgs),
}

#[derive(Args, Debug)]
pub(crate) struct LatestCommitArgs {
    #[command(subcommand)]
    command: LatestCommitSubcommand,
}

#[derive(Debug, Subcommand)]
enum LatestCommitSubcommand {
    /// Get the commit hash.
    Hash,
}

#[derive(Args, Debug)]
pub(crate) struct DependenciesArgs {
    #[command(flatten)]
    package_manager_args: PackageManagerArgs,
}

fn jj_get_latest_commmit_hash() -> Result<String, String> {
    let mut jj_command = Command::new("jj");
    jj_command.args([
        "--no-graph",
        "--ignore-working-copy",
        "--color=never",
        "--revisions",
        "::@ & ((~description(exact:\"\") & ~empty()) | merges())",
        "--limit=1",
        "--template",
        "commit_id",
    ]);
    if let Some(hash) = get_stdout(jj_command) {
        return Ok(hash.trim().to_owned());
    }

    Err("Could not get latest hash from `jj`.".to_owned())
}

fn git_get_latest_commmit_hash() -> Result<String, String> {
    let mut git_command = Command::new("git");
    git_command.args(["rev-parse", "HEAD"]);
    if let Some(hash) = get_stdout(git_command) {
        return Ok(hash.trim().to_owned());
    }

    Err("Could not get latest hash from `git`.".to_owned())
}

pub(crate) fn vcs_command(vcs_args: VcsArgs) -> Result<(), String> {
    match vcs_args.command {
        VcsCommand::Kind => {
            match auto_detect_preferred_vcs_and_repo_root_for_ecosystem(&current_dir().unwrap()) {
                Some((vcs, _)) => print!("{}", vcs),
                None => return Err("Could not detect a VCS repo.".to_owned()),
            }
        }
        VcsCommand::Root => {
            match auto_detect_preferred_vcs_and_repo_root_for_ecosystem(&current_dir().unwrap()) {
                Some((_, path)) => print!("{}", path),
                None => return Err("Could not detect a VCS repo.".to_owned()),
            }
        }
        VcsCommand::LatestCommit(latest_commit_args) => {
            match latest_commit_args.command {
                LatestCommitSubcommand::Hash => {
                    match auto_detect_preferred_vcs_and_repo_root_for_ecosystem(
                        &current_dir().unwrap(),
                    ) {
                        Some((VcsKind::Jj, _)) => print!("{}", jj_get_latest_commmit_hash()?),
                        Some((VcsKind::Git, _)) => print!("{}", git_get_latest_commmit_hash()?),
                        Some((VcsKind::Mercurial, _)) => {
                            return Err("Mercurial is unsupported for this operation.".into());
                        }
                        None => return Err("Could not detect a VCS repo.".to_owned()),
                    }
                    // dbg!(latest_commit_args);
                }
            }
        }
    };
    Ok(())
}

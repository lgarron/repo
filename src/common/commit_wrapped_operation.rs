use std::process::Command;

use crate::{
    commands::version::CommitArgs,
    common::{command::command_must_succeed, inference::get_stdout, vcs::VcsKind},
};

pub struct CommitWrappedOperation {
    perform_commit: bool,
    commit_using: VcsKind,
}

impl TryFrom<CommitArgs> for CommitWrappedOperation {
    type Error = String;

    fn try_from(commit_args: CommitArgs) -> Result<Self, Self::Error> {
        Ok(Self {
            perform_commit: commit_args.commit,
            commit_using: commit_args.vcs()?,
        })
    }
}

impl CommitWrappedOperation {
    pub fn prep_commit(&self) -> Result<(), String> {
        match self.commit_using {
            VcsKind::Git => {
                let mut command = Command::new("git");
                command.args(["status", "--porcelain"]);
                let Some(stdout) = get_stdout(command) else {
                    return Err("Could not get `git status` output".to_owned());
                };
                if stdout.trim() != "" {
                    return Err("`git status` is not clean.".into());
                }
                Ok(())
            }
            VcsKind::Jj => {
                let mut command = Command::new("jj");
                command.args([
                    "log",
                    "--color=never",
                    "--no-graph",
                    "--revisions",
                    "@ & empty() & ~merges() & description(exact:\"\")",
                    "--template",
                    "'.'",
                ]);
                let Some(stdout) = get_stdout(command) else {
                    return Err("Could not get `jj log` output.".to_owned());
                };
                if stdout.trim() != "." {
                    let mut command = Command::new("jj");
                    command.args(["new"]);
                    command_must_succeed(command)?;
                }
                Ok(())
            }
            VcsKind::Mercurial => Err("Mercurial is unsupported for this operation.".into()),
        }
    }

    /// Includes all changes added since a prior [CommitWrappable::prep_commit] call.
    pub fn finalize_commit(&self, message: &str) -> Result<(), String> {
        if !self.perform_commit {
            return Ok(());
        }
        match self.commit_using {
            VcsKind::Git => {
                let mut command = Command::new("git");
                command.args(["commit", "--all", "--message", message]);
                command_must_succeed(command)?;
                Ok(())
            }
            VcsKind::Jj => {
                let mut command = Command::new("jj");
                command.args(["commit", "--message", message]);
                command_must_succeed(command)?;
                Ok(())
            }
            VcsKind::Mercurial => Err("Mercurial is unsupported for this operation.".into()),
        }
    }

    pub fn perform_operation(
        &self,
        operation: &dyn Fn() -> Result<String, String>,
    ) -> Result<(), String> {
        self.prep_commit()?;
        let message = operation()?;
        self.finalize_commit(&message)?;
        Ok(())
    }
}

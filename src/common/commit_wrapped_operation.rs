use printable_shell_command::PrintableShellCommand;

use crate::{
    commands::version::CommitOperationArgs,
    common::{
        command::command_must_succeed,
        inference::get_stdout,
        vcs::{vcs_or_infer, VcsKind},
    },
};

pub struct CommitWrappedOperation {
    perform_commit: bool,
    commit_using: VcsKind,
}

impl TryFrom<&CommitOperationArgs> for CommitWrappedOperation {
    type Error = String;

    fn try_from(commit_args: &CommitOperationArgs) -> Result<Self, Self::Error> {
        Ok(Self {
            perform_commit: commit_args.perform_commit(),
            commit_using: vcs_or_infer(commit_args.commit_using)?,
        })
    }
}

impl CommitWrappedOperation {
    pub fn prep_commit(&self) -> Result<(), String> {
        match self.commit_using {
            VcsKind::Git => {
                let mut command = PrintableShellCommand::new("git");
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
                let mut command = PrintableShellCommand::new("jj");
                command.args(["log", "--color=never", "--no-graph"]);
                command.args([
                    "--revisions",
                    "@ & empty() & ~merges() & description(exact:\"\")",
                ]);
                command.args(["--template", "'.'"]);
                let Some(stdout) = get_stdout(command) else {
                    return Err("Could not get `jj log` output.".to_owned());
                };
                if stdout.trim() != "." {
                    let mut command = PrintableShellCommand::new("jj");
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
                let mut command = PrintableShellCommand::new("git");
                command.arg_each(["commit", "--all"]);
                command.args(["--message", message]);
                command_must_succeed(command)?;
                Ok(())
            }
            VcsKind::Jj => {
                let mut command = PrintableShellCommand::new("jj");
                command.arg("commit");
                command.args(["--message", message]);
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

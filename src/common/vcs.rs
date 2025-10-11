use std::{env::current_dir, fmt::Display, fs::exists, path::Path};

use clap::ValueEnum;
use printable_shell_command::PrintableShellCommand;

use super::inference::get_stdout;

#[derive(Debug, Copy, Clone, ValueEnum)]
pub(crate) enum VcsKind {
    Git,
    Jj,
    Mercurial,
}

impl Display for VcsKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Git => "git",
                Self::Jj => "jj",
                Self::Mercurial => "Mercurial",
            }
        )
    }
}

// Note that `.git` can be either a folder or a file.
// pub(crate) const GIT_PATH: &str = "./.git";
// pub(crate) const JJ_PATH: &str = "./.jj";
pub(crate) const HG_PATH: &str = "./.hg";

pub(crate) fn auto_detect_preferred_vcs_and_repo_root(
    // TODO: accept file path?
    path_of_folder_or_subfolder: &Path,
) -> Option<(VcsKind, String)> {
    {
        let mut jj_command = PrintableShellCommand::new("jj");
        jj_command
            .current_dir(path_of_folder_or_subfolder)
            .args(["root"]);
        if let Some(path) = get_stdout(jj_command) {
            return Some((VcsKind::Jj, path));
        }
    }
    {
        let mut git_command = PrintableShellCommand::new("git");
        git_command
            .current_dir(path_of_folder_or_subfolder)
            .args(["rev-parse", "--show-toplevel"]);
        if let Some(path) = get_stdout(git_command) {
            return Some((VcsKind::Git, path));
        }
    }
    let mut dir_or_ancestor = Some(path_of_folder_or_subfolder);
    while let Some(dir) = dir_or_ancestor {
        let mercurial_path = dir.join(Path::new(HG_PATH));
        if exists(mercurial_path).unwrap_or(false) {
            // TODO: check that the folder contains the expected `.hg` dir?
            return Some((VcsKind::Mercurial, dir.to_string_lossy().to_string()));
        }
        dir_or_ancestor = dir.parent();
    }
    None
}

pub fn vcs_or_infer(vcs: Option<VcsKind>) -> Result<VcsKind, String> {
    Ok(match vcs {
        Some(vcs_kind) => vcs_kind,
        None => {
            let Some((vcs_kind, _)) =
                auto_detect_preferred_vcs_and_repo_root(&current_dir().unwrap())
            else {
                return Err("No VCS specified or found.".to_owned());
            };
            vcs_kind
        }
    })
}

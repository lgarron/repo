use std::{
    fmt::Display,
    fs::exists,
    path::Path,
    process::{Command, Stdio},
};

use clap::ValueEnum;

// #[derive(Args, Debug)]
// pub(crate) struct VcsOptionArgs {
//     #[clap(long)]
//     pub(crate) vcs: Option<VCS>,
// }

#[derive(Debug, Clone, ValueEnum)]
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

// Gracefully recovers from any error by returning `None`.
fn get_stdout(mut command: Command) -> Option<String> {
    command.stdout(Stdio::piped()).stderr(Stdio::null());
    let Ok(exit_status) = command.status() else {
        return None;
    };

    if exit_status.code() == Some(0) {
        if let Ok(stdout) = command.output() {
            if let Ok(path) = String::from_utf8(stdout.stdout) {
                // TODO: check that the folder contains the expected `.git` dir/file?
                return Some(path);
            }
        }
    }
    None
}

// Note that `.git` can be either a folder or a file.
// pub(crate) const GIT_PATH: &str = "./.git";
// pub(crate) const JJ_PATH: &str = "./.jj";
pub(crate) const HG_PATH: &str = "./.hg";

impl VcsKind {
    pub(crate) fn auto_detect_preferred_vcs_and_repo_root_for_ecosystem(
        // TODO: accept file path?
        path_of_folder_or_subfolder: &Path,
    ) -> Option<(Self, String)> {
        {
            let mut jj_command = Command::new("jj");
            jj_command
                .current_dir(path_of_folder_or_subfolder)
                .args(["root"]);
            if let Some(path) = get_stdout(jj_command) {
                return Some((Self::Jj, path));
            }
        }
        {
            let mut git_command = Command::new("git");
            git_command
                .current_dir(path_of_folder_or_subfolder)
                .args(["rev-parse", "--show-toplevel"]);
            if let Some(path) = get_stdout(git_command) {
                return Some((Self::Git, path));
            }
        }
        let mut dir_or_ancestor = Some(path_of_folder_or_subfolder);
        while let Some(dir) = dir_or_ancestor {
            dbg!(dir);
            let mercurial_path = dir.join(Path::new(HG_PATH));
            dbg!(&mercurial_path);
            if exists(mercurial_path).unwrap_or(false) {
                // TODO: check that the folder contains the expected `.hg` dir?
                return Some((Self::Mercurial, dir.to_string_lossy().to_string()));
            }
            dir_or_ancestor = dir.parent();
        }
        None
    }
}

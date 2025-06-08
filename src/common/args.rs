use std::{env::current_dir, path::PathBuf};

use clap::Args;

#[derive(Args, Debug)]
pub(crate) struct PathArgs {
    /// Path to an existing file or folder to use. Defaults to the current working directory.
    #[clap(long = "path")]
    maybe_path: Option<PathBuf>,
}

impl PathArgs {
    pub(crate) fn path(&self) -> PathBuf {
        match &self.maybe_path {
            Some(path) => path.clone(),
            None => current_dir().expect("Could not access the current working directory."),
        }
    }
}

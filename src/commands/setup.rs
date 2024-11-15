use std::path::PathBuf;

use clap::{Args, Subcommand};

use crate::common::template_file::{TemplateFile, TemplateFileWriteArgs};

use super::ci::ci_template;

#[derive(Args, Debug)]
pub(crate) struct SetupArgs {
    #[command(subcommand)]
    command: SetupCommand,
}

#[derive(Debug, Subcommand)]
enum SetupCommand {
    /// Set up a CI template for GitHub and open for editing at: `.github/workflows/CI.yaml`
    CI(TemplateFileWriteArgs),
    /// Set up a CI template for auto-publishing releases from tags pushed to GitHub, at: .github/workflows/publish-github-release.yaml
    AutoPublishGithubRelease(TemplateFileWriteArgs),
}

pub(crate) fn publish_github_release_template() -> TemplateFile<'static> {
    let bytes = include_bytes!("../templates/.github/workflows/publish-github-release.yaml");
    TemplateFile {
        relative_path: PathBuf::from("./.github/workflows/publish-github-release.yaml"),
        bytes,
    }
}

// TODO: use traits to abstract across ecosystems
// TODO: support cross-checking Setups across ecosystems
pub(crate) fn setup_command(setup_args: SetupArgs) {
    match setup_args.command {
        SetupCommand::CI(template_file_write_args) => ci_template().write(template_file_write_args),
        SetupCommand::AutoPublishGithubRelease(template_file_write_args) => {
            publish_github_release_template().write(template_file_write_args)
        }
    };
}
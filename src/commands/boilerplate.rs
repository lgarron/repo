use std::path::PathBuf;

use clap::{Args, Subcommand};

use crate::common::template_file::{TemplateFile, TemplateFileArgs};

#[derive(Args, Debug)]
pub(crate) struct BoilerplateArgs {
    #[command(subcommand)]
    command: BoilerplateCommand,
}

#[derive(Debug, Subcommand)]
enum BoilerplateCommand {
    /// Set up a CI template for GitHub and open for editing at: `.github/workflows/CI.yaml`
    CI(TemplateFileArgs),
    /// Set up a CI template for auto-publishing releases from tags pushed to GitHub, at: .github/workflows/publish-github-release.yaml
    AutoPublishGithubRelease(TemplateFileArgs),
}

pub(crate) fn ci_template() -> TemplateFile<'static> {
    let bytes = include_bytes!("../templates/.github/workflows/CI.yaml");
    TemplateFile {
        relative_path: PathBuf::from("./.github/workflows/CI.yaml"),
        bytes,
    }
}
pub(crate) fn publish_github_release_template() -> TemplateFile<'static> {
    let bytes = include_bytes!("../templates/.github/workflows/publish-github-release.yaml");
    TemplateFile {
        relative_path: PathBuf::from("./.github/workflows/publish-github-release.yaml"),
        bytes,
    }
}

// TODO: use traits to abstract across ecosystems
pub(crate) fn boilerplate(boilerplate_args: BoilerplateArgs) {
    match boilerplate_args.command {
        BoilerplateCommand::CI(template_file_args) => {
            ci_template().handle_command(template_file_args);
        }
        BoilerplateCommand::AutoPublishGithubRelease(template_file_args) => {
            publish_github_release_template().handle_command(template_file_args);
        }
    };
}

use clap::{Args, Subcommand};
use std::path::PathBuf;

use crate::common::template_file::{TemplateFile, TemplateFileWriteArgs};

#[derive(Args, Debug)]
pub(crate) struct CIArgs {
    #[command(subcommand)]
    command: CICommand,
}

#[derive(Debug, Subcommand)]
enum CICommand {
    /// Alias for `repo boilerplate ci`
    Boilerplate(TemplateFileWriteArgs),
    // TODO: support `Open` as a version of `Edit` that doesn't wait.
    /// Open the CI file.
    Edit,
}

pub(crate) fn ci_template() -> TemplateFile<'static> {
    let bytes = include_bytes!("../templates/.github/workflows/CI.yaml");
    TemplateFile {
        relative_path: PathBuf::from("./.github/workflows/CI.yaml"),
        bytes,
    }
}

pub(crate) fn ci_command(ci_args: CIArgs) {
    match ci_args.command {
        CICommand::Edit => {
            ci_template().open_for_editing();
        }
        CICommand::Boilerplate(template_file_write_args) => {
            ci_template().write(template_file_write_args);
        }
    }
}

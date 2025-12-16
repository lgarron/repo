use std::{
    fs::{create_dir_all, exists, File},
    io::Write,
    path::PathBuf,
    process::exit,
};

use clap::{Args, FromArgMatches, Subcommand, ValueEnum};
use edit::edit_file_without_waiting;
use opener::reveal;

#[derive(Args, Debug)]
pub(crate) struct TemplateFileArgs<CustomAddArgs: Args + FromArgMatches = BlankArgs> {
    #[command(subcommand)]
    pub(crate) command: TemplateFileCommand<CustomAddArgs>,
}

#[derive(Clone, Debug, Subcommand)]
pub(crate) enum TemplateFileCommand<CustomAddArgs: Args + FromArgMatches = BlankArgs> {
    Add(TemplateFileCreateArgs<CustomAddArgs>),
    Edit,
    Reveal,
}

#[derive(Args, Clone, Debug)]
pub(crate) struct TemplateFileCreateArgs<CustomAddArgs: Args + FromArgMatches = BlankArgs> {
    #[clap(long)]
    followup: Option<TemplateFileCreateFollowup>,
    #[clap(long)]
    overwrite: bool,
    #[clap(flatten)]
    pub(crate) custom_args: CustomAddArgs,
}

#[derive(Args, Clone, Debug)]
pub(crate) struct BlankArgs {}

#[derive(Debug, Clone, ValueEnum)]
enum TemplateFileCreateFollowup {
    // TODO: support `Open` as a version of `Edit` that doesn't wait.
    Edit,
    Reveal,
    None,
}

pub(crate) struct TemplateFile<'a> {
    pub(crate) relative_path: PathBuf,
    pub(crate) bytes: &'a [u8],
}

impl TemplateFile<'_> {
    pub(crate) fn handle_command<CustomAddArgs: Args + FromArgMatches>(
        &self,
        template_file_args: TemplateFileArgs<CustomAddArgs>,
    ) {
        match template_file_args.command {
            TemplateFileCommand::Add(template_file_create_args) => {
                self.create(template_file_create_args);
            }
            TemplateFileCommand::Edit => self.open_for_editing(),
            TemplateFileCommand::Reveal => self.reveal(),
        }
    }

    /// Autoamtically performs the followup from the `template_file_write_args` argument.
    fn create<CustomAddArgs: Args + FromArgMatches>(
        &self,
        template_file_write_args: TemplateFileCreateArgs<CustomAddArgs>,
    ) {
        if exists(&self.relative_path).expect("Could not access file system.") {
            if template_file_write_args.overwrite {
                eprintln!(
                    "Overwriting file due to `--overwrite` flag: {}",
                    self.relative_path.to_string_lossy()
                );
            } else {
                eprintln!(
                    "File already exists (pass `--overwrite` to overwrite): {}",
                    self.relative_path.to_string_lossy()
                );
                exit(1);
            }
        }

        let Some(_) = self.relative_path.parent().map(create_dir_all) else {
            eprintln!(
                "Unable to create directory for file: {}",
                self.relative_path.to_string_lossy(),
            );
            exit(1);
        };

        let Ok(mut file) = File::create(&self.relative_path) else {
            eprintln!(
                "Could not open file to write: {}",
                self.relative_path.to_string_lossy()
            );
            exit(1);
        };
        let Ok(()) = file.write_all(self.bytes) else {
            eprintln!(
                "Unable to write file: {}",
                self.relative_path.to_string_lossy()
            );
            exit(1);
        };

        match template_file_write_args.followup {
            Some(TemplateFileCreateFollowup::Edit) => self.open_for_editing(),
            Some(TemplateFileCreateFollowup::Reveal) => {
                self.reveal();
            }
            Some(TemplateFileCreateFollowup::None) => {}
            None => {
                self.open_for_editing();
            }
        };
    }

    pub fn open_for_editing(&self) {
        let Ok(_) = edit_file_without_waiting(&self.relative_path) else {
            eprintln!(
                "Could not open file for editing: {}",
                self.relative_path.to_string_lossy()
            );
            exit(1);
        };
    }

    pub fn reveal(&self) {
        let Ok(()) = reveal(&self.relative_path) else {
            eprintln!(
                "Could not open file for editing {}",
                self.relative_path.to_string_lossy()
            );
            exit(1);
        };
    }
}

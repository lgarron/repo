use std::{
    fs::{create_dir_all, exists, File},
    io::Write,
    path::PathBuf,
};

use clap::{Args, ValueEnum};
use edit::edit_file;
use opener::reveal;

#[derive(Args, Debug)]
pub(crate) struct TemplateFileWriteArgs {
    #[clap(long)]
    followup: Option<TemplateFileWriteFollowup>,
    #[clap(long)]
    overwrite: bool,
}

#[derive(Debug, Clone, ValueEnum)]
enum TemplateFileWriteFollowup {
    // TODO: support `Open` as a version of `Edit` that doesn't wait.
    Edit,
    Reveal,
    None,
}

pub(crate) struct TemplateFile<'a> {
    pub(crate) relative_path: PathBuf,
    pub(crate) bytes: &'a [u8],
}

impl<'a> TemplateFile<'a> {
    /// Autoamtically performs the followup from the `template_file_write_args` argument.
    pub fn write(&self, template_file_write_args: TemplateFileWriteArgs) {
        println!("write");
        if exists(&self.relative_path).expect("Could not access file system.") {
            if template_file_write_args.overwrite {
                eprintln!(
                    "Overwriting file due to `--overwrite` flag: {}",
                    self.relative_path.to_string_lossy()
                );
            } else {
                panic!(
                    "File already exists (pass `--overwrite` to overwrite): {}",
                    self.relative_path.to_string_lossy()
                );
            }
        }

        let Some(_) = self.relative_path.parent().map(create_dir_all) else {
            panic!(
                "Unable to create directory for file: {}",
                self.relative_path.to_string_lossy(),
            );
        };

        let Ok(mut file) = File::create(&self.relative_path) else {
            panic!(
                "Could not open file to write: {}",
                self.relative_path.to_string_lossy()
            );
        };
        file.write_all(self.bytes)
            .expect("Unable to write CI template");

        println!("endy");

        match template_file_write_args.followup {
            Some(TemplateFileWriteFollowup::Edit) => self.open_for_editing(),
            Some(TemplateFileWriteFollowup::Reveal) => {
                self.reveal();
            }
            Some(TemplateFileWriteFollowup::None) => {}
            None => {
                self.open_for_editing();
            }
        };
    }

    pub fn open_for_editing(&self) {
        let Ok(()) = edit_file(&self.relative_path) else {
            panic!(
                "Could not open file for editing {}",
                self.relative_path.to_string_lossy()
            );
        };
    }

    pub fn reveal(&self) {
        let Ok(()) = reveal(&self.relative_path) else {
            panic!(
                "Could not open file for editing {}",
                self.relative_path.to_string_lossy()
            );
        };
    }
}

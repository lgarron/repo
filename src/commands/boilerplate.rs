use std::{path::PathBuf, process::Command};

use clap::{Args, Subcommand};

use crate::common::{
    ecosystem::Ecosystem,
    package_manager::PackageManager,
    template_file::{TemplateFile, TemplateFileArgs},
};

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
    /// Set up linting using Biome
    Biome(TemplateFileArgs),
    /// Set up `rust-toolchain.toml`
    RustToolchain(TemplateFileArgs),
}

fn ci_template() -> TemplateFile<'static> {
    let bytes = include_bytes!("../templates/.github/workflows/CI.yaml");
    TemplateFile {
        relative_path: PathBuf::from("./.github/workflows/CI.yaml"),
        bytes,
    }
}

fn publish_github_release_template() -> TemplateFile<'static> {
    let bytes = include_bytes!("../templates/.github/workflows/publish-github-release.yaml");
    TemplateFile {
        relative_path: PathBuf::from("./.github/workflows/publish-github-release.yaml"),
        bytes,
    }
}

fn biome_json_template() -> TemplateFile<'static> {
    let bytes = include_bytes!("../templates/biome.json");
    TemplateFile {
        relative_path: PathBuf::from("./biome.json"),
        bytes,
    }
}

fn rust_toolchain_template() -> TemplateFile<'static> {
    let bytes = include_bytes!("../templates/rust-toolchain.toml");
    TemplateFile {
        relative_path: PathBuf::from("./rust-toolchain.toml"),
        bytes,
    }
}

fn add_biome(template_file_args: TemplateFileArgs) {
    let (binary, args, biome_command_prefix) =
        match PackageManager::auto_detect_preferred_package_manager_for_ecosystem(
            Ecosystem::JavaScript,
        ) {
            // TODO: generalize to a function to add a dependency
            Some(PackageManager::Npm) => (
                "npm",
                ["install", "--save-dev", "@biomejs/biome"],
                "bun x @biomejs/biome",
            ),
            Some(PackageManager::Bun) => (
                "bun",
                ["add", "--development", "@biomejs/biome"],
                "bun x biome",
            ),
            Some(PackageManager::Yarn) => (
                "yarn",
                ["add", "--dev", "@biomejs/biome"],
                "npx yarn exec @biomejs/biome",
            ),
            Some(PackageManager::Pnpm) => (
                "pnpm",
                ["install", "--save-dev", "@biomejs/biome"],
                "npx pnpm exec biome",
            ),
            Some(PackageManager::Cargo) => panic!("unrechachable"),
            None => {
                panic!("No JS package detected.")
            }
        };
    Command::new(binary)
        .args(args)
        .spawn()
        .expect("Could not add development dependency")
        .wait()
        .unwrap();
    biome_json_template().handle_command(template_file_args);
    println!(
        "Use the following commands:

`package.json`:

\"lint\": \"{} check\"
\"format\": \"{} write\"

`Makefile`:

.PHONY: lint
lint:
\t{} check

.PHONY: format
format:
\t{} check --write
",
        biome_command_prefix, biome_command_prefix, biome_command_prefix, biome_command_prefix,
    )
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
        BoilerplateCommand::Biome(template_file_args) => add_biome(template_file_args),
        BoilerplateCommand::RustToolchain(template_file_args) => {
            rust_toolchain_template().handle_command(template_file_args)
        }
    };
}

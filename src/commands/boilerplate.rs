use std::path::PathBuf;

use clap::{Args, Subcommand};
use printable_shell_command::PrintableShellCommand;

use crate::common::{
    debug::DebugPrintable,
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
    /// Set up `tsconfig.json`
    Tsconfig(TemplateFileArgs),
    /// Set up `readme-cli-help.json`
    ReadmeCliHelp(TemplateFileArgs),
    /// Set up `bunfig.toml`
    Bunfig(TemplateFileArgs),
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
    let bytes = include_bytes!("../templates/workaround-indirection-folder/biome.json");
    TemplateFile {
        relative_path: PathBuf::from("./biome.json"),
        bytes,
    }
}

fn tsconfig_template() -> TemplateFile<'static> {
    let bytes = include_bytes!("../templates/tsconfig.json");
    TemplateFile {
        relative_path: PathBuf::from("./tsconfig.json"),
        bytes,
    }
}

fn bunfig_template() -> TemplateFile<'static> {
    let bytes = include_bytes!("../templates/bunfig.toml");
    TemplateFile {
        relative_path: PathBuf::from("./bunfig.toml"),
        bytes,
    }
}

fn readme_cli_help_template() -> TemplateFile<'static> {
    let bytes = include_bytes!("../templates/.config/readme-cli-help.json");
    TemplateFile {
        relative_path: PathBuf::from("./.config/readme-cli-help.json"),
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
                [
                    "install",
                    "--save-dev",
                    "@biomejs/biome",
                    "@cubing/dev-config",
                ],
                "bun x @biomejs/biome",
            ),
            Some(PackageManager::Bun) => (
                "bun",
                [
                    "add",
                    "--development",
                    "@biomejs/biome",
                    "@cubing/dev-config",
                ],
                "bun x @biomejs/biome",
            ),
            Some(PackageManager::Yarn) => (
                "yarn",
                ["add", "--dev", "@biomejs/biome", "@cubing/dev-config"],
                "npx yarn exec @biomejs/biome",
            ),
            Some(PackageManager::Pnpm) => (
                "pnpm",
                [
                    "install",
                    "--save-dev",
                    "@biomejs/biome",
                    "@cubing/dev-config",
                ],
                "npx pnpm exec biome",
            ),
            Some(PackageManager::Cargo) => panic!("unrechachable"),
            None => {
                panic!("No JS package detected.")
            }
        };
    PrintableShellCommand::new(binary)
        .arg_each(args)
        .debug_print()
        .spawn()
        .expect("Could not add development dependency")
        .wait()
        .unwrap();
    biome_json_template().handle_command(template_file_args);
    println!(
        "Use the following commands:

`package.json`:

\"lint\": \"{} check\"
\"format\": \"{} check --write\"

`Makefile` (make sure to convert ⇥ to tab indentation):

.PHONY: lint
lint:
⇥{} check

.PHONY: format
format:
⇥{} check --write
",
        biome_command_prefix, biome_command_prefix, biome_command_prefix, biome_command_prefix,
    )
}

fn add_tsconfig(template_file_args: TemplateFileArgs) {
    // Note that we don't install the `typescript` package because
    // `tsconfig.json` is still needed to get VS Code's built-in TypeScript
    // annotations to accept some well-established features like top-level
    // `await` (even if the project itself doesn't use `tsc`).
    let (binary, args) = match PackageManager::auto_detect_preferred_package_manager_for_ecosystem(
        Ecosystem::JavaScript,
    ) {
        // TODO: generalize to a function to add a dependency
        Some(PackageManager::Npm) => ("npm", ["install", "--save-dev", "@cubing/dev-config"]),
        Some(PackageManager::Bun) => ("bun", ["add", "--development", "@cubing/dev-config"]),
        Some(PackageManager::Yarn) => ("yarn", ["add", "--dev", "@cubing/dev-config"]),
        Some(PackageManager::Pnpm) => ("pnpm", ["install", "--save-dev", "@cubing/dev-config"]),
        Some(PackageManager::Cargo) => panic!("unrechachable"),
        None => {
            panic!("No JS package detected.")
        }
    };
    PrintableShellCommand::new(binary)
        .arg_each(args)
        .spawn()
        .expect("Could not add development dependency")
        .wait()
        .unwrap();
    tsconfig_template().handle_command(template_file_args);
    // TODO: print `tsc` invocation (requires installation)
}

fn add_bunfig(template_file_args: TemplateFileArgs) {
    bunfig_template().handle_command(template_file_args);
    // TODO: print `tsc` invocation (requires installation)
}

fn add_readme_cli_help(template_file_args: TemplateFileArgs) {
    readme_cli_help_template().handle_command(template_file_args);
    // TODO: print `readme-cli-help` invocation
}

fn add_rust_toolchain(template_file_args: TemplateFileArgs) {
    rust_toolchain_template().handle_command(template_file_args);
    // TODO: mention `test-cargo-doc`?
    println!(
        "Use the following commands:

`Makefile` (make sure to convert ⇥ to tab indentation):

.PHONY: lint-rust
lint-rust:
⇥cargo clippy -- --deny warnings
⇥cargo fmt --check


.PHONY: format-rust
format-rust:
⇥cargo clippy --fix --allow-no-vcs
⇥cargo fmt
"
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
        BoilerplateCommand::Tsconfig(template_file_args) => add_tsconfig(template_file_args),
        BoilerplateCommand::Bunfig(template_file_args) => add_bunfig(template_file_args),
        BoilerplateCommand::ReadmeCliHelp(template_file_args) => {
            add_readme_cli_help(template_file_args)
        }
        BoilerplateCommand::RustToolchain(template_file_args) => {
            add_rust_toolchain(template_file_args)
        }
    };
}

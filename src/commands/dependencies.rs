use std::{collections::HashMap, fs::File, io::BufReader, path::PathBuf};

use cargo_metadata::semver::Version;
use clap::{Args, Subcommand};
use printable_shell_command::PrintableShellCommand;
use serde::{Deserialize, Serialize};

use crate::{
    commands::version::{detect_ecosystem_by_getting_version, CommitOperationArgs},
    common::{
        commit_wrapped_operation::CommitWrappedOperation,
        ecosystem::EcosystemArgs,
        inference::get_stdout,
        package_manager::{PackageManager, PackageManagerArgs},
    },
};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
struct DependencyName(String);

impl From<String> for DependencyName {
    fn from(value: String) -> Self {
        Self(value)
    }
}

#[derive(Args, Debug)]
pub(crate) struct DependenciesArgs {
    #[command(flatten)]
    ecosystem_args: EcosystemArgs,

    #[command(flatten)]
    package_manager_args: PackageManagerArgs,

    #[command(subcommand)]
    command: DependenciesCommand,
}

#[derive(Debug, Subcommand)]
enum DependenciesCommand {
    Roll(DependenciesCommandArgs),
}

#[derive(Args, Debug)]
pub(crate) struct DependenciesCommandArgs {
    dependency_name: DependencyName,

    #[command(flatten)]
    commit_args: CommitOperationArgs,
}

#[derive(Debug)]
enum NpmDependencyType {
    // `npm` uses "prod" as a synonym.
    Prod,
    Dev,
    Peer,
    Optional,
}

impl NpmDependencyType {
    fn all_types() -> &'static [NpmDependencyType] {
        &[Self::Prod, Self::Dev, Self::Peer, Self::Optional]
    }

    fn npm_install_arg(&self) -> &str {
        match self {
            NpmDependencyType::Prod => "--save",
            NpmDependencyType::Dev => "--save-dev",
            NpmDependencyType::Peer => "--save-peer",
            NpmDependencyType::Optional => "--save-optional",
        }
    }

    fn bun_add_arg(&self) -> Option<&str> {
        match self {
            NpmDependencyType::Prod => None,
            NpmDependencyType::Dev => Some("--dev"),
            NpmDependencyType::Peer => Some("--peer"),
            NpmDependencyType::Optional => Some("--optional"),
        }
    }
}

type DependencyInfo = Option<HashMap<DependencyName, String>>;

// TODO: can field values be non-strings?
#[derive(Deserialize)]
struct PackageJSONSubset {
    dependencies: DependencyInfo,
    #[serde(rename = "devDependencies")]
    dev_dependencies: DependencyInfo,
    #[serde(rename = "peerDependencies")]
    peer_dependencies: DependencyInfo,
    #[serde(rename = "optionalDependencies")]
    optional_dependencies: DependencyInfo,
}

impl PackageJSONSubset {
    fn get_dependencies_of_type(&self, t: &NpmDependencyType) -> &DependencyInfo {
        match t {
            NpmDependencyType::Prod => &self.dependencies,
            NpmDependencyType::Dev => &self.dev_dependencies,
            NpmDependencyType::Peer => &self.peer_dependencies,
            NpmDependencyType::Optional => &self.optional_dependencies,
        }
    }

    fn has_dependency_of_type(
        &self,
        npm_dependency_type: &NpmDependencyType,
        dependency_name: &DependencyName,
    ) -> bool {
        let v = self.get_dependencies_of_type(npm_dependency_type);
        if let Some(v) = v {
            if v.contains_key(dependency_name) {
                return true;
            }
        }
        false
    }
}

fn must_get_package_json() -> PackageJSONSubset {
    let mut npm_command = PrintableShellCommand::new("npm");
    npm_command.args(["root"]);
    let node_modules_folder = get_stdout(npm_command).unwrap();
    let package_json_path = PathBuf::from(node_modules_folder)
        .parent()
        .unwrap()
        .join("package.json");
    let file = File::open(package_json_path).unwrap();
    let reader = BufReader::new(file);

    // TODO: get a stream instead?
    serde_json::from_reader(reader).unwrap()
}

fn npm_show_version(dependency_name: &DependencyName) -> Version {
    let mut npm_command = PrintableShellCommand::new("npm");
    // `--` is needed because packages can start with `-` and we want to prevent any chance of argument injection.
    npm_command.args(["show", "--", &dependency_name.0, "version"]);
    Version::parse(get_stdout(npm_command).unwrap().trim()).unwrap()
}

fn npm_install(
    dependency_type: &NpmDependencyType,
    dependency_name: &DependencyName,
    new_version: &Version,
) -> String {
    let mut npm_command = PrintableShellCommand::new("npm");
    let args = [
        "install",
        dependency_type.npm_install_arg(),
        // `--` is needed because packages can start with `-` and we want to prevent any chance of argument injection.
        "--",
        &format!("{}@^{}", &dependency_name.0, new_version),
    ];
    npm_command.args(args);
    let _ = get_stdout(npm_command).unwrap();
    // TODO: robust escaping.
    format!(
        "npm {} {} {} '{}'",
        args[0],
        args[1],
        args[2],
        args[3].replace("'", "\\'")
    )
}

fn bun_add(
    dependency_type: &NpmDependencyType,
    dependency_name: &DependencyName,
    new_version: &Version,
) -> String {
    let mut bun_command = PrintableShellCommand::new("bun");
    let mut args = vec!["add"];
    if let Some(arg) = dependency_type.bun_add_arg() {
        args.push(arg);
    }
    // `--` is needed because packages can start with `-` and we want to prevent any chance of argument injection.
    args.push("--");
    let dependency_arg = format!("{}@^{}", &dependency_name.0, new_version);
    args.push(&dependency_arg);
    // TODO: robust escaping.
    let command_string = format!(
        "bun {} '{}'",
        args[0..(args.len() - 2)].join(" "),
        args[args.len() - 1].replace("'", "\\'")
    );
    bun_command.args(args);
    let _ = get_stdout(bun_command).unwrap();
    command_string
}

pub(crate) fn dependencies_command(dependencies_args: DependenciesArgs) -> Result<(), String> {
    match dependencies_args.command {
        DependenciesCommand::Roll(dependencies_command_args) => {
            let package_manager = match &dependencies_args.package_manager_args.package_manager {
                Some(package_manager) => package_manager.clone(),
                None => {
                    // TODO: handle projects without version.
                    let Some((ecosystem, _)) =
                        detect_ecosystem_by_getting_version(&dependencies_args.ecosystem_args)
                    else {
                        return Err("Could not detect ecosystem.".to_owned());
                    };

                    let Some(package_manager) =
                        PackageManager::auto_detect_preferred_package_manager_for_ecosystem(
                            ecosystem,
                        )
                    else {
                        return Err("Could not detect package manager.".to_owned());
                    };

                    package_manager
                }
            };

            let dependency_name = &dependencies_command_args.dependency_name;

            match package_manager {
                PackageManager::Npm | PackageManager::Bun => {
                    let package_json = must_get_package_json();
                    let new_version = &npm_show_version(dependency_name);
                    // TODO: compare version against installed.
                    let mut any_rolled = false;
                    for npm_dependency_type in NpmDependencyType::all_types() {
                        if package_json.has_dependency_of_type(npm_dependency_type, dependency_name)
                        {
                            let commit_wrapped_operation = CommitWrappedOperation::try_from(
                                &dependencies_command_args.commit_args,
                            )
                            .unwrap();
                            commit_wrapped_operation
                                .perform_operation(&|| {
                                    let command = if package_manager == PackageManager::Npm {
                                        npm_install(
                                            npm_dependency_type,
                                            dependency_name,
                                            new_version,
                                        )
                                    } else {
                                        bun_add(npm_dependency_type, dependency_name, new_version)
                                    };
                                    println!("{}", command);
                                    Ok(command)
                                })
                                .unwrap();
                            any_rolled = true;
                        }
                    }
                    if !any_rolled {
                        eprintln!(
                            "⚠️ Must already have as a dependency in order to roll versions: {}",
                            &dependency_name.0
                        )
                    }
                    Ok(())
                }
                package_manager => Err(format!(
                    "Dependency rolling is not implemented for package manager: {}",
                    package_manager
                )),
            }
        }
    }
}

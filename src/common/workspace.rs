use std::{fs::exists, path::Path};

use printable_shell_command::PrintableShellCommand;
use serde::Deserialize;

use crate::common::package_manager::PACKAGE_JSON_PATH;

use super::{inference::get_stdout, vcs::auto_detect_preferred_vcs_and_repo_root};

#[derive(Deserialize)]
struct CargoMetadataSubset {
    workspace_root: String,
}

const GO_MOD_PATH: &str = "go.mod";

// TODO: return the inference reason(s)
pub(crate) fn auto_detect_workspace_root(
    // TODO: accept file path?
    path_of_folder_or_subfolder: &Path,
) -> Option<String> {
    if let Some((_, root)) = auto_detect_preferred_vcs_and_repo_root(path_of_folder_or_subfolder) {
        return Some(root);
    }
    {
        // We would use https://docs.rs/cargo_metadata/0.20.0/cargo_metadata/
        // but that requires a manifest path… which is what we're trying to
        // fins in the first place.
        let mut cargo_command = PrintableShellCommand::new("cargo");
        cargo_command
            .current_dir(path_of_folder_or_subfolder)
            .args(["-C", &path_of_folder_or_subfolder.to_string_lossy()])
            .arg("metadata")
            .args(["--format-version", "1"]);
        if let Some(metadata_json_string) = get_stdout(cargo_command) {
            if let Ok(metadata) = serde_json::from_str::<CargoMetadataSubset>(&metadata_json_string)
            {
                return Some(metadata.workspace_root);
            }
        }
    }
    let mut dir_or_ancestor = Some(path_of_folder_or_subfolder);
    while let Some(dir) = dir_or_ancestor {
        {
            let package_json_path = dir.join(Path::new(PACKAGE_JSON_PATH));
            if exists(package_json_path).unwrap_or(false) {
                // TODO: check that the folder contains the expected `.hg` dir?
                return Some(dir.to_string_lossy().to_string());
            }
        }
        {
            let go_mod_path = dir.join(Path::new(GO_MOD_PATH));
            if exists(go_mod_path).unwrap_or(false) {
                // TODO: check that the folder contains the expected `.hg` dir?
                return Some(dir.to_string_lossy().to_string());
            }
        }
        dir_or_ancestor = dir.parent();
    }
    None
}

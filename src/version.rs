use cargo_metadata::MetadataCommand;

use crate::args::VersionArgs;

pub(crate) fn version(version_args: VersionArgs) {
    match version_args.command {
        crate::args::VersionCommand::Get => {
            println!(
                "{}",
                MetadataCommand::new()
                    .manifest_path("./Cargo.toml")
                    .current_dir(".")
                    .exec()
                    .unwrap()
                    .root_package()
                    .unwrap()
                    .version
            );
        }
    }
}

use args::get_args;
use ci::ci_command;
use publish::publish_command;
use version::version_command;

mod args;
mod ci;
mod ecosystem;
mod publish;
mod version;

fn main() {
    let args = get_args();

    match args.command {
        args::RepoCommand::Version(version_args) => version_command(version_args),
        args::RepoCommand::CI(ci_args) => ci_command(ci_args),
        args::RepoCommand::Publish(publish_args) => publish_command(publish_args),
        args::RepoCommand::Completions(_) => panic!("We should have exited earlier."),
    }
}

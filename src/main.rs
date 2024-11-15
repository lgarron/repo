use args::get_args;
use ci::ci_command;
use version::version_command;

mod args;
mod ci;
mod version;

fn main() {
    let args = get_args();

    match args.command {
        args::RepoCommand::Version(args) => version_command(args),
        args::RepoCommand::CI(args) => ci_command(args),
        args::RepoCommand::Completions(_) => panic!("We should have exited earlier."),
    }
}

use args::get_args;
use version::version;

mod args;
mod version;

fn main() {
    let args = get_args();

    match args.command {
        args::RepoCommand::Version(version_args) => version(version_args),
        args::RepoCommand::Completions(_) => panic!("We should have exited earlier."),
    }
}

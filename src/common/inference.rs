use std::process::{Command, Stdio};

use crate::common::debug::DebugPrintable;

// Gracefully recovers from any error by returning `None`.
// Currently trims the output.
pub(crate) fn get_stdout(mut command: Command) -> Option<String> {
    command.debug_print();
    command.stdout(Stdio::piped()).stderr(Stdio::null());
    let Ok(exit_status) = command.status() else {
        return None;
    };

    if exit_status.code() == Some(0) {
        if let Ok(stdout) = command.output() {
            if let Ok(path) = String::from_utf8(stdout.stdout) {
                // TODO: check that the folder contains the expected `.git` dir/file?
                return Some(path.trim().to_string());
            }
        }
    }
    None
}

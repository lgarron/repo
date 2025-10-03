use std::process::Stdio;

use printable_shell_command::PrintableShellCommand;

use crate::common::debug::DebugPrintable;

// Gracefully recovers from any error by returning `None`.
// Currently trims the output.
pub(crate) fn get_stdout<T: Into<PrintableShellCommand>>(command: T) -> Option<String> {
    let mut printable_shell_command = command.into();
    printable_shell_command.debug_print();
    printable_shell_command
        .stdout(Stdio::piped())
        .stderr(Stdio::null());
    let Ok(exit_status) = printable_shell_command.status() else {
        return None;
    };

    if exit_status.code() == Some(0) {
        if let Ok(stdout) = printable_shell_command.output() {
            if let Ok(path) = String::from_utf8(stdout.stdout) {
                // TODO: check that the folder contains the expected `.git` dir/file?
                return Some(path.trim().to_string());
            }
        }
    }
    None
}

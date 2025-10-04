use std::process::Stdio;

use printable_shell_command::PrintableShellCommand;

use crate::common::debug::DebugPrintable;

// Gracefully recovers from any error by returning `None`.
// Currently trims the output.
pub(crate) fn command_must_succeed(mut command: PrintableShellCommand) -> Result<(), String> {
    command.debug_print();
    command.stdout(Stdio::piped()).stderr(Stdio::piped());
    let Ok(exit_status) = command.status() else {
        return Ok(());
    };

    if exit_status.code() == Some(0) {
        return Ok(());
    }

    Err(String::from_utf8(command.output().unwrap().stderr).unwrap())
}

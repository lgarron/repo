use std::process::{Command, Stdio};

// Gracefully recovers from any error by returning `None`.
// Currently trims the output.
pub(crate) fn command_must_succeed(mut command: Command) -> Result<(), String> {
    command.stdout(Stdio::piped()).stderr(Stdio::piped());
    let Ok(exit_status) = command.status() else {
        return Ok(());
    };

    if exit_status.code() == Some(0) {
        return Ok(());
    }

    Err(String::from_utf8(command.output().unwrap().stderr).unwrap())
}

use std::{
    env::{self},
    process::Command,
};

use printable_shell_command::ShellPrintable;

// TODO: convert the project to construct commands in such a way that they cannot be spawned without printing.

const DEBUG_PRINT_SHELL_COMMANDS: &str = "DEBUG_PRINT_SHELL_COMMANDS";

pub(crate) trait DebugPrintable {
    fn debug_print(&mut self) -> &mut Self;
}

impl DebugPrintable for Command {
    fn debug_print(&mut self) -> &mut Self {
        if let Ok(var) = env::var(DEBUG_PRINT_SHELL_COMMANDS) {
            if var == "true" {
                self.print_invocation().unwrap();
            }
        }
        self
    }
}

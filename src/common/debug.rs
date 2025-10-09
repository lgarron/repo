use std::{
    env::{self},
    io::{stderr, IsTerminal},
};

use colored::{Colorize, CustomColor};
use printable_shell_command::{
    FormattingOptions, PrintableShellCommand, ShellPrintableWithOptions,
};

// TODO: convert the project to construct commands in such a way that they cannot be spawned without printing.

const DEBUG_PRINT_SHELL_COMMANDS: &str = "DEBUG_PRINT_SHELL_COMMANDS";

pub(crate) trait DebugPrintable {
    fn debug_print(&mut self) -> &mut Self;
}

impl DebugPrintable for PrintableShellCommand {
    fn debug_print(&mut self) -> &mut Self {
        if let Ok(var) = env::var(DEBUG_PRINT_SHELL_COMMANDS) {
            if var == "true" {
                let s = self
                    .printable_invocation_string_with_options(FormattingOptions {
                        skip_line_wrap_before_first_arg: Some(true),
                        ..Default::default()
                    })
                    .unwrap();
                if stderr().is_terminal() {
                    eprintln!(
                        "{}",
                        s.custom_color(CustomColor {
                            r: 128,
                            g: 128,
                            b: 128
                        })
                    );
                } else {
                    eprintln!("{}", s);
                }
            }
        }
        self
    }
}

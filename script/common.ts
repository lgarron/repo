import { Path } from "path-class";
import { PrintableShellCommand } from "printable-shell-command";

export const MAIN_PACKAGE_FOLDER = new Path("./src/js/@lgarron-bin/repo");

export const version = (
  await new PrintableShellCommand("cargo", [
    "run",
    "--quiet",
    "--",
    "version",
    "get",
  ])
    .stdout()
    .text()
).trim();

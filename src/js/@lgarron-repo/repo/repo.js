import { exists } from "node:fs/promises";
import { join } from "node:path";
import { argv, exit } from "node:process";
import { fileURLToPath } from "node:url";
import { PrintableShellCommand } from "printable-shell-command";

for (const architectureTriple of [
  "aarch64-apple-darwin",
  "x86_64-unknown-linux-gnu",
  "x86_64-apple-darwin",
  "x86_64-pc-windows",
]) {
  const path = fileURLToPath(
    import.meta.resolve(join("..", `repo-${architectureTriple}`, "repo")),
  );
  if (await exists(path)) {
    let command;
    try {
      command = new PrintableShellCommand(
        path,
        argv.slice(2),
      ).spawnNodeInherit();
    } catch (e) {
      if (e.code === "EBADARCH") {
        continue;
      }
    }
    try {
      await command.success;
    } catch (e) {
      console.log(e);
    }
    exit(command.exitCode);
  }
}

console.error(
  "Could not find a `repo` binary compatible with the current architecture.",
);
exit(1);

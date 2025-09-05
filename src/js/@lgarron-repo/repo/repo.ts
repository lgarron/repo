import { exists } from "node:fs/promises";
import { join } from "node:path";
import { argv, exit } from "node:process";
import { fileURLToPath } from "node:url";
import { PrintableShellCommand } from "printable-shell-command";

// const require = createRequire(import.meta.url);

for (const architectureTriple of [
  "x86_64-apple-darwin",
  "aarch64-apple-darwin",
  "x86_64-pc-windows",
  "x86_64-unknown-linux-gnu",
]) {
  const path = fileURLToPath(
    import.meta.resolve(join("..", `repo-${architectureTriple}`, "repo")),
  );
  if (await exists(path)) {
    const command = new PrintableShellCommand(
      path,
      argv.slice(2),
    ).spawnNodeInherit();
    try {
      await command.success;
    } catch {}
    exit(command.exitCode);
  }
}

console.error(
  "Could not find a `repo` binary compatible with the current architecture.",
);
exit(1);

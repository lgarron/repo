import { spawn } from "node:child_process";
import { existsSync } from "node:fs";
import { argv, exit } from "node:process";
import { fileURLToPath } from "node:url";

for (const architectureTriple of [
  "aarch64-apple-darwin",
  "x86_64-unknown-linux-gnu",
  "x86_64-apple-darwin",
  "x86_64-pc-windows",
]) {
  let path;
  try {
    path = fileURLToPath(
      import.meta.resolve(`@lgarron-repo/repo-${architectureTriple}`),
    );
  } catch (e) {
    if (e.code === "ERR_MODULE_NOT_FOUND") {
      continue;
    }
  }
  if (await existsSync(path)) {
    let command;
    try {
      command = spawn(path, argv.slice(2), { stdio: "inherit" });
    } catch (e) {
      if (e.code === "EBADARCH") {
        continue;
      }
    }
    await new Promise((resolve) => command.addListener("exit", resolve));
    exit(command.exitCode);
  }
}

console.error(
  "Could not find a `repo` binary compatible with the current architecture.",
);
exit(1);

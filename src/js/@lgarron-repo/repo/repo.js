#!/usr/bin/env node

import { spawn } from "node:child_process";
import { existsSync } from "node:fs";
import { argv, env, exit } from "node:process";
import { fileURLToPath } from "node:url";
import { architectures } from "./architectures";

// biome-ignore lint/complexity/useLiteralKeys: https://github.com/biomejs/biome/issues/463 or https://github.com/biomejs/biome/issues/6736
const DEBUG = env["REPO_DEBUG_NPM_RESOLUTION"] === "true";

for (const { rustTarget } of architectures) {
  if (DEBUG) {
    console.error(
      `--------
[${rustTarget}] Testing Rust target: ${rustTarget}`,
    );
  }
  let path;
  try {
    path = fileURLToPath(
      import.meta.resolve(`@lgarron-repo/repo-${rustTarget}`),
    );
    if (DEBUG) {
      console.error(`[${rustTarget}] Resolved to path: `, path);
    }
  } catch (/** @type {any} */ e) {
    if (e.code === "ERR_MODULE_NOT_FOUND") {
      if (DEBUG) {
        console.error(
          `[${rustTarget}] Failed to resolve. Continuing to next target.`,
        );
      }
      continue;
    }
    if (DEBUG) {
      console.error(`[${rustTarget}] Unexpected error during resolution: `, e);
    }
    throw e;
  }
  if (await existsSync(path)) {
    if (DEBUG) {
      console.error(`[${rustTarget}] Path exists: `, path);
    }
    let command;
    try {
      command = spawn(path, argv.slice(2), { stdio: "inherit" });
    } catch (/** @type {any} */ e) {
      if (e.code === "EBADARCH") {
        if (DEBUG) {
          console.error(
            `[${rustTarget}] Bad architecture. Continuing to next target`,
          );
        }
        continue;
      }
      if (DEBUG) {
        console.error(
          `[${rustTarget}] Unexpected error during binary launch: `,
          e,
        );
      }
      throw e;
    }
    await new Promise((resolve) => command.addListener("exit", resolve));
    exit(command.exitCode);
  }
}

console.error(
  "Could not find a `repo` binary compatible with the current architecture.",
);
exit(1);

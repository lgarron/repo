#!/usr/bin/env node

import { spawn } from "node:child_process";
import { existsSync } from "node:fs";
import { argv, env, exit } from "node:process";
import { fileURLToPath } from "node:url";

// biome-ignore lint/complexity/useLiteralKeys: https://github.com/biomejs/biome/issues/463 or https://github.com/biomejs/biome/issues/6736
const DEBUG = env["REPO_DEBUG_NPM_RESOLUTION"] === "true";

for (const architectureTriple of [
  "aarch64-apple-darwin",
  "x86_64-unknown-linux-gnu",
  "x86_64-apple-darwin",
  "aarch64-unknown-linux-gnu",
  "x86_64-pc-windows",
  "aarch64-pc-windows",
]) {
  if (DEBUG) {
    console.error(
      `--------
[${architectureTriple}] Testing architecture triple: ${architectureTriple}`,
    );
  }
  let path;
  try {
    path = fileURLToPath(
      import.meta.resolve(`@lgarron-bin/repo-${architectureTriple}`),
    );
    if (DEBUG) {
      console.error(`[${architectureTriple}] Resolved to path: `, path);
    }
  } catch (/** @type {any} */ e) {
    if (e.code === "ERR_MODULE_NOT_FOUND") {
      if (DEBUG) {
        console.error(
          `[${architectureTriple}] Failed to resolve. Continuing to next architecture.`,
        );
      }
      continue;
    }
    if (DEBUG) {
      console.error(
        `[${architectureTriple}] Unexpected error during resolution: `,
        e,
      );
    }
    throw e;
  }
  if (await existsSync(path)) {
    if (DEBUG) {
      console.error(`[${architectureTriple}] Path exists: `, path);
    }
    let command;
    try {
      command = spawn(path, argv.slice(2), { stdio: "inherit" });
    } catch (/** @type {any} */ e) {
      if (e.code === "EBADARCH") {
        if (DEBUG) {
          console.error(
            `[${architectureTriple}] Bad architecture. Continuing to next architecture.`,
          );
        }
        continue;
      }
      if (DEBUG) {
        console.error(
          `[${architectureTriple}] Unexpected error during binary launch: `,
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

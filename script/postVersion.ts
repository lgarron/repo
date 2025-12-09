#!/usr/bin/env -S bun run --

import { stdin } from "node:process";
import { Readable } from "node:stream";

console.log("`postVersion` was called using the following JSON:");
console.log(
  JSON.stringify(
    // biome-ignore lint/suspicious/noExplicitAny: Why doesn't this type check?
    await new Response(Readable.from(stdin) as any).json(),
    null,
    "  ",
  ),
);

import assert from "node:assert";
import { compile, type JSONSchema } from "json-schema-to-typescript";
import { PrintableShellCommand } from "printable-shell-command";
import { MAIN_PACKAGE_FOLDER } from "./common";

await MAIN_PACKAGE_FOLDER.join("schemas/repo.json").write(
  new PrintableShellCommand("cargo", [
    "run",
    "print-schema",
    "config",
  ]).stdout(),
);

const postVersionSchemaJSON: JSONSchema = await new PrintableShellCommand(
  "cargo",
  ["run", "--quiet", "--", "print-schema", "postVersion"],
).json();
// biome-ignore lint/complexity/useLiteralKeys: https://github.com/biomejs/biome/discussions/7404
// biome-ignore lint/style/noNonNullAssertion: The `"properties"` field is always present in our output.
const magnitudeAnyOf = postVersionSchemaJSON.properties!["magnitude"].anyOf;
assert(magnitudeAnyOf);
assert(magnitudeAnyOf.length === 2);
assert.deepEqual(magnitudeAnyOf[1], { type: "null" });
// Work around https://github.com/GREsau/schemars/issues/491
// biome-ignore lint/complexity/useLiteralKeys: https://github.com/biomejs/biome/discussions/7404
// biome-ignore lint/style/noNonNullAssertion: The `"properties"` field is always present in our output.
postVersionSchemaJSON.properties!["magnitude"] = magnitudeAnyOf[0];

await MAIN_PACKAGE_FOLDER.join("./types/postVersion.d.ts").write(
  // TODO: use https://github.com/Aleph-Alpha/ts-rs directly?
  compile(postVersionSchemaJSON, "postVersion", {
    bannerComment: "",
  }),
);

new PrintableShellCommand("bun", [
  "x",
  "@biomejs/biome",
  "./src/js/@lgarron-bin/repo/",
]);

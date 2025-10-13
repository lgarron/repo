import { cp, mkdir, rename, rm } from "node:fs/promises";
import { join } from "node:path";
import { exit } from "node:process";
import { $, file, sleep } from "bun";

const WORKFLOW_NAME = "Build release binaries";
const MILLISECONDS_PER_SECOND = 1000;

await mkdir("./.temp", { recursive: true });

// Dogfood our own hash calculation.
const commitSHA = await $`cargo run --quiet  -- vcs latest-commit hash`.text();

const run = await (async () => {
  while (true) {
    const runs: {
      workflow_runs: {
        id: number;
        name: string;
        // https://docs.github.com/en/rest/actions/workflow-runs?apiVersion=2022-11-28#list-workflow-runs-for-a-repository
        status: "completed" | string;
      }[];
    } =
      await $`gh api "/repos/lgarron/repo/actions/runs?head_sha=${commitSHA}"`.json();

    const run = runs.workflow_runs.filter(
      (run) => run.name === WORKFLOW_NAME,
    )[0];
    if (!run) {
      console.error(
        `Workflow run \"${WORKFLOW_NAME}\" is not available for this commit: ${commitSHA}
Push a tag to run the release flow.`,
      );
      exit(2);
    }
    console.log(`Workflow run id: ${run.id}`);

    if (run.status === "completed") {
      return run;
    }

    console.info("Workflow is not complete, waiting 10 seconds…");
    await sleep(10 * MILLISECONDS_PER_SECOND);
  }
})();

const data: {
  artifacts: { name: string; id: number; archive_download_url: string }[];
} = await (
  await fetch(
    "https://api.github.com/repos/lgarron/repo/actions/runs/" +
      run.id +
      "/artifacts",
  )
).json();

const downloads = Object.fromEntries(
  data.artifacts.map((entry) => [entry.name, entry]),
);

console.log(downloads);

const ARCHITECTURE_TRIPLES: {
  triple: string;
  npmOS: string[];
  npmCPU: string[];
}[] = [
  { triple: "x86_64-apple-darwin", npmOS: ["darwin"], npmCPU: ["x64"] },
  { triple: "aarch64-apple-darwin", npmOS: ["darwin"], npmCPU: ["arm64"] },
  { triple: "x86_64-pc-windows", npmOS: ["win32"], npmCPU: ["x64"] },
  { triple: "aarch64-pc-windows", npmOS: ["win32"], npmCPU: ["arm64"] },
  { triple: "x86_64-unknown-linux-gnu", npmOS: ["linux"], npmCPU: ["x64"] },
  { triple: "aarch64-unknown-linux-gnu", npmOS: ["linux"], npmCPU: ["arm64"] },
];

function isWindows(architectureTriple: string): boolean {
  return architectureTriple.endsWith("-windows");
}

const TEMP_DIR = "./.temp/artifacts";
await rm(TEMP_DIR, { recursive: true, force: true });

const version = (await $`cargo run --quiet -- version get`.text()).trim();

for (const { triple, npmOS, npmCPU } of ARCHITECTURE_TRIPLES) {
  const downloadInfo = downloads[`repo.${triple}`];
  console.log(triple);
  const ZIP_PARENT_DIR = join(TEMP_DIR, triple);
  const ZIP_PATH = join(TEMP_DIR, `${triple}.zip`);
  await mkdir(ZIP_PARENT_DIR, { recursive: true });
  await $`gh api /repos/lgarron/repo/actions/artifacts/${downloadInfo.id}/zip > ${ZIP_PATH}`;
  // `-o` means "overwrite"
  const PACKAGE_DIR = `./src/js/@lgarron-bin/repo-${triple}`;
  await $`unzip -o -d ${PACKAGE_DIR} ${ZIP_PATH}`;

  const suffix = isWindows(triple) ? ".exe" : "";
  await rename(
    join(PACKAGE_DIR, `repo${suffix}`),
    join(PACKAGE_DIR, `repo-${triple}${suffix}`),
  );

  const name = `@lgarron-bin/repo-${triple}`;
  await file(join(PACKAGE_DIR, "package.json")).write(
    JSON.stringify(
      {
        name,
        version: version,
        repository: "github:lgarron/repo",
        type: "module",
        os: npmOS,
        cpu: npmCPU,
        bin: {
          [`repo-${triple}`]: `repo-${triple}${suffix}`,
        },
        exports: {
          ".": {
            default: `./repo-${triple}${suffix}`,
          },
        },
      },
      null,
      "  ",
    ),
  );

  await file(join(PACKAGE_DIR, "README.md")).write(`# \`${name}\`

Platform-specific package for: https://www.npmjs.com/package/@lgarron-bin/repo`);

  await $`cd ${PACKAGE_DIR} && npm publish --access public || echo "Already published?"`;
}

await file("./src/js/@lgarron-bin/repo/package.json").write(
  JSON.stringify(
    {
      name: "@lgarron-bin/repo",
      version: version,
      repository: "github:lgarron/repo",
      type: "module",
      bin: {
        repo: "repo.js",
      },
      optionalDependencies: {
        "@lgarron-bin/repo-aarch64-apple-darwin": version,
        "@lgarron-bin/repo-x86_64-apple-darwin": version,
        "@lgarron-bin/repo-x86_64-pc-windows": version,
        "@lgarron-bin/repo-x86_64-unknown-linux-gnu": version,
        "@lgarron-bin/aarch64-unknown-linux-gnu": version,
      },
      engines: {
        node: ">=20.6.0",
      },
    },
    null,
    "  ",
  ),
);
await cp("./README.md", "./src/js/@lgarron-bin/repo/README.md");
await $`cd ./src/js/@lgarron-bin/repo && npm publish --access public || echo "Already published?"`;

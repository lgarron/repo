import { mkdir, rm } from "node:fs/promises";
import { join } from "node:path";
import { $ } from "bun";

const WORKFLOW_NAME = "Build release binaries";

await mkdir("./.temp", { recursive: true });

// TODO: implement `repo vcs current-commit hash`;
const commitSHA = (await $`git rev-parse HEAD`.text()).trim();
const runs: { workflow_runs: { id: number; name: string }[] } =
  await $`gh api "/repos/lgarron/repo/actions/runs?head_sha=${commitSHA}"`.json();

console.log(runs);

const run = runs.workflow_runs.filter((run) => run.name === WORKFLOW_NAME)[0];
console.log(`Workflow run id: ${run.id}`);

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

const ARCHITECTURE_TRIPLES = [
  "x86_64-apple-darwin",
  "aarch64-apple-darwin",
  "x86_64-pc-windows",
  "x86_64-unknown-linux-gnu",
];

const TEMP_DIR = "./.temp/artifacts";
await rm(TEMP_DIR, { recursive: true, force: true });

for (const architectureTriple of ARCHITECTURE_TRIPLES) {
  const downloadInfo = downloads[`repo.${architectureTriple}`];
  console.log(architectureTriple);
  const ZIP_PARENT_DIR = join(TEMP_DIR, architectureTriple);
  const ZIP_PATH = join(TEMP_DIR, `${architectureTriple}.zip`);
  await mkdir(ZIP_PARENT_DIR, { recursive: true });
  await $`gh api /repos/lgarron/repo/actions/artifacts/${downloadInfo.id}/zip > ${ZIP_PATH}`;
  // `-o` means "overwrite"
  const PACKAGE_DIR = `./src/js/@lgarron-repo/repo-${architectureTriple}`;
  await $`unzip -o -d ${PACKAGE_DIR} ${ZIP_PATH}`;

  await $`cd ${PACKAGE_DIR} && npm publish --tag dev --access public || echo "Already published?"`;
}

await $`cd ./src/js/@lgarron-repo/repo && npm publish --tag dev --access public || echo "Already published?"`;

import { exit } from "node:process";
import { Path } from "path-class";
import { PrintableShellCommand } from "printable-shell-command";
import { Temporal } from "temporal-ponyfill";
import { MAIN_PACKAGE_FOLDER, version } from "./common";

const WORKFLOW_NAME = "Build release binaries";

const TEMP_ROOT = new Path("./.temp");
await TEMP_ROOT.mkdir();

// Dogfood our own hash calculation.
const commitSHA = await new PrintableShellCommand("cargo", [
  "run",
  "--quiet",
  "--",
  "vcs",
  "latest-commit",
  "hash",
])
  .stdout()
  .text();

const run = await (async () => {
  while (true) {
    const runs: {
      workflow_runs: {
        id: number;
        name: string;
        // https://docs.github.com/en/rest/actions/workflow-runs?apiVersion=2022-11-28#list-workflow-runs-for-a-repository
        status: "completed" | string;
        conclusion: "success" | string;
        jobs_url: string;
      }[];
    } = await new PrintableShellCommand("gh", [
      "api",
      `/repos/lgarron/repo/actions/runs?head_sha=${commitSHA}`,
    ])
      .stdout()
      .json();

    const run = runs.workflow_runs.filter(
      (run) => run.name === WORKFLOW_NAME,
    )[0];
    if (!run) {
      console.error(
        `Workflow run "${WORKFLOW_NAME}" is not available for this commit: ${commitSHA}
Push a tag to run the release flow.`,
      );
      exit(2);
    }
    console.log(`Workflow run id: ${run.id}`);

    if (run.status === "completed") {
      if (run.conclusion !== "success") {
        console.error("❌ Workflow conclusion was not a success. Exiting.");
        exit(1);
      }
      return run;
    }

    console.info("Workflow is not complete, waiting 10 seconds…");
    // Intentionally not awaited. The `fetch(…)` call is usually less than 10
    // seconds, so we hope it prints while we sleep for the next attempt. If the
    // `fetch(…)` takes more than 10 seconds, we don't do any special handling,
    // and just allow inappropriately interleaved output.
    (async () => {
      const jobs: {
        jobs: {
          name: string;
          status?: "completed" | string;
          conclusion?: "success" | string;
        }[];
      } = await (await fetch(run.jobs_url)).json();
      console.log(
        jobs.jobs
          .map((job) => {
            const emoji = (() => {
              if (job.status !== "completed") {
                return "⏳";
              }
              if (job.conclusion === "success") {
                return "✅";
              }
              // This is never reached in practice, as we detect the entire job as completed (above) first.
              return "❌";
            })();
            // TODO: parse start/end times and print elapsed times?
            return `${emoji} Job: \`${job.name}\``;
          })
          .join("\n"),
      );
    })();

    await new Promise((resolve) =>
      setTimeout(
        resolve,
        Temporal.Duration.from({ seconds: 10 }).total({
          unit: "milliseconds",
        }),
      ),
    );
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

const TEMP_DIR = new Path("./.temp/artifacts");
await TEMP_DIR.rm_rf();

async function publish(cwd: Path) {
  try {
    await new PrintableShellCommand("npm", [
      "publish",
      ["--access", "public"],
    ]).shellOut({ cwd: cwd.toString() });
  } catch (e) {
    console.error(e);
    console.error("Already published? Skipping…");
  }
}

for (const { triple, npmOS, npmCPU } of ARCHITECTURE_TRIPLES) {
  const downloadInfo = downloads[`repo.${triple}`];
  console.log(triple);

  const ZIP_PARENT_DIR = TEMP_DIR.join(triple);
  const ZIP_PATH = TEMP_DIR.join(`${triple}.zip`);
  ZIP_PARENT_DIR.mkdir();

  {
    const bytes = await new PrintableShellCommand("gh", [
      "api",
      `/repos/lgarron/repo/actions/artifacts/${downloadInfo.id}/zip`,
    ])
      .stdout()
      .bytes();
    await ZIP_PATH.write(bytes);
  }

  const PACKAGE_DIR = new Path(`./src/js/@lgarron-bin/repo-${triple}`);
  await new PrintableShellCommand("unzip", [
    // `-o` means "overwrite"
    "-o",
    ["-d", `${PACKAGE_DIR}`],
    `${ZIP_PATH}`,
  ]).shellOut();

  const suffix = isWindows(triple) ? ".exe" : "";
  const binName = `repo-${triple}${suffix}`;
  const binPath = PACKAGE_DIR.join(binName);
  await PACKAGE_DIR.join(`repo${suffix}`).rename(binPath);

  const name = `@lgarron-bin/repo-${triple}`;
  await PACKAGE_DIR.join("package.json").writeJSON({
    name,
    version: version,
    repository: "github:lgarron/repo",
    type: "module",
    os: npmOS,
    cpu: npmCPU,
    bin: {
      [`repo-${triple}`]: binName,
    },
    exports: {
      ".": {
        default: `./${binName}`,
      },
    },
  });

  await PACKAGE_DIR.join("README.md").write(`# \`${name}\`

Platform-specific package for: https://www.npmjs.com/package/@lgarron-bin/repo`);

  await publish(PACKAGE_DIR);
}

await MAIN_PACKAGE_FOLDER.join("package.json").writeJSON({
  name: "@lgarron-bin/repo",
  version,
  repository: "github:lgarron/repo",
  type: "module",
  bin: {
    repo: "repo.js",
  },
  exports: {
    "./bin": {
      default: "./repo.js",
    },
    "./schemas/repo.json": {
      default: "./schemas/repo.json",
    },
    "./types/postVersion": {
      types: "./types/postVersion",
    },
  },
  optionalDependencies: {
    "@lgarron-bin/repo-aarch64-apple-darwin": version,
    "@lgarron-bin/repo-x86_64-apple-darwin": version,
    "@lgarron-bin/repo-x86_64-pc-windows": version,
    "@lgarron-bin/repo-aarch64-pc-windows": version,
    "@lgarron-bin/repo-x86_64-unknown-linux-gnu": version,
    "@lgarron-bin/repo-aarch64-unknown-linux-gnu": version,
  },
  engines: {
    node: ">=20.6.0",
  },
});
await import("./write-schemas-and-types");

await new Path("./README.md").cp(MAIN_PACKAGE_FOLDER.join("README.md"));
await publish(MAIN_PACKAGE_FOLDER);

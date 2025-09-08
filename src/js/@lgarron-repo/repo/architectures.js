/** @satisfies { { rustTarget: string; npmOS: string[]; npmCPU: string[]; }[] } */
export const architectures = [
  {
    rustTarget: "x86_64-apple-darwin",
    npmOS: ["darwin"],
    npmCPU: ["x64"],
  },
  {
    rustTarget: "aarch64-apple-darwin",
    npmOS: ["darwin"],
    npmCPU: ["arm64"],
  },
  {
    rustTarget: "x86_64-pc-windows",
    npmOS: ["windows"],
    npmCPU: ["x64"],
  },
  {
    rustTarget: "x86_64-unknown-linux-gnu",
    npmOS: ["linux"],
    npmCPU: ["x64"],
  },
  {
    rustTarget: "aarch64-unknown-linux-gnu",
    npmOS: ["linux"],
    npmCPU: ["arm64"],
  },
];

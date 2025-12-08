# `repo`

## `repo`

````text help
An opinionated tool for repo management.

Usage: repo <COMMAND>

Commands:
  version       Perform operations on the repo version
  publish       Publish
  boilerplate   Set up boilerplate for the repo
  setup         Set up a repository checkout
  vcs           Get information about the current VCS
  workspace     Get information about the current workspace
  dependencies  Operate on dependencies
  completions   Print completions for the given shell
  print-schema  Print the given JSON schema
  help          Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
````

## `repo version`

````text help-version
Perform operations on the repo version

Usage: repo version [OPTIONS] <COMMAND>

Commands:
  get       Get the current version
  describe  Get more detailed version info, similar to `git describe --tags`
  set       Set the current version
  bump      Bump the current version
  help      Print this message or the help of the given subcommand(s)

Options:
      --ecosystem <ECOSYSTEM>  [possible values: javascript, rust]
  -h, --help                   Print help
````

## `repo publish`

````text help-publish
Publish

Usage: repo publish [OPTIONS]

Options:
      --ecosystem <ECOSYSTEM>  [possible values: javascript, rust]
  -h, --help                   Print help
````

## `repo boilerplate`

````text help-boilerplate
Set up boilerplate for the repo

Usage: repo boilerplate <COMMAND>

Commands:
  ci                           Set up a CI template for GitHub and open for editing at: `.github/workflows/CI.yaml`
  auto-publish-github-release  Set up a CI template for auto-publishing releases from tags pushed to GitHub, at: .github/workflows/publish-github-release.yaml
  biome                        Set up linting using Biome
  tsconfig                     Set up `tsconfig.json`
  readme-cli-help              Set up `readme-cli-help.json`
  bunfig                       Set up `bunfig.toml`
  rust-toolchain               Set up `rust-toolchain.toml`
  help                         Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help
````

## `repo setup`

````text help-setup
Set up a repository checkout

Usage: repo setup [COMMAND]

Commands:
  dependencies  Install dependencies
  help          Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help
````

## `repo vcs`

````text help-vcs
Get information about the current VCS

Usage: repo vcs <COMMAND>

Commands:
  kind           Get the kind of VCS. If there are multiple in the same project (e.g. `jj` + `git`), at most one will be returned (consistent with the `root` subcommand)
  root           Get the repository root folder If the folder is part of multiple repositories, at most one will be returned (consistent with the `kind` subcommand)
  latest-commit  Operate on the latest commit. This does not include the working copy (or a non-merge `@` if it is empty or has an empty description, in case of `jj`)
  help           Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help
````

## `repo workspace`

````text help-workspace
Get information about the current workspace

Usage: repo workspace <COMMAND>

Commands:
  root  Get the workspace root folder based on VCS or other litmus files (e.g. `package.json`, `Cargo.toml`) If the folder is part of multiple repositories, at most one will be returned (consistent with the `kind` subcommand)
  help  Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help
````

## `repo dependencies`

````text help-dependencies
Operate on dependencies

Usage: repo dependencies [OPTIONS] <COMMAND>

Commands:
  roll  
  help  Print this message or the help of the given subcommand(s)

Options:
      --ecosystem <ECOSYSTEM>              [possible values: javascript, rust]
      --package-manager <PACKAGE_MANAGER>  [possible values: npm, bun, yarn, pnpm, cargo]
  -h, --help                               Print help
````

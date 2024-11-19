# `repo`

## `repo`

````cli-help
An opinionated tool for repo management.

Usage: repo <COMMAND>

Commands:
  version      Perform operations on the repo version
  publish      Publish
  boilerplate  Set up boilerplate for the repo
  setup        Set up a repository checkout
  completions  Print completions for the given shell
  help         Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
````

## `repo version`

````cli-help-version
Perform operations on the repo version

Usage: repo version [OPTIONS] <COMMAND>

Commands:
  get   Get the current version
  set   Set the current version
  bump  Bump the current version
  help  Print this message or the help of the given subcommand(s)

Options:
      --ecosystem <ECOSYSTEM>  [possible values: javascript, rust]
  -h, --help                   Print help
````

## `repo publish`

````cli-help-publish
Publish

Usage: repo publish [OPTIONS]

Options:
      --ecosystem <ECOSYSTEM>  [possible values: javascript, rust]
  -h, --help                   Print help
````

## `repo boilerplate`

````cli-help-boilerplate
Set up boilerplate for the repo

Usage: repo boilerplate <COMMAND>

Commands:
  ci                           Set up a CI template for GitHub and open for editing at: `.github/workflows/CI.yaml`
  auto-publish-github-release  Set up a CI template for auto-publishing releases from tags pushed to GitHub, at: .github/workflows/publish-github-release.yaml
  biome                        Set up linting using Biome
  help                         Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help
````

## `repo setup`

````cli-help-setup
Set up a repository checkout

Usage: repo setup [OPTIONS] <COMMAND>

Commands:
  dependencies  Install dependencies
  help          Print this message or the help of the given subcommand(s)

Options:
      --package-manager <PACKAGE_MANAGER>  [possible values: npm, bun, yarn, pnpm, cargo]
  -h, --help                               Print help
````

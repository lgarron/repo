# `repo`

## `repo`

````cli-help
A tool for repo management.

Usage: repo <COMMAND>

Commands:
  version      Perform operations on the repo version
  publish      Publish
  setup        Set up the repo
  ci           Manage CI (continuous integration) at `.github/workflows/CI.yaml`
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
  bump  Bump the current version
  help  Print this message or the help of the given subcommand(s)

Options:
      --ecosystem <ECOSYSTEM>  [possible values: npm, cargo]
  -h, --help                   Print help
````

## `repo publish`

````cli-help-publish
Publish

Usage: repo publish [OPTIONS]

Options:
      --ecosystem <ECOSYSTEM>  [possible values: npm, cargo]
  -h, --help                   Print help
````

## `repo setup`

````cli-help-setup
Set up the repo

Usage: repo setup <COMMAND>

Commands:
  ci                           
  auto-publish-github-release  
  help                         Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help
````

## `repo ci`

````cli-help-ci
Manage CI (continuous integration) at `.github/workflows/CI.yaml`

Usage: repo ci <COMMAND>

Commands:
  setup  Alias for `repo setup ci`
  edit   Open the CI file
  help   Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help
````

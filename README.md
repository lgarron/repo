# `repo`

## `repo`

````cli-help
A tool for repo management.

Usage: repo <COMMAND>

Commands:
  version      Perform operations on the repo version
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
      --ecosystem <ECOSYSTEM>  [default: auto] [possible values: auto, npm, cargo]
  -h, --help                   Print help
````

## `repo ci`

````cli-help-ci
Manage CI (continuous integration) at `.github/workflows/CI.yaml`

Usage: repo ci <COMMAND>

Commands:
  setup  Set up a CI template for GitHub and open for editing
  edit   Open the CI file
  help   Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help
````

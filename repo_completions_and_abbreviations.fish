# Usage: source "repo_abbreviations.fish"

if not functions abbr_subcommand > /dev/null
  echo "These abbreviations require: https://github.com/lgarron/dotfiles/blob/e5227c8aec1fdf8ffdd3fa3e0a57b934e5f8f9bd/dotfiles/fish/.config/fish/abbr.fish" >&2
  exit 1
end

repo completions fish | source

abbr -a "p" "repo"

abbr_subcommand repo v "version"
abbr_subcommand_arg repo b bump version
abbr_subcommand repo vm "version bump minor"
abbr_subcommand repo vp "version bump patch"
abbr_subcommand repo major "version bump major"
abbr_subcommand repo minor "version bump minor"
abbr_subcommand repo patch "version bump patch"

abbr_subcommand repo p "publish"

abbr_subcommand repo b "boilerplate"
abbr_subcommand repo ci "boilerplate ci"
abbr_subcommand repo gr "boilerplate auto-publish-github-release"
abbr_subcommand_arg repo c create boilerplate
abbr_subcommand_arg repo e edit boilerplate
abbr_subcommand_arg repo r reveal boilerplate

abbr_subcommand repo s "setup"
abbr_subcommand repo d "setup dependencies"
abbr_subcommand_arg repo d dependencies setup

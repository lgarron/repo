# Usage: source "repo_abbreviations.fish"

if not functions abbr_subcommand > /dev/null
  echo "These abbreviations require: https://github.com/lgarron/dotfiles/blob/e5227c8aec1fdf8ffdd3fa3e0a57b934e5f8f9bd/dotfiles/fish/.config/fish/abbr.fish" >&2
  exit 1
end

repo completions fish | source

abbr_subcommand repo v "version"
abbr_subcommand repo bump "version bump"
abbr_subcommand repo major "version bump major"
abbr_subcommand repo minor "version bump minor"
abbr_subcommand repo patch "version bump patch"

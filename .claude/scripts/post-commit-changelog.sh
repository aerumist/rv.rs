#!/usr/bin/env bash
# Claude Code hook: regenerate CHANGELOG.md after each commit, amend it in.
set -euo pipefail

cd "$(git rev-parse --show-toplevel)"

# Only act if git-cliff is available
if ! command -v git-cliff &>/dev/null; then
  echo "git-cliff not found. Install: cargo install git-cliff (or see https://git-cliff.org)"
  exit 0
fi

# Snapshot current changelog (or empty if missing)
old=""
[[ -f CHANGELOG.md ]] && old=$(md5sum CHANGELOG.md | cut -d' ' -f1)

# Generate changelog
if ! output=$(git cliff --output CHANGELOG.md 2>&1); then
  echo "git-cliff failed:"
  echo "  command: git cliff --output CHANGELOG.md"
  echo "  output:  $output"
  exit 0
fi

# Check if anything changed
new=""
[[ -f CHANGELOG.md ]] && new=$(md5sum CHANGELOG.md | cut -d' ' -f1)

if [[ "$old" == "$new" ]]; then
  exit 0
fi

# Amend the changelog into the commit
git add CHANGELOG.md
git commit --amend --no-edit --quiet

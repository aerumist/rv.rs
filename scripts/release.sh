#!/usr/bin/env bash
# Manual release script: generates changelog, commits, and tags.
# Usage: cargo set-version <patch|minor|major> && ./scripts/release.sh
set -euo pipefail

cd "$(git rev-parse --show-toplevel)"

# --- Guards ---

if [[ ! -f Cargo.toml ]]; then
  echo "error: Cargo.toml not found" >&2
  exit 1
fi

if ! command -v git-cliff &>/dev/null; then
  echo "error: git-cliff not found (cargo install git-cliff)" >&2
  exit 1
fi

version=$(sed -n 's/^version = "\(.*\)"/\1/p' Cargo.toml | head -1)
if [[ -z "$version" ]]; then
  echo "error: could not read version from Cargo.toml" >&2
  exit 1
fi

tag="v${version}"

if git rev-parse "$tag" &>/dev/null; then
  echo "error: tag $tag already exists — version already released" >&2
  exit 1
fi

# Check for unexpected dirty state (allow Cargo.toml/Cargo.lock/ROADMAP.md changes)
dirty=$(git status --porcelain | grep -v '^ M Cargo.toml$' | grep -v '^ M Cargo.lock$' | grep -v '^M  Cargo.toml$' | grep -v '^M  Cargo.lock$' | grep -v '^ M ROADMAP.md$' | grep -v '^M  ROADMAP.md$' || true)
if [[ -n "$dirty" ]]; then
  echo "error: working tree has unexpected changes:" >&2
  echo "$dirty" >&2
  echo "commit or stash them before releasing" >&2
  exit 1
fi

# --- Release ---

git cliff --tag "$tag" --output CHANGELOG.md

git add Cargo.toml CHANGELOG.md
[[ -f Cargo.lock ]] && git add Cargo.lock
git diff --quiet ROADMAP.md 2>/dev/null || git add ROADMAP.md

git commit -m "release: $tag"
git tag -a "$tag" -m "Release $tag"

echo "released $tag — push with: git push && git push --tags"

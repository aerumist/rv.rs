# Claude Code Hooks

This project uses Claude Code hooks to automate quality checks and commits after code modifications.

## What's configured

### Post-edit hook

A `PostToolUse` hook triggers after Claude uses `Edit`, `Write`, or `NotebookEdit` tools. It runs `.claude/scripts/post-edit-hook.sh`, which:

1. **Formats** code with `cargo fmt --all`
2. **Lints** with `cargo clippy --all-targets -- -D warnings`
3. **Tests** with `cargo test` (skipped if no tests exist)
4. **Stages** all changes with `git add .`
5. **Commits** with a generated Conventional Commit message

If any step fails, the workflow stops and no commit is created.

### Post-commit changelog hook

A `PostToolUse` hook triggers after Claude runs a `git commit` command via Bash. It runs `.claude/scripts/post-commit-changelog.sh`, which:

1. Checks that `git-cliff` is installed
2. Runs `git cliff --output CHANGELOG.md`
3. If the changelog changed, stages it and amends the commit (`git commit --amend --no-edit`)

If `git-cliff` is missing or fails, the hook exits gracefully without modifying the repository.

**When it runs:** After any successful `git commit *` Bash command.

**Idempotency:** Safe to run multiple times — if `CHANGELOG.md` is already up to date, nothing happens.

## Files

| File | Purpose |
|------|---------|
| `.claude/settings.json` | Hook configuration (when to trigger) |
| `.claude/scripts/post-edit-hook.sh` | Post-edit workflow script |
| `.claude/scripts/post-commit-changelog.sh` | Post-commit changelog generation |

## Disabling hooks

**All hooks temporarily** — rename the settings file:

```bash
mv .claude/settings.json .claude/settings.json.disabled
```

**Changelog hook only** — remove the second `PostToolUse` entry (the one with `"if": "Bash(git commit *)"`) from `.claude/settings.json`.

**Permanently** — delete the `hooks` key from `.claude/settings.json` or remove the file entirely.

## Modifying the workflow

Edit `.claude/scripts/post-edit-hook.sh` directly. The script uses `set -euo pipefail` so any failing command stops execution.

To add a new check (e.g., `cargo deny`, `cargo audit`):

```bash
# --- Audit ---
echo -e "${GREEN}[hook]${NC} Running cargo audit..."
AUDIT_OUTPUT=$(cargo audit 2>&1) || fail "Audit failed" "cargo audit" "$AUDIT_OUTPUT"
```

Insert it between the test and git sections.

## Bypassing for a single commit

If you need to commit without the hook (e.g., WIP save):

```bash
git add .
git commit -m "wip: save progress" --no-verify
```

The hook itself uses `--no-verify` on its commits to avoid recursion with any git hooks.

## Safety

The hook never runs destructive git operations. It will not:

- Push to remote
- Reset or rebase
- Delete branches
- Checkout or switch branches
- Clean untracked files

It only creates local commits from staged changes.

## Customizing the changelog

Edit the `git cliff` command in `.claude/scripts/post-commit-changelog.sh`. Examples:

```bash
# Include only tagged releases
git cliff --latest --output CHANGELOG.md

# Use a different config file
git cliff --config my-cliff.toml --output CHANGELOG.md

# Prepend to existing changelog instead of overwriting
git cliff --prepend CHANGELOG.md
```

The git-cliff configuration lives in `cliff.toml` at the repo root.

## Manually regenerating the changelog

```bash
git cliff --output CHANGELOG.md
```

## Future expansion ideas

- `cargo deny check` for license/advisory compliance
- `cargo audit` for security vulnerabilities
- Release automation via `cargo-release`

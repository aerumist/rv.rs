#!/usr/bin/env bash
set -euo pipefail

# Post-edit hook: format, lint, test, and commit if everything passes.
# Runs after Claude successfully modifies the codebase via Edit/Write/NotebookEdit.

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[0;33m'
NC='\033[0m'

fail() {
    echo -e "${RED}HOOK FAILED:${NC} $1"
    echo -e "${RED}Command:${NC} $2"
    echo -e "${RED}Output:${NC}"
    echo "$3"
    exit 1
}

# --- Format ---
echo -e "${GREEN}[hook]${NC} Running cargo fmt..."
FMT_OUTPUT=$(cargo fmt --all 2>&1) || fail "Formatting failed" "cargo fmt --all" "$FMT_OUTPUT"

# --- Lint ---
echo -e "${GREEN}[hook]${NC} Running cargo clippy..."
CLIPPY_OUTPUT=$(cargo clippy --all-targets -- -D warnings 2>&1) || fail "Clippy found issues" "cargo clippy --all-targets -- -D warnings" "$CLIPPY_OUTPUT"

# --- Test ---
if grep -rq '#\[test\]' src/ 2>/dev/null || [ -d tests ]; then
    echo -e "${GREEN}[hook]${NC} Running cargo test..."
    TEST_OUTPUT=$(cargo test 2>&1) || fail "Tests failed" "cargo test" "$TEST_OUTPUT"
else
    echo -e "${YELLOW}[hook]${NC} No tests found, skipping."
fi

# --- Git ---
git add .

if git diff --cached --quiet; then
    echo -e "${YELLOW}[hook]${NC} No changes to commit."
    exit 0
fi

DIFF_FILES=$(git diff --cached --name-only)

# Determine conventional commit type
TYPE="chore"
if echo "$DIFF_FILES" | grep -qE '^src/(commands|cli)/'; then
    TYPE="feat"
elif echo "$DIFF_FILES" | grep -qE '^src/'; then
    TYPE="refactor"
fi
if echo "$DIFF_FILES" | grep -qiE '(README|CONTRIBUTING|CHANGELOG|docs/|\.md$)' && ! echo "$DIFF_FILES" | grep -q '^src/'; then
    TYPE="docs"
fi
if echo "$DIFF_FILES" | grep -qE '(^tests/|_test\.rs$)'; then
    TYPE="test"
fi
if echo "$DIFF_FILES" | grep -qE '(Cargo\.toml|Cargo\.lock|\.claude/|\.github/)' && ! echo "$DIFF_FILES" | grep -q '^src/'; then
    TYPE="chore"
fi

# Build a descriptive scope from changed paths
describe_changes() {
    local files="$1"
    local count
    count=$(echo "$files" | wc -l)

    if [ "$count" -eq 1 ]; then
        local file="$files"
        local basename
        basename=$(basename "$file" | sed 's/\.[^.]*$//')
        case "$file" in
            src/commands/*) echo "add ${basename} command" ;;
            src/*)          echo "update ${basename} module" ;;
            *.md)           echo "update ${basename,,}" ;;
            *)              echo "update ${basename}" ;;
        esac
    else
        # Summarize by directory
        local dirs
        dirs=$(echo "$files" | xargs -I{} dirname {} | sort -u)
        local dir_count
        dir_count=$(echo "$dirs" | wc -l)

        if [ "$dir_count" -eq 1 ]; then
            local dir="$dirs"
            case "$dir" in
                src/commands) echo "update command modules" ;;
                src/*)        echo "update $(basename "$dir") module" ;;
                .)            echo "update project files" ;;
                *)            echo "update $(basename "$dir")" ;;
            esac
        else
            echo "update ${count} files across ${dir_count} directories"
        fi
    fi
}

SCOPE=$(describe_changes "$DIFF_FILES")
COMMIT_MSG="${TYPE}: ${SCOPE}"

git commit -m "$COMMIT_MSG" --no-verify

echo -e "${GREEN}[hook]${NC} Committed: ${COMMIT_MSG}"

#!/bin/sh
set -eu

repo_root=$(git rev-parse --show-toplevel)
cd "$repo_root"

configured_path=$(git config --get core.hooksPath || true)
if [ -n "$configured_path" ]; then
    if [ "$configured_path" = ".githooks" ]; then
        echo "Hooks already use .githooks."
    else
        echo "Leaving core.hooksPath unchanged ($configured_path); install .githooks/pre-commit there if desired."
    fi
    exit 0
fi

default_hooks_dir=$(git rev-parse --git-path hooks)
for existing_hook in "$default_hooks_dir"/*; do
    [ -e "$existing_hook" ] || [ -L "$existing_hook" ] || continue
    case "$existing_hook" in
        *.sample) continue ;;
    esac
    if [ -x "$existing_hook" ]; then
        echo "Leaving existing executable hook $existing_hook unchanged; configure core.hooksPath manually to use .githooks."
        exit 0
    fi
done

git config --local core.hooksPath .githooks
echo "Configured core.hooksPath=.githooks."

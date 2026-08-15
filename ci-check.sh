#!/bin/bash
set -euo pipefail

PROJECT_ROOT=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)
CI_LOCK_DIR="$PROJECT_ROOT/target"
CI_LOCK_FILE="$CI_LOCK_DIR/.rs-ci-ci-check.lock"

mkdir -p "$CI_LOCK_DIR"
exec flock "$CI_LOCK_FILE" env RS_CI_PROJECT_ROOT="$PROJECT_ROOT" \
    "$PROJECT_ROOT/.rs-ci/ci-check.sh" "$@"

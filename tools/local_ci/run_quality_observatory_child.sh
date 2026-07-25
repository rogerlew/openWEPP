#!/usr/bin/env bash
set -euo pipefail

: "${SOURCE_SHA:?SOURCE_SHA is required}"
: "${QUALITY_ATTEMPT_ROOT:?QUALITY_ATTEMPT_ROOT is required}"
: "${QUALITY_WORKFLOW_SHA256:?QUALITY_WORKFLOW_SHA256 is required}"
: "${QUALITY_WORKFLOW_REVISION:?QUALITY_WORKFLOW_REVISION is required}"
: "${QUALITY_PRIORITY_SENTINEL:?QUALITY_PRIORITY_SENTINEL is required}"

test "$(git rev-parse HEAD)" = "${SOURCE_SHA}"
test "$(gh --version | awk 'NR == 1 { print $3 }')" = '2.96.0'
rustc --version | grep -F 'rustc 1.92.0'
cargo nextest --version | grep -F 'cargo-nextest 0.9.138'
cargo llvm-cov --version
cargo crap --version
python3 --version | grep -F 'Python 3.12.'

bash tools/ci/omarchy-runner/bootstrap_dependencies.sh \
  "${SOURCE_SHA}" \
  "${SOURCE_SHA}"

.venv/bin/python tools/local_ci/quality_observatory.py transition \
  --repo . \
  --attempt-root "${QUALITY_ATTEMPT_ROOT}" \
  --runner forest1 \
  --workflow "quality-observatory.yml@${SOURCE_SHA}:${QUALITY_WORKFLOW_SHA256}" \
  --run-id "${GITHUB_RUN_ID}" \
  --run-attempt "${GITHUB_RUN_ATTEMPT}" \
  --admission-mode workflow \
  --workflow-revision "${QUALITY_WORKFLOW_REVISION}" \
  --workflow-sha256 "${QUALITY_WORKFLOW_SHA256}" \
  --priority-sentinel "${QUALITY_PRIORITY_SENTINEL}"

#!/usr/bin/env bash
set -o pipefail
set -u
RUN_DIR="$1"
TMP_DIR="$2"
START="$3"
TARGET_DIR="$4"
LOG="$RUN_DIR/cargo_nextest.attempt1.log"
export TMPDIR="$TMP_DIR"
export TMP="$TMP_DIR"
export TEMP="$TMP_DIR"
export CARGO_TARGET_DIR="$TARGET_DIR"
cd /home/workdir/openWEPP
write_result() {
  local code="$1"
  END="$(date -u +%Y-%m-%dT%H:%M:%SZ)"
  STATUS=FAIL
  if [ "$code" -eq 0 ]; then
    STATUS=PASS
  fi
  printf '%s\t%s\t%s\t%s\t%s\t%s\n' "cargo_nextest" "$START" "$END" "$code" "$LOG" "$STATUS" > "$RUN_DIR/command-summary.tsv"
  printf '["{\n"' > /tmp/.noop 2>/dev/null
  printf '{"command":"cargo_nextest","attempt":1,"start":"%s","end":"%s","exit_code":%s,"status":"%s","log":"%s"}\n' "$START" "$END" "$code" "$STATUS" "$LOG" > "$RUN_DIR/command-log.json"
  printf 'cargo_nextest %s start=%s end=%s exit=%s log=%s\n' "$STATUS" "$START" "$END" "$code" "$LOG" > "$RUN_DIR/command-result.txt"
  printf '%s\n' "$RUN_DIR" > "$RUN_DIR/done.txt"
}
trap 'write_result "$?"' EXIT
/home/roger/.cargo/bin/cargo-nextest nextest run --workspace --profile full > "$LOG" 2>&1

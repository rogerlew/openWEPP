#!/usr/bin/env bash
set -euo pipefail
set -o pipefail
cd /home/workdir/openWEPP
RUN_DIR="${RUN_DIR}"
LOG="$RUN_DIR/cargo_nextest.attempt1.log"
START="$(date -u +%Y-%m-%dT%H:%M:%SZ)"
export TMPDIR="${TMPDIR}"
set +e
/home/roger/.cargo/bin/cargo-nextest nextest run --workspace --profile full > "$LOG" 2>&1
CODE=$?
set -e
END="$(date -u +%Y-%m-%dT%H:%M:%SZ)"
STATUS='FAIL'
if [ "$CODE" -eq 0 ]; then
  STATUS='PASS'
fi
SUMMARY_TMP="$RUN_DIR/.command-summary.nextest"
JSON_TMP="$RUN_DIR/.command-log.nextest"
printf 'cargo_nextest\t%s\t%s\t%s\t%s\t%s\n' "$START" "$END" "$CODE" "$LOG" "$STATUS" > "$SUMMARY_TMP"
printf '[\n{"command":"cargo_nextest","attempt":1,"start":"%s","end":"%s","exit_code":%s,"status":"%s","log":"%s"}\n]\n' "$START" "$END" "$CODE" "$STATUS" "$LOG" > "$JSON_TMP"
mv "$SUMMARY_TMP" "$RUN_DIR/command-summary.tsv"
mv "$JSON_TMP" "$RUN_DIR/command-log.json"
printf '%s %s start=%s end=%s exit=%s log=%s\n' cargo_nextest "$STATUS" "$START" "$END" "$CODE" "$LOG" > "$RUN_DIR/command-result.txt"
printf '%s\n' "$RUN_DIR" > "$RUN_DIR/done.txt"

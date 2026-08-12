#!/usr/bin/env bash
set -o pipefail
set -u
RUN_DIR="$1"
TMP_DIR="$2"
START="$3"
cd /home/workdir/openWEPP
LOG="$RUN_DIR/cargo_nextest.attempt1.log"
export TMPDIR="$TMP_DIR"
/home/roger/.cargo/bin/cargo-nextest nextest run --workspace --profile full > "$LOG" 2>&1
CODE=$?
END="$(date -u +%Y-%m-%dT%H:%M:%SZ)"
STATUS=FAIL
if [ "$CODE" -eq 0 ]; then
  STATUS=PASS
fi
TMP_SUM="$(mktemp "$RUN_DIR/.tmp-summary.XXXXXX")"
printf 'cargo_nextest\t%s\t%s\t%s\t%s\t%s\n' "$START" "$END" "$CODE" "$LOG" "$STATUS" > "$TMP_SUM"
mv "$TMP_SUM" "$RUN_DIR/command-summary.tsv"
TMP_LOG="$(mktemp "$RUN_DIR/.tmp-log.XXXXXX")"
printf '[\\n{"command":"cargo_nextest","attempt":1,"start":"%s","end":"%s","exit_code":%s,"status":"%s","log":"%s"}\\n]\\n' "$START" "$END" "$CODE" "$STATUS" "$LOG" > "$TMP_LOG"
mv "$TMP_LOG" "$RUN_DIR/command-log.json"
TMP_RES="$(mktemp "$RUN_DIR/.tmp-result.XXXXXX")"
printf 'cargo_nextest %s start=%s end=%s exit=%s log=%s\n' "$STATUS" "$START" "$END" "$CODE" "$LOG" > "$TMP_RES"
mv "$TMP_RES" "$RUN_DIR/command-result.txt"
echo "$RUN_DIR" > "$RUN_DIR/done.txt"

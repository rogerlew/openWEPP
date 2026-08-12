#!/usr/bin/env bash
set -o pipefail
cd /home/workdir/openWEPP
RUN_DIR="${RUN_DIR}"
TMP_DIR="${TMP_DIR}"
START="${START}"
LOG="$RUN_DIR/cargo_nextest.attempt1.log"
export TMPDIR="$TMP_DIR"
/home/roger/.cargo/bin/cargo-nextest nextest run --workspace --profile full > "$LOG" 2>&1
CODE=$?
END="$(date -u +%Y-%m-%dT%H:%M:%SZ)"
STATUS=FAIL
if [ "$CODE" -eq 0 ]; then
  STATUS=PASS
fi
tmp=$(mktemp "$RUN_DIR/.tmp-summary.XXXXXX")
printf 'cargo_nextest\t%s\t%s\t%s\t%s\t%s\n' "$START" "$END" "$CODE" "$LOG" "$STATUS" > "$tmp"
mv "$tmp" "$RUN_DIR/command-summary.tsv"
tmp=$(mktemp "$RUN_DIR/.tmp-log.XXXXXX")
printf '[\\n{"command":"cargo_nextest","attempt":1,"start":"%s","end":"%s","exit_code":%s,"status":"%s","log":"%s"}\\n]\\n' "$START" "$END" "$CODE" "$STATUS" "$LOG" > "$tmp"
mv "$tmp" "$RUN_DIR/command-log.json"
tmp=$(mktemp "$RUN_DIR/.tmp-result.XXXXXX")
printf 'cargo_nextest %s start=%s end=%s exit=%s log=%s\n' "$STATUS" "$START" "$END" "$CODE" "$LOG" > "$tmp"
mv "$tmp" "$RUN_DIR/command-result.txt"
echo "$RUN_DIR" > "$RUN_DIR/done.txt"

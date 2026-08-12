#!/usr/bin/env bash
set -o pipefail
set -u
RUN_DIR="$1"
TMP_DIR="$2"
START="$3"
cd /home/workdir/openWEPP
LOG="/home/workdir/openWEPP/docs/work-packages/20260811-c3-woody-tile-liquid-topology-authority-001/artifacts/comparator-gate-logs/20260812T165606Z/cargo_nextest.attempt1.log"
export TMPDIR="/tmp/c3_v2_heavy_gates.h2l2AHloRR"
/home/roger/.cargo/bin/cargo-nextest nextest run --workspace --profile full > "" 2>&1
CODE=$?
END="$(date -u +%Y-%m-%dT%H:%M:%SZ)"
STATUS=FAIL
if [ "$CODE" -eq 0 ]; then
  STATUS=PASS
fi
TMP_SUM="$(mktemp "/home/workdir/openWEPP/docs/work-packages/20260811-c3-woody-tile-liquid-topology-authority-001/artifacts/comparator-gate-logs/20260812T165606Z/.tmp-summary.XXXXXX")"
printf 'cargo_nextest\t%s\t%s\t%s\t%s\t%s\n' "$START" "$END" "$CODE" "$LOG" "$STATUS" > "$TMP_SUM"
mv "$TMP_SUM" "/home/workdir/openWEPP/docs/work-packages/20260811-c3-woody-tile-liquid-topology-authority-001/artifacts/comparator-gate-logs/20260812T165606Z/command-summary.tsv"
TMP_LOG="$(mktemp "/home/workdir/openWEPP/docs/work-packages/20260811-c3-woody-tile-liquid-topology-authority-001/artifacts/comparator-gate-logs/20260812T165606Z/.tmp-log.XXXXXX")"
printf '[\n{"command":"cargo_nextest","attempt":1,"start":"%s","end":"%s","exit_code":%s,"status":"%s","log":"%s"}\n]\n' "$START" "$END" "$CODE" "$STATUS" "$LOG" > "$TMP_LOG"
mv "$TMP_LOG" "/home/workdir/openWEPP/docs/work-packages/20260811-c3-woody-tile-liquid-topology-authority-001/artifacts/comparator-gate-logs/20260812T165606Z/command-log.json"
TMP_RES="$(mktemp "/home/workdir/openWEPP/docs/work-packages/20260811-c3-woody-tile-liquid-topology-authority-001/artifacts/comparator-gate-logs/20260812T165606Z/.tmp-result.XXXXXX")"
printf 'cargo_nextest %s start=%s end=%s exit=%s log=%s\n' "$STATUS" "$START" "$END" "$CODE" "$LOG" > "$TMP_RES"
mv "$TMP_RES" "/home/workdir/openWEPP/docs/work-packages/20260811-c3-woody-tile-liquid-topology-authority-001/artifacts/comparator-gate-logs/20260812T165606Z/command-result.txt"
echo "/home/workdir/openWEPP/docs/work-packages/20260811-c3-woody-tile-liquid-topology-authority-001/artifacts/comparator-gate-logs/20260812T165606Z" > "/home/workdir/openWEPP/docs/work-packages/20260811-c3-woody-tile-liquid-topology-authority-001/artifacts/comparator-gate-logs/20260812T165606Z/done.txt"

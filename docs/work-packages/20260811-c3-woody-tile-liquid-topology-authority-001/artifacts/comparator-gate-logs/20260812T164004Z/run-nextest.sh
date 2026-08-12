#!/usr/bin/env bash
set -o pipefail
set -u
RUN_DIR='/home/workdir/openWEPP/docs/work-packages/20260811-c3-woody-tile-liquid-topology-authority-001/artifacts/comparator-gate-logs/20260812T164004Z'
LOG="/home/workdir/openWEPP/docs/work-packages/20260811-c3-woody-tile-liquid-topology-authority-001/artifacts/comparator-gate-logs/20260812T164004Z/cargo_nextest.attempt1.log"
START="2026-08-12T16:40:04Z"
export TMPDIR='/tmp/c3_v2_heavy_gates.lRYDfy8zY7'
cd /home/workdir/openWEPP
set +e
/home/roger/.cargo/bin/cargo-nextest nextest run --workspace --profile full > "" 2>&1
CODE=0
set -e
END="2026-08-12T16:40:04Z"
STATUS='FAIL'
if [ "" -eq 0 ]; then STATUS='PASS'; fi
SUMMARY_TMP="/home/workdir/openWEPP/docs/work-packages/20260811-c3-woody-tile-liquid-topology-authority-001/artifacts/comparator-gate-logs/20260812T164004Z/.command-summary.nextest"
JSON_TMP="/home/workdir/openWEPP/docs/work-packages/20260811-c3-woody-tile-liquid-topology-authority-001/artifacts/comparator-gate-logs/20260812T164004Z/.command-log.nextest"
printf '%s\t%s\t%s\t%s\t%s\t%s\n' cargo_nextest "" "" "" "" "" > ""
printf '[\n{"command":"cargo_nextest","attempt":1,"start":"%s","end":"%s","exit_code":%s,"status":"%s","log":"%s"}\n]\n' "" "" "" "" "" > ""
mv "" "/home/workdir/openWEPP/docs/work-packages/20260811-c3-woody-tile-liquid-topology-authority-001/artifacts/comparator-gate-logs/20260812T164004Z/command-summary.tsv"
mv "" "/home/workdir/openWEPP/docs/work-packages/20260811-c3-woody-tile-liquid-topology-authority-001/artifacts/comparator-gate-logs/20260812T164004Z/command-log.json"
printf '%s %s start=%s end=%s exit=%s log=%s\n' cargo_nextest "" "" "" "" "" > "/home/workdir/openWEPP/docs/work-packages/20260811-c3-woody-tile-liquid-topology-authority-001/artifacts/comparator-gate-logs/20260812T164004Z/command-result.txt"
printf '%s\n' "/home/workdir/openWEPP/docs/work-packages/20260811-c3-woody-tile-liquid-topology-authority-001/artifacts/comparator-gate-logs/20260812T164004Z" > "/home/workdir/openWEPP/docs/work-packages/20260811-c3-woody-tile-liquid-topology-authority-001/artifacts/comparator-gate-logs/20260812T164004Z/done.txt"

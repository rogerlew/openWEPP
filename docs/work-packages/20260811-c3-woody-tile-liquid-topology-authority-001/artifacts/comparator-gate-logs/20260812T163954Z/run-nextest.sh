#!/usr/bin/env bash
set -euo pipefail
set -o pipefail
RUN_DIR="/home/workdir/openWEPP/docs/work-packages/20260811-c3-woody-tile-liquid-topology-authority-001/artifacts/comparator-gate-logs/20260812T163954Z"
LOG="/home/workdir/openWEPP/docs/work-packages/20260811-c3-woody-tile-liquid-topology-authority-001/artifacts/comparator-gate-logs/20260812T163954Z/cargo_nextest.attempt1.log"
START="2026-08-12T16:39:54Z"
export TMPDIR="/tmp/c3_v2_heavy_gates.7H93khRSmr"
cd /home/workdir/openWEPP
mkdir -p "/home/workdir/openWEPP/docs/work-packages/20260811-c3-woody-tile-liquid-topology-authority-001/artifacts/comparator-gate-logs/20260812T163954Z"
cd /home/workdir/openWEPP
/home/roger/.cargo/bin/cargo-nextest nextest run --workspace --profile full > "" 2>&1
CODE=$?
END="$(date -u +%Y-%m-%dT%H:%M:%SZ)"
STATUS=FAIL
if [ \"$CODE\" -eq 0 ]; then STATUS=PASS; fi
printf '%s\t%s\t%s\t%s\t%s\t%s\n' cargo_nextest \"\" \"\" \"\" \"\" \"\" > "/home/workdir/openWEPP/docs/work-packages/20260811-c3-woody-tile-liquid-topology-authority-001/artifacts/comparator-gate-logs/20260812T163954Z/.command-summary.nextest"
printf '[\n{"command":"cargo_nextest","attempt":1,"start":"%s","end":"%s","exit_code":%s,"status":"%s","log":"%s"}\n]\n' \"\" \"\" \"\" \"\" \"\" > "/home/workdir/openWEPP/docs/work-packages/20260811-c3-woody-tile-liquid-topology-authority-001/artifacts/comparator-gate-logs/20260812T163954Z/.command-log.nextest"
mv "/home/workdir/openWEPP/docs/work-packages/20260811-c3-woody-tile-liquid-topology-authority-001/artifacts/comparator-gate-logs/20260812T163954Z/.command-summary.nextest" "/home/workdir/openWEPP/docs/work-packages/20260811-c3-woody-tile-liquid-topology-authority-001/artifacts/comparator-gate-logs/20260812T163954Z/command-summary.tsv"
mv "/home/workdir/openWEPP/docs/work-packages/20260811-c3-woody-tile-liquid-topology-authority-001/artifacts/comparator-gate-logs/20260812T163954Z/.command-log.nextest" "/home/workdir/openWEPP/docs/work-packages/20260811-c3-woody-tile-liquid-topology-authority-001/artifacts/comparator-gate-logs/20260812T163954Z/command-log.json"
printf '%s %s start=%s end=%s exit=%s log=%s\n' cargo_nextest \"\" \"\" \"\" \"\" \"\" > "/home/workdir/openWEPP/docs/work-packages/20260811-c3-woody-tile-liquid-topology-authority-001/artifacts/comparator-gate-logs/20260812T163954Z/command-result.txt"

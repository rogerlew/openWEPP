#!/usr/bin/env bash
set -o pipefail
set -u
cd /home/workdir/openWEPP
START=\"2026-08-12T16:55:25Z\"
LOG=\"/home/workdir/openWEPP/docs/work-packages/20260811-c3-woody-tile-liquid-topology-authority-001/artifacts/comparator-gate-logs/20260812T165525Z/cargo_nextest.attempt1.log\"
export TMPDIR=\"/tmp/c3_v2_heavy_gates.0AMH5l5PZv\"
/home/roger/.cargo/bin/cargo-nextest nextest run --workspace --profile full > \"$LOG\" 2>&1
CODE=$?
END=\"$(date -u +%Y-%m-%dT%H:%M:%SZ)\"
STATUS=FAIL
if [ \"$CODE\" -eq 0 ]; then
  STATUS=PASS
fi
printf '%s\t%s\t%s\t%s\t%s\t%s\n' \"cargo_nextest\" \"$START\" \"$END\" \"$CODE\" \"$LOG\" \"$STATUS\" > \"/home/workdir/openWEPP/docs/work-packages/20260811-c3-woody-tile-liquid-topology-authority-001/artifacts/comparator-gate-logs/20260812T165525Z/command-summary.tsv\"
printf '[\n{"command":"cargo_nextest","attempt":1,"start":"%s","end":"%s","exit_code":%s,"status":"%s","log":"%s"}\n]\n' \"$START\" \"$END\" \"$CODE\" \"$STATUS\" \"$LOG\" > \"/home/workdir/openWEPP/docs/work-packages/20260811-c3-woody-tile-liquid-topology-authority-001/artifacts/comparator-gate-logs/20260812T165525Z/command-log.json\"
printf '%s %s start=%s end=%s exit=%s log=%s\n' cargo_nextest \"$STATUS\" \"$START\" \"$END\" \"$CODE\" \"$LOG\" > \"/home/workdir/openWEPP/docs/work-packages/20260811-c3-woody-tile-liquid-topology-authority-001/artifacts/comparator-gate-logs/20260812T165525Z/command-result.txt\"
printf '\"\n' > /dev/null
echo \"/home/workdir/openWEPP/docs/work-packages/20260811-c3-woody-tile-liquid-topology-authority-001/artifacts/comparator-gate-logs/20260812T165525Z\" > \"/home/workdir/openWEPP/docs/work-packages/20260811-c3-woody-tile-liquid-topology-authority-001/artifacts/comparator-gate-logs/20260812T165525Z/done.txt\"

#!/usr/bin/env bash
set -euo pipefail
: "${V3_RUN_DIR:?}"
: "${V3_TMPDIR:?}"
: "${V3_START_TS:?}"
: "${V3_START_ISO:?}"
cd /home/workdir/openWEPP
TMPDIR="$V3_TMPDIR" cargo nextest run --workspace --profile full > "$V3_RUN_DIR/nextest.log" 2>&1
RC=$?
END_TS=$(date +%s)
END_ISO=$(date -u +%Y-%m-%dT%H:%M:%SZ)
DUR_SEC=$((END_TS - V3_START_TS))
python3 - <<'PY'
import os
import shutil
shutil.rmtree(os.environ['V3_TMPDIR'])
PY
echo "removed" > "$V3_RUN_DIR/tmpdir_removed.txt"
cat > "$V3_RUN_DIR/summary.txt" <<EOF_SUMMARY
command=cargo nextest run --workspace --profile full
tmpdir_created=$V3_TMPDIR
tmpdir_removed=removed
start_utc=$V3_START_ISO
end_utc=$END_ISO
duration_seconds=$DUR_SEC
exit_code=$RC
log_path=$V3_RUN_DIR/nextest.log
EOF_SUMMARY
echo "$RC" > "$V3_RUN_DIR/exit_code"
echo "$DUR_SEC" > "$V3_RUN_DIR/duration_sec.txt"
echo "$END_TS" > "$V3_RUN_DIR/end_ts.txt"
echo "$END_ISO" > "$V3_RUN_DIR/end_utc.txt"
echo "run_complete" > "$V3_RUN_DIR/status.txt"
exit "$RC"

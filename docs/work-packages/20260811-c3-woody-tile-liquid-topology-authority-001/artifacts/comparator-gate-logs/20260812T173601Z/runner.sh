#!/usr/bin/env bash
set -u
run_dir="/home/workdir/openWEPP/docs/work-packages/20260811-c3-woody-tile-liquid-topology-authority-001/artifacts/comparator-gate-logs/20260812T173601Z"
scratch_dir="/home/workdir/c3_v2_heavy_gates.mvOgYZN5Oe"
start="$(date -u +%Y-%m-%dT%H:%M:%SZ)"
mkdir -p "$scratch_dir"
cd /home/workdir/openWEPP || exit 125
export TMPDIR="$scratch_dir"
cargo nextest run --workspace --profile full > "$run_dir/cargo_nextest.attempt2.log" 2>&1
code=$?
end="$(date -u +%Y-%m-%dT%H:%M:%SZ)"
status="FAIL"
if [ "$code" -eq 0 ]; then
    status="PASS"
fi
printf 'cargo_nextest\t%s\t%s\t%s\t%s\t%s\n' \
    "$start" "$end" "$code" "$run_dir/cargo_nextest.attempt2.log" "$status" \
    > "$run_dir/command-summary.tsv"
printf '[\n{"command":"cargo nextest run --workspace --profile full","attempt":2,"start":"%s","end":"%s","exit_code":%s,"status":"%s","log":"%s"}\n]\n' \
    "$start" "$end" "$code" "$status" "$run_dir/cargo_nextest.attempt2.log" \
    > "$run_dir/command-log.json"
printf '%s\n' "$status" > "$run_dir/done.txt"
exit "$code"

#!/usr/bin/env bash
set -euo pipefail

binary="/workdir/.cache/openwepp/targets/openWEPP-295c6e060aa9/release/deps/openwepp_runner-fc552493dc3c6cc2"
root_dir="$(cd "$(dirname "$0")" && pwd)"
sample_file="$root_dir/current_memory_samples.csv"
result_file="$root_dir/current_memory_results.jsonl"
manifest_file="$root_dir/current_memory_manifest.jsonl"
probe_name="hillslope::tests::stage3_laned_release_one_ofe_positive_baseline_profile"
probe_args=("$probe_name" --ignored --exact --nocapture --test-threads=1)
sample_period_s="0.020"
run_timeout_s=120

binary_sha="$(sha256sum "$binary" | awk '{print $1}')"
cpu_affinity="0"
printf '{"binary":"%s","binary_sha256":"%s","probe":"%s","argv":["%s","--ignored","--exact","--nocapture","--test-threads=1"],"cpu_affinity":"%s","environment":{"RUST_MIN_STACK":"67108864","CARGO_PROFILE_RELEASE_LTO":"false"},"sample_period_s":%s,"run_timeout_s":%s,"observer":"/proc/<pid>/status and /proc/<pid>/smaps_rollup","started_at":"%s"}\n' \
  "$binary" "$binary_sha" "$probe_name" "$probe_name" "$cpu_affinity" "$sample_period_s" "$run_timeout_s" "$(date --iso-8601=seconds)" > "$manifest_file"
printf 'block,order,arm,run,pid,elapsed_ms,rss_kib,hwm_kib,smaps_rss_kib,private_clean_kib,private_dirty_kib,private_total_kib\n' > "$sample_file"
: > "$result_file"

probe_json_from_log() {
  local log_file="$1"
  rg 'STAGE3_LANED_RELEASE_PROBE ' "$log_file" | sed 's/^.*STAGE3_LANED_RELEASE_PROBE //' | tail -n 1
}

run_unobserved() {
  local block="$1" order="$2" arm="$3" run="$4" log_file="$5"
  local started_ns ended_ns duration_ms rc probe_json
  started_ns="$(date +%s%N)"
  set +e
  timeout --signal=TERM --kill-after=2s "$run_timeout_s" env RUST_MIN_STACK=67108864 CARGO_PROFILE_RELEASE_LTO=false \
    taskset -c "$cpu_affinity" "$binary" "${probe_args[@]}" > "$log_file" 2>&1
  rc=$?
  set -e
  ended_ns="$(date +%s%N)"
  duration_ms=$(( (ended_ns - started_ns) / 1000000 ))
  probe_json="$(probe_json_from_log "$log_file" || true)"
  probe_status="present"
  if [[ -z "$probe_json" ]]; then
    probe_status="missing"
    [[ "$rc" -eq 0 ]] && rc=98
    probe_json='null'
  fi
  printf '{"block":%s,"order":"%s","arm":"%s","run":%s,"exit_status":%s,"duration_ms":%s,"observer_status":"not_attached","probe_status":"%s","probe":%s}\n' \
    "$block" "$order" "$arm" "$run" "$rc" "$duration_ms" "$probe_status" "$probe_json" >> "$result_file"
  return "$rc"
}

run_observed() {
  local block="$1" order="$2" arm="$3" run="$4" log_file="$5"
  local started_ns now_ns ended_ns elapsed_ms duration_ms rc probe_json
  local peak_rss=0 peak_hwm=0 peak_smaps=0 peak_private=0 sample_count=0 field_gap_count=0
  started_ns="$(date +%s%N)"
  env RUST_MIN_STACK=67108864 CARGO_PROFILE_RELEASE_LTO=false \
    taskset -c "$cpu_affinity" "$binary" "${probe_args[@]}" > "$log_file" 2>&1 &
  local pid=$!
  local proc_exe="/proc/$pid/exe"
  local ready=0
  local readiness_status="not_ready"
  local ready_deadline_ns=$(( $(date +%s%N) + 1000000000 ))
  while kill -0 "$pid" 2>/dev/null; do
    if [[ -L "$proc_exe" ]] && [[ "$(readlink "$proc_exe" 2>/dev/null || true)" == "$binary" ]]; then
      ready=1
      readiness_status="ready"
      break
    fi
    if (( $(date +%s%N) >= ready_deadline_ns )); then
      break
    fi
    sleep 0.005
  done
  while kill -0 "$pid" 2>/dev/null; do
    now_ns="$(date +%s%N)"
    elapsed_ms=$(( (now_ns - started_ns) / 1000000 ))
    if (( elapsed_ms > run_timeout_s * 1000 )); then
      kill -TERM "$pid" 2>/dev/null || true
      sleep 1
      kill -KILL "$pid" 2>/dev/null || true
      break
    fi
    status_file="/proc/$pid/status"
    smaps_file="/proc/$pid/smaps_rollup"
    if [[ ! -r "$status_file" || ! -r "$smaps_file" ]]; then
      break
    fi
    status_snapshot="$(cat "$status_file" 2>/dev/null || true)"
    smaps_snapshot="$(cat "$smaps_file" 2>/dev/null || true)"
    rss="$(awk '$1 == "VmRSS:" {print $2; exit}' <<< "$status_snapshot")"
    hwm="$(awk '$1 == "VmHWM:" {print $2; exit}' <<< "$status_snapshot")"
    smaps_rss="$(awk '$1 == "Rss:" {print $2; exit}' <<< "$smaps_snapshot")"
    private_clean="$(awk '$1 == "Private_Clean:" {print $2; exit}' <<< "$smaps_snapshot")"
    private_dirty="$(awk '$1 == "Private_Dirty:" {print $2; exit}' <<< "$smaps_snapshot")"
    if [[ -z "$rss" || -z "$hwm" || -z "$smaps_rss" || -z "$private_clean" || -z "$private_dirty" ]]; then
      if (( ready == 0 )) && { [[ ! -e "$proc_exe" ]] || [[ "$(readlink "$proc_exe" 2>/dev/null || true)" != "$binary" ]]; }; then
        continue
      fi
      if [[ ! -e "$proc_exe" ]]; then
        break
      fi
      field_gap_count=$((field_gap_count + 1))
    else
      private_total=$((private_clean + private_dirty))
      printf '%s,%s,%s,%s,%s,%s,%s,%s,%s,%s,%s,%s\n' "$block" "$order" "$arm" "$run" "$pid" "$elapsed_ms" "$rss" "$hwm" "$smaps_rss" "$private_clean" "$private_dirty" "$private_total" >> "$sample_file"
      sample_count=$((sample_count + 1))
      if (( rss > peak_rss )); then peak_rss=$rss; fi
      if (( hwm > peak_hwm )); then peak_hwm=$hwm; fi
      if (( smaps_rss > peak_smaps )); then peak_smaps=$smaps_rss; fi
      if (( private_total > peak_private )); then peak_private=$private_total; fi
    fi
    sleep "$sample_period_s"
  done
  set +e
  wait "$pid"
  rc=$?
  set -e
  ended_ns="$(date +%s%N)"
  duration_ms=$(( (ended_ns - started_ns) / 1000000 ))
  probe_json="$(probe_json_from_log "$log_file" || true)"
  probe_status="present"
  if [[ -z "$probe_json" ]]; then
    probe_status="missing"
    [[ "$rc" -eq 0 ]] && rc=98
    probe_json='null'
  fi
  observer_status="complete"
  (( field_gap_count == 0 )) || observer_status="field_gap"
  (( sample_count > 0 )) || observer_status="no_samples"
  if (( duration_ms > run_timeout_s * 1000 )); then observer_status="timeout"; fi
  printf '{"block":%s,"order":"%s","arm":"%s","run":%s,"pid":%s,"exit_status":%s,"duration_ms":%s,"readiness_status":"%s","observer_status":"%s","probe_status":"%s","sample_count":%s,"field_gap_count":%s,"observer_peak_rss_kib":%s,"observer_peak_hwm_kib":%s,"observer_peak_smaps_rss_kib":%s,"observer_peak_private_total_kib":%s,"probe":%s}\n' \
    "$block" "$order" "$arm" "$run" "$pid" "$rc" "$duration_ms" "$readiness_status" "$observer_status" "$probe_status" "$sample_count" "$field_gap_count" "$peak_rss" "$peak_hwm" "$peak_smaps" "$peak_private" "$probe_json" >> "$result_file"
  [[ "$rc" -eq 0 && "$readiness_status" == "ready" && "$observer_status" == "complete" && "$probe_status" == "present" && "$sample_count" -gt 0 && "$field_gap_count" -eq 0 ]]
}

run=0
for warmup in 1 2; do
  run=$((run + 1))
  run_unobserved 0 WARMUP WARMUP "$run" "$root_dir/current_warmup${warmup}.log"
done

for block in $(seq 1 12); do
  if (( block % 2 == 1 )); then
    order="AB"
    first_arm="A"
    second_arm="B"
  else
    order="BA"
    first_arm="B"
    second_arm="A"
  fi
  for arm in "$first_arm" "$second_arm"; do
    run=$((run + 1))
    log_file="$root_dir/current_${arm,,}_block${block}.log"
    if [[ "$arm" == "A" ]]; then
      run_unobserved "$block" "$order" "$arm" "$run" "$log_file"
    else
      run_observed "$block" "$order" "$arm" "$run" "$log_file"
    fi
  done
done

printf '{"completed_at":"%s","result_sha256":"%s","sample_sha256":"%s"}\n' \
  "$(date --iso-8601=seconds)" \
  "$(sha256sum "$result_file" | awk '{print $1}')" \
  "$(sha256sum "$sample_file" | awk '{print $1}')" >> "$manifest_file"

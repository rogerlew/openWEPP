#!/usr/bin/env bash
set -euo pipefail

# A job must not leave processes, executable files, or dependency state for the
# next increment. Preserve only this hook's runner/worker ancestor chain and
# terminate any other process owned by the dedicated runner UID.
declare -A preserved=()
pid="$$"
while [[ "${pid}" -ge 1 && -r "/proc/${pid}/status" ]]; do
  preserved["${pid}"]=1
  [[ "${pid}" -eq 1 ]] && break
  pid="$(awk '/^PPid:/ { print $2 }' "/proc/${pid}/status")"
done

runner_uid="$(id -u)"
for round in {1..10}; do
  stale=()
  for status in /proc/[0-9]*/status; do
    candidate="${status#/proc/}"
    candidate="${candidate%/status}"
    [[ -n "${preserved[${candidate}]+x}" ]] && continue
    owner="$(awk '/^Uid:/ { print $2; exit }' "${status}" 2>/dev/null || true)"
    [[ "${owner}" == "${runner_uid}" ]] && stale+=("${candidate}")
  done
  [[ "${#stale[@]}" -eq 0 ]] && break
  if [[ "${round}" -eq 1 ]]; then
    kill -TERM "${stale[@]}" 2>/dev/null || true
  else
    kill -KILL "${stale[@]}" 2>/dev/null || true
  fi
  sleep 0.1
done

for status in /proc/[0-9]*/status; do
  candidate="${status#/proc/}"
  candidate="${candidate%/status}"
  [[ -n "${preserved[${candidate}]+x}" ]] && continue
  owner="$(awk '/^Uid:/ { print $2; exit }' "${status}" 2>/dev/null || true)"
  if [[ "${owner}" == "${runner_uid}" ]]; then
    echo "ERROR: runner-owned process ${candidate} survived cleanup" >&2
    exit 1
  fi
done

# The runner registration is read-only at runtime; all writable job surfaces
# are bounded tmpfs mounts and are emptied after every job.
for root in /runner-work /cache/cargo /cache/target /home/runner /tmp /runner-state/_diag; do
  find "${root}" -mindepth 1 -delete
done

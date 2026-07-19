#!/usr/bin/env bash
set -euo pipefail

readonly DIST_ROOT=/opt/actions-runner
readonly STATE_ROOT=/runner-state

seed_runner() {
  if [[ ! -x "${STATE_ROOT}/run.sh" ]]; then
    cp -a "${DIST_ROOT}/." "${STATE_ROOT}/"
  fi
}

register_runner() {
  : "${RUNNER_URL:?RUNNER_URL is required}"
  : "${RUNNER_NAME:?RUNNER_NAME is required}"
  : "${RUNNER_LABELS:?RUNNER_LABELS is required}"
  local registration_token
  IFS= read -r registration_token
  if [[ -z "${registration_token}" ]]; then
    echo "registration token was not provided on stdin" >&2
    exit 2
  fi
  seed_runner
  cd "${STATE_ROOT}"
  ./config.sh --unattended --replace \
    --disableupdate \
    --url "${RUNNER_URL}" \
    --token "${registration_token}" \
    --name "${RUNNER_NAME}" \
    --labels "${RUNNER_LABELS}" \
    --work /runner-work
  unset registration_token
}

run_runner() {
  seed_runner
  if [[ ! -f "${STATE_ROOT}/.runner" || ! -f "${STATE_ROOT}/.credentials" ]]; then
    echo "runner is not registered" >&2
    exit 2
  fi
  cd "${STATE_ROOT}"
  # run.sh rewrites run-helper.sh on every launch. Runtime state is deliberately
  # read-only, so invoke the pinned listener directly and let Docker's bounded
  # restart policy handle listener exits.
  exec ./bin/Runner.Listener run
}

case "${1:-run}" in
  register) register_runner ;;
  run) run_runner ;;
  *) echo "usage: openwepp-runner-entrypoint {register|run}" >&2; exit 2 ;;
esac

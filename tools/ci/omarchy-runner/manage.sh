#!/usr/bin/env bash
set -euo pipefail

readonly HOST="${OPENWEPP_RUNNER_HOST:-omarchy}"
readonly REPOSITORY="${OPENWEPP_RUNNER_REPOSITORY:-rogerlew/openWEPP}"
readonly IMAGE="openwepp-actions-runner:2.335.1"
readonly EXPECTED_IMAGE_ID="sha256:17c413a944e4a456cfceee254425f5c7f081a22b74b9cd88f1b9e8f1f37fcf7a"
readonly CONTAINER="openwepp-actions-runner"
readonly RUNNER_NAME="omarchy-openwepp-01"
readonly LABELS="openwepp,omarchy,trusted"
readonly STATE_VOLUME="openwepp-runner-state"
readonly SCRIPT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"

remote() {
  local command
  printf -v command ' %q' "$@"
  ssh -o BatchMode=yes "${HOST}" "${command:1}"
}

provider_runner_id() {
  gh api "repos/${REPOSITORY}/actions/runners" \
    --jq "[.runners[] | select(.name == \"${RUNNER_NAME}\")][0].id // empty"
}

state_is_registered() {
  remote docker run --rm --entrypoint /bin/bash \
    --mount "type=volume,src=${STATE_VOLUME},dst=/runner-state" \
    "${IMAGE}" -c 'test -f /runner-state/.runner -a -f /runner-state/.credentials'
}

clear_registration_state() {
  remote docker run --rm --user root --entrypoint /bin/bash \
    --mount "type=volume,src=${STATE_VOLUME},dst=/runner-state" \
    "${IMAGE}" -c 'find /runner-state -mindepth 1 -delete'
}

setup_runner() {
  local actual_image_id build_dir registration_token
  build_dir="$(remote mktemp -d /tmp/openwepp-runner-build.XXXXXX)"
  trap 'remote rm -rf -- "${build_dir}" >/dev/null 2>&1 || true' RETURN
  scp -q "${SCRIPT_DIR}/Dockerfile" "${SCRIPT_DIR}/entrypoint.sh" \
    "${SCRIPT_DIR}/job-completed-hook.sh" "${SCRIPT_DIR}/uk2us_rules.json" \
    /usr/local/bin/markdown-doc "${HOST}:${build_dir}/"
  remote docker build --pull --provenance=false --tag "${IMAGE}" "${build_dir}"
  actual_image_id="$(remote docker image inspect --format '{{.Id}}' "${IMAGE}")"
  if [[ "${actual_image_id}" != "${EXPECTED_IMAGE_ID}" ]]; then
    echo "ERROR: runner image identity mismatch: ${actual_image_id}" >&2
    return 1
  fi
  remote docker volume create "${STATE_VOLUME}" >/dev/null
  remote docker stop --time 30 "${CONTAINER}" >/dev/null 2>&1 || true
  remote docker rm "${CONTAINER}" >/dev/null 2>&1 || true
  for obsolete_volume in openwepp-runner-work openwepp-runner-cargo openwepp-runner-target; do
    remote docker volume rm "${obsolete_volume}" >/dev/null 2>&1 || true
  done

  local provider_id
  provider_id="$(provider_runner_id)"
  if state_is_registered && [[ -z "${provider_id}" ]]; then
    clear_registration_state
  elif ! state_is_registered && [[ -n "${provider_id}" ]]; then
    gh api --method DELETE "repos/${REPOSITORY}/actions/runners/${provider_id}"
    provider_id=""
  fi
  if ! state_is_registered; then
    registration_token="$(gh api --method POST \
      "repos/${REPOSITORY}/actions/runners/registration-token" --jq .token)"
    printf '%s\n' "${registration_token}" | ssh -o BatchMode=yes "${HOST}" \
      docker run --rm --interactive \
        --env "RUNNER_URL=https://github.com/${REPOSITORY}" \
        --env "RUNNER_NAME=${RUNNER_NAME}" \
        --env "RUNNER_LABELS=${LABELS}" \
        --mount "type=volume,src=${STATE_VOLUME},dst=/runner-state" \
        "${IMAGE}" register
    unset registration_token
  fi
  remote docker run --detach --name "${CONTAINER}" --restart unless-stopped \
    --cpus 16 --memory 28g --pids-limit 4096 \
    --read-only \
    --security-opt no-new-privileges=true \
    --cap-drop ALL \
    --mount "type=volume,src=${STATE_VOLUME},dst=/runner-state,readonly" \
    --tmpfs "/runner-state/_diag:rw,nosuid,nodev,noexec,size=256m,uid=10001,gid=10001,mode=0700" \
    --tmpfs "/runner-work:rw,nosuid,nodev,size=16g,uid=10001,gid=10001,mode=0770" \
    --tmpfs "/cache/cargo:rw,nosuid,nodev,size=4g,uid=10001,gid=10001,mode=0700" \
    --tmpfs "/t:rw,exec,nosuid,nodev,size=40g,uid=10001,gid=10001,mode=0700" \
    --tmpfs "/home/runner:rw,nosuid,nodev,size=512m,uid=10001,gid=10001,mode=0700" \
    --tmpfs "/tmp:rw,nosuid,nodev,size=1g,uid=10001,gid=10001,mode=1777" \
    "${IMAGE}" run >/dev/null
  for _ in {1..30}; do
    if gh api "repos/${REPOSITORY}/actions/runners" \
      --jq "any(.runners[]; .name == \"${RUNNER_NAME}\" and .status == \"online\")" \
      | grep -Fx true >/dev/null; then
      status_runner
      return
    fi
    sleep 1
  done
  echo "ERROR: runner did not become online within 30 seconds" >&2
  status_runner
  return 1
}

status_runner() {
  remote docker inspect --format '{{.State.Status}} {{.State.Health.Status}}' "${CONTAINER}" 2>/dev/null \
    || remote docker inspect --format '{{.State.Status}}' "${CONTAINER}"
  gh api "repos/${REPOSITORY}/actions/runners" \
    --jq "[.runners[] | select(.name == \"${RUNNER_NAME}\")] | if length == 1 then .[0] | {name,status,busy,labels:[.labels[].name]} else error(\"runner registration missing or duplicated\") end"
}

remove_runner() {
  local provider_id
  provider_id="$(provider_runner_id)"
  if [[ -n "${provider_id}" ]]; then
    gh api --method DELETE "repos/${REPOSITORY}/actions/runners/${provider_id}"
  fi
  remote docker stop --time 30 "${CONTAINER}" >/dev/null 2>&1 || true
  remote docker rm "${CONTAINER}" >/dev/null 2>&1 || true
  remote docker volume rm "${STATE_VOLUME}" >/dev/null 2>&1 || true
  echo "Registration, container, and dedicated registration state removed."
}

case "${1:-}" in
  setup) setup_runner ;;
  status) status_runner ;;
  remove) remove_runner ;;
  *) echo "usage: $0 {setup|status|remove}" >&2; exit 2 ;;
esac

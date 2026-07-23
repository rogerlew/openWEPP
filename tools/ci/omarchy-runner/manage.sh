#!/usr/bin/env bash
set -euo pipefail

readonly HOST="${OPENWEPP_RUNNER_HOST:-forest1}"
readonly REPOSITORY="${OPENWEPP_RUNNER_REPOSITORY:-rogerlew/openWEPP}"
readonly IMAGE="openwepp-actions-runner:2.335.1"
readonly EXPECTED_IMAGE_ID="sha256:8a551a87d0784a74be1a76452beb1e4e6726cc36135722020e20a042e04bae84"
readonly CONTAINER="openwepp-actions-runner"
readonly RUNNER_NAME="${OPENWEPP_RUNNER_NAME:-forest1-openwepp-01}"
readonly SITE_LABEL="${OPENWEPP_RUNNER_SITE_LABEL:-forest1}"
readonly LABELS="${OPENWEPP_RUNNER_LABELS:-openwepp,${SITE_LABEL},trusted}"
readonly LEGACY_RUNNER_NAME="omarchy-openwepp-01"
readonly STATE_VOLUME="openwepp-runner-state"
readonly HISTORY_VOLUME="openwepp-testgate-history"
readonly MIN_AVAILABLE_KIB="$((60 * 1024 * 1024))"
readonly SCRIPT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"

remote() {
  local command
  printf -v command ' %q' "$@"
  ssh -o BatchMode=yes "${HOST}" "${command:1}"
}

provider_runner_id() {
  local runner_name="${1:-${RUNNER_NAME}}"
  gh api "repos/${REPOSITORY}/actions/runners" \
    --jq "[.runners[] | select(.name == \"${runner_name}\")][0].id // empty"
}

provider_contract_matches() {
  gh api "repos/${REPOSITORY}/actions/runners" \
    --jq "[.runners[] | select(.name == \"${RUNNER_NAME}\")] | length == 1 and .[0].status == \"online\" and .[0].busy == false and (([.[0].labels[].name] | sort) == ([\"self-hosted\",\"Linux\",\"X64\",\"openwepp\",\"${SITE_LABEL}\",\"trusted\"] | sort))"
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

build_image() {
  local build_dir
  build_dir="$(mktemp -d /tmp/openwepp-runner-build.XXXXXX)"
  trap 'rm -rf -- "${build_dir}"' RETURN
  cp "${SCRIPT_DIR}/Dockerfile" "${SCRIPT_DIR}/entrypoint.sh" \
    "${SCRIPT_DIR}/job-completed-hook.sh" "${SCRIPT_DIR}/uk2us_rules.json" \
    /usr/local/bin/markdown-doc "${build_dir}/"
  if [[ "$(docker buildx inspect default | awk '/^Driver:/ {print $2}')" != "docker" ]]; then
    echo "ERROR: controller default Buildx driver must be docker" >&2
    return 1
  fi
  docker buildx build --builder default \
    --resource cpuset-cpus=0-7 \
    --resource memory=24g \
    --resource memory-swap=24g \
    --pull --provenance=false --load --tag "${IMAGE}" "${build_dir}"
  docker image inspect --format '{{.Id}} {{.Size}}' "${IMAGE}"
  echo "Review the controller image receipt and bind EXPECTED_IMAGE_ID before install-image."
}

install_image() {
  local actual_image_id archive archive_sha remote_archive remote_sha
  actual_image_id="$(docker image inspect --format '{{.Id}}' "${IMAGE}" 2>/dev/null || true)"
  if [[ "${actual_image_id}" != "${EXPECTED_IMAGE_ID}" ]]; then
    echo "ERROR: controller image identity mismatch: ${actual_image_id}" >&2
    return 1
  fi
  archive="$(mktemp /tmp/openwepp-runner-image.XXXXXX.tar)"
  archive_sha=""
  remote_archive=""
  trap 'rm -f -- "${archive}"; if [[ -n "${remote_archive}" ]]; then remote rm -f -- "${remote_archive}" >/dev/null 2>&1 || true; fi' RETURN
  docker save --output "${archive}" "${IMAGE}"
  archive_sha="$(sha256sum "${archive}" | awk '{print $1}')"
  remote_archive="/tmp/openwepp-runner-image-${archive_sha}.tar"
  scp -q "${archive}" "${HOST}:${remote_archive}"
  remote_sha="$(remote sha256sum "${remote_archive}" | awk '{print $1}')"
  if [[ "${remote_sha}" != "${archive_sha}" ]]; then
    echo "ERROR: transferred runner image archive digest mismatch" >&2
    return 1
  fi
  remote docker load --input "${remote_archive}" >/dev/null
  if [[ "$(remote docker image inspect --format '{{.Id}}' "${IMAGE}")" != "${EXPECTED_IMAGE_ID}" ]]; then
    echo "ERROR: installed forest1 runner image identity mismatch" >&2
    return 1
  fi
  printf 'image_id=%s archive_sha256=%s\n' "${EXPECTED_IMAGE_ID}" "${archive_sha}"
}

setup_runner() {
  local actual_image_id available_kib registration_token
  actual_image_id="$(remote docker image inspect --format '{{.Id}}' "${IMAGE}" 2>/dev/null || true)"
  if [[ -z "${actual_image_id}" ]]; then
    echo "ERROR: reviewed runner image is absent; run build-image, review/rebind it, then run install-image" >&2
    return 1
  fi
  if [[ "${actual_image_id}" != "${EXPECTED_IMAGE_ID}" ]]; then
    echo "ERROR: runner image identity mismatch: ${actual_image_id}" >&2
    return 1
  fi
  available_kib="$(remote awk '/^MemAvailable:/ {print $2}' /proc/meminfo)"
  if [[ ! "${available_kib}" =~ ^[0-9]+$ || "${available_kib}" -lt "${MIN_AVAILABLE_KIB}" ]]; then
    echo "ERROR: forest1 requires at least ${MIN_AVAILABLE_KIB} KiB available before runner launch; observed ${available_kib}" >&2
    return 1
  fi
  remote docker volume create "${STATE_VOLUME}" >/dev/null
  remote docker stop --time 30 "${CONTAINER}" >/dev/null 2>&1 || true
  remote docker rm "${CONTAINER}" >/dev/null 2>&1 || true
  for obsolete_volume in openwepp-runner-work openwepp-runner-cargo openwepp-runner-target; do
    remote docker volume rm "${obsolete_volume}" >/dev/null 2>&1 || true
  done

  local legacy_provider_id provider_id
  legacy_provider_id="$(provider_runner_id "${LEGACY_RUNNER_NAME}")"
  if [[ -n "${legacy_provider_id}" ]]; then
    gh api --method DELETE "repos/${REPOSITORY}/actions/runners/${legacy_provider_id}"
  fi
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
  remote docker run --rm --user root --entrypoint /bin/bash \
    --mount "type=volume,src=${HISTORY_VOLUME},dst=/testgate-history" \
    "${IMAGE}" -c 'chown 10001:10001 /testgate-history && chmod 0700 /testgate-history'
  remote docker run --detach --name "${CONTAINER}" --restart unless-stopped \
    --cpus 32 --cpuset-cpus 0-31 --memory 64g --memory-swap 64g --pids-limit 8192 \
    --read-only \
    --security-opt no-new-privileges=true \
    --cap-drop ALL \
    --mount "type=volume,src=${STATE_VOLUME},dst=/runner-state,readonly" \
    --mount "type=volume,src=${HISTORY_VOLUME},dst=/testgate-history" \
    --tmpfs "/runner-state/_diag:rw,nosuid,nodev,noexec,size=256m,uid=10001,gid=10001,mode=0700" \
    --tmpfs "/runner-work:rw,nosuid,nodev,size=24g,uid=10001,gid=10001,mode=0770" \
    --tmpfs "/cache/cargo:rw,nosuid,nodev,size=8g,uid=10001,gid=10001,mode=0700" \
    --tmpfs "/t:rw,exec,nosuid,nodev,size=56g,uid=10001,gid=10001,mode=0700" \
    --tmpfs "/home/runner:rw,nosuid,nodev,size=1g,uid=10001,gid=10001,mode=0700" \
    --tmpfs "/tmp:rw,nosuid,nodev,size=2g,uid=10001,gid=10001,mode=1777" \
    "${IMAGE}" run >/dev/null
  for _ in {1..30}; do
    if provider_contract_matches | grep -Fx true >/dev/null; then
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
    --jq "[.runners[] | select(.name == \"${RUNNER_NAME}\")] | if length == 1 and .[0].status == \"online\" and .[0].busy == false and (([.[0].labels[].name] | sort) == ([\"self-hosted\",\"Linux\",\"X64\",\"openwepp\",\"${SITE_LABEL}\",\"trusted\"] | sort)) then .[0] | {name,status,busy,labels:[.labels[].name]} else error(\"runner must be uniquely online, idle, and exactly labeled\") end"
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
  build-image) build_image ;;
  install-image) install_image ;;
  setup) setup_runner ;;
  status) status_runner ;;
  remove) remove_runner ;;
  *) echo "usage: $0 {build-image|install-image|setup|status|remove}" >&2; exit 2 ;;
esac

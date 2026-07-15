# openWEPP Release Procedure (Draft)

Status: `planned`  
Document type: `draft-runbook`  
Last reviewed: `2026-07-15`

Execution note:
- This runbook is a draft release procedure synthesized from in-repo
  contracts, ADRs, and code surfaces. It does not assert that a production
  release has been performed.
- `ASSURE03-REL-001` is closed in the ASSURE-03 terminal source: ordinary CI
  explicitly selects validation mode and uploads validation evidence only;
  explicit release mode runs a fail-closed assurance transition preflight and
  snapshots the neutral zero-report state. This runbook still does not claim
  that a production release has been performed.

## Purpose

Define a single maintainer-facing procedure to assemble and validate an
openWEPP release candidate using the current in-repo runner/release contracts
and post-HILLSTAB06 stability gate expectations.

## Normative Inputs

- `docs/contracts/openwepp-binary-release-contract.md`
- `docs/contracts/openwepp-runner-contract.md`
- `docs/decisions/0007-openwepp-runner-and-release-governance.md`
- `docs/governance/scientific-assurance-dossier-lifecycle.md`
- `docs/work-packages/20260529-hillstab06-wb16-peak-closure-and-p24-climate-triage-001/artifacts/worker-handoff.md`
- `crates/openwepp-runner/src/bin/open_wepp_runner.rs`
- `crates/openwepp-runner/src/release.rs`

## Scope

- Candidate build and artifact assembly guidance for:
  - watershed binary release artifact (`openwepp_YYMMDD*`),
  - hillslope binary release artifact (`openwepp_YYMMDD*_hill`),
  - optional replay binary release artifact (`openwepp_YYMMDD*_replay`).
- Required workspace validation gates.
- Required release lint checks.
- Required stability/regression evidence expectations.

Out of scope:
- Tag creation, changelog publication, and external distribution hosting.
- External CI provider settings (for example branch protection and required
  check policy).

## Preconditions

1. Operator is in `/home/workdir/openWEPP` with a clean worktree at the commit
   intended for release.
2. Toolchain availability:
   - `cargo`,
   - `python3`,
   - `sha256sum`.
3. A release tag token `YYMMDD` has been selected.

## Required Validation Gates

Run from `/home/workdir/openWEPP`:

```bash
cargo fmt --check
cargo clippy --workspace --all-targets -- -D warnings
cargo nextest run --workspace --profile full
cargo deny check
bash tools/release/check_assurance_dossier_exports.sh
```

If any command fails, candidate assembly stops.

## Authority-Stack Gate Policy

Release candidate execution must include authority-suite lane outcomes derived
from `docs/specifications/external-authority/registry.yaml`.

Before lane execution, release-gate automation must verify fixture integrity for
all active suites:

1. `fixtures.sha256` exists and passes `sha256sum --check --strict`.
2. `fixtures.provenance.yaml` exists and contains required per-fixture source
   provenance keys.
3. Missing lock/provenance files or hash/provenance mismatches are blocking.
4. `.sol` producer-contract anti-drift suite
   `cas_l4_infile_soil_producer_contract_001` is a required/hard-fail lane and
   blocks release on symbol/order/arity or fixture-integrity regressions.

Default and optional lanes:

1. Required lane (`gate_lane=required`)
   - always runs in release-gate automation.
   - any `failure_class=hard-fail` failure blocks candidate acceptance.
2. Periodic lane (`gate_lane=periodic`)
   - runs on scheduled release-gate workflows or when explicitly requested.
3. Manual lane (`gate_lane=manual`)
   - runs only when explicitly requested.

Failure-class handling:

1. `hard-fail`: workflow exits non-zero and candidate remains blocked.
2. `investigation`: workflow records non-blocking failure in
   `authority_suite_results.md`; operator disposition follow-through is still
   required.

## Automation Entry Points

Workspace validation automation (no snapshot, binary staging, sidecars, release
lint, or release-candidate artifact):

```bash
bash tools/release/run_release_candidate_gates.sh \
  --mode validate \
  --skip-stability
```

Run validation plus required + periodic authority lanes:

```bash
bash tools/release/run_release_candidate_gates.sh \
  --mode validate \
  --skip-stability \
  --run-authority-periodic
```

Explicitly assemble a release candidate with manual/periodic lanes and the
stability cohort:

```bash
bash tools/release/run_release_candidate_gates.sh \
  --mode release \
  --run-authority-periodic \
  --run-authority-manual \
  --cohort-seeds-csv /workdir/wepp-forest/docs/work-packages/20260503-wb05b-forest-hillslope-closure-sweep/artifacts/audits/_meta/defect_seeds.csv \
  --watchlist-csv /workdir/wepp-forest/docs/ablation/hillslope_watchlist.csv \
  --expect-suite wb05b_1166=1166 \
  --expect-suite release_gate_watchlist=19
```

Full gate automation (includes stability cohort and expected suite counts):

```bash
bash tools/release/run_release_candidate_gates.sh \
  --mode release \
  --cohort-seeds-csv /workdir/wepp-forest/docs/work-packages/20260503-wb05b-forest-hillslope-closure-sweep/artifacts/audits/_meta/defect_seeds.csv \
  --watchlist-csv /workdir/wepp-forest/docs/ablation/hillslope_watchlist.csv \
  --expect-suite wb05b_1166=1166 \
  --expect-suite release_gate_watchlist=19
```

CI workflow surface:
- `.github/workflows/release-gates.yml`
  - `push`, `pull_request`, and `schedule` invoke validation mode and upload
    `openwepp-validation-evidence-*`; they cannot snapshot or assemble.
  - `schedule`: validation plus required + periodic authority lanes.
  - `workflow_dispatch` + `run_stability=true`: self-hosted stability cohort
    lane with suite assertions.
  - `workflow_dispatch` + both `assemble_release=true` and
    `run_stability=true`: explicit release mode and
    `openwepp-release-candidate-*` upload only after validation, the separately
    bound stability job, transition preflight, and assembly all succeed.
    Preflight or assembly failure uploads only
    `openwepp-release-failure-evidence-*`; it never uploads a candidate-named
    artifact.
  - `workflow_dispatch` inputs:
    - `run_authority_periodic=true` enables periodic authority lane.
    - `run_authority_manual=true` enables manual authority lane.

## Candidate Build and Assembly

Candidate assembly is explicit. The aggregate script requires `--mode release`
and runs `check_assurance_release_transition.sh` before creating the release
directory. The preflight rejects a transition marker, a nonempty or ambiguous
legacy catalog, any byte-level departure from the exact typed zero-report
transition catalog, any active retired v1 source/public route, and symlink or
special-file evasions of those controls. A retired root may exist only as a
real, non-symlink, completely empty directory. The workflow runs the same
preflight before it creates its candidate directory. The retained transition
compiler then snapshots exactly zero reports and two neutral outputs.

```bash
cargo run --quiet -p openwepp-assurance -- build --all \
  --snapshot "${OPENWEPP_RELEASE_TAG}" \
  --snapshot-root "${OPENWEPP_RELEASE_DIR}/assurance-snapshots"
sha256sum \
  "${OPENWEPP_RELEASE_DIR}/assurance-snapshots/${OPENWEPP_RELEASE_TAG}/manifest.json"
```

An identical zero-report rerun confirms the existing immutable snapshot;
different content under one snapshot ID is a blocking conflict. The manifest
records zero reports, the two source identities, and the two generated-output
identities. This transition snapshot is release evidence that v1 is absent; it
is not a scientific report or v2 publication authority. Future v2 snapshots
will record approved report, supplement, dependency, review, release, and
public-file identities.

### 1) Build runner and CLI binaries

```bash
cargo build --release -p openwepp-runner --bin open_wepp_runner --bin openwepp-cli-hill --bin openwepp-cli-watershed
```

### 2) Stage release directory and canonical names

```bash
export OPENWEPP_RELEASE_TAG=260529
export OPENWEPP_RELEASE_DIR=/tmp/openwepp_release_${OPENWEPP_RELEASE_TAG}
rm -rf "${OPENWEPP_RELEASE_DIR}"
mkdir -p "${OPENWEPP_RELEASE_DIR}"

cp target/release/openwepp-cli-watershed "${OPENWEPP_RELEASE_DIR}/openwepp_${OPENWEPP_RELEASE_TAG}"
cp target/release/openwepp-cli-hill "${OPENWEPP_RELEASE_DIR}/openwepp_${OPENWEPP_RELEASE_TAG}_hill"
```

If replay binary exists in a future revision, copy it to:
`openwepp_${OPENWEPP_RELEASE_TAG}_replay`.

### 3) Generate release sidecars for staged binaries

Use runner sidecar emission for each staged binary:

```bash
target/release/open_wepp_runner release sidecar \
  --binary "${OPENWEPP_RELEASE_DIR}/openwepp_${OPENWEPP_RELEASE_TAG}" \
  --role watershed

target/release/open_wepp_runner release sidecar \
  --binary "${OPENWEPP_RELEASE_DIR}/openwepp_${OPENWEPP_RELEASE_TAG}_hill" \
  --role hillslope
```

If replay binary is staged, emit its sidecar:

```bash
if [ -f "${OPENWEPP_RELEASE_DIR}/openwepp_${OPENWEPP_RELEASE_TAG}_replay" ]; then
  target/release/open_wepp_runner release sidecar \
    --binary "${OPENWEPP_RELEASE_DIR}/openwepp_${OPENWEPP_RELEASE_TAG}_replay" \
    --role replay
fi
```

## Release Lint Gate

Run runner contract lint on staged artifacts:

```bash
target/release/open_wepp_runner release lint --release-dir "${OPENWEPP_RELEASE_DIR}"
```

Expected outcome: command exits `0`.

Failure IDs:
- naming violations: `RUNNER-E-006`
- sidecar existence/schema/pairing failures: `RUNNER-E-005`

## Stability Evidence Gate

Run broad hillslope stability cohort before release signoff:

```bash
bash tools/release/run_hillstab_gate.sh \
  --openwepp-binary /home/workdir/openWEPP/target/release/openwepp-cli-hill \
  --cohort-seeds-csv /workdir/wepp-forest/docs/work-packages/20260503-wb05b-forest-hillslope-closure-sweep/artifacts/audits/_meta/defect_seeds.csv \
  --watchlist-csv /workdir/wepp-forest/docs/ablation/hillslope_watchlist.csv \
  --output-json /tmp/openwepp_release_gate_hillstab_results.json \
  --expect-suite wb05b_1166=1166 \
  --expect-suite release_gate_watchlist=19
```

Minimum expectation for pass:
- `wb05b_1166`: `1166/1166` pass,
- `release_gate_watchlist`: `19/19` pass,
- no reintroduction of HILLSTAB06-closed residual families.

## Release Candidate Evidence Bundle

A release candidate must archive:

1. workspace gate logs (`fmt`, `clippy`, full-profile `nextest`, `deny`),
2. staged release directory listing with sidecars,
3. successful `open_wepp_runner release lint` output,
4. hillslope stability JSON report and a delta summary against the latest
   baseline package (currently HILLSTAB06),
5. commit SHA and selected release tag,
6. zero-report assurance snapshot manifest and two copied transition outputs,
   `assurance-snapshot-build.txt`, and `assurance-snapshot.sha256`, and
7. authority lane report (`authority_suite_results.md`) with explicit lane and
   failure-class outcomes, including fixture-integrity results for active
   suites.

## Known Gaps (Draft Follow-On)

1. Promote status from `planned` to `completed` after first full release
   candidate run that archives all evidence-bundle artifacts.
2. Stability lane requires runner environments with
   `/workdir/wepp-forest` and `/wc1/runs` data roots available.

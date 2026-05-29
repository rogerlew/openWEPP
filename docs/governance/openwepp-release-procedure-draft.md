# openWEPP Release Procedure (Draft)

Status: `planned`  
Document type: `draft-runbook`  
Last reviewed: `2026-05-29`

Execution note:
- This runbook is a draft release procedure synthesized from in-repo
  contracts, ADRs, and code surfaces. It does not assert that a production
  release has been performed.

## Purpose

Define a single maintainer-facing procedure to assemble and validate an
openWEPP release candidate using the current in-repo runner/release contracts
and post-HILLSTAB06 stability gate expectations.

## Normative Inputs

- `docs/contracts/openwepp-binary-release-contract.md`
- `docs/contracts/openwepp-runner-contract.md`
- `docs/decisions/0007-openwepp-runner-and-release-governance.md`
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
cargo test --workspace
cargo deny check
```

If any command fails, candidate assembly stops.

## Automation Entry Points

Workspace + release-lint automation (no stability cohort):

```bash
bash tools/release/run_release_candidate_gates.sh --skip-stability
```

Full gate automation (includes stability cohort and expected suite counts):

```bash
bash tools/release/run_release_candidate_gates.sh \
  --cohort-seeds-csv /workdir/wepp-forest/docs/work-packages/20260503-wb05b-forest-hillslope-closure-sweep/artifacts/audits/_meta/defect_seeds.csv \
  --watchlist-csv /workdir/wepp-forest/docs/ablation/hillslope_watchlist.csv \
  --expect-suite wb05b_1166=1166 \
  --expect-suite release_gate_watchlist=19
```

CI workflow surface:
- `.github/workflows/release-gates.yml`
  - `push` / `pull_request`: workspace + release lint lane.
  - `workflow_dispatch` + `run_stability=true`: self-hosted stability cohort
    lane with suite assertions.

## Candidate Build and Assembly

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

1. workspace gate logs (`fmt`, `clippy`, `test`, `deny`),
2. staged release directory listing with sidecars,
3. successful `open_wepp_runner release lint` output,
4. hillslope stability JSON report and a delta summary against the latest
   baseline package (currently HILLSTAB06),
5. commit SHA and selected release tag.

## Known Gaps (Draft Follow-On)

1. Promote status from `planned` to `completed` after first full release
   candidate run that archives all evidence-bundle artifacts.
2. Stability lane requires runner environments with
   `/workdir/wepp-forest` and `/wc1/runs` data roots available.

# Groundwater/Baseflow For Lane D Single-OFE And MOFE Implementation

Status: `QUEUED`
Package ID: `20260708-groundwater-baseflow-laned-single-ofe-mofe-implementation-001`
Queue row: `M-T2B`
Owner: Codex
Scaffold date: `2026-07-08`
Evidence mode: `Static scaffold; implementation not executed`

## Objective

Implement `SC-GWBASEFLOW-001` groundwater-reservoir baseflow for the canonical
Lane D execution path, including both:

- Lane D single-OFE runs (`lane_count = 1`); and
- Lane D MOFE runs (`lane_count > 1`).

The package must consume `SC-INFILE-GWCOEFF-001` parser state, implement the
daily Srivastava linear-reservoir recurrence, export generated groundwater
baseflow and reservoir deep seepage without feeding them into the Lane D surface
router, and prove the active water ledger distinguishes surface routing,
`ui_SCrunf` exfiltration, `latqcc`, generated groundwater baseflow, and
groundwater-reservoir deep seepage.

## Framing Decision

M-T2B is intentionally reframed from "single-OFE and Lane D MOFE" to
"Groundwater/baseflow for Lane D single-OFE and MOFE." The implementation target
is the canonical Lane D hourly production path. The degenerate one-lane Lane D
case is the production single-OFE path for new physics.

Legacy non-Lane-D single-OFE/MOFE paths remain compatibility, validation, and
rollback surfaces. This package must preserve them, but must not add new
production groundwater/baseflow behavior to those paths unless a contract-first
amendment explicitly widens scope before implementation.

## Readiness Summary

Ready to scaffold and execute after the two management packages finish their
closing tests.

Closed or closing prerequisites:

- M-T2A created `SC-GWBASEFLOW-001` and handed off exact implementation
  obligations.
- M-T2P rejected coefficient projection; no hidden route-coefficient inference
  is allowed.
- M-T2Q locked `ow-lanuse-1+` native route coefficients as Lane D production
  route authority.
- M-T2S implemented canonical management YAML and proved runtime route
  coefficient projection from YAML.
- M-T2R implements `openwepp-landuse-migrate` for coefficient-complete native
  YAML migration; package artifacts currently indicate implementation complete
  while local closing-test edits remain uncommitted.

Current code already has a `gwcoeff` parser with explicit missing/parsed
branches. The missing implementation is runtime ownership: direct state,
recurrence, Lane D aggregation, boundary registry/output metadata, consumer
path proof, and fail-closed guards.

## Scope

### Included

- Package-local execution artifacts, prompt, review, verification, gates, and
  disposition.
- Contract-first review/amendment of `SC-GWBASEFLOW-001`,
  `SC-OFEROUTE-001`, and boundary/output registries only where implementation
  needs executable binding or metadata.
- Contract-derived tests for `TV-GWBASEFLOW-001` through
  `TV-GWBASEFLOW-008`.
- Runtime intake for `GwcoeffFile` parser state into the hillslope/direct
  execution request.
- Lane D single-OFE and MOFE recurrence:
  - daily recharge from deep percolation;
  - groundwater storage carry;
  - generated groundwater baseflow;
  - groundwater-reservoir deep seepage;
  - disabled branch when `gwcoeff.txt` is absent.
- Active Lane D ledger/export proof:
  - generated baseflow is not a surface-router source;
  - reservoir deep seepage is not re-routed;
  - `ui_SCrunf` remains the only subsurface-to-surface active-router source;
  - `latqcc` remains separate from generated groundwater baseflow.
- Protected legacy/off behavior and native-authority fail-closed tests.
- HBP/pass or watershed consumer proof for generated baseflow/deep seepage when
  claiming export closure.

### Excluded

- No nonlinear Srivastava et al. (2017) baseflow algorithms.
- No surrogate, provisional, proxy, empirical stand-in, or heuristic
  groundwater/baseflow physics.
- No coefficient defaults when `gwcoeff.txt` is absent.
- No optional sidecar route-coefficient authority.
- No legacy-field projection for route coefficients.
- No new production behavior for non-Lane-D legacy/off paths.
- No watershed HBP hourly water/sediment consumption; that remains M-T3 unless
  this package explicitly closes only the generated baseflow export leg needed
  before M-T3.
- No wepppy edits.

## Required Reading

Core:

- `AGENTS.md`
- `docs/work-packages/AGENTS.md`
- `docs/standards/AGENTS.md`
- `docs/standards/prompt-wording-guidance.md`
- `docs/specifications/science-contracts/AGENTS.md`
- `crates/AGENTS.md`
- `tests/AGENTS.md`
- `docs/ROADMAP.md` `## Watershed Runtime Performance Queue`
- `docs/specifications/science-contracts/contracts/SC-GWBASEFLOW-001.md`
- `docs/specifications/science-contracts/contracts/SC-INFILE-GWCOEFF-001.md`
- `docs/work-packages/20260708-groundwater-baseflow-srivastava-authority-001/package.md`
- `docs/work-packages/20260708-groundwater-baseflow-srivastava-authority-001/artifacts/worker-handoff.md`
- `docs/work-packages/20260708-openwepp-management-yaml-canonical-authorization-001/package.md`
- `docs/work-packages/20260708-openwepp-management-yaml-canonical-authorization-001/artifacts/worker-handoff.md`
- `docs/work-packages/20260708-landuse-migration-cli-spec-implementation-001/package.md`
- `docs/work-packages/20260708-landuse-migration-cli-spec-implementation-001/artifacts/worker-handoff.md`
- this package's `artifacts/required-reading-map.md`

Conditional:

- `docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md` before
  active Lane D ledger or `INV-OFEROUTE-012` edits.
- `docs/specifications/science-contracts/contracts/SC-SUBHYD-001.md` before
  touching `latqcc` lineage.
- `docs/specifications/science-contracts/contracts/SC-INFILE-MANAGEMENT-YAML-001.md`
  before YAML input-surface or native-authority changes.
- `docs/contracts/openwepp-management-lanuse-authority-contract.md` before
  changing Lane D default/native eligibility.
- `docs/specifications/science-contract-authoring-procedure.md` and
  `docs/specifications/science-contracts/kernel-process-contract-profile.md`
  before amending `SC-*` contracts.

On demand:

- `/workdir/wepp-forest_260430_baseline/src/main.for`
- `/workdir/wepp-forest_260430_baseline/src/contin.for`
- `/workdir/wepp-forest_260430_baseline/src/wshpas.for`
- `/workdir/wepp-forest_260430_baseline/src/wshdrv.for`
- `/workdir/wepp-forest_260430_baseline/src/wshchr.for`
- `/workdir/wepp-forest_260430_baseline/src/wshcqi.for`
- `/workdir/wepp-forest_260430_baseline/src/watbalprint.for`
- `crates/openwepp-input-contract/src/parsers/gwcoeff.rs`
- `crates/openwepp-hillslope-orchestrator/src/direct_runtime/**`
- `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/**`
- `crates/openwepp-runner/src/**`
- `crates/openwepp-watershed-orchestrator/src/**`
- `crates/openwepp-watershed-output/src/**`
- focused integration tests under `tests/integration/**`

## Intended Write Set

Package:

- `docs/work-packages/20260708-groundwater-baseflow-laned-single-ofe-mofe-implementation-001/**`
- `docs/work-packages/README.md`
- `docs/ROADMAP.md`

Contract/metadata, if needed:

- `docs/specifications/science-contracts/contracts/SC-GWBASEFLOW-001.md`
- `docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md`
- `docs/specifications/science-contracts/index.md`
- boundary/output registry files under `crates/openwepp-sim-contract/**`
- relevant input/output specs when metadata changes

Implementation:

- `crates/openwepp-input-contract/src/parsers/gwcoeff.rs` only for missing
  executable handoff APIs or tests; do not weaken parser guards.
- `crates/openwepp-hillslope-orchestrator/src/direct_runtime/**`
- `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/**`
- `crates/openwepp-runner/src/**`
- `crates/openwepp-watershed-orchestrator/src/**` only for generated baseflow
  consumer proof or a recorded hold boundary.
- `crates/openwepp-watershed-output/src/**` only if publication metadata lands.
- focused tests under `tests/integration/**` and crate-local test modules.

Protected:

- Do not modify M-T2S/M-T2R package artifacts unless the operator explicitly
  asks to finish their closing-test cleanup.
- Do not create or switch branches.
- Do not touch wepppy.
- Do not add native flat management writers, sidecars, route-coefficient
  projection, or hidden defaults.
- Do not revive legacy DC01 daily-lump surface runon for active lanes.

## Phase Plan

### Phase A - Contract And Current-State Audit

1. Reconfirm M-T2S and M-T2R closing state and record any stale-artifact caveat.
2. Build a source map from `gwcoeff` parser output to direct runtime state,
   Lane D active ledger, HBP/pass surfaces, and watershed/channel consumers.
3. Identify whether `SC-GWBASEFLOW-001` needs promotion/amendment before code.
4. Record existing symbols that must not be conflated:
   - current deep-percolation/deep-seepage surfaces;
   - generated groundwater-reservoir baseflow `gwbfv`;
   - generated groundwater-reservoir deep seepage `gwdsv`;
   - lateral export `latqcc`;
   - channel `cbase`.

### Phase B - Contract-Derived Tests First

1. Add failing tests for `TV-GWBASEFLOW-001` through `TV-GWBASEFLOW-008`.
2. Include a one-lane Lane D recurrence vector and a multi-lane MOFE vector.
3. Include namespace guards proving wrong aliases fail:
   `latqcc`, current soil deep percolation, generated `gwbfv`/`gwdsv`, and
   `cbase` cannot satisfy each other.
4. Include active mode tests proving generated groundwater baseflow does not
   enter the routed surface source series.

### Phase C - Runtime Implementation

1. Thread `GwcoeffFile`/groundwater authority state into direct runtime inputs.
2. Add typed groundwater/baseflow state with daily storage carry.
3. Compute daily recharge from deep percolation volume over Lane D lanes.
4. Implement the Srivastava recurrence:
   `S_i = S_{i-1} + D_i - Qb_{i-1} - Qs_{i-1}`,
   `Qb_i = bfcoeff * S_i`, and `Qs_i = dscoeff * S_i`.
5. Keep generated baseflow/deep seepage out of the Lane D surface router.
6. Add explicit disabled/missing-authority state for absent `gwcoeff.txt`.
7. Fail closed on malformed present sidecar, non-finite state, negative state,
   outflow-over-storage without carry evidence, and mixed active authority.

### Phase D - Export, Publication, And Consumer Proof

1. Add boundary-symbol registry or output metadata for storage, recharge,
   generated baseflow, and generated groundwater-reservoir deep seepage.
2. Populate HBP/pass or watershed handoff fields, or stop in hold if the real
   consumer boundary cannot safely close in this package.
3. Prove the real downstream consumer reads generated `gwbfv`/`gwdsv` before
   claiming export/publication closure.
4. Preserve explicit distinction between generated zero, disabled process,
   missing authority, and legacy-carried generated baseflow.

### Phase E - Evidence, Review, Verification, And Disposition

1. Run focused tests and the full implementation closure loop.
2. Complete dual review and dual verification.
3. Disposition every finding.
4. If export closure is complete, update M-T2 status and unblock M-T3.
5. If a consumer boundary remains outside envelope, close as
   `EXECUTED-HOLD-*` with a hold legitimacy audit and scaffold/handoff a
   narrow hold-lift.

## Subagent Authorization

This package explicitly authorizes subagent spawning/delegation to read-only
review, verification, and comparator/gate runner subagents for contract review,
implementation review, consumer-path verification, focused test/comparator
execution, and full closure gate execution. Expected outputs are package-local
`artifacts/review-*.md`, `artifacts/verification-*.md`, and compact gate logs
or command summaries in `artifacts/gate-results.md`. Write access is read-only
unless the operator explicitly assigns a bounded implementation write set.

Subagent requirement: REQUIRED for heavy batch/closure/comparator runs when
available. The parent agent must not run full workspace closure gates directly
when an authorized gate-runner subagent is available; if unavailable, record
the tool-policy or spawn failure before running locally.

## Required Artifacts

- `artifacts/README.md`
- `artifacts/required-reading-map.md`
- `artifacts/readiness-assessment.md`
- `artifacts/source-map.md`
- `artifacts/operand-lineage.md`
- `artifacts/contract-disposition.md`
- `artifacts/pre-implementation-contract-gate.md`
- `artifacts/test-plan.md`
- `artifacts/implementation.md`
- `artifacts/consumer-path-proof.md`
- `artifacts/gate-results.md`
- `artifacts/review-*.md`
- `artifacts/verification-*.md`
- `artifacts/disposition.md`
- `artifacts/hold-legitimacy-audit.md`
- `artifacts/final-disposition.md`
- `artifacts/worker-handoff.md`

## Gates

Required for scaffold:

- Package scaffold, prompt, and placeholder artifacts exist.
- `docs/ROADMAP.md` reframes M-T2B as Lane D single-OFE/MOFE only.
- `docs/work-packages/README.md` points to the queued package.
- `markdown-doc lint` for touched package/docs.
- `git diff --check`.

Required for implementation closure:

- Contract-first sequencing completed before production code.
- Contract/profile/BEI checks required by touched `SC-*` contracts.
- `TV-GWBASEFLOW-001` through `TV-GWBASEFLOW-008` have direct tests or an
  explicit hold boundary.
- Focused `gwcoeff` parser/handoff tests.
- Focused Lane D single-OFE recurrence vector.
- Focused Lane D MOFE aggregation vector.
- Active-mode proof that generated groundwater baseflow/deep seepage do not
  enter the surface-router source series.
- Active ledger proof separating routed surface outflow, `ui_SCrunf`,
  `latqcc`, generated groundwater baseflow, and groundwater-reservoir deep
  seepage.
- Protected legacy/off/default identity or explicitly unchanged fallback proof.
- Native missing/malformed/mixed groundwater authority fail-closed proof.
- Real HBP/pass or watershed consumer proof for generated `gwbfv`/`gwdsv`, or
  `EXECUTED-HOLD-*` with a legitimacy audit.
- Markdown/doc lint for touched docs.
- `git diff --check`
- `cargo fmt --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo nextest run --workspace --profile full`
- `cargo deny check`
- Source-level anti-evasion guards if any required-case binding, cohort
  fixture, or external-authority suite posture is touched:
  - `bash tools/release/check_authority_suite_antievasion.sh`
  - `cargo nextest run --test auth11_required_suite_obligation_guards_contract`

## Exit Criteria

`SCAFFOLDED`:

- Package scaffold, artifacts, and kickoff prompt exist.
- M-T2B roadmap row uses the Lane D single-OFE/MOFE framing.
- Readiness and implementation-gap assessment is recorded.
- Scaffold docs lint and diff hygiene pass.
- No implementation claim is made.

`EXECUTED-COMPLETE`:

- `SC-GWBASEFLOW-001` recurrence is implemented for Lane D single-OFE and MOFE.
- Generated groundwater baseflow/deep seepage are exported with explicit
  lineage and are not fed to the active surface router.
- Real consumer proof closes the generated baseflow/deep-seepage export path.
- Required gates pass and findings are dispositioned.
- M-T3 Lane D watershed hourly water/sediment consumption is unblocked on the
  baseflow/export leg.

`EXECUTED-HOLD-*`:

- A named consumer boundary, missing authority, or unavailable evidence blocks
  closure after in-envelope implementation/test work is complete.
- `artifacts/hold-legitimacy-audit.md` names the blocker, evidence, considered
  in-envelope correction route, why it cannot safely close now, and the first
  actionable follow-on package/action.

## Final Outcome

Queued scaffold. Implementation has not started.

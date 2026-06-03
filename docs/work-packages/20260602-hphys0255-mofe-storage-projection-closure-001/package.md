# 20260602-hphys0255-mofe-storage-projection-closure-001

Status: complete/HOLD

This package is a living ExecPlan. Keep `Progress`, `Surprises & Discoveries`,
`Decision Log`, and `Outcomes & Retrospective` current while executing it. It
follows `/workdir/openWEPP/docs/codex_exec_plans.md`.

## Purpose / Big Picture

HPHYS0254 corrected WB11 seed projection for single-OFE HPHYS lanes. This
package closes the immediate MOFE coverage gap: multi-OFE soil intake must make
WB11 seed aliases, scoped OFE soil symbols, and WB13/H.wat storage publication
semantics explicit so later HPHYS/MOFE agents do not infer accidental primary
OFE behavior or silently aggregate storage without authority.

## Objective

Diagnose, correct where contract-authorized, and validate MOFE storage
projection for WB11 seed/runtime aliases and WB13/H.wat storage publication.
The outcome is a contract-backed implementation posture with tests showing
whether MOFE storage is deliberately primary-runtime-state publication,
contract-authorized aggregate storage publication, or held for a future
per-OFE hydrology-state migration.

## Rationale

The seed projection suite currently proves normalized WB11 storage for a
single OFE only. MOFE04 proves aggregate publication area/provenance, but does
not define storage semantics for `Total-Soil`, `SoilWaterTotal`,
`ProfileDepth`, `ProfilePorosityCap`, `ProfileFCStore`, or `ProfileWPStore` in
multi-OFE contexts. This gap is dangerous because openWEPP currently publishes
aggregate area while WB13 storage fields consume a single WB11 runtime state.

## Included Scope

- Amend canonical `SC-WATBAL-001`, `SC-SOIL-001`, and `SC-SYSTEM-001` as
  needed to define MOFE storage projection and publication semantics.
- Add contract-derived tests before any production edits.
- Diagnose current runtime surfaces for asymmetric multi-OFE soil input:
  `wb11_nsl`, `wb19_*`, `wb18_perc_*`, `wb13_profile_*`, generic scoped
  `ofeN_*` soil symbols, and runner WB13/H.wat storage publication lineage.
- Correct production code only when canonical contract authority and available
  runtime state support a non-heuristic implementation.
- Record targeted test/gate evidence, reviews, verification, and final
  `GO`/`HOLD` disposition.

## Excluded Scope

- No heuristic area-weighted storage synthesis from static soil rows when the
  dynamic WB11/WB17/WB18/WB19 hydrology state remains single-state.
- No per-OFE hydrology scheduler/vectorization migration unless baseline
  authority and package evidence prove it is already narrowly implementable.
- No WB18/WB19/WB17 residual correction beyond storage projection guards.
- No full `H1..H39` semantic rerun unless needed to support a production code
  change; this package is primarily a contract/test closure package.
- No commit/push unless separately requested.

## Deliverables

1. Package-local evidence artifacts under this directory.
2. Canonical contract amendments for MOFE storage projection semantics.
3. Contract-derived MOFE seed/publication tests.
4. Minimal production correction if a contract-authorized defect is confirmed.
5. Gate results and truthful final disposition.

## Dependencies

- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SOIL-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `/workdir/openWEPP/docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `/workdir/openWEPP/docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`
- `/workdir/openWEPP/docs/work-packages/20260525-mofe04-output-publication-closure-001/package.md`
- `/workdir/openWEPP/docs/work-packages/20260602-hphys0254-wb11-initial-storage-projection-closure-001/package.md`
- `/workdir/wepp-forest_260430_baseline/src/watbal.for`
- `/workdir/wepp-forest_260430_baseline/src/watbal_hourly.for`
- `/workdir/wepp-forest_260430_baseline/src/scon.for`

## Intended Write Set

- `docs/work-packages/20260602-hphys0255-mofe-storage-projection-closure-001/**`
- `docs/work-packages/README.md`
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `docs/specifications/science-contracts/contracts/SC-SOIL-001.md`
- `docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `tests/integration/wb11_storage_projection_kernel_contract.rs`
- `tests/integration/cli03_runner_contract_derived_tests.rs`
- `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/02_soil_slope.rs`
- `crates/openwepp-runner/src/hillslope/mod.rs`

## Contract-First Sequence

1. Implement required canonical contract amendments.
2. Implement contract-derived tests.
3. Record pre-implementation contract gate evidence.
4. Modify production code.

No production code edits are allowed before steps 1-3 are complete.

## Phase Plan

### Phase A — MOFE Storage Diagnosis

Read MOFE04, HPHYS0254, canonical contracts, and current runtime/runner code.
Record whether multi-OFE storage surfaces are primary-runtime-state,
area-weighted aggregate, or undefined.

### Phase B — Contract and Test Gate

Amend canonical contracts to make the storage semantics explicit. Add tests for
asymmetric multi-OFE soil projection and runner publication invariants. Run the
targeted tests before any production edit and record the evidence.

### Phase C — Production Correction

If tests expose a contract violation that can be corrected without heuristic
physics, apply the minimal code change. If dynamic aggregate storage requires a
per-OFE hydrology-state migration not present in the architecture, keep the
package in `HOLD` and document the follow-on.

### Phase D — Review, Verification, and Disposition

Run targeted gates. Record dual review/verification artifacts, gate results,
owned files, worker handoff, and final disposition.

## Exit Criteria

- MOFE storage semantics are explicit in canonical contracts.
- Contract-derived tests cover asymmetric multi-OFE storage projection.
- Production behavior either satisfies those tests or the package records a
  precise blocker with no false closure.
- All evidence artifacts use truthfulness labels (`Static:` or `Ran:`).
- Disposition is truthful; unimplemented per-OFE dynamic storage remains
  `HOLD`, not `GO`.

## Security-Impact Gate

No external systems or network actions are required. Work is local repository
engineering over flat files and local diagnostic commands. No secrets,
credentials, user data, or production service state are accessed or modified.

## Progress

- [x] (2026-06-02) Scaffold package.
- [x] (2026-06-02) Complete MOFE storage diagnosis.
- [x] (2026-06-02) Amend contracts and add tests.
- [x] (2026-06-02) Add manifest storage-lineage provenance field.
- [x] (2026-06-02) Run targeted and full Rust validation gates.
- [x] (2026-06-02) Record review, verification, disposition, and handoff.

## Surprises & Discoveries

- Current runtime projection already keeps asymmetric MOFE parser/corrected
  soil symbols scoped (`ofeN_*`) while unqualified WB11 seed aliases remain the
  active single WB11 runtime state.
- The actual production gap was provenance: MOFE WB13 publication exposed
  aggregate `Area` but did not state that storage fields remained single
  runtime-state lineage.
- Dynamic per-OFE storage aggregation is still not implemented and should not
  be inferred from aggregate area.

## Decision Log

- Decision: add `single-runtime-wb11-state` storage-lineage policy provenance
  to WB13 publication manifests.
  Rationale: it makes current storage semantics explicit without inventing
  static area-weighted storage physics.
  Date/Author: 2026-06-02 / Codex.
- Decision: final disposition is `HOLD`.
  Rationale: HPHYS0255 closes the immediate projection/provenance gap, but true
  dynamic per-OFE aggregate storage would require a future per-OFE hydrology
  state migration and contract amendment.
  Date/Author: 2026-06-02 / Codex.

## Outcomes & Retrospective

- Canonical `SC-*` contracts now separate MOFE aggregate area from WB11/WB13
  storage lineage.
- `tests/integration/wb11_storage_projection_kernel_contract.rs` now includes
  asymmetric MOFE seed projection coverage.
- `tests/integration/cli03_runner_contract_derived_tests.rs` now requires
  `storage_lineage_policy = "single-runtime-wb11-state"` in MOFE publication
  manifests.
- Full Rust gates passed; `cargo deny check` retained existing duplicate-crate
  and unmatched-license-allowance warnings only.
- Continuation, if desired, is a larger per-OFE dynamic hydrology-state
  migration package, not a publication-side aggregation patch.

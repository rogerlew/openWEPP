# MOFEFID-D13 - Routed-Hydrograph Erosion Shape

Status: **EXECUTED-COMPLETE** (2026-07-06). Campaign:
[MOFEFID](../../planning/mofe-fidelity-campaign-strategy.md) Lane D.
Contract focus: ADR-0036 active-routed-water touchpoint across
`SC-OFEROUTE-001` and `SC-SED-001`.

## Objective

Close the ADR-0036/Lane D erosion-shape blocker: when Lane D routing owns the
surface-water path, the Wave-1 hourly erosion substrate must consume the
routed hydrograph, not the DC01 source-shape authority. D13 must author and
implement the contract-backed active-mode consumer path or close in a
legitimate `HOLD` with a named authority/interface boundary.

D13 must keep the default/runtime-off path byte-flat and must not activate
Lane D production routing.

## Rationale

D12 completed the source-shape input side for the opt-in shadow/DC01 path:
`snow.hourly_routed_melt_m` now joins WB14 excess and `ui_SCrunf` lineage in
the shared source-shape helper. That solves the local source distribution, but
it does not decide what erosion should consume after the OFE router owns water.

ADR-0036 requires water and sediment timing to stay paired. If active Lane D
routes water through `ofe_routing`, then leaving erosion's hourly substrate on
the pre-routing DC01 shape would silently time sediment against the old
hydrograph. Water conservation could pass while sediment deposition/export is
computed on the wrong hours. D13 owns that consumer-path seam.

## Scope

### Included

- Contract-first amendment or confirmation for the active-routed-water erosion
  shape rule in `SC-OFEROUTE-001`, `SC-SED-001`, and any directly affected
  ADR-0036 cross-reference text.
- Source audit of the current erosion hourly-shape path:
  - `dc01_surface_runoff_hourly_weights`,
  - `wave1_hourly_weights` / `wave1_hourly_plan`,
  - `hourly_runoff_fraction`,
  - `hourly_sediment_mass_kg`,
  - `publish_erosion_inflow_to_downstream`,
  - HBP `V_h` / `S_h` assembly, and
  - Lane D shadow routed-hydrograph outputs/diagnostics.
- Add or prove an activation-candidate routed-hydrograph shape surface that
  the Wave-1 hourly erosion substrate can consume when routing owns water.
- Add contract-derived tests proving:
  - the default/off path remains byte-identical,
  - the active-mode/candidate erosion substrate reads the routed hydrograph,
    not DC01 weights,
  - paired hourly water/sediment closures hold (`Σ V_h`, `Σ S_h`),
  - malformed/missing routed hydrograph surfaces fail closed, and
  - the old DC01 shape cannot carry the active-mode closure claim.
- Update package artifacts, work-package catalog, Lane D planning, and worker
  handoff.

### Excluded

- No production/default Lane D activation.
- No DC01 disable / routing-owns-water flip; D15 owns that activation.
- No D10 `GAP-OFEROUTE-005` shock-numerics correction or Case-4 acceptance.
- No D11 friction-source changes beyond preserving the rev-21 operand path.
- No D12 melt-limb source-shape changes except read-only regression proof.
- No D14 profiling/optimization.
- No D16 default-promotion policy.
- No watershed/channel routing changes and no channel-hourly sediment
  extension.
- No HBP/pass schema change unless the contract gate proves a current D13
  consumer-path closure requires an additive metadata/schema field now.
- No surrogate, provisional, proxy, empirical stand-in, or heuristic process
  physics.

## Dependencies

- `SC-OFEROUTE-001` rev 22.
- `SC-SED-001` current `INV-SED-013`, `INV-SED-014`, `INV-SED-016`, and
  `GAP-SED-008` rows.
- ADR-0036 accepted decision:
  `docs/decisions/0036-hydrograph-resolved-sediment-transport-and-routing.md`.
- D12 final disposition and handoff:
  `docs/work-packages/20260705-mofefid-d12-melt-limb-hourly-shape-001/artifacts/final-disposition.md`
  and `artifacts/worker-handoff.md`.
- Lane D runtime shadow package:
  `docs/work-packages/20260705-mofefid-laned-activation-increment-001/`.
- Current runtime surfaces:
  - `crates/openwepp-hillslope-orchestrator/src/direct_runtime/03_executor.rs`,
  - `crates/openwepp-hillslope-orchestrator/src/direct_runtime/runoff.rs`,
  - `crates/openwepp-hillslope-orchestrator/src/direct_runtime/erosion.rs`,
  - `crates/openwepp-hillslope-orchestrator/src/direct_runtime/erosion_continuity.rs`,
  - `crates/openwepp-hillslope-orchestrator/src/direct_runtime/00_core_frames.rs`,
  - `crates/openwepp-runner/src/hillslope/laned_shadow.rs`, and
  - `crates/openwepp-runner/src/hillslope/04_direct_publication.rs`.

## Intended Write Set

Primary:

- `docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md`
- `docs/specifications/science-contracts/contracts/SC-SED-001.md`
- `docs/work-packages/20260705-mofefid-d13-routed-hydrograph-erosion-shape-001/`
- `docs/work-packages/README.md`
- `docs/planning/mofe-fidelity-campaign-strategy.md`
- `docs/ROADMAP.md`

Conditional, only if contract authority supports implementation:

- `crates/openwepp-hillslope-orchestrator/src/direct_runtime/03_executor.rs`
- `crates/openwepp-hillslope-orchestrator/src/direct_runtime/runoff.rs`
- `crates/openwepp-hillslope-orchestrator/src/direct_runtime/erosion.rs`
- `crates/openwepp-hillslope-orchestrator/src/direct_runtime/erosion_continuity.rs`
- `crates/openwepp-hillslope-orchestrator/src/direct_runtime/00_core_frames.rs`
- `crates/openwepp-runner/src/hillslope/laned_shadow.rs`
- `crates/openwepp-runner/src/hillslope/04_direct_publication.rs`
- Focused Lane D, erosion-hourly, HBP EVENT, or H2637 tests.
- `docs/decisions/0036-hydrograph-resolved-sediment-transport-and-routing.md`
  only if the accepted ADR needs a clarifying cross-reference; do not
  re-litigate the decision.

Protected:

- Production activation selector and default runtime policy.
- D10 numerical-method implementation and D-val acceptance.
- D11 friction operand source policy.
- D12 melt-limb source-shape rule.
- D14 profiling/optimization machinery.
- D15/D16 activation/default-promotion policy.
- Watershed/channel routing behavior and public schemas unless D13 records a
  contract-backed need before implementation.

## Phase Plan

1. **D13-S0 - Intake and baseline.** Read required authority, inspect current
   DC01/hourly erosion shape path, and record default/off baseline evidence.
2. **D13-S1 - Consumer-path audit.** Trace producer source, frame state,
   runner handoff, erosion hourly substrate, HBP EVENT assembly, and any old
   DC01 compatibility path that could still carry active-mode sediment timing.
3. **D13-S2 - Contract-first decision.** Amend or confirm `SC-OFEROUTE-001`
   and `SC-SED-001` before runtime edits. Name the active-mode routed
   hydrograph source, unit/normalization basis, closure identities, typed
   guards, and non-goals.
4. **D13-S3 - Contract-derived tests and pre-implementation gate.** Add tests
   that fail if active-mode erosion still reads DC01 weights, and fail-closed
   tests for malformed/missing routed hydrograph surfaces.
5. **D13-S4 - Implementation or legitimate HOLD.** Wire the activation-
   candidate routed hydrograph erosion shape if authorized, or close in
   `HOLD` only after a hold-legitimacy audit proves the boundary.
6. **D13-S5 - Evidence and closure.** Run required gates, update artifacts,
   complete dual review/disposition/verification, and set final package status.

## Exit Criteria

- Active-routed-water mode has a contract-backed erosion hourly-shape rule:
  Wave-1 consumes the routed hydrograph rather than DC01 weights.
- The real consumer path is proven: producer source, in-memory frame, runner
  handoff, erosion hourly substrate, HBP EVENT surfaces, and negative proof
  that the old DC01 shape is not carrying the active-mode closure claim.
- Paired water/sediment timing closures hold for the candidate path:
  `Σ V_h = runvol` or the routed active equivalent, and `Σ S_h =` exported
  sediment mass, with units and tolerances recorded.
- Invalid/missing routed hydrograph surfaces fail closed with typed errors.
- Default/off protected outputs remain byte-identical.
- No production/default activation, D10, D11, D12, D14, D15, or D16 work
  occurs.
- Accepted review findings are fixed and verified before completion.
- Line-count governance is recorded for every touched `.rs` file.

## Required Gates

Selection follows `docs/standards/local-ci-gate-selection.md` where relevant,
but D13 cannot close without recording:

- `git diff --check`
- Markdown lint for touched docs.
- Contract/profile/BEI checks for changed `SC-OFEROUTE-001`, `SC-SED-001`, or
  ADR cross-reference text.
- Unit-governance checks for any new or changed hourly hydrograph unit
  conversion or metadata.
- Focused tests for routed-hydrograph erosion shape selection, invalid-surface
  fail-closed guards, and paired hourly closures.
- H2637/Lane D evidence showing default/off byte identity and active-candidate
  routed-hydrograph consumer proof when executable.
- `cargo fmt --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo nextest run --workspace --profile full`
- `cargo deny check`
- Source-level anti-evasion guards if required-case bindings, fixtures, or
  authority-suite posture are touched.

If heavy gates are delegated, record subagent output and log paths in
`artifacts/gate-results.md`.

## Conservation / Output Acceptance

D13 is conservation-sensitive: it changes the hourly timing that erosion uses
for sediment detachment/deposition/export when routed water is active. Before
runtime edits, record operand lineage for each hourly surface, units,
normalization/denominator, area/volume basis, source authority, and diagnostic
vs authoritative status. Acceptance must include independent reconstruction
from produced rows and a real H2637 or equivalent closure audit. Exact
self-consistency alone is not sufficient.

## HOLD Legitimacy

D13 may close in `HOLD` only for a named boundary that cannot be closed inside
this write set: missing routed-hydrograph frame surface, contradictory
contract authority, unavailable active-candidate consumer, or a mechanism
proven to belong to D10/D14/D15. A hold must cite evidence, list the
in-envelope implementation route considered, and explain why it cannot close in
D13.

## Subagent Authorization

Subagent authorization: this package explicitly authorizes
spawning/delegating to `rust_code_reviewer`, `rust_qa_reviewer`, `explorer`,
and `comparator_suite_runner` subagents for read-only source/authority audit,
review, verification, fixture inspection, H2637/Lane D evidence, and heavy gate
execution. Expected outputs are compact findings, gate metrics, log paths, and
package-local review or verification artifact text. Write access is read-only
unless a later operator explicitly assigns a bounded write set.

Subagent requirement: `comparator_suite_runner` is REQUIRED for full workspace
nextest, H2637/fixture batches, and other heavy closure gates when available.
Do not run those heavy batches on the parent model unless the subagent is
unavailable; if unavailable, record command-level evidence and run locally
only when package governance permits substitution.

## Required Artifacts

- `artifacts/README.md`
- `artifacts/required-reading-map.md`
- `artifacts/hydrograph-shape-lineage.md`
- `artifacts/consumer-path-audit.md`
- `artifacts/conservation-output-lineage.md`
- `artifacts/pre-implementation-contract-gate.md`
- `artifacts/contract-implementation-evidence.md`
- `artifacts/contract-test-implementation-evidence.md`
- `artifacts/implementation-test-evidence.md`
- `artifacts/h2637-routed-hydrograph-evidence.md`
- `artifacts/hold-legitimacy-audit.md`
- `artifacts/gate-results.md`
- `artifacts/line-count-governance-checklist.md`
- `artifacts/review_agent_a.md`
- `artifacts/review_agent_b.md`
- `artifacts/review-disposition.md`
- `artifacts/verification_agent_a.md`
- `artifacts/verification_agent_b.md`
- `artifacts/worker-handoff.md`
- `artifacts/final-disposition.md`

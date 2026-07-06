# MOFEFID-D12 - Melt-Limb Hourly Source Shape

Status: **EXECUTED-COMPLETE** (2026-07-06). Campaign:
[MOFEFID](../../planning/mofe-fidelity-campaign-strategy.md) Lane D.
Contract focus: `SC-OFEROUTE-001#INV-OFEROUTE-012` melt-limb activation
precondition and the shared DC01/ADR-0036 hourly source-shape authority.

## Objective

Close the Lane D melt-limb hourly source-shape gap surfaced by the opt-in
runtime shadow: H2637 had runoff days whose lane-local `runvol/area` source
volume was routed through a uniform DC01 fallback because the current hourly
shape authority sees only two limbs:

- WB14 infiltration-excess profile (`wb14_hourly_excess_m[h]`), and
- subsurface saturation/exfiltration carry (`ui_SCrunf` lineage).

D12 must either add/prove the snowmelt/routed-liquid hourly source limb with
exact daily-sum closure to the lane-local routed supply, or formally
disposition any residual uniform-shape day class under canonical contract
authority. It must not activate Lane D production routing.

## Rationale

The runtime shadow proved the real frame surfaces can drive the Lane D cascade
without changing protected outputs, but it also found `days_uniform_shape`:
runoff days with positive source volume and no hourly shape from the two
GAP-006 D1 limbs. Uniform fallback is acceptable diagnostic plumbing; it is
not an activation source-authority rule. Before routing owns the surface-water
path, every routed source volume must have either:

- a source-authorized hourly distribution over the existing 24-hour time base,
  or
- a contract-backed non-routing disposition that explains why no hourly
  surface-source shape is required.

D12 is therefore a source-shape authority and runtime-surface package. It is
not a numerical-method package, erosion-hydrograph package, performance
package, activation flip, or default-promotion package.

## Scope

### Included

- Contract-first disposition for the melt/routed-liquid hourly source-shape
  limb in `SC-OFEROUTE-001` and any directly owning companion contract if the
  source surface belongs there.
- Source audit of existing snow/liquid/melt surfaces:
  - `snow_liquid.raw_melt_m`,
  - `snow_liquid.redistributed_melt_m`,
  - `snow_liquid.routed_melt_m`,
  - `DirectDayFrame` and direct-publication snow/liquid fields,
  - WB14 hourly rainfall/excess profiles,
  - `ui_SCrunf` / hourly saturation carry lineage,
  - `dc01_surface_runoff_hourly_weights`, and
  - `LanedShadowSummary.days_uniform_shape`.
- Add/prove an hourly source-shape limb for snowmelt/routed liquid when
  contract authority supports it. Daily runoff volume authority must remain
  unchanged; D12 owns distribution of existing source volume, not creation of
  new water.
- Add contract-derived tests for:
  - melt/routed-liquid runoff days no longer taking uniform fallback when a
    source-authorized melt shape exists,
  - exact daily-sum closure between hourly source depths and lane-local supply,
  - fail-closed handling for non-finite, negative, or contradictory limb
    surfaces,
  - H2637 `days_uniform_shape` elimination or a contract-backed residual class,
    and
  - default-off protected-output identity.
- Update Lane D planning, work-package catalog, and package artifacts with
  evidence, review, verification, and final disposition.

### Excluded

- No production/default Lane D activation.
- No `OPENWEPP_LANED_SHADOW` default enablement or runtime policy promotion.
- No D10 `GAP-OFEROUTE-005` shock-numerics correction or Case-4 acceptance.
- No D11 friction operand sourcing beyond preserving the rev-21 operand path.
- No D13 ADR-0036 erosion hourly-shape switch.
- No D14 runtime profiling/optimization.
- No D15 opt-in production flip or D16 default-promotion policy.
- No new snowmelt physics, surrogate melt timing, empirical stand-in, or
  tuned hourly distribution.
- No public HBP/pass/watershed schema changes unless a contract amendment
  proves a current source-shape field must be serialized now; the expected D12
  path is in-memory activation-candidate wiring.

## Dependencies

- `SC-OFEROUTE-001` rev 21.
- Runtime shadow package:
  `docs/work-packages/20260705-mofefid-laned-activation-increment-001/`.
- D11 dynamic friction closure:
  `docs/work-packages/20260706-mofefid-d11-gap007-dynamic-friction-closure-001/`.
- MOFEFID strategy §6.1 D12 row.
- ADR-0036 hourly substrate:
  `docs/decisions/0036-hydrograph-resolved-sediment-transport-and-routing.md`.
- Current source-shape code:
  - `crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/infiltration_reconciliation.rs`,
  - `crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/runoff_reconciliation.rs`,
  - `crates/openwepp-hillslope-orchestrator/src/direct_runtime/runoff.rs`,
  - `crates/openwepp-hillslope-orchestrator/src/direct_runtime/01_publication.rs`,
  - `crates/openwepp-hillslope-orchestrator/src/direct_runtime/03_executor.rs`,
  - `crates/openwepp-hillslope-orchestrator/src/direct_runtime/00_core_frames.rs`,
  - `crates/openwepp-hillslope-orchestrator/src/direct_runtime/storage.rs`,
  - `crates/openwepp-hillslope-orchestrator/src/direct_runtime/erosion.rs`,
  - `crates/openwepp-runner/src/hillslope/laned_shadow.rs`, and
  - direct-publication day-input helpers under
    `crates/openwepp-runner/src/hillslope/direct_publication/`.

## Intended Write Set

Primary:

- `docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md`
- `docs/work-packages/20260705-mofefid-d12-melt-limb-hourly-shape-001/`
- `docs/work-packages/README.md`
- `docs/planning/mofe-fidelity-campaign-strategy.md`

Conditional, only if D12 source authority supports implementation:

- `crates/openwepp-hillslope-orchestrator/src/direct_runtime/runoff.rs`
- `crates/openwepp-hillslope-orchestrator/src/direct_runtime/01_publication.rs`
- `crates/openwepp-hillslope-orchestrator/src/direct_runtime/03_executor.rs`
- `crates/openwepp-hillslope-orchestrator/src/direct_runtime/00_core_frames.rs`
- `crates/openwepp-hillslope-orchestrator/src/direct_runtime/storage.rs`
- `crates/openwepp-hillslope-orchestrator/src/direct_runtime/erosion.rs`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_00_support_helpers.rs`
- `crates/openwepp-runner/src/hillslope/00_runner_intake_and_lane_setup.rs`
- `crates/openwepp-runner/src/hillslope/05_runner_execution_and_outputs.rs`
- `crates/openwepp-runner/src/hillslope/laned_shadow.rs`
- `crates/openwepp-runner/src/hillslope/direct_publication/`
- Focused Lane D, H2637, runoff/shape, or direct-publication tests.
- Companion `SC-*` contracts only if the source-shape owner is not
  `SC-OFEROUTE-001`.

Execution boundary note: D12 touched `erosion.rs` only to keep the
already-shared DC01/ADR-0036 source-shape helper consuming the same D12 limb.
D12 does not claim the D13 erosion hourly-shape switch, Wave-1 acceptance, or
erosion production promotion.

Protected:

- Production activation selector and default runtime policy.
- D10 numerical-method implementation and D-val acceptance.
- D11 friction operand source policy except read-only proof that rev-21 remains
  consumed.
- D13 erosion hourly sediment substrate.
- D14 profiling/optimization machinery.
- D15/D16 activation/default-promotion policy.
- Public output schemas unless D12 records a contract-backed schema need before
  implementation.

## Phase Plan

1. **D12-S0 - Intake and baseline.** Read required authority, reproduce or
   inspect the H2637 `days_uniform_shape` evidence, and record the current
   source-shape lineage from source volume to `LanedShadowCollector`.
2. **D12-S1 - Source audit.** Audit existing snowmelt/routed-liquid surfaces
   and classify candidate hourly shapes as source-authorized, unavailable,
   contradictory, or out-of-scope. Record units, timing basis, lane/OFE basis,
   and rejected aliases.
3. **D12-S2 - Contract-first decision.** Amend or confirm canonical contract
   authority before code edits. Name the melt-limb source-shape rule, exact
   closure identity, typed guards, and residual-class disposition.
4. **D12-S3 - Contract-derived tests and pre-implementation gate.** Add tests
   that fail on the current uniform-fallback path when a melt shape is
   source-authorized; add negative tests for invalid/contradictory limbs.
5. **D12-S4 - Implementation or legitimate HOLD.** Wire the shared shape
   authority to the source-authorized melt/routed-liquid limb, or close in
   `HOLD` only after a hold-legitimacy audit proves the boundary.
6. **D12-S5 - Evidence and closure.** Run required gates, update artifacts,
   complete dual review/disposition/verification, and set final package status.

## Execution Result

D12 closed the melt-limb hourly source-shape blocker for the opt-in Lane D
shadow. `SC-OFEROUTE-001` rev 22 ratifies a producer-owned
`snow.hourly_routed_melt_m[h]` limb bound to
`SC-RUNOFFPART-001#INV-RUNOFFPART-022`, and the runtime passes that closed
vector through the DC01/ADR-0036/Lane D consumer path. H2637 evidence records
`days_uniform_shape_with_routed_melt=0`; the remaining `6` uniform-shape days
are classified as no-authorized-source-shape residuals, diagnostic-only.

No production/default Lane D activation, D10 shock-numerics, D11 friction
source, D13 erosion promotion, D14 profiling, or D15/D16 policy work is
claimed.

## Exit Criteria

- The `days_uniform_shape` activation blocker is closed for H2637, or each
  residual uniform day class has a contract-backed non-routing disposition.
- Every positive lane-local routed source volume consumed by the Lane D shadow
  has a source-authorized hourly shape or an explicit non-routing reason.
- The hourly source depths close exactly to the lane-local supply basis used by
  the shadow (`runvol/area` × area), with tolerances recorded in contract text.
- Invalid source limbs fail closed with typed errors; no `.max(0.0)` or
  uniform canonicalization masks corrupt process state.
- The shared source-shape authority remains one path for DC01 transfer
  publication, Lane D shadow forcing, and ADR-0036 consumers unless a contract
  amendment explicitly separates them.
- Default-off protected outputs remain byte-identical.
- No production/default activation, D10, D11, D13, D14, D15, or D16 work occurs.
- Accepted review findings are fixed and verified before completion.
- Line-count governance is recorded for every touched `.rs` file.

## Required Gates

Selection follows `docs/standards/local-ci-gate-selection.md` where relevant,
but D12 cannot close without recording:

- `git diff --check`
- Markdown lint for touched docs.
- Contract/profile/BEI checks for changed `SC-OFEROUTE-001` or companion
  contracts.
- Unit-governance checks for any new or changed source-shape unit conversion.
- Focused tests for source-shape closure, invalid-limb fail-closed guards, and
  no-uniform fallback on source-authorized melt/routed-liquid days.
- H2637/Lane D shadow fixture evidence for `days_uniform_shape` disposition.
- Default-off protected-output identity if runtime code changes.
- `cargo fmt --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo nextest run --workspace --profile full`
- `cargo deny check`
- Source-level anti-evasion guards if required-case bindings, fixtures, or
  authority-suite posture are touched.

If heavy gates are delegated, record subagent output and log paths in
`artifacts/gate-results.md`.

## Conservation / Output Acceptance

D12 changes the timing distribution of already-owned water, so it is
conservation-sensitive even if no public schema changes. Before runtime edits,
record an operand-lineage table with each source limb, units, denominator,
lane/OFE basis, daily-sum closure, source authority, and whether it is
authoritative or diagnostic. Acceptance must include independent reconstruction
from produced rows and a real H2637 closure/shape audit. Exact
self-consistency alone is not sufficient.

## HOLD Legitimacy

D12 may close in `HOLD` only for a source-authority boundary that cannot be
closed inside this write set: missing/contradictory melt timing authority,
unavailable hourly surface with no contract-backed reconstruction rule, or a
mechanism proven to belong to D10/D13/D14/D15. A hold must name the residual
day class, cite evidence, list the in-envelope correction route considered,
and explain why that route cannot close in D12.

## Subagent Authorization

User authorization: the 2026-07-06 operator request to scaffold D12 explicitly
authorizes Codex to dispatch subagents for this work package.

Subagent authorization: this package explicitly authorizes
spawning/delegating to `rust_code_reviewer`, `rust_qa_reviewer`, `explorer`,
and `comparator_suite_runner` subagents for read-only source/authority audit,
review, verification, fixture inspection, H2637/Lane D shadow evidence, and
heavy gate execution. Expected outputs are compact findings, gate metrics, log
paths, and package-local review or verification artifact text. Write access is
read-only unless a later operator explicitly assigns a bounded write set.

Subagent requirement: `comparator_suite_runner` is REQUIRED for full workspace
nextest, H2637/fixture batches, and other heavy closure gates when available.
Do not run those heavy batches on the parent model unless the subagent is
unavailable; if unavailable, record command-level evidence and run locally
only when package governance permits substitution.

## Required Artifacts

- `artifacts/README.md`
- `artifacts/required-reading-map.md`
- `artifacts/source-shape-lineage.md`
- `artifacts/source-audit-evidence.md`
- `artifacts/conservation-output-lineage.md`
- `artifacts/pre-implementation-contract-gate.md`
- `artifacts/contract-implementation-evidence.md`
- `artifacts/contract-test-implementation-evidence.md`
- `artifacts/implementation-test-evidence.md`
- `artifacts/h2637-melt-limb-evidence.md`
- `artifacts/consumer-path-evidence.md`
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

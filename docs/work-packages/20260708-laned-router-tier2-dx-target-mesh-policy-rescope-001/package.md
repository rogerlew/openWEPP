# LANED Router Tier-2 dx-Target Mesh-Policy Re-scope

Status: QUEUED

Package id uses the ADR-0037 handoff-suggested `20260708` successor name even
though it was scaffolded on 2026-07-07.

## Objective

Re-scope Tier-2 mesh-resolution adjudication after ADR-0037 abandoned the
hybrid implicit stepper. Decide whether the plain Lane D active router should
retain the current fixed `10 cells/OFE` mesh or move to a contract-authorized
target-`dx` per-OFE mesh policy. Land a production mesh-policy change only if
oracle, self-convergence, selected-cohort fidelity, timing, and protected
default/off evidence support it.

## Rationale

The abandoned hybrid arc exposed a separate mesh-policy issue: fixed
`cells/OFE` is not a spatial resolution. The current `10 cells/OFE` setting
prices H2637's short synthetic OFEs at roughly meter-scale resolution while
long real OFEs can run at much coarser `dx`. H2637 is now a synthetic stress
case under ADR-0037; future performance and promotion claims must be priced
against real selected-cohort members separately from H2637.

The old queued package
`20260707-laned-router-tier2-mesh-resolution-adjudication-001` is superseded
for execution because it is hybrid-era and 5-cells/OFE framed. This package is
the executable successor for the surviving plain-active mesh-policy question.

## Required Reading

Core:
- `AGENTS.md`
- `docs/codex_exec_plans.md`
- `docs/work-packages/AGENTS.md`
- `docs/work-packages/README.md`
- `docs/work-packages/20260708-laned-router-tier2-dx-target-mesh-policy-rescope-001/package.md`
- `docs/work-packages/20260708-laned-router-tier2-dx-target-mesh-policy-rescope-001/artifacts/required-reading-map.md`
- `docs/decisions/0037-abandon-hybrid-implicit-stepping.md`
- `docs/work-packages/20260707-laned-router-hybrid-abandonment-removal-001/artifacts/worker-handoff.md`
- `docs/work-packages/20260707-laned-router-tier2-mesh-resolution-adjudication-001/package.md`
- `docs/work-packages/20260707-laned-router-d16-rowcrop-canhgt-active-runtime-publication-001/artifacts/selected-cohort-materialization.json`
- `docs/work-packages/20260707-laned-router-hybrid-abandonment-removal-001/artifacts/plain-identity-materialization.json`
- `docs/backlog/20260706-laned-router-numerics-performance-tiers.md`

Conditional:
- `docs/specifications/science-contracts/AGENTS.md` before any contract,
  contract-derived test, or kernel semantic edit.
- `docs/specifications/science-contract-authoring-procedure.md` before
  changing canonical `SC-*` text.
- `docs/specifications/science-contracts/kernel-process-contract-profile.md`
  before changing canonical `SC-*` binding exposure, invariants, or tests.
- `docs/specifications/science-contracts/index.md` before registry edits.
- `crates/AGENTS.md` before Rust edits under `crates/`.
- `tests/AGENTS.md` before test edits under `tests/`.

On-demand:
- `docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md`
  when proposing, testing, or implementing a mesh-policy tolerance or
  production routing change.
- `crates/openwepp-hillslope-orchestrator/src/direct_runtime/laned_active.rs`
  for the active plain mesh builder and runtime counters.
- `crates/openwepp-hillslope-orchestrator/src/ofe_routing/` for Case-4
  oracle, mesh, and router tests.
- `crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/`
  for runner handoff and active-route configuration projection.
- Prior selected-cohort and active-suite artifacts as cited by
  `artifacts/fixture-cohort-plan.md`.

## Scope

Included:
- Mark the old Tier-2 package as superseded by this package.
- Inventory current active-plain mesh construction, fixed `10 cells/OFE`
  behavior, OFE lengths, and effective current `dx` for every selected member.
- Build or extend package-local fixture materialization for the selected
  cohort: `mn_corn_h4`, `n_idaho_forest_h1`, `wa_cascades_forest_h1`, plus
  H2637 as synthetic stress only.
- Run active-plain mesh ladders against:
  - Case-4 dimensionless oracle/convergence surfaces.
  - Real selected-cohort outputs and timings.
  - H2637 synthetic stress outputs and timings, reported separately.
- Propose `SC-OFEROUTE-001` mesh-policy tolerance text before any production
  mesh-policy implementation.
- Implement a production `dx`-target policy only if the package evidence
  supports it and contract authority is amended first.
- If accepted and implemented, prove protected default/off behavior,
  active-mode closure, and downstream routed-hydrograph consumer surfaces.

Excluded:
- Any hybrid implicit stepper revival, optimization, selector, contract, or
  tolerance work. ADR-0037 requires any revival to start from the archive
  branch under new contract authority.
- H2637-only performance or promotion claims.
- Fleet-topology surveys based on inventories that inherit legacy WEPP's OFE
  ceiling.
- Retuning routing physics or accepting mesh-induced output deltas without
  named contract tolerances.
- WEPPpy-side management generation changes.

## Fixture and Climate Plan

Primary real selected-cohort members:

| Member | Role | Climate | Prior routed-day shape | Source authority |
|--------|------|---------|------------------------|------------------|
| `mn_corn_h4` | real row-crop/agriculture | `p4.cli` | 2557 days seen, 209 routed | Disturbed native `ow-lanuse-1` route coefficients |
| `n_idaho_forest_h1` | real forest | `p1.cli` | 1461 days seen, 185 routed | Disturbed native `ow-lanuse-1` route coefficients |
| `wa_cascades_forest_h1` | real wet forest/runtime stress | `p1.cli` | 2192 days seen, 1381 routed | Disturbed native `ow-lanuse-1` route coefficients |

Synthetic stress member:

| Member | Role | Climate | Prior routed-day shape | Constraint |
|--------|------|---------|------------------------|------------|
| `h2637` | synthetic short-OFE stress case only | `p2637.cli` | 731 days seen, 610 routed | Do not use as fleet-general proof |

Repo-local fallback fixtures may be used only as diagnostic backup and must
not replace the selected-cohort decision unless the package records a
fixture-authority hold:
- `tests/fixtures/dff_ws1_native_forest/hjandrews_conifer_forest/`
- `tests/fixtures/laned_shadow_h2637/`
- `tests/fixtures/erosion_multi_ofe_p102/` if route-coefficient authority is
  explicitly supplied before use.

## Candidate Mesh Ladder

The package must compute actual OFE lengths before finalizing candidates. The
initial real-member ladder for pricing is:

- Baseline rung: current fixed `10 cells/OFE`. This is not the truth source;
  it is judged on the same tolerances as every candidate.
- Fine/reference rung: start at target `dx` about `2.5 m`, then prove
  adequacy with the reference-independence rule below.
- Candidate A: target `dx` about `5 m`.
- Candidate B: target `dx` about `10 m`.
- Candidate C: target `dx` about `20 m`.

Any implemented policy must use an explicit bounded form such as
`cells_per_ofe = clamp(ceil(ofe_length_m / target_dx_m), min_cells, max_cells)`,
with the bounds, units, and authority recorded before production code edits.

### Error Basis and Reference Rule

All mesh-policy errors are measured candidate-vs-adequate-fine-reference, not
candidate-vs-current-baseline. The current fixed `10 cells/OFE` baseline is
itself a judged rung, so the package may legitimately conclude that the
current setting is fidelity-inadequate on long real OFEs and that the ratified
policy costs more on some members.

The fine reference is adequate only when one further halving of its target
`dx` moves every judged surface by no more than one third of that surface's
predeclared tolerance. If any surface exceeds that threshold, refine and repeat
or hold with the unclosed reference-adequacy blocker. If a judged surface has
no named tolerance, T2R-C must define it before evidence is used for
acceptance.

### Case-4 Oracle Role

Case-4 must not be interpreted at absolute production candidate `dx`. Case-4's
ratified flume geometry is 24 m total with three 8 m reaches; candidate
`dx` values of 5-20 m produce 1-2 cells per reach or silently collapse to a
`min_cells` floor. Both outcomes are non-evidence for production candidates.

Case-4 evidence is dimensionless: cells per reach, convergence order, limiter
behavior, conservation, and shock-regime non-divergence. Its role is to
validate the mesh-convergence machinery. Candidate `dx` acceptance rests on
real selected-cohort self-convergence against the adequate fine reference.

### Predeclared Judged Surfaces

T2R-C must predeclare tolerances and bounds for:

- Per-day routed outlet mass.
- Hourly-weight hydrograph shape feeding the D13 erosion consumer.
- Annual pass-sediment sums.
- Conservation closure residuals. These are expected to remain exact at any
  mesh under rev 25/26 bookkeeping, so they are required guards but not
  fidelity discriminators.
- `routed_end_window_storage_m3`.
- `routed_tail_fold_m3`.
- `lane_days_erosion_source_shape_degenerate`.
- `days_uniform_shape`.

### Clamp, Shadow, and Time-Step Rules

`min_cells` is a scheme-regime constraint, not a convenience floor. The
package must justify it against the TVD-MacCormack stencil/limiter regime,
include a short-OFE-at-the-floor rung, and must not inherit the internal
`KinematicWaveMesh::uniform` one-cell clamp as policy behavior.

T2R-C must decide whether the shadow lane follows the active production mesh
policy or remains fixed, and must record the rationale. If production mesh
policy changes, the conditional write set includes the separate shadow constant
in `crates/openwepp-runner/src/hillslope/laned_shadow.rs`.

The production time-step caps are fixed across this package's mesh ladders:
`LANED_ACTIVE_SAMPLE_DT_S = 900` and `LANED_ACTIVE_MAX_DT_S = 300`. Do not
co-tune `dt` caps here. Cost expectations must be recorded as measured; the
old backlog's `n^2` estimate applies only where CFL-bound stepping dominates,
and savings are sub-quadratic where the 300 s cap binds.

### Ratification Envelope

This package adjudicates routing numerics under the current uniform
per-lane/OFE parameter projection, including the current mean-gradient routing
input. It is not a within-OFE terrain-fidelity ratification. Any future
within-OFE parameter-profile routing change reopens the mesh question. The
ratified envelope must name the evidenced OFE-length range and clamp posture
outside that range.

## Dependencies

- ADR-0037 accepted and executed on main.
- `SC-OFEROUTE-001` remains the surviving Lane D plain active contract
  authority.
- Selected-cohort materialization from the row-crop `canhgt` publication
  package and the ADR-0037 plain-identity package.
- Release-binary timing provenance guidance from QA-M3.

## Intended Write Set

Expected docs/artifacts:
- `docs/work-packages/20260708-laned-router-tier2-dx-target-mesh-policy-rescope-001/**`
- `docs/work-packages/README.md`
- `docs/ROADMAP.md`
- Supersession note in
  `docs/work-packages/20260707-laned-router-tier2-mesh-resolution-adjudication-001/package.md`
  and its active prompt/gate placeholder.

Conditional implementation write set, only if accepted by contract-first
evidence:
- `docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md`
- `docs/specifications/science-contracts/index.md` if lifecycle metadata
  changes.
- Active Lane D routing code under
  `crates/openwepp-hillslope-orchestrator/src/direct_runtime/` and
  `crates/openwepp-hillslope-orchestrator/src/ofe_routing/`.
- Runner configuration projection under
  `crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/`.
- Shadow mesh policy under
  `crates/openwepp-runner/src/hillslope/laned_shadow.rs` if T2R-C decides the
  shadow lane follows the active mesh policy.
- Focused tests under `tests/` and crate-local test modules.

Do not edit hybrid code or `SC-OFEROUTE-002`; they are removed from main.

## Phase Plan

1. **T2R-A Scaffold and supersession audit.** Confirm this package supersedes
   the old hybrid-era Tier-2 package. Update package-local artifacts with the
   authority map and fixture/cohort plan.
2. **T2R-B Baseline inventory.** Record the current active plain mesh builder,
   fixed-cell policy, shadow mesh constant, OFE lengths, effective `dx`,
   current `dt` caps, release binary provenance, and baseline output/timing
   surfaces for the real cohort and H2637 stress case. Treat baseline as a
   judged rung, not as reference truth.
3. **T2R-C Contract-first tolerance proposal.** If execution will compare or
   implement a non-bit-identical mesh policy, amend/propose `SC-OFEROUTE-001`
   tolerance and guard text before tests or production edits. This phase must
   name the judged surfaces, candidate-vs-reference error basis, reference
   adequacy rule, `min_cells` scheme-regime rationale, short-OFE floor rung,
   fixed-`dt` constraint, shadow-lane posture, uniform-parameter premise, and
   timing-budget posture. If authority is missing or contradictory, hold here
   with a legitimacy audit.
4. **T2R-D Oracle/self-convergence ladder.** Run Case-4 as a dimensionless
   cells-per-reach convergence ladder, not at absolute candidate `dx`. Run
   active-router self-convergence ladders across baseline, adequate
   fine/reference, and candidate `dx` policies. Record peak, timing,
   hydrograph-shape, conservation, counted residual classes, and
   non-divergence evidence.
5. **T2R-E Selected-cohort and H2637 pricing.** Run the real selected cohort
   and H2637 stress case separately. Record timing, routed-day counts, solver
   counters, closure surfaces, HBP/pass parquet candidate-vs-reference deltas,
   baseline-vs-reference deltas, and routed-hydrograph consumer deltas. Hold
   `dt` caps fixed across the ladder.
6. **T2R-F Adjudication.** Decide accept/reject/hold for a production
   `dx`-target mesh policy. Do not land a policy change unless named
   tolerances pass against the adequate fine reference and real-cohort
   evidence supports it. Timing is priced unless T2R-C declares a timing budget
   as an acceptance gate.
7. **T2R-G Implementation if accepted.** Implement only the ratified plain
   active mesh policy. Prove default/off byte identity if default/off surfaces
   are touched, active closure, DC01/no-double-feed invariants, and routed
   hydrograph consumer behavior.
8. **T2R-H Review, verification, and disposition.** Complete dual review,
   finding disposition, dual verification, line-count governance, gate results,
   worker handoff, and final disposition.

## Subagent Authorization

Subagent authorization: this package explicitly authorizes
spawning/delegation to `comparator_suite_runner`, `rust_code_reviewer`,
`rust_qa_reviewer`, `explorer`, and bounded `worker` subagents for fixture
materialization, release timing runs, mesh comparator ladders, review,
verification, focused codebase questions, and bounded implementation help if
T2R-G is reached. Expected outputs are package-local review, verification,
timing, comparator, and fixture-inventory artifacts. Write access is read-only
for review/verification/comparator/explorer roles; worker write access is
bounded to package artifacts unless explicitly assigned a disjoint
implementation write set by the executing parent.

Subagent requirement: REQUIRED for heavy comparator/timing/full closure gates
unless unavailable; record any tool-policy block before running locally.

## Required Artifacts

- `artifacts/required-reading-map.md`
- `artifacts/fixture-cohort-plan.md`
- `artifacts/mesh-baseline-inventory.md`
- `artifacts/oracle-ladder.md`
- `artifacts/selected-cohort-mesh-timing.md`
- `artifacts/mesh-fidelity-adjudication.md`
- `artifacts/implementation.md` or `artifacts/hold-legitimacy-audit.md`
- `artifacts/gate-results.md`
- `artifacts/line-count-governance.md`
- `artifacts/review-agent-a.md`
- `artifacts/review-agent-b.md`
- `artifacts/review-claude-preexecution.md`
- `artifacts/review-disposition-preexecution.md`
- `artifacts/verification-agent-a.md`
- `artifacts/verification-agent-b.md`
- `artifacts/disposition.md`
- `artifacts/final-disposition.md`
- `artifacts/worker-handoff.md`

## Gates

- `git diff --check`
- Markdown/doc lint for touched docs.
- Required-reading budget recorded in
  `artifacts/required-reading-map.md`.
- Contract/profile/BEI checks for touched `SC-*` contracts.
- Case-4 dimensionless cells-per-reach convergence ladder. Case-4-at-absolute
  candidate-`dx` is explicitly non-acceptance evidence.
- Fine-reference adequacy: one further `dx` halving moves every judged surface
  by no more than one third of its tolerance.
- Candidate-vs-reference and baseline-vs-reference error tables for every
  predeclared judged surface.
- `min_cells` scheme-regime rationale plus short-OFE floor rung.
- Fixed `dt` caps across all mesh ladders; no time-step co-tuning.
- Shadow-lane policy decision and evidence if production active mesh changes.
- Selected-cohort active plain timing and fidelity surfaces for
  `mn_corn_h4`, `n_idaho_forest_h1`, and `wa_cascades_forest_h1`.
- H2637 active plain timing and fidelity as synthetic stress evidence only.
- Exact runner-binary provenance for timing/comparator evidence.
- Protected default/off byte identity if default/off surfaces are touched.
- Active-mode closure evidence for routed days under candidate policy.
- DC01-disable / no-double-feed proof if production active routing changes.
- Routed-hydrograph-to-erosion consumer proof if production active routing
  changes.
- Focused Lane D / `ofe_routing` tests.
- `cargo fmt --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo nextest run --workspace --profile full`
- `cargo deny check`
- Source-level anti-evasion guards if required-case bindings, cohort fixtures,
  or external-authority suite posture are touched:
  - `bash tools/release/check_authority_suite_antievasion.sh`
  - `cargo nextest run --test auth11_required_suite_obligation_guards_contract`
- `.rs` line-count governance.

## Exit Criteria

`EXECUTED-COMPLETE` requires:
- Old Tier-2 supersession is documented.
- Real selected cohort and H2637 stress case are materialized or an explicit
  fixture-authority hold is recorded.
- A named mesh policy is either rejected with candidate-vs-reference evidence
  or accepted with contract-first authority. The current `10 cells/OFE`
  baseline is judged on the same basis.
- If accepted, production code implements only the ratified plain active policy
  and all gates pass.
- Dual reviews and dual verifications are complete, all findings are
  dispositioned, and final disposition names the next work item.

`EXECUTED-HOLD-*` is required when:
- Selected-cohort fixture authority or materialization cannot be established.
- `SC-OFEROUTE-001` lacks authority for the necessary tolerance or guard.
- Fine-reference adequacy cannot be established.
- Oracle/self-convergence or real-cohort evidence fails a required tolerance.
- Runtime cost becomes unacceptable under the only fidelity-backed policy.
- A required active production consumer proof cannot be produced in-envelope.

No package phase may be marked complete by deferring its own required evidence
to a later package. If evidence is missing and cannot be produced in-envelope,
record a hold legitimacy audit naming the blocker, evidence, considered
in-envelope route, and first actionable follow-on.

# LANED Router Active Timestep Policy Adjudication

Status: `EXECUTED-COMPLETE`

## Objective

Adjudicate whether active-router target-`dx` mesh adequacy must be treated as
a coupled space-time convergence policy before any renewed `dx5` production
mesh-policy promotion.

## Rationale

`20260708-laned-router-mn-corn-h4-day792-raw-hydrograph-numerics-001` ruled
out source, upstream, clamp, limiter, boundary-sign, and outlet-bin
attribution defects for the `mn_corn_h4` day-792 lane-1 miss. The remaining
blocker is that the fine reference pair mixes timestep regimes:

- `dx1p25`: 65 cells, 228 steps, max Courant `0.85874995859419834`.
- `dx0p625`: 130 cells, 330 steps, max Courant `0.9`.

The existing `SC-OFEROUTE-001` rev-42 mesh-policy evidence row fixed
`LANED_ACTIVE_MAX_DT_S = 300` for T2R. This package tests whether that fixed
cap is a valid spatial adequacy basis or whether the active target-`dx` policy
requires a contract-first coupled timestep/mesh rule.

## Required Reading

Core:

- `AGENTS.md`
- `docs/work-packages/AGENTS.md`
- `docs/specifications/science-contracts/AGENTS.md`
- `docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md`
- `docs/standards/AGENTS.md`
- `docs/standards/prompt-wording-guidance.md`
- `crates/AGENTS.md`
- `tests/AGENTS.md`
- `docs/work-packages/20260708-laned-router-mn-corn-h4-day792-raw-hydrograph-numerics-001/package.md`
- `docs/work-packages/20260708-laned-router-mn-corn-h4-day792-raw-hydrograph-numerics-001/artifacts/final-disposition.md`
- `docs/work-packages/20260708-laned-router-mn-corn-h4-day792-raw-hydrograph-numerics-001/artifacts/mechanism-attribution.md`
- `docs/work-packages/20260708-laned-router-mn-corn-h4-day792-raw-hydrograph-numerics-001/artifacts/worker-handoff.md`

Conditional:

- `docs/specifications/science-contract-authoring-procedure.md` if
  `SC-OFEROUTE-001` is amended.
- `docs/specifications/science-contracts/kernel-process-contract-profile.md`
  if `SC-OFEROUTE-001` is amended.
- Prior Tier-2 mesh-policy packages when comparing candidate/reference
  adjudication surfaces.

## Scope

Included:

- Scaffold package-local evidence, prompt, review, verification, gates,
  disposition, and handoff artifacts.
- Add a diagnostic-only active max-`dt` selector if no equivalent surface
  exists.
- The diagnostic selector must not change default/off behavior and must not
  be usable as a silent production knob.
- Rerun `mn_corn_h4` day 792 lane 1 with selected day/lane step trace at
  controlled `max_dt` values.
- Compare `dx1p25` and `dx0p625` under the same or systematically halved
  timestep controls.
- Decide whether the day-792 adequacy miss is explained by timestep policy,
  remains a deeper solver mechanism, or requires a contract-first mesh/time
  adequacy amendment.

Excluded:

- `dx5` production mesh-policy promotion.
- Routed-shape tolerance widening.
- Hybrid solver revival.
- Production active default changes.
- Shadow mesh-policy changes.
- Source/coefficient tuning.

## Correction Authority Envelope

Allowed:

- Diagnostic env parsing and fail-closed validation for a bounded active
  max-`dt` selector.
- Passing a selected diagnostic max-`dt` into the already active routed path.
- Package-local run/analyzer tooling.
- Contract text amendment only if evidence proves current mesh-policy
  authority is incomplete or misleading.

Not allowed:

- Changing `LANED_ACTIVE_MAX_DT_S` production default.
- Letting `OPENWEPP_LANED_ACTIVE_MAX_DT_S` run without active trace evidence.
- Treating timestep-refined diagnostic evidence as a production default flip.
- Changing physics, source terms, friction coefficients, or erosion coupling.

## Intended Write Set

Expected:

- `docs/work-packages/20260708-laned-router-active-router-timestep-policy-adjudication-001/**`
- `docs/work-packages/README.md`
- `docs/ROADMAP.md`

Conditional diagnostic Rust write set:

- `crates/openwepp-hillslope-orchestrator/src/direct_runtime/laned_active.rs`
- `crates/openwepp-hillslope-orchestrator/src/direct_runtime/03_executor.rs`
- `crates/openwepp-hillslope-orchestrator/src/direct_runtime.rs`
- `crates/openwepp-hillslope-orchestrator/src/lib.rs`
- `crates/openwepp-runner/src/hillslope/laned_active.rs`
- `crates/openwepp-runner/src/hillslope/05_runner_execution_and_outputs.rs`
- `crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/00_builders_and_authority.rs`

Conditional contract write set:

- `docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md`
- `docs/specifications/science-contracts/index.md` only if registry metadata
  changes.

## Phase Plan

1. **TSA-A Scaffold and authority map.** Create package files, prompt, ignored
   raw run root, required-reading map, and catalog pointers.
2. **TSA-B Diagnostic max-`dt` surface.** Add a bounded, trace-required
   diagnostic max-`dt` selector if absent; prove default/off behavior remains
   unchanged.
3. **TSA-C Controlled reruns.** Run `mn_corn_h4` day 792 lane 1 at
   `dx1p25` and `dx0p625` with `max_dt` values `300`, `150`, and `75`
   seconds, with step trace enabled.
4. **TSA-D Adjudication.** Compare same-`dx` timestep refinement and
   same-`max_dt` spatial refinement. Decide whether the old miss is a
   timestep-policy artifact, deeper solver issue, or contract-policy gap.
5. **TSA-E Contract/disposition.** If evidence requires contract text, amend
   `SC-OFEROUTE-001` before final disposition. Otherwise record a no-amendment
   adjudication.
6. **TSA-F Review, verification, and gates.** Complete dual review,
   disposition, dual verification, line-count governance, gate results, final
   disposition, and handoff.

## Subagent Authorization

Subagent authorization: this package explicitly authorizes subagent
spawning/delegation to review, verification, comparator/timing, and bounded
worker subagents for timestep-policy evidence review, gate verification, and
run/analyzer checks. Expected outputs are package-local review,
verification, timing/comparator, and disposition artifacts. Write access is
read-only for review/verification/comparator roles; worker write access is
bounded to package artifacts unless explicitly assigned a disjoint diagnostic
implementation write set.

## Required Artifacts

- `artifacts/README.md`
- `artifacts/required-reading-map.md`
- `artifacts/implementation.md`
- `artifacts/timestep-policy-summary.md`
- `artifacts/timestep-policy-summary.json`
- `artifacts/timestep-policy-adjudication.md`
- `artifacts/timestep-policy-adjudication.json`
- `artifacts/contract-disposition.md`
- `artifacts/gate-results.md`
- `artifacts/line-count-governance.md`
- `artifacts/review-agent-a.md`
- `artifacts/review-agent-b.md`
- `artifacts/disposition.md`
- `artifacts/verification-agent-a.md`
- `artifacts/verification-agent-b.md`
- `artifacts/final-disposition.md`
- `artifacts/worker-handoff.md`

## Gates

- `git diff --check`
- Markdown/doc lint for touched docs.
- Exact release-binary provenance for reruns.
- Controlled `mn_corn_h4` `dx1p25`/`dx0p625` max-`dt` ladder.
- Analyzer replay from package-local tooling after rerun.
- Focused active trace / selector tests.
- Focused Lane D / `ofe_routing` tests for Rust changes.
- Contract/profile/BEI checks if `SC-OFEROUTE-001` changes.
- `cargo fmt --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo nextest run --workspace --profile full`
- `cargo deny check`
- Source-level anti-evasion guards if required-case bindings, cohort fixtures,
  or external-authority suite posture are touched.

## Exit Criteria

`EXECUTED-COMPLETE` requires:

- The controlled timestep ladder is run with exact release-binary provenance.
- The package decides, with evidence, whether the day-792 miss is a
  timestep-policy artifact, deeper solver issue, or contract-policy gap.
- Any contract implication is dispositioned contract-first.
- No target-`dx` production promotion or tolerance widening lands.
- Reviews, disposition, verification, gates, and handoff are complete.

`EXECUTED-HOLD-*` is required when:

- Diagnostic evidence cannot be produced in-envelope.
- A valid correction or contract amendment is needed but outside this package's
  authority.
- Reviews or verification leave a blocker open.

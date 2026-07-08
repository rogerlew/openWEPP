# DX5 Production Mesh Policy Ratification

Status: `EXECUTED-COMPLETE-DX5-PRODUCTION-MESH-POLICY`
Evidence mode: Ran.
Date: 2026-07-08

## Objective

Ratify and, if all current-scope gates pass, implement `dx5` as the active
Lane D production mesh-policy default under `SC-OFEROUTE-001`.

The predecessor annual-sediment package amended `SC-OFEROUTE-001` rev 44 so
annual pass-sediment mesh-policy evidence uses the material-year plus
annual-vector rule. This package rebuilds the selected-cohort promotion matrix
on that authority, then makes the production default flip only if the matrix
and runtime proof gates close.

## Scope

In scope:

- Scaffold package-local artifacts and kickoff prompt.
- Rebuild the selected-cohort `dx5` promotion matrix from the coupled
  space-time ladder plus rev-44 annual sediment replay.
- Amend `SC-OFEROUTE-001` before code if production promotion is supported.
- Implement the active production default as `target_dx_m = 5.0` with
  `min_cells = 10`, `max_cells = 4096`, and unchanged 300 s production
  max-substep cap.
- Keep `OPENWEPP_LANED_ACTIVE_MESH_TARGET_DX_M` as an explicit diagnostic
  override, not as the production mechanism.
- Decide shadow mesh explicitly.
- Prove default/off byte identity remains protected and active production
  evidence now uses `dx5` without the diagnostic env selector.
- Record review, verification, gates, disposition, and handoff.

Out of scope:

- No routed-shape tolerance widening.
- No annual sediment threshold widening.
- No sediment process-physics change.
- No active max-`dt` production default change.
- No shadow mesh default change unless explicitly contract-amended here.
- No revival of abandoned hybrid stepping.

## Required Reading

Core:

- `AGENTS.md`
- `docs/work-packages/AGENTS.md`
- `docs/specifications/science-contracts/AGENTS.md`
- `docs/standards/AGENTS.md`
- `docs/standards/prompt-wording-guidance.md`
- `docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md`
- `docs/work-packages/20260708-laned-router-annual-sediment-adequacy-metric-authority-001/artifacts/worker-handoff.md`
- `docs/work-packages/20260708-laned-router-annual-sediment-adequacy-metric-authority-001/artifacts/annual-sediment-metric-replay.json`
- `docs/work-packages/20260708-laned-router-tier2-dx5-coupled-spacetime-ratification-001/artifacts/coupled-spacetime-summary.json`
- `docs/work-packages/20260708-laned-router-tier2-dx5-coupled-spacetime-ratification-001/artifacts/mesh-policy-ratification.md`
- `docs/work-packages/20260708-laned-router-active-router-timestep-policy-adjudication-001/artifacts/final-disposition.md`
- `docs/work-packages/20260708-laned-router-wa-sediment-reference-adequacy-attribution-001/artifacts/classification.md`

Conditional:

- `crates/AGENTS.md` and `tests/AGENTS.md` before Rust/test edits.
- `SC-SED-001` routed-hydrograph consumer surfaces if consumer proof needs
  contract citations.
- Prior D15A artifacts if active owner/DC01-disable/consumer proof needs
  source crosswalk.

## Phase Plan

### Phase A - Scaffold and Authority Map

- Create package-local `package.md`, `artifacts/`, `prompts/active/`,
  `prompts/archived/`, `.gitignore`, and catalog/roadmap pointers.
- Record required-reading and source-evidence provenance.

### Phase B - Rev-44 Promotion Matrix

- Run package-local analyzer over:
  - coupled space-time selected-cohort summary;
  - rev-44 annual sediment replay.
- Confirm:
  - `dx5` candidate-vs-`dx2p5` passes;
  - adequate fine-reference comparisons pass on the coupled space-time basis;
  - timestep controls pass;
  - annual pass-sediment has zero rev-44 blockers.
- If any surface remains open, stop at `EXECUTED-HOLD-*`.

### Phase C - Contract and Implementation

- Amend `SC-OFEROUTE-001` to promote active production default to
  `target_dx_m = 5.0` if Phase B passes.
- Preserve the diagnostic target-`dx` selector as an explicit override and
  keep selector guards fail-closed.
- Update active mesh-policy code and contract-derived tests.
- Decide shadow mesh explicitly; default is no change unless evidence demands
  otherwise.

### Phase D - Runtime Proof

- Build the release runner binary with exact provenance.
- Run selected-cohort active default/no-target-selector evidence and prove the
  manifest reports `target_dx_m = 5.0` from the production default.
- Run an explicit `OPENWEPP_LANED_ACTIVE_MESH_TARGET_DX_M=5.0` control for at
  least the selected real cohort and prove default/no-env outputs match.
- Prove protected subsystem-off/default output byte identity remains protected.
- Record active closure, DC01/no-double-feed, and routed-hydrograph erosion
  consumer proof.

### Phase E - Gates, Review, Disposition

- Run required gates.
- Complete dual review and dual verification.
- Disposition findings.
- Record final disposition and worker handoff.

## Subagent Authorization

Subagent authorization: this package explicitly authorizes subagent
spawning/delegation to review, verification, and comparator/closure-gate
subagents for package-local review, verification, selected-cohort/default
comparator work, and heavy Rust closure gates. Expected outputs are
package-local `artifacts/review-*.md`, `artifacts/verification-*.md`, and
compact gate metrics/log paths. Write access is bounded to this package's
artifact directory unless a subagent is explicitly assigned implementation
fixes.

Subagent requirement: REQUIRED for heavy comparator/timing/full closure gates
when available. Spawn `comparator_suite_runner` for selected-cohort/default
comparator work or full closure/comparator runs; do not run those heavy gates
on the parent model unless the subagent is unavailable, in which case record
command-level evidence.

## Required Artifacts

- `artifacts/README.md`
- `artifacts/required-reading-map.md`
- `artifacts/rev44-promotion-matrix.json`
- `artifacts/rev44-promotion-matrix.md`
- `artifacts/contract-disposition.md`
- `artifacts/implementation.md`
- `artifacts/default-dx5-evidence.json`
- `artifacts/default-dx5-evidence.md`
- `artifacts/consumer-path-proof.md`
- `artifacts/gate-results.md`
- `artifacts/review-*.md`
- `artifacts/verification-*.md`
- `artifacts/disposition.md`
- `artifacts/final-disposition.md`
- `artifacts/worker-handoff.md`

## Gates

Required before completion:

- Package analyzer compile and replay.
- Exact release-binary provenance for runtime evidence.
- Selected-cohort active default/no-env `dx5` evidence.
- Default/no-env versus explicit target-`dx=5.0` output identity evidence.
- Protected default/off byte identity.
- Active closure and `INV-OFEROUTE-012` evidence.
- DC01-disable / no-double-feed proof.
- Routed-hydrograph-to-erosion consumer proof.
- Focused Lane D / `ofe_routing` / mesh-policy tests.
- `git diff --check`.
- Markdown/doc lint for touched docs.
- Contract/profile/BEI checks required by touched contracts.
- `cargo fmt --check`.
- `cargo clippy --workspace --all-targets -- -D warnings`.
- `cargo nextest run --workspace --profile full`.
- `cargo deny check`.

Conditional:

- Authority anti-evasion guard only if required-case bindings, cohort fixture
  posture, or external-authority suite posture are touched.

## Exit Criteria

`EXECUTED-COMPLETE-DX5-PRODUCTION-MESH-POLICY`:

- `SC-OFEROUTE-001` authorizes active production `dx5`.
- Code default/no-env active mesh resolves to `target_dx_m = 5.0`.
- Diagnostic target-`dx` override remains fail-closed and explicit.
- Shadow mesh decision is recorded.
- Protected default/off behavior remains byte-identical.
- Selected-cohort active runtime evidence proves no-env production uses `dx5`
  and default/no-env outputs match explicit target-`dx=5.0`.
- Active closure, no-double-feed, and routed erosion consumer proof are
  recorded.
- Required gates, review, verification, and finding disposition are complete.

`EXECUTED-HOLD-*`:

- Any required promotion matrix or runtime proof gate fails.
- Hold audit names exact blocker, evidence, considered in-envelope correction
  route, and first actionable follow-on.

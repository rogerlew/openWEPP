# SNOWDENSITY-10.3.5c - Phase Partition Snow-Depth Impact Adjudication

Status: complete (executed by Codex, 2026-06-27).

Package type: diagnostic defect-closure style adjudication package; coupled WAT
evidence only; no production default activation.

Closure target: `COMPLETE-10-3-5C-PHASE-PARTITION-SNOW-DEPTH-ADJUDICATED`.

## Objective

Run the SNOWDENSITY-10.3.5b opt-in `harder_pomeroy_hourly` precipitation-phase
partition through the real direct-production WAT path for the maritime and
mixed/deciduous snow-depth surfaces from SNOWDENSITY-10.3.4, then decide whether
the phase selector materially improves paired snow-depth failures. This package
answers impact, not activation.

## Primary Authority

- `docs/planning/snow-frost-fidelity-strategy.md` section 10.3: 10.3.4 ranked
  near-0 degC snow/rain partition as the lead defect-eligible mechanism; 10.3.5a
  and 10.3.5b created and validated the opt-in candidate.
- `docs/work-packages/20260627-snowdensity-10-3-5b-hourly-partition-jennings-validation-001/`:
  the opt-in selector exists and default rollback is already proven.
- `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
  `INV-SNOWFREEZE-047`, `INV-SNOWFREEZE-048`, `INV-SNOWFREEZE-050`, and
  `INV-SNOWFREEZE-065`.
- ADR-0017: legacy/comparator agreement is flag evidence; observed snow-depth
  authority controls defect attribution.

## Correction Authority Envelope

In scope:

- A bounded robustness correction for the already-authorized 10.3.5b
  `harder_pomeroy_hourly` hydrometeor-temperature solver if coupled WAT execution
  exposes a valid-input non-convergence blocker. Such a correction must preserve
  the same Harder-Pomeroy equation, be opt-in-only, and add focused tests.
- A package-local diagnostic tool that runs direct-production WAT outputs for the
  10.3.4 maritime surfaces under:
  - default `legacy_rst`;
  - opt-in `OPENWEPP_SNOWDENSITY1035_PHASE_MODEL=harder_pomeroy_hourly`.
- Paired snow-depth residuals for Sleepers South, Sleepers W9, Harvard hardwood,
  and Harvard open.
- Observation-blocked reporting for HJ Andrews and Hubbard Brook without defect
  labels.
- WAT-level default-vs-opt-in deltas for snow depth and snow water.
- Artifacts, package guards, focused tests, reviews, verification, and closure
  disposition.

Protected boundaries:

- No default activation or parser/runfile/user CLI selector.
- No fixture input edits.
- No public output schema changes.
- No snow density, melt, canopy, radiation, albedo, frost, compatibility runtime,
  or coefficient changes.
- No site-specific calibration.
- No defect verdict for observation-blocked surfaces.
- No claim that Jennings observed-phase validation alone proves snow-depth
  remediation.

## Intended Write Set

- `tools/snowfreeze_observed/phase_partition_snowdepth_adjudication.py`
- `tests/integration/snowdensity10_3_5c_phase_partition_snowdepth_impact.rs`
- `Cargo.toml`
- `crates/openwepp-meteorology/src/phase.rs` only if needed for a bounded
  valid-input hydrometeor solver robustness blocker.
- `docs/planning/snow-frost-fidelity-strategy.md`
- `docs/work-packages/README.md`
- Package-local files under
  `docs/work-packages/20260627-snowdensity-10-3-5c-phase-partition-snow-depth-impact-001/**`

Any production Rust edit outside test registration is out of scope except the
bounded `openwepp-meteorology` solver robustness correction above.

## Required Evidence Artifacts

- `artifacts/required-reading-map.md`
- `artifacts/scaffold-evidence.md`
- `artifacts/phase-partition-snowdepth-impact.json`
- `artifacts/phase-partition-snowdepth-impact.md`
- `artifacts/no-scope-creep-scan.md`
- `artifacts/test-evidence.md`
- `artifacts/gate-results.md`
- `artifacts/review_agent_a.md`
- `artifacts/review_agent_b.md`
- `artifacts/review-disposition.md`
- `artifacts/verification_agent_a.md`
- `artifacts/verification_agent_b.md`
- `artifacts/line-count-governance-checklist.md`
- `artifacts/owned-file-manifest.md`
- `artifacts/worker-handoff.md`
- `artifacts/disposition.md`

Evidence must label `Static:` versus `Ran:`. Placeholder artifacts must be
updated during execution.

## Acceptance Gates

- The package tool uses the real direct-production `openwepp-cli-hill` WAT path,
  not `openwepp-snowbench coe-melt`.
- Default runs leave `OPENWEPP_SNOWDENSITY1035_PHASE_MODEL` absent; opt-in runs
  set it to `harder_pomeroy_hourly`.
- The report includes paired residual summaries and default-vs-opt-in deltas for
  all paired surfaces.
- If a solver robustness correction is needed, focused tests prove the
  non-convergent valid-input class solves without changing saturated identity or
  default `legacy_rst` behavior.
- The report names observation-blocked surfaces and prevents them from carrying
  defect verdicts.
- The report disposition is one of:
  - `PHASE-PARTITION-PROMOTION-CANDIDATE`;
  - `PHASE-PARTITION-PARTIAL-IMPROVEMENT`;
  - `PHASE-PARTITION-NEUTRAL-OR-WORSE`;
  - `PHASE-PARTITION-HOLD`.
- No production physics, schema, fixture, default activation, or parser/CLI
  selector changes occur.
- Focused gates:
  - `.venv/bin/python tools/snowfreeze_observed/phase_partition_snowdepth_adjudication.py`
  - `cargo test --test snowdensity10_3_5c_phase_partition_snowdepth_impact`
- Final gates:
  - `cargo fmt --check`
  - `cargo clippy --workspace --all-targets -- -D warnings`
  - `cargo test --workspace`
  - `cargo deny check`
  - `wctl doc-lint --path docs/work-packages`

Any failed, blocked, or unjustified not-run required gate prevents `COMPLETE`.

## Phase Plan

1. Scaffold package and record required reading.
2. Implement the package-local WAT adjudication tool and Rust package guard.
3. Execute default and opt-in coupled WAT reruns.
4. Update strategy/index documentation with the observed disposition and next
   route.
5. Run focused/full gates and complete review, verification, line-count,
   handoff, and disposition artifacts.

## Subagent Authorization

Subagent authorization: none. This package does not explicitly authorize
spawning/delegating to subagents; required review and verification are performed
locally with evidence labels.

## Downstream

The phase selector did not materially reduce snow-depth failures; it worsened
all four paired surfaces. The next work package should target the 10.3.4 rank-2
winter-thaw melt response before sub-canopy longwave or rain-heat changes.

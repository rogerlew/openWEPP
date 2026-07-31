# SNOW-SURFACE-EB-04A Failure Observability And State Capture

Status: `complete / pass`

Date: `2026-07-30`

Campaign: `SNOW-SURFACE-EB`

This ExecPlan is a living document maintained under
`docs/codex_exec_plans.md`.

## Purpose / Big Picture

Make every EB-04 rejected snow-surface-energy step independently diagnosable
and replayable before any physical correction is considered. Preserve EB-04
as the frozen first experiment and use targeted diagnostic reruns only.

## Implementation Intent

Intent: `implementation` plus `independent-validation` of diagnostic
publication. This package changes observability and typed error context only.
It does not change process physics or calibrate coefficients.

## Objective

- Preserve the exact typed meteorology cause and the correctly attributed
  density, temperature, pressure, and layer operands at conductivity failure.
- Preserve complete prior-layer state at thickness-reconciliation failure.
- Publish daily shortwave energy, signed vapor mass, and the independently
  reconstructable latent conversion total already produced by Stage 3.
- Provide fail-closed typed snapshots that contain the inputs required for
  deterministic primitive replay.
- Target-rerun and classify all 24 EB-04 failures.
- Independently reconstruct mass, surface-energy, and latent/mass identities.
- Publish accessible diagnostic plots with Markdown sidecars.

## Authority And Protected Boundaries

Authority remains `SC-SNOWENERGY-001` invariants 017, 019, 021, 025 and
producer/consumer obligations 004, 005, 011, together with
`SC-SNOWFREEZE-001` invariants 085 and 086. No contract amendment is required
because this increment exposes already-authorized operands and exact rejected
state.

Protected boundaries:

- no equation, constant, coefficient, threshold, tolerance, cadence, forcing,
  fixture, observation, selector, default, parser, or user-schema change;
- no clamping, fallback, retry, error swallowing, or continued execution after
  a rejected step;
- no observation scoring, calibration, promotion, or causal correction claim;
- EB-04 results remain immutable historical evidence.

## Intended Write Set

- this package tree;
- the Stage 3 typed error, diagnostics, and direct research-trace publication
  paths in `crates/openwepp-hillslope-orchestrator` and `crates/openwepp-runner`;
- directly affected unit and integration tests;
- `docs/ROADMAP.md`;
- `docs/planning/snow-surface-energy-balance-roadmap.md`;
- `docs/work-packages/README.md`.

## Conservation / Publication Acceptance

The package-local independent consumer must reconstruct:

- `surface = shortwave + longwave + latent` before bounded application;
- `latent = signed vapor mass * temperature-specific latent heat`;
- the existing complete snow mass and cold-content ledgers;
- layer SWE and depth aggregates from the captured layer vectors.

The kernel accepted residual bounds remain `1e-9 m` water equivalent and
`1e-6 J m^-2`. Serialized independent reconstruction additionally records a
standard floating-point roundoff bound, but cannot use it to excuse a producer
residual above the canonical threshold. A failed step remains rejected.

## Phase Plan

1. Scaffold and freeze intent, authority, lineage, and validation.
2. Add contract-derived failing tests.
3. Implement typed snapshots and additive diagnostic publication.
4. Build and run the 24-cell targeted replay, classify failures, reconstruct
   ledgers, and generate figures/sidecars.
5. Run terminal validation, dual review, finding disposition, dual
   verification, exact-diff reconciliation, and roadmap/catalog disposition.

## Validation Requirements

- focused unit and EB-03/03A/04A integration tests;
- deterministic package-tool self-check and regeneration;
- all 24 original failures attempted and bound to fixture, selector, day, and
  executable identity;
- independent operand reconstruction and snapshot replay tests;
- `cargo fmt --all -- --check`;
- `cargo clippy --workspace --all-targets -- -D warnings`;
- `cargo nextest run --workspace --profile quick`;
- `cargo nextest run --workspace --profile frost`;
- `cargo nextest run --workspace --profile full`;
- scoped Markdown lint/reference checks;
- SVG parse and figure/sidecar one-to-one checks;
- exact-diff, line-count, placeholder/stub, security, and assurance-impact
  reconciliation.

## Exit Criteria

1. Exact lower-level causes replace the misleading conductivity label/value.
2. Both failure families carry sufficient typed state for primitive replay.
3. Every new trace operand has explicit units and independent reconstruction.
4. All 24 original failed lanes are classified by the targeted rerun.
5. No process result changes except diagnostic/error representation.
6. Figures are plot-only and each has a complete Markdown sidecar.
7. All current-scope validation, review, and verification gates pass.

Any unmet criterion forces `HOLD`.

## Security Impact Gate

No secrets, network access, authentication changes, unsafe Rust, fixture
mutation, or shell interpolation are authorized. Subprocess arguments are
explicit.

## Subagent Authorization

Subagent authorization: this package explicitly authorizes subagent
spawning/delegation to two independent science/code reviewers and two terminal
verification agents. Expected outputs are `artifacts/review_agent_a.md`,
`review_agent_b.md`, `verification_agent_a.md`, and
`verification_agent_b.md`. Review access is read-only. Verification write
access is limited to the assigned package artifact.

## Progress

- [x] (2026-07-30) User authorized scaffolding and full execution.
- [x] (2026-07-30) Authority and the two lossy boundaries identified.
- [x] (2026-07-30) Contract-derived replay and publication tests added.
- [x] (2026-07-30) Typed snapshots and additive diagnostics implemented.
- [x] (2026-07-30) All 24 targeted replays, reconstruction, figures, and
  sidecars complete.
- [x] (2026-07-30) Focused, clippy, quick, frost, and full validation pass.
- [x] (2026-07-30) Dual review findings corrected and accepted.
- [x] (2026-07-30) Dual terminal verification and final handoff complete.

## Surprises & Discoveries

- The hourly Stage 3 producer already retains net shortwave, signed vapor mass,
  latent heat, and full thermal control-volume operands. EB-04 lost them only
  because the daily trace omitted their aggregates.
- A water-depth epsilon was incorrectly reused for a nonzero `kg m^-2` vapor
  diagnostic. The exchange and energy were correct, but tiny exchanges were
  not independently reconstructable until the diagnostic was corrected.
- The 22 masked conductivity-path failures decompose into 17 absolute-zero
  crossings and five saturation-vapor-pressure underflows before conductivity.

## Decision Log

- Decision: use typed error payloads as the fail-closed snapshot surface.
  Rationale: a rejected step cannot emit a successful daily trace row; the
  error is therefore the only truthful production boundary for its exact
  operands.
  Date/Author: 2026-07-30 / Codex.
- Decision: preserve canonical WB14 domain code `E-003` for richer payloads.
  Rationale: observability does not authorize an error-code contract expansion.
  Date/Author: 2026-07-30 / Codex.

## Outcomes & Retrospective

Every EB-04 failure is now exact, reproducible, and independently auditable.
The dominant signature is extreme cold in a vanishingly small active snow
mass; two cases instead expose nanometer-scale filtered depth reconciliation.
EB-04B now has the state needed to characterize causality before correction.

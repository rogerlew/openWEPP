# SNOWFROST-FIDELITY-B No-Qwet Heat-Flow Benchmarks

Status: complete

Package type: benchmark/contract-gate implementation.

Primary gap: `GAP-SNOWFREEZE-002`.

Objective: add no-migration heat-flow benchmark gates before any field-data
calibration or frost-physics tuning. Closure requires contract-derived tests
that independently reconstruct surface resistance, snow/residue insulation,
lower-front heat, and latent-energy/fine-layer mutation while proving the
production Rust frost path still has no `Qwet`/migration-heat implementation.

This package follows `AGENTS.md`, `docs/work-packages/AGENTS.md`,
`docs/specifications/science-contracts/AGENTS.md`,
`docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`,
`tests/AGENTS.md`, `crates/AGENTS.md`, and the literature synthesis in
`docs/work-packages/20260625-snowfreeze-frost-depth-literature-annotation-001/artifacts/literature-synthesis.md`.

Subagent authorization: this package explicitly authorizes
spawning/delegating to read-only Rust-review, science-review, and verification
subagents for benchmark legitimacy, no-`Qwet` source-scan review,
anti-tautology review, and final evidence review. Expected outputs are compact
findings summarized into `artifacts/review-disposition.md` and
`artifacts/verification.md`; subagents may not edit files. Current execution
used local reviews because the user did not separately request subagent
dispatch in this turn.

## Purpose

SNOWFROST-FIDELITY-A proved field residuals cannot yet be attributed to frost
physics because modeled snow depth is absent. B therefore proceeds only as
benchmark work: validate the existing no-`Qwet` frost heat-flow column against
independent formula checks and conservation/energy bounds before later
snow-control, SFCC/frozen-K, or migration/fringe packages.

## Non-Goals

- Do not enable, port, or approximate `Qwet`.
- Do not change snow or frost production physics.
- Do not tune coefficients, observation thresholds, or field-site residuals.
- Do not default-activate direct runtime or delete compatibility runtime.
- Do not add SFCC, frozen conductivity, impedance, or migration/fringe models.

## Authority Envelope

In scope:

- `SC-SNOWFREEZE-001#INV-SNOWFREEZE-006` heat-flow formulation;
- `SC-SNOWFREEZE-001#INV-SNOWFREEZE-012` frost dispatch chain;
- ROADMAP W.1 no-`Qwet` sequence;
- integration-test benchmark vectors over current published frost hourly and
  fine-layer state surfaces;
- production-source scan proving no Rust `qwet`/`frzftp`/migration-heat path is
  active.

Out of scope:

- field observation verdicts;
- production code edits unless a benchmark exposes an in-envelope bug;
- legacy/compatibility bit parity;
- new external data acquisition.

## Intended Write Set

- `docs/work-packages/20260625-snowfrost-fidelity-b-no-qwet-heatflow-benchmarks-001/**`
- `docs/work-packages/README.md`
- `docs/ROADMAP.md`
- `tests/integration/clim06_frost_frozen_soil_kernel_contract.rs`
- `tests/integration/clim06_frost_frozen_soil_kernel_contract/benchmark.rs`

## Phase Plan

### Phase 0: Scaffold and Authority Lock

- Create package scaffold and prompt.
- Record required reading and benchmark scope.
- Define current-scope closure gates.

Exit criteria:

- Package artifacts exist.
- The package explicitly blocks `Qwet` promotion and field residual tuning.

### Phase 1: Benchmark Tests

- Add integration tests for:
  - a Kurylyk/Stefan-style one-dimensional latent-only freezing-front upper
    bound;
  - analytic surface series resistance including snow/residue/frozen-soil path;
  - snow insulation reducing freezing flux relative to bare soil;
  - lower-front heat from the existing harmonic unfrozen conductivity path;
  - latent heat consumed by fine-layer front mutation from published state
    deltas.
- Add the new module to the existing CLIM06 integration suite.

Exit criteria:

- Tests fail if surface heat bypasses snow/residue resistance.
- Tests fail if frozen-front advance is not energy/latent-heat bounded.
- Tests fail if the controlled one-dimensional freeze front exceeds a
  latent-only Stefan upper bound.
- Tests fail if the benchmark depends on field observations or `Qwet`.

### Phase 2: Validation

- Run focused CLIM06 benchmark tests.
- Run source scans for forbidden production `qwet`/`frzftp` symbols.
- Run Rust and docs closure gates.

Exit criteria:

- Focused tests pass.
- Source scan confirms no production Rust `qwet`/`frzftp` implementation.
- Final gate table has no unjustified `FAIL`, `BLOCKED`, or `NOT RUN`.

### Phase 3: Review and Disposition

- Complete dual review, review disposition, line-count, worker handoff, and
  package disposition artifacts.
- Update ROADMAP and package catalog.

Exit criteria:

- Package closes complete only as benchmark closure.
- Any physics-changing follow-on remains separately scoped.

## Validation Commands

Run from `/home/workdir/openWEPP`.

- `cargo test --test clim06_frost_frozen_soil_kernel_contract snowfrost_b_ -- --nocapture`
- `rg -n "qwet|Qwet|frzftp" crates tests/integration tools`
- `cargo fmt --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo test --workspace`
- `cargo deny check`
- `git diff --check`

## HOLD Boundaries

Close as `HOLD` only if current frost outputs cannot expose enough state for an
independent benchmark, the source scan finds an active Rust `Qwet`/migration
implementation, or focused tests expose an in-envelope physics defect that
requires production correction beyond this benchmark package.

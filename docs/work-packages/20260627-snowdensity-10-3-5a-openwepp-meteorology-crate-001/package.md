# SNOWDENSITY-10.3.5a - `openwepp-meteorology` Crate

Status: queued (revised by Codex, 2026-06-27); ready for execution after
operator approval.

Package type: foundational physics/numerics crate; clean-room implementation;
candidate-only, production-free increment.

Closure target: `COMPLETE-10-3-5A-METEOROLOGY-CRATE` or named `HOLD-...`.

## Objective

Create `crates/openwepp-meteorology`: pure psychrometric primitives plus the
Harder & Pomeroy (2013) hydrometeor-temperature precipitation-phase method, as
the foundation for SNOWDENSITY-10.3.5 robust rain/snow partition work.

This package does not wire the method into production snow/frost execution.
Production partition routing, default activation, and Jennings-corpus validation
belong to 10.3.5b.

## Primary Authority

- `docs/planning/snow-frost-fidelity-strategy.md` section 10.3 step 5:
  partition near 0 degC is the lead defect-eligible maritime snow-depth blocker.
- `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`:
  canonical snow/freeze process authority. This package must amend it before
  crate implementation because precipitation phase belongs to this contract.
- Harder, P. & Pomeroy, J. (2013), *Estimating precipitation phase using a
  psychrometric energy balance method*, Hydrol. Process. 27, 1901-1914
  (R-57; `references/copyrighted/source_pdfs/harder2013.pdf`).
- ADR-0011 and ADR-0017: contract-first new physics and comparator-is-flag
  posture.

## Rationale

- SNOWDENSITY-10.3.4 ranked snow/rain partition near 0 degC as the top
  defect-eligible mechanism behind maritime over-accumulation.
- The strategy chooses a physical hydrometeor energy-balance method rather than
  a tuned `RST`, site threshold, or table. The method consumes air temperature
  plus humidity and is intended to generalize across maritime and continental
  regimes without site calibration.
- openWEPP already has hourly winter partition seams and hourly air/dew-point
  surfaces. This package builds the reusable, tested numerical core before any
  runtime integration.

## License And Clean-Room Discipline

- openWEPP is Apache-2.0; `deny.toml` denies GPL, AGPL, and LGPL licenses.
- The Canadian Hydrological Model has a Harder-Pomeroy implementation but is
  GPLv3. Do not read, port, paraphrase, or copy CHM code. Do not use CHM as an
  implementation reference.
- MetPy is available locally at `/home/workdir/MetPy` and is BSD-3-Clause. It may
  be consulted only as a cross-check for standard meteorological primitives such
  as saturation vapor pressure and dew point. Implement Rust equations from the
  cited paper and cited standard formulae, not by translating MetPy code.
- The package must produce `artifacts/clean-room-provenance.md` with:
  - every implemented equation or constant mapped to a cited source;
  - any MetPy cross-check named as numeric/reference-only;
  - an explicit statement that CHM/GPL code was not read or used.
- `cargo deny check` must remain clean.

## Scope

- Add workspace member `crates/openwepp-meteorology`.
- Keep the crate pure: no filesystem I/O, no network I/O, no runtime/global
  configuration reads, and no production snow/frost dependencies.
- Use existing `openwepp-unit-boundary` types where applicable
  (`TemperatureCelsius`, `FractionUnitInterval`, etc.). If pressure or vapor
  pressure needs typed public inputs and no existing boundary type exists, define
  checked local crate input types rather than passing raw, unchecked scalars.
- Implement pure psychrometric primitives:
  - saturation vapor pressure over water and ice;
  - actual vapor pressure, dew point, and relative humidity conversions;
  - temperature-dependent latent heat of vaporization and sublimation;
  - temperature-dependent thermal conductivity of air and molecular diffusivity
    of water vapor where required by Harder-Pomeroy;
  - finite/domain validation and typed errors.
- Implement the Harder-Pomeroy hydrometeor-temperature solver:
  - solve hydrometeor temperature `Ti` from air temperature and humidity using
    the paper-authoritative energy-balance equations;
  - expose convergence tolerance, maximum-iteration behavior, and typed
    non-convergence/domain errors;
  - preserve expected unsaturated ordering (`Td <= Ti <= Ta`) where the paper
    states it applies and inputs are valid.
- Implement the Harder-Pomeroy `Ti` to rainfall-fraction mapping using the
  paper-published coefficients. The output must be fractional and bounded in
  `[0, 1]`, not a binary step.
- Add crate-local unit tests for primitives, solver behavior, phase-fraction
  monotonicity/bounds, domain errors, and published or independently
  reconstructed reference vectors.
- Add package evidence artifacts and close the package truthfully.

## Non-Scope

- No production winter-partition wiring.
- No edits to `snow.hourly.stmtim.rst_c`, legacy `RST` partition behavior,
  `winter`, `stmtim`, runfile/parser selectors, public WAT/HBP/PASS schemas,
  snowbench production routing, or default model activation.
- No full Jennings observed-phase validation run. The 17.8M-row Jennings file is
  local and gitignored for 10.3.5b; this package may use committed metadata only
  for context.
- No site-specific calibration and no fitted openWEPP constants.
- No reading or porting GPL/CHM code.

## Contract-First Requirement

This package must amend `SC-SNOWFREEZE-001` before crate implementation. A
`HOLD` disposition is required if the contract cannot be amended.

The amendment must, at minimum, define:

- a candidate-only precipitation-phase method identifier for the
  Harder-Pomeroy psychrometric hydrometeor-temperature method;
- input surfaces and units: air temperature, humidity source
  (dew point, relative humidity, or vapor pressure as accepted by the API),
  pressure if required, and output rainfall/snowfall fractions;
- equation/provenance anchors for the `Ti` solver and `Ti -> rainfall fraction`
  mapping;
- invariants for finite inputs, bounded fractions, monotonic phase behavior,
  no site calibration, convergence or fail-closed non-convergence, and expected
  `Td <= Ti <= Ta` ordering for valid unsaturated cases;
- rollback boundary: this package is crate-only and cannot change production
  partition behavior until a later 10.3.5b package explicitly wires and validates
  the candidate.

The standard psychrometric primitives may be documented as reusable numerics, but
the precipitation-phase law and candidate status belong in `SC-SNOWFREEZE-001`.

## Intended Write Set

- `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
- `docs/specifications/science-contracts/index.md` only if lifecycle metadata
  requires an update.
- `Cargo.toml`
- `crates/openwepp-meteorology/**`
- package-local files under
  `docs/work-packages/20260627-snowdensity-10-3-5a-openwepp-meteorology-crate-001/**`

Do not edit production snow/frost runtime crates except to prove they were not
wired. If execution discovers a required production edit, stop and close `HOLD`
or scaffold 10.3.5b.

## Required Evidence Artifacts

- `artifacts/required-reading-map.md`
- `artifacts/contract-implementation-evidence.md`
- `artifacts/contract-test-implementation-evidence.md`
- `artifacts/pre-implementation-contract-gate.md`
- `artifacts/clean-room-provenance.md`
- `artifacts/implementation-test-evidence.md`
- `artifacts/no-production-wiring-scan.md`
- `artifacts/kernel-profile-compliance-checklist.md`
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
updated during execution; do not close on empty or stale placeholders.

## Acceptance Gates

- `SC-SNOWFREEZE-001` is amended with candidate Harder-Pomeroy phase authority
  before crate implementation, or the package closes `HOLD`.
- Contract-derived tests or source-level assertions prove the new contract text
  exists and names the candidate-only rollback boundary.
- `crates/openwepp-meteorology` builds as a pure crate with no I/O.
- Unit tests pass for:
  - saturation vapor pressure reference values over water and ice;
  - dew-point / vapor-pressure / relative-humidity round trips;
  - latent heat, thermal conductivity, and diffusivity values or bounds from the
    cited authority used;
  - Harder-Pomeroy `Ti` solver reference vectors from paper examples when
    available, otherwise independently reconstructed vectors recorded in
    `clean-room-provenance.md`;
  - `Ti -> rainfall fraction` monotonicity and `[0, 1]` bounds;
  - invalid input and non-convergence typed errors.
- `artifacts/clean-room-provenance.md` maps every equation/constant to its source
  and records no CHM/GPL code use.
- `artifacts/no-production-wiring-scan.md` proves no production snow/frost
  partition/default/schema/selector path changed in this package.
- Required review, finding disposition, verification, line-count governance,
  worker handoff, and final disposition artifacts are complete and current.
- Final gates:
  - `cargo fmt --check`
  - `cargo clippy --workspace --all-targets -- -D warnings`
  - `cargo test --workspace`
  - `cargo deny check`

Any failed, blocked, or unjustified not-run required gate prevents `COMPLETE`.

## Phase Plan

1. Orientation and authority
   - Read the required documents and complete `required-reading-map.md`.
   - Amend `SC-SNOWFREEZE-001` for the candidate Harder-Pomeroy phase method.
   - Add contract-derived assertions/tests or a source-level contract gate.
   - Record `pre-implementation-contract-gate.md`.
2. Crate scaffold and primitives
   - Add the workspace crate and minimal public API.
   - Implement checked input/output types, typed errors, and primitive functions.
   - Add reference-value and round-trip tests.
3. Harder-Pomeroy solver
   - Implement the `Ti` solver and rainfall-fraction mapping from the paper.
   - Add solver, monotonicity, bounds, invalid-domain, and convergence tests.
4. Provenance and isolation
   - Complete `clean-room-provenance.md`.
   - Run and record no-production-wiring scans.
   - Run focused and workspace gates.
5. Closure
   - Complete dual review, finding disposition, dual verification, line-count
     governance, worker handoff, and final disposition.

## Downstream

10.3.5b consumes this crate to wire the candidate into the existing hourly
partition path and validate against the Jennings observed-phase corpus under the
no-site-calibration cross-climate gate. That later package must prove
conservation, opt-in/default isolation, rollback, and maritime/continental
phase-threshold behavior before any activation decision.

## Subagent Authorization

Subagent authorization: this package explicitly authorizes spawning/delegating to
read-only review and verification subagents for dual package review, clean-room
provenance review, no-production-wiring review, and final verification. Expected
outputs are `review_agent_a.md`, `review_agent_b.md`,
`verification_agent_a.md`, and `verification_agent_b.md`. Subagents have
read-only access unless a later operator request explicitly grants bounded
write access.

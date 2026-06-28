# PARADIGM-2 STAGE 0 - Surface Energy Balance

Status: `executed-complete`  
Date: `2026-06-28`  
Owner: Codex  
Type: pure-crate implementation package

## Objective

Add surface-energy-balance primitives to `crates/openwepp-meteorology` for reuse
by snow, soil, and water surfaces. Stage 0 is a pure crate package: no runtime
wiring, no production default/schema/fixture/frost change, and no multilayer snow
state.

Boundary shorthand: pure crate, no runtime wiring.

## Final Disposition

`EXECUTED-COMPLETE`.

Stage 0 added pure, surface-agnostic energy-balance primitives to
`openwepp-meteorology` and did not wire them into production runtime paths. The
crate now exposes typed functions for net all-wave radiation, Monin-Obukhov
turbulent H/LvE/vapor mass flux, conductive heat exchange, precipitation
advected heat, latent<->mass conversion, and balance-term summation.

No SC amendment, selector, default, fixture, output schema, density cap, frost,
or runtime behavior changed.

Implemented primitive families:

- net all-wave radiation: net shortwave plus net longwave;
- Monin-Obukhov bulk-aerodynamic turbulent sensible heat, latent heat, and
  vapor mass flux with explicit latent<->mass coupling;
- ground/substrate conductive heat exchange;
- advected heat from rain and snowfall;
- a sum helper for the surface energy balance.

## Authority Envelope

- `docs/planning/paradigm2-multilayer-snow-specification.md` requirement 6 and
  Stage 0.
- `docs/planning/snow-frost-fidelity-strategy.md` section 10.3 step 10.
- `docs/backlog/20260627-stream-water-temperature-surface-energy-balance.md`
  decision to scope `openwepp-meteorology` as shared surface energy balance.
- ADR-0011 contract-first posture and ADR-0028 admission posture for later
  candidates.
- libsnobal CC0 source as equation/reference implementation authority.
- Marks et al. 1999 as published SNOBAL/ISNOBAL surface energy-balance context.

No SC amendment is made in Stage 0 because the package adds reusable numerics
only; it does not authorize a production snow/frost candidate, selector, default,
runtime dependency, or observed-data admission decision.

## Included Scope

- Add `openwepp_meteorology::surface_energy` as a pure module.
- Use `openwepp-unit-boundary` types where already available and checked local
  types otherwise.
- Add typed finite/domain validation and typed errors.
- Add unit tests for reference values, closure, latent<->mass coupling,
  turbulent stability, and domain errors.
- Record clean-room provenance for each equation and constant.
- Prove no production runtime wiring changed by source scan.

## Excluded Scope

- No multilayer snow state.
- No per-layer thermal solver.
- No snow/frost/runtime wiring.
- No selectors, production defaults, parser/runfile/user controls, fixtures,
  output schema, density cap, or frost changes.
- No fixture tuning or fitted constants.

## Intended Write Set

- `crates/openwepp-meteorology/src/lib.rs`
- `crates/openwepp-meteorology/src/error.rs`
- `crates/openwepp-meteorology/src/surface_energy.rs`
- `tests/integration/paradigm2_stage0_surface_energy_balance_contract.rs`
- `Cargo.toml`
- `docs/work-packages/README.md`
- `docs/work-packages/20260628-paradigm-2-stage-0-surface-energy-balance-001/**`

## Gates

- `cargo fmt --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo test --workspace`
- `cargo deny check`
- Focused crate and integration tests pass.
- Clean-room provenance artifact maps every equation/constant to source.
- No-production-wiring scan records no runtime source references.
- Line-count governance recorded.

## Execution Log

- [x] Required reading completed.
- [x] Work package scaffolded.
- [x] Pure surface-energy module implemented.
- [x] Focused tests and guards completed.
- [x] Workspace gates completed.
- [x] Review, verification, and final disposition recorded.

## Evidence Artifacts

- `artifacts/clean-room-provenance.md`
- `artifacts/no-production-wiring-scan.md`
- `artifacts/line-count-governance.md`
- `artifacts/review.md`
- `artifacts/verification.md`

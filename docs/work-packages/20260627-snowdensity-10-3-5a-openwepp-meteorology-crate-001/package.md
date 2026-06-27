# SNOWDENSITY-10.3.5a — `openwepp-meteorology` Crate (Psychrometric Phase Core)

Status: scaffolded (Claude Code, 2026-06-27); ready for Codex.

Package type: foundational physics/numerics crate — **clean-room** implementation.

Primary authority:
- `docs/planning/snow-frost-fidelity-strategy.md` §10.3 step 5 (Robust Rain/Snow
  Partition) — the strategy selects Harder & Pomeroy (2013) psychrometric energy
  balance as the most physically defensible, no-site-calibration partition method.
- Harder, P. & Pomeroy, J. (2013), *Estimating precipitation phase using a
  psychrometric energy balance method*, Hydrol. Process. 27, 1901–1914
  (R-57; `references/copyrighted/source_pdfs/harder2013.pdf`).
- ADR-0011 (contract-first new physics), ADR-0017.

Closure target: `COMPLETE-10-3-5A-METEOROLOGY-CRATE` or `HOLD-…`.

## Objective

Create `crates/openwepp-meteorology`: pure psychrometric primitives plus the
**Harder & Pomeroy (2013) precipitation-phase method**, implemented clean-room
from the paper, as the foundation for the SNOWDENSITY-10.3.5 robust rain/snow
partition. **No production wiring** — that is 10.3.5.b.

## Why

- 10.3.4 ranked **snow/rain partition near 0 °C** as the #1 defect-eligible cause
  of the maritime over-accumulation that blocks non-SNOTEL frost attribution.
- The strategy chose the **most robust** method, not the cheapest: a physical
  hydrometeor energy balance (a law, not a tuned `RST` or a fitted threshold) that
  self-adjusts maritime↔continental with no site calibration. The method
  *improves at finer timesteps*, and openWEPP already partitions **hourly**
  (`snow.hourly.stmtim.rst_c`) with hourly air temp + dew-point present.
- The supporting psychrometric functions (saturation vapour pressure, dew-point,
  latent heats) are currently scattered across the snow/ET code; a dedicated crate
  consolidates them as reusable pure functions (partition now; ET, snow later).

## License discipline (load-bearing — read first)

- openWEPP is **Apache-2.0**; `deny.toml` **denies GPL/AGPL/LGPL**.
- The **Canadian Hydrological Model (CHM)** has a Harder-Pomeroy implementation
  but is **GPLv3** — **do not read, port, paraphrase, or copy CHM code.** Implement
  clean-room from the paper only.
- **MetPy** (`/workdir/metpy`, BSD-3-Clause, allow-listed) may be consulted as a
  cross-check reference for the *standard* primitives (saturation vapour pressure,
  dew-point). Implement in Rust from first principles / the cited formulae; **cite
  MetPy where its formulation is used; do not copy.**
- `cargo deny check licenses` must stay clean.

## Scope

- New crate `crates/openwepp-meteorology` (Apache-2.0, workspace member; pure
  functions, **no I/O**; use `openwepp-unit-boundary` for typed units where
  applicable).
- **Psychrometric primitives** (pure functions over typed inputs):
  - saturation vapour pressure over water and over ice;
  - actual vapour pressure / dew-point / relative-humidity conversions;
  - latent heat of vaporization and sublimation (temperature-dependent);
  - thermal conductivity of air and molecular diffusivity of water vapour
    (temperature-dependent);
  - the **hydrometeor energy-balance solver**: iteratively solve the falling
    particle's temperature `Ti` from `Ta` + humidity (+ ventilation as the paper
    specifies). This is the Harder-Pomeroy physical core.
- **Phase mapping**: the Harder & Pomeroy `Ti → rainfall fraction` relationship
  with the paper's published coefficients (the one small empirical piece; `Ti`
  itself is physical). Expose a fractional `[0,1]` phase, not a binary step.
- **Tests**: unit tests against published/known values — saturation vapour
  pressure at reference temperatures, dew-point round-trips, and the paper's
  reported `Ti` / phase examples (e.g. its tabulated cases).
- **Provenance map**: a `clean-room-provenance.md` artifact mapping each
  implemented equation → its source (Harder-Pomeroy equation number; MetPy
  reference where a standard primitive's formulation is used). The auditable
  record that no GPL/CHM code was used.

## Non-Scope

- No production winter-partition wiring; no change to `snow.hourly.stmtim.rst_c`
  or the `RST` path (that is 10.3.5.b).
- No default activation, parser/runfile/CLI selector, or output-schema change.
- No full Jennings-corpus validation run (10.3.5.b); `.a` ships unit tests + the
  paper-example checks only.
- No reading or porting of CHM (GPL) code.

## Contract-first question (resolve early)

Harder-Pomeroy phase is **new physics** → per ADR-0011 it likely needs a
science-contract surface: the phase law, the `Ti` energy-balance invariants
(bounds, monotonicity, conservation), and the **no-site-calibration** obligation.
Decide whether to author a new `SC-*` contract or extend `SC-SNOWFREEZE-001`
before the crate is consumed by the partition (10.3.5.b). The standard primitives
(vapour pressure etc.) are textbook numerics and may not need a contract; the
phase law does. Route the decision through top-down contract authoring.

## Acceptance Gates

- `crates/openwepp-meteorology` builds; pure functions, no I/O; uses
  `openwepp-unit-boundary` units where applicable.
- Unit tests pass: primitives against reference values; Harder-Pomeroy `Ti` +
  phase against the paper's examples.
- `clean-room-provenance.md` present (equation → source); explicit statement that
  no CHM-derived code was used.
- Contract-first decision recorded (SC surface authored or explicitly deferred
  with rationale).
- `cargo fmt --check`, `cargo clippy --workspace --all-targets -- -D warnings`,
  `cargo test --workspace`, `cargo deny check` (licenses clean).

## Phase Plan

1. Scaffold the crate + workspace membership; resolve the contract-first question.
2. Implement + unit-test the psychrometric primitives (clean-room; MetPy
   cross-check; cite).
3. Implement + unit-test the Harder-Pomeroy `Ti` solver + `Ti → fraction` mapping
   (from the paper).
4. Write `clean-room-provenance.md`; run all gates.

## Downstream (not this package)

10.3.5.b consumes this crate to replace the `RST` step at the existing hourly
partition, and validates against the Jennings observed-phase corpus
(`tests/fixtures/precip_phase_observed/`: per-station 50% thresholds + the 17.8M
hourly obs) with the **no-site-calibration cross-climate gate** (§10.3 step 5).

## Subagent Authorization

Not authorized for spawning subagents unless a later operator request adds it.

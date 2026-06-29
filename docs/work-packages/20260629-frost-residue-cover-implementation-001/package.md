# FROST Residue-Cover Implementation

Status: executed complete.

Package type: contract-first implementation work package.

Objective: wire dynamic seasonal forest litter/residue depth to the frost
surface energy balance by connecting the residue mass producer to the
`frost.runtime_residue_depth_m` consumer.

Primary gap: `GAP-SNOWFREEZE-002`.

## Scope

Included:

- Phase 0 characterization of the existing `Dec_*` dynamic
  `surface_residue_kg_m2` trajectory.
- Contract amendments before production physics changes.
- Dynamic mass-to-depth-to-frost coupling, preserving the initial residue-depth
  seed as the day-zero/t0 value.
- Identity-stable inert/non-seasonal path behavior when senescence and
  decomposition are off.
- Sleepers A-versus-B frost-timing re-score after the coupling is present.

Excluded:

- No canopy leaf-on/leaf-off implementation outside residue/litter cover.
- No `Qwet`, frozen-K, SFCC, impedance, or legacy-envelope frost-model work.
- No frost-default activation.
- No public output-schema change unless already supported diagnostics are used.

## Required Reading

- `docs/backlog/20260626-frost-daylength-canopy-decline-hemisphere-robust.md`
  surface residue / litter cover section.
- `docs/planning/snow-frost-fidelity-strategy.md` section 11 step 3.
- `docs/work-packages/20260629-frost-step3-residue-parameterization-001/`.
- `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`.
- `docs/specifications/science-contracts/contracts/SC-RESIDUE-001.md`.
- `docs/specifications/science-contracts/contracts/SC-PLANT-001.md`.
- `crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/coupling/frost.rs`.
- `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/01_management.rs`.
- `crates/openwepp-hillslope-orchestrator/src/direct_runtime/decomposition.rs`.
- `crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/`.

## Intended Write Set

- `docs/work-packages/20260629-frost-residue-cover-implementation-001/**`
- `docs/work-packages/README.md`
- `docs/backlog/20260626-frost-daylength-canopy-decline-hemisphere-robust.md`
- `docs/planning/snow-frost-fidelity-strategy.md`
- `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
- `docs/specifications/science-contracts/contracts/SC-RESIDUE-001.md`
- `crates/openwepp-hillslope-orchestrator/src/direct_runtime/**`
- `crates/openwepp-runner/src/hillslope/direct_publication/**`
- Targeted tests under `crates/openwepp-hillslope-orchestrator/src/tests/` and
  `crates/openwepp-runner/tests/`.

Transient run directories are confined to `target/`. Curated evidence artifacts
are recorded under this package and the rerun Step 3 package artifacts.

## Execution Plan

1. Phase 0: characterize the current `Dec_*` residue-mass producer trajectory
   and decide whether the package is pure wiring or also needs a litter-input
   limb.
2. Phase 1: amend the governing science contract before production physics.
3. Phase 2: implement dynamic residue-depth coupling from residue mass to frost
   thermal inputs, including tests for dynamic behavior and inert identity.
4. Phase 3: rerun the Sleepers A-versus-B timing validation and backlog residue
   gates.
5. Close with verification, review, disposition, line-count governance, and
   evidence-class labelling.

## Phase 0 Branch

- `Mass seasonal`: proceed with pure mass-to-depth-to-frost wiring.
- `Mass decay-only`: add the missing litter-input limb before wiring.
- `Mass unavailable from the real direct-production path`: close `HOLD` unless
  a scoped diagnostic proves the producer path without changing production
  physics.

## Exit Criteria

- Phase 0 monthly trajectory is captured with command provenance.
- Contract amendments land before production behavior changes.
- Frost thermal inputs consume dynamic residue depth, not the static init seed.
- Inert/non-forest fixtures remain identity-stable.
- Residue mass/depth conservation and typed domain validation close.
- Sleepers A-versus-B frost timing is rerun or an honest `HOLD` is recorded.
- Rust and documentation gates are recorded.

## Disposition

Executed complete after review disposition. Phase 0 proved the existing `Dec_*`
mass path was flat under zero decomposition and no recurring litter input, so the
implementation added the missing litter-input limb plus dynamic
mass-to-depth-to-frost coupling. Claude review found the first-pass
forest-litter decay fallback inconsistent with its cited authority; the
contract and implementation now use the authority-backed `k=0.5 yr^-1`
fallback (`0.5 / 365.25 d^-1`). The fall litter-drop window remains anchored to
the management fall date (`jdharv`) as a known limitation until the physical
frost/daylength phenology backlog lands.

Phase 3 passed the seasonal residue entry gate under the corrected constant and
routed the Sleepers A-versus-B test to branch A as a partial contributor:
seasonal residue reduced candidate-defect timing cells from 18 to 13. The
cleared cells establish residue lifecycle as a contributor, but
`GAP-SNOWFREEZE-002` remains open because 13 candidate-defect cells remain for
follow-up frost attribution.

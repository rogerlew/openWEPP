# SNOWDENSITY-05 Melt Modernization Contract-First

Status: queued.

Package type: kernel-affecting contract-first implementation package.

Primary contract: `SC-SNOWFREEZE-001`.

Objective: modernize the production Corps-of-Engineers snowmelt energy-balance
path by feeding the existing `amelt`/`cmelt` lineage with contract-authorized
shortwave/albedo operands while keeping melt separate from snow density. This
package must amend canonical contract authority before any production code,
must keep the new path opt-in, and must preserve the current legacy CoE melt as
default and rollback.

Operator decision, 2026-06-25: do **not** promote the SNOWDENSITY-04
`dense_slow_melt_v1` degree-day snowbench candidate. Its profile improvement is
a negative benchmark showing melt/density conflation. The production target is
the WEPP Chapter 3 / CoE energy-balance melt:

```text
hrmelt = 0.0254 * (amelt - bmelt + cmelt + dmelt)
amelt  = 0.0607 * hrrad * (1 - cancov)
cmelt  = 0.0188 * U * (1 - 0.8 * cancov) * (...)
```

Architecture disposition:

- Put modernization in the winter-column melt-term path in
  `crates/openwepp-hillslope-orchestrator/src/hydrology/`, adjacent to the
  existing `SnowMeltTerms` / `compute_snow_melt` lineage. Do not move this into
  `crates/openwepp-runner/src/hillslope/snowbench_physics_bulk.rs`.
- Introduce an opt-in melt selector separate from snow-density model selection:
  planned selector shape is `snow_melt_model = legacy_coe |
  coe_shortwave_albedo_v1`. `legacy_coe` remains default.
- Maintain the existing melt-term family (`amelt`, `bmelt`, `cmelt`, `dmelt`,
  signed raw melt, corrected daily redistribution) and add typed operands for
  shortwave source provenance and albedo state.
- Initial albedo state shape: surface albedo fraction, snow-cover
  temperature-age index, surface-refresh marker from fresh snowfall, and
  model-id/provenance. Exact Brock-2000 formula/constants must be ratified in
  `SC-SNOWFREEZE-001` before code.
- Use existing `SC-CLIMATE-001#INV-CLIMATE-013` hourly-radiation unit authority.
  Do not tune or rescale the shared radiation forcing to fit SNOTEL snowmelt.
- Density compaction remains SNOWDENSITY-06; runtime density activation remains
  SNOWDENSITY-07. This package must not use a melt change to claim density
  closure.

## Conflict / Gap Flag Before Coding

Confirmed before scaffold:

- No direct conflict was found in `SC-SNOWFREEZE-001`: `INV-SNOWFREEZE-051` and
  `OBL-SNOWFREEZE-P-026` keep `physics_bulk` candidate-only and do not ratify
  the SNOWDENSITY-04 degree-day candidate as production physics.
- Required authority is missing: `SC-SNOWFREEZE-001` does not yet contain the
  melt-modernization invariant, shortwave/albedo operands, albedo-state
  domains, no-radiation-tuning guard, or opt-in melt selector. This package must
  add them before production code.
- Existing package evidence conflicts with the new decision:
  `docs/work-packages/20260625-snowdensity-04-offline-adjudication-loop-001/artifacts/worker-handoff.md`
  says to runtime-opt-in `dense_slow_melt_v1`. That route is superseded by this
  package and must be treated as stale handoff evidence, not current authority.
- Existing Rust/test term accounting requires sign reconciliation before code:
  `tests/integration/clim05_snow_runtime_kernel_contract.rs` currently asserts
  raw melt from `amelt + bmelt + cmelt + dmelt`, while the operator decision and
  WEPP Chapter 3 prose use `amelt - bmelt + cmelt + dmelt`. The likely
  explanation is that openWEPP stores `bmelt` as a signed term. The contract
  amendment must bind the sign/alias convention before any melt-formula edit.

## Required Reading

Core:

- `AGENTS.md`
- `docs/work-packages/AGENTS.md`
- `docs/specifications/science-contracts/AGENTS.md`
- `docs/standards/kernel-work-package-preparation.md`
- `docs/planning/snow-frost-fidelity-strategy.md` sections 2, 4, 5, 7, and 10
- This package file

Contract and architecture:

- `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
- `docs/specifications/science-contracts/contracts/SC-CLIMATE-001.md`
- `docs/specifications/science-contract-authoring-procedure.md`
- `docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `docs/decisions/0017-re-pin-operational-distrust-comparator-is-flag-not-target.md`
- `docs/decisions/0027-opt-in-physics-bulk-snow-model.md`

Code and tests:

- `crates/AGENTS.md`
- `tests/AGENTS.md`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/infiltration_reconciliation.rs`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_00_support_helpers.rs`
- `tests/integration/clim05_snow_runtime_kernel_contract.rs`
- `tests/integration/snowdensity02_contract_adr_guard.rs`
- `tests/integration/snowdensity03_physics_bulk_offline_contract.rs`

Literature authority from `docs/planning/snow-frost-fidelity-strategy.md`:

- WEPP Chapter 3: `references/50201000/chap3.pdf`
- Ohmura 2001: `references/copyrighted/Ohmura2001_meltindex.pdf`
- Pellicciotti 2005: `references/copyrighted/pellicciotti2005.pdf`
- Carenzo 2009: `references/copyrighted/carenzo2009.pdf`
- Brock 2000: `references/copyrighted/brock2000.pdf`
- Walter 2005: `references/copyrighted/walter2005.pdf`
- Gupta 2023: `references/vendorable/Gupta2023_HESS.pdf` if present locally;
  otherwise use the annotated bibliography/strategy citation as static
  authority and record the missing local file.

## Scope

In scope:

- Amend `SC-SNOWFREEZE-001` for:
  - a new melt-modernization invariant, planned as `INV-SNOWFREEZE-052`;
  - a new producer obligation, planned as `OBL-SNOWFREEZE-P-027`;
  - variables for opt-in melt selector, shortwave source/provenance, albedo
    state, temperature-age index, and albedo model id;
  - explicit sign/alias semantics for `bmelt` so Chapter 3 prose and current
    `melt_bmelt_in` trace fields are reconciled before formula changes;
  - authority anchors for WEPP Ch. 3, Ohmura 2001, Pellicciotti 2005,
    Carenzo 2009, Brock 2000, Walter 2005, and Gupta 2023;
  - invalid states forbidding radiation-forcing tuning, missing albedo state on
    opt-in path, degree-day snowbench promotion, and melt/density substitution.
- Add contract-derived tests proving the new authority exists and that
  SNOWDENSITY-04 `dense_slow_melt_v1` is retained only as a negative benchmark.
- Implement opt-in `coe_shortwave_albedo_v1` production melt path after
  contract/test gates are complete.
- Preserve `legacy_coe` default output identity for default runs.
- Record operand lineage for conservation-sensitive melt/routing outputs before
  production edits.
- Add focused tests for albedo state transitions, shortwave/albedo effect on
  `amelt`, no radiation-forcing retune, conservation/routed-melt identity, and
  default/rollback behavior.
- Run the SNOTEL rubric profile only as validation evidence under
  `INV-SNOWFREEZE-050`; forcing-robust cells carry verdict weight.

Out of scope:

- No default activation.
- No promotion of `snowbench_physics_bulk.rs` degree-day variants.
- No per-site constants or SNOTEL-fitted defaults.
- No radiation-source rescaling to improve melt.
- No density compaction implementation; SNOWDENSITY-06 owns density.
- No runtime opt-in for the full `physics_bulk` snow-density model;
  SNOWDENSITY-07 owns that.
- No frost heat-flow, frozen-K/SFCC, impedance, or migration-heat changes.
- No compatibility runtime deletion.

## Intended Write Set

Expected:

- `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
- `tests/integration/snowdensity05_melt_modernization_contract.rs` or equivalent
- `crates/openwepp-hillslope-orchestrator/src/hydrology/**`
- `crates/openwepp-runner/src/hillslope/**` only for trace/publication of new
  opt-in diagnostics, not for `physics_bulk` melt promotion
- `crates/openwepp-sim-contract/src/units_mod/**` if new symbols require unit
  registry entries
- package artifacts under this directory

Protected:

- `crates/openwepp-runner/src/hillslope/snowbench_physics_bulk.rs` must not be
  promoted into production runtime.
- Existing default `legacy_coe` / `legacy_wepp` behavior must remain default.
- Shared climate radiation surfaces must not be tuned or rescaled.

## Phase Plan

1. Required reading and conflict audit.
2. Contract amendment:
   - add melt-modernization variables, anchors, invariant, obligation, invalid
     states, boundary disposition, and change-log row;
   - record conflict/gap disposition in artifacts.
3. Contract-derived tests:
   - assert `INV-SNOWFREEZE-052`, `OBL-SNOWFREEZE-P-027`, opt-in/default
     selector language, no-radiation-tuning language, and degree-day negative
     benchmark language;
   - assert the `bmelt` sign/alias convention so the implementation cannot
     silently flip the energy-balance sign.
4. Pre-implementation gate:
   - run contract-focused tests and record green evidence before production
     code.
5. Implementation:
   - add typed opt-in melt selector and albedo state;
   - add `coe_shortwave_albedo_v1` in the existing CoE melt-term path;
   - preserve signed raw melt, daily redistribution, and routed melt
     conservation;
   - expose diagnostics only where needed for rubric/trace evidence.
6. Focused tests and diagnostics:
   - albedo state, shortwave/albedo `amelt`, default rollback, no density/melt
     substitution, and conservation identities.
7. Validation:
   - run focused gates, SNOTEL rubric comparison where available, full closure
     gates, dual review/verification, line-count governance, and handoff.

## Exit Criteria

The package may close `complete` only when all are true:

- `SC-SNOWFREEZE-001` contains the melt-modernization authority and records that
  SNOWDENSITY-04 degree-day variants are negative benchmarks, not production
  candidates.
- Contract-derived tests fail before the amendment and pass after it, or the
  package records equivalent before/after evidence.
- Production code changes occur only after contract/test gate evidence.
- Default runs preserve legacy CoE melt behavior and manifest/trace identity.
- Opt-in runs use the modernized CoE shortwave/albedo path without changing the
  shared radiation forcing.
- Melt/routed-water conservation is independently reconstructed from produced
  operands.
- SNOTEL rubric evidence is recorded as profile evidence and does not fit
  defaults to the five sites.
- Dual reviews and verifications disposition every finding.
- Final gates pass:
  - `cargo fmt --check`
  - `cargo clippy --workspace --all-targets -- -D warnings`
  - `cargo test --workspace`
  - `cargo deny check`
  - `wctl doc-lint --path docs/work-packages/README.md`
  - applicable contract-focused tests

## HOLD Boundaries

Close as `HOLD`, not complete, if:

- A gridded daily shortwave source/provenance cannot be identified at the
  openWEPP runtime seam without duplicating wepppy concerns.
- The contract amendment cannot reconcile with `SC-CLIMATE-001#INV-CLIMATE-013`.
- The contract amendment cannot reconcile WEPP Chapter 3 `- bmelt` prose with
  the current signed `melt_bmelt_in` implementation/test convention.
- The only path to improvement requires radiation-forcing retuning or per-site
  constants.
- The opt-in path cannot preserve default rollback identity.
- Conservation/routed-melt reconstruction fails or aliases adjacent SWE/depth
  surfaces.

## Subagent Authorization

Subagent authorization: this package explicitly authorizes spawning/delegating
to two read-only review/verification subagents for contract-science review and
implementation-gate review. Expected outputs are concise findings suitable for
`artifacts/review_agent_a.md`, `artifacts/review_agent_b.md`,
`artifacts/verification_agent_a.md`, and
`artifacts/verification_agent_b.md`. Write access is not authorized for
subagents.

## Security Impact Gate

No external network access is required. Do not fetch or redistribute
copyrighted PDFs. Do not add secrets, tokens, or generated credentials. New
runtime selectors must fail closed on invalid values and must not silently
activate modernized melt.

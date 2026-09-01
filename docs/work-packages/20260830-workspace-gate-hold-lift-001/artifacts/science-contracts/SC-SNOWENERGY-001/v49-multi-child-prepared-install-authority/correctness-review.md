# V49 correctness review

Disposition: `APPROVE`

Evidence mode: `Static + Ran`

## Findings

No blocking correctness findings remain.

### Resolved HIGH — exact outer-source authority

`crates/openwepp-hillslope-orchestrator/src/v9_real_consumer_shadow/v10_soil_thermal_v2.rs:2369-2407,2413-2450`

The V49 constructor now independently derives the complete mutually equal
vegetation/LSE/BGC source from both the install candidate and the authenticated
beginning, and refuses unless those exact source IDs match. Installation calls
that constructor again before the atomic mutation. The opaque authority still
retains the exact physical source/target authority, complete authoritative
resident, and complete prepared beginning; the atomic posture independently
validates resident/prepared/accepted custody without reintroducing a
source/predecessor alias or numeric adjacency.

The exact jointly rebased three-owner poisons change the candidate's complete
outer source from 42 to 41 while leaving the authenticated beginning at 42.
Both authority construction and installation with a previously minted exact
authority refuse; full-shadow rollback includes soil bytes, vegetation, LSE,
BGC, and publication history.

### Resolved MEDIUM — shared V48/V49 validation pipeline

`crates/openwepp-hillslope-orchestrator/src/v9_real_consumer_shadow/v10_soil_thermal_v2.rs:2327-2363,2413-2474`

Both authenticated prepared installers now call the private
`validated_authenticated_prepared_accepted_resident_v2` engine for
authoritative-resident validation, accepted-resident/seal construction,
current-resident validation, beginning-or-ending join, and exact-no-op
selection. Their authority reconstruction and atomic posture remain explicitly
separate, preserving V48's predecessor-equals-source rule while admitting only
V49's complete three-domain authority.

## Contract and regression assessment

- The exact R124 source42/resident43/predecessor43/target44 vector executes
  `install_v2_soil_from_authenticated_prepared_beginning_v1`, the production
  helper invoked by both real non-continuation finalizer branches.
- The further source42/resident44/predecessor44/target45 vector proves the
  authority is not a one-child special case.
- Individual authoritative-resident transaction, support, receipt, state,
  layer, latest-accepted custody, and seal poisons refuse with full-shadow
  rollback. Prepared, accepted, seal, and opaque-authority substitution
  matrices remain present.
- Generic missing-authority installation and retained V48 authority stay
  strict. The new path performs no outer-owner rebase, receipt repair, private
  publication, or transaction arithmetic.
- V49 changes transaction custody only. Arithmetic, physics, energy/mass
  ledgers, tolerances, support duration, events, topology, serialization, and
  persistence behavior are unchanged.

## Ran evidence

Independent re-review runs:

- `nix develop -c cargo nextest run -p openwepp-hillslope-orchestrator -E 'test(/v49_/)'`
  — final strengthening rerun Nextest
  `2aa4f6af-170f-4303-b62c-f546faa49d00`, 5/5 passed.
- `nix develop -c cargo nextest run --test snow_terminal_enthalpy_event_numerics_contract -E 'test(/v49_/)'`
  — Nextest `986149c6-666c-4cac-934b-a94022adb2d5`, 2/2 passed.

Reviewed retained corrective evidence:

- Retained V39/V46/V47/V48/V49 runtime: Nextest
  `d1cf7bb1-4da8-4cd3-a48f-34b656a1cecd`, 41/41 passed.
- Complete snow source-contract target: Nextest
  `208ec63c-40e2-4d66-a264-7e852f0cf9fa`, 42/42 passed.
- Persisted restart: Nextest `a90a80f2-81c8-4f94-a7a5-3e5b64c7ae7b`,
  40/40 passed.
- Orchestrator all-target/all-feature check, formatting, diff hygiene,
  anti-evasion, and required-suite guards are recorded green in the V49
  implementation artifact.

## Residual risk

Parent-owned canonical R125 remains pending. This review approves the V49
implementation and its focused/retained contract evidence; it does not
prejudge the canonical one-day disposition.

## Review statement

`APPROVE`: the prior source-custody and duplication findings are corrected.
The current V49 diff conforms to `INV-SNOWENERGY-073` and
`OBL-SNOWENERGY-C-041`, preserves V47/V48 strictness, and has no remaining
correctness blocker before canonical R125.

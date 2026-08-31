# Frozen-litter V3 runtime coordinator

Status: `CANDIDATE COORDINATOR COMPLETE — REAL CONSUMER ADOPTION OPEN`

Evidence mode: `Static` and `Ran`

## Exact implementation boundary

The candidate-only coordinator implements the authenticated chronology

```text
accepted phase-free D/A/F
  -> one bounded litter phase projection
  -> immutable SurfaceLiquidOwnerEnvelopeV2 replacement
  -> one complete LSE V3 phase transaction
  -> current ingress
  -> liquid-only WB14 child
  -> complete SurfaceLiquidCompleteOwnerProjectionV3 + parent SoilThermal V2 join
```

The implementation is isolated in:

- `crates/openwepp-hillslope-orchestrator/src/land_surface_energy_shadow/v3_input_projection.rs`;
- `crates/openwepp-hillslope-orchestrator/src/land_surface_energy_shadow/v3_execution.rs`;
- `crates/openwepp-hillslope-orchestrator/src/land_surface_energy_shadow/v3_rollback.rs`;
- `crates/openwepp-hillslope-orchestrator/src/land_surface_energy_shadow/v3_tests.rs`;
- module-only wiring in `land_surface_energy_shadow/mod.rs`;
- the manifest-authorized crate-private V2-ingress handoff in `direct_runtime.rs`;
- this evidence artifact.

The admitted existing runtime files `multi_tile_runtime.rs`,
`covered_derived_ingress.rs`, `real_hydrology_execution.rs`, and
`covered_v8_owner.rs` were inspected but did not require edits for this
bounded candidate seam. No runner, persistence, active-snow, p61, native
consumer, or legacy V8 path was changed.

## Static invariant disposition

- The input constructor accepts the sealed `V3PhaseFreeCoveredEvaluation`;
  its accepted vapor, post-vapor state, and surface-energy ledger are private
  fields and cannot be independently assembled by production callers.
- Support must be positive, at least exactly `60_000_000_000 ns`, and an
  exact multiple of that floor. The child must be contained by the parent
  support and the coupled WB14 binding.
- Configuration order, OFE/tile identities, beginning surface V2 owner,
  beginning LSE V3 state, predecessor receipt chain, current-ingress identity,
  coupled child identity, parent soil V2 owner, and parent soil restart are
  checked before mutation.
- Each litter row invokes `apply_bounded_litter_phase` exactly once to preseal
  the phase-adjusted owner. `execute_litter_phase_v3` is then invoked exactly
  once with the real beginning and candidate owner digests. Its accepted
  vapor, post-vapor state, and ending phase state are compared bitwise to the
  fixed-final and presealed operands. No same-support constitutive or
  fixed-point solve is repeated.
- Independent owner closure debits positive liquid/ice vapor and freeze/melt
  from the correct phase and credits condensation/deposition and melt/freeze
  to the correct phase. The owner V2 closure validator reconstructs both
  ledgers.
- Current ingress is presented only after the phase-adjusted owner and LSE V3
  ending candidate exist. The actual V2 ingress adapter admits liquid only;
  it carries litter ice bitwise through WB14, and the complete projection
  revalidates that no ice was donated or substituted.
- The complete projection joins the exact ending surface V2 bytes, ordered
  phase receipts, ordered ingress receipts, open WB14 parent bytes, parent
  soil V2 owner/restart, and predecessor/final receipt-chain heads. Canonical
  replay is required before return.
- The rollback snapshot captures exact surface V2 canonical bytes, LSE V3
  canonical JSON, soil V2 owner/restart JSON, and optional WB14 parent bytes;
  the poison vector proves byte-exact unchanged beginnings after rejection.
- Candidate and rollback carriers are not serializable. The coordinator
  persists no iteration, microstep, residual, solver, or diagnostic data.

## Ran

Focused coordinator vectors:

```text
nix develop -c cargo nextest run \
  -p openwepp-hillslope-orchestrator --lib \
  land_surface_energy_shadow::v3_tests --no-fail-fast
```

Result: `PASS`, 7/7; Nextest run
`350d6b08-e3b3-4a2e-824b-e3b637b45094`. The vectors cover frozen and thawing
chronology, phase-specific vapor without double debit, fusion closure, wrong
constant/order/identity, wrong sign, exact rollback, complete surface/soil
join, exact floor/off-grid refusal, and the crate-private V2 ingress binding.

Surface V14/V15 authority filters:

```text
nix develop -c cargo nextest run \
  --test surface_liquid_hydrology_custody_authority_contract \
  -E 'test(~version_14_) or test(~version_15_)' --no-fail-fast
```

Result: `PASS`, 3/3; Nextest run
`5971aad5-8503-4b2f-b10d-bc27ddbe1192`.

LSE V14 authority filters:

```text
nix develop -c cargo nextest run \
  --test land_surface_energy_balance_authority_contract \
  -E 'test(~version_fourteen_)' --no-fail-fast
```

Result: 2/3 passed; Nextest run
`431b27a4-c889-49c6-80ee-967f291e0490`. The sole expected red was
`version_fourteen_requires_p61_and_native_real_consumer_adoption`: the
unchanged p61 source does not yet contain `OPENWEPP_SNOW_FREE_LSE_V3`. This
candidate-only slice makes no real-consumer or package-closure claim.

Existing complete-projection vectors:

```text
nix develop -c cargo nextest run \
  -p openwepp-hillslope-orchestrator -E 'test(~v3_projection)'
```

Result: `PASS`, 4/4; Nextest run
`14628a33-d14e-4be3-bf9f-0ee8396ced7f`.

LSE litter-phase vectors:

```text
nix develop -c cargo nextest run \
  -p openwepp-land-surface-energy --lib litter_phase
```

Result: `PASS`, 14/14; Nextest run
`ed728a45-ac00-4903-ba28-3bba10aea5b3`.

An earlier crate check passed with only shared unrelated V11 warnings. The
terminal retry after the final focused pass was blocked by 23 concurrent
V1/V2 soil-candidate integration errors in the excluded
`v11_covered/open_snow.rs`; no coordinator diagnostic was emitted.

Warnings-denied Clippy for the shared crate remains red on unrelated existing
diagnostics. An exact diagnostic scan for `v3_*`, the new module declaration
lines, and the crate-private re-export returned no owned-path diagnostics.
No lint level was weakened; the two explicit `dead_code` annotations document
the candidate-only seam that the next real-consumer slice must consume.

Ran isolated Rustfmt over all six touched Rust paths: `PASS`.

Ran `git diff --check` over all six touched Rust paths: `PASS`.

Line counts: input projection 345; coordinator 256; rollback 75; focused tests
636. Existing files are `direct_runtime.rs` 992 and
`land_surface_energy_shadow/mod.rs` 1333; only narrow export/module lines were
added to them.

Terminal source SHA-256 values:

```text
v3_input_projection.rs  badaa20d14dd5b5b859b54f76e5e7f218e9051d26d2c054b820a862d8e7d5c48
v3_execution.rs         eb02eb7436377425221aa7d7424dd91898b20937ccb807fbbc76664325fc6a2f
v3_rollback.rs          c75a59f0f89eafa06bf13e21249e37577d730c987ecafa54ad7128dbb4cc6081
v3_tests.rs             ba7d3c51e5b3bf725195413949b713586b1c4e3a6db47b87467f6ddff04d2919
```

## Exact next integration seam

The next manifest-authorized real-consumer slice must construct
`FrozenLitterV3PhaseFreeInput::from_accepted_fixed_final` from the accepted
fixed-final multi-tile D/A/F evaluation, pass the already authenticated V2
surface beginning plus parent-sealed soil V2 owner/restart into
`execute_frozen_litter_v3_runtime`, and publish only the returned complete
projection after the enclosing transaction accepts. It must then run the
unchanged p61 and native-forest workflows and reopen the V14 authority
real-consumer gate. The parent soil restart currently has no public independent
digest-recompute validator, so this seam deliberately accepts only the
parent-sealed restart and verifies all exposed identity/support/chain joins; it
does not invent a surrogate validator.

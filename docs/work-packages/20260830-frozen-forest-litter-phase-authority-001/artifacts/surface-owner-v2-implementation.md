# Surface-owner V2 implementation evidence

Status: `PASS — OWNER SCHEMA SLICE; INTEGRATION HANDOFF OPEN`

Evidence mode: `Ran + Static`

## Implemented bounded slice

The frozen V1 configuration/state structs and their serializers were not
changed. New modules below `direct_runtime/surface_liquid_owner/` implement:

- a canonical model definition binding both terminal-contract digests, the
  frozen parent-model digest, all five retained authority hashes, selected
  constants/formulas/order, and every named refusal;
- explicit V2 configuration with forest-litter depth and bit-exact
  `0.85*rho_w*litter_depth` ice capacity, with no bare-surface ice fields;
- explicit finite nonnegative liquid/liquid-water-equivalent-ice state and
  finite surface enthalpy;
- checked V1-to-V2 migration with exact positive-zero ice and caller-supplied
  enthalpy, without temperature-derived ice, including the public one-way
  `migrate_v1_to_v2` contract handoff and no production downgrade API;
- immutable tagged V1/V2 owner envelopes, deterministic canonical bytes and
  digests, V2 restart framing/round-trip, and typed identity/domain failures;
- candidate replacement that preserves the immutable beginning envelope and
  records complete beginning/attempted hashes on failure;
- a test-only exact-zero-ice V1 representability proof. No production
  downgrade API exists;
- generic independent per-key liquid/ice custody closure. No vapor or phase
  physics and no orchestrator/consumer adoption were implemented in this
  bounded slice.

## Ran evidence

Ran:

```text
nix develop -c cargo nextest run -p openwepp-hillslope-orchestrator \
  --lib surface_liquid_owner::v2_tests --no-fail-fast
```

Result: `PASS`; Nextest run `73a65f16-4489-4623-93fa-6a1137c08e7f`;
8 passed, 0 failed, 1157 skipped. Vectors cover source/constant/capacity
identity, frozen V1 byte preservation, exact-zero migration, explicit-ice
seed, bare-ice rejection, configuration/state/envelope/restart round-trip,
named-wrapper equivalence and poisoned-V1 fail-closed rejection, test-only
downgrade refusal, exact rollback, and independent phase-separated mass
closure.

Ran:

```text
nix develop -c cargo check -p openwepp-hillslope-orchestrator
```

Result: `PASS`, exit 0, after the prospectively authorized exact re-exports
were added in both `direct_runtime.rs` and crate-root `lib.rs`. All legacy
exports remain present.

Ran the broader orchestrator library suite once:

```text
nix develop -c cargo nextest run \
  -p openwepp-hillslope-orchestrator --lib --no-fail-fast
```

Result: shared-suite `FAIL`; Nextest run
`e85f3d0d-0e2a-4081-9629-d56c38a1b53e`. At manual cancellation after
162 seconds, 1133/1142 tests had run: 1123 passed, 10 failed, 3 skipped, and 9
had not run. Reported failures were outside this bounded owner slice, including
V9 real-consumer stack overflows and a snow convergence preimplementation
`CoordinateClosure`; five additional V9 tests were still running after 23--150
seconds. The owner-focused suite above remains the applicable green schema
evidence.

Ran the requested warnings-denied crate Clippy:

```text
nix develop -c cargo clippy -p openwepp-hillslope-orchestrator \
  --lib --no-deps -- -D warnings
```

Result: shared-crate `FAIL`; 785 diagnostics are in concurrent/unowned
modules. A path-filtered rerun initially found two owner diagnostics (a
private-field-name lint and Rust-1.85-incompatible `is_multiple_of` use); both
were corrected. The terminal path-filtered `-D warnings` scan reported zero
diagnostics in `surface_liquid_owner`, its two export blocks, or the new
crate-root export block. No lint suppression was added, and this artifact does
not misstate the full shared-crate Clippy as passing.

Ran the v14 and v15 contract-derived focused vectors after the projection and
named migration handoff were present:

```text
nix develop -c cargo nextest run \
  --test surface_liquid_hydrology_custody_authority_contract \
  version_14_binds_frozen_litter_surface_owner_v2_before_production \
  --no-fail-fast
```

Result: `PASS`; Nextest run `cd135635-23e7-408e-804f-a5048ebb8f72`; 1
passed, 14 skipped.

```text
nix develop -c cargo nextest run \
  --test surface_liquid_hydrology_custody_authority_contract \
  version_15_binds_exact_soil_energy_credit_custody_before_production \
  --no-fail-fast
```

Result: `PASS`; Nextest run `5d74be15-9d75-420a-b095-8d182d465aa5`; 1
passed, 14 skipped. These source-level authority filters do not replace the
successor integration slice's real-consumer evidence.

Ran: explicit `rustfmt` over all six touched Rust files: `PASS`.

Ran: `git diff --check` over the owned Rust paths: `PASS`.

## Integration handoff

The authorized `direct_runtime.rs` and crate-root `lib.rs` export wiring now
re-exports:

```text
SurfaceLiquidConfigurationRecordV2
SurfaceLiquidConfigurationV2
SurfaceLiquidOwnedStateV2
SurfaceLiquidOwnerClosureRecordV2
SurfaceLiquidOwnerEnvelopeV2
SurfaceLiquidOwnerModelDefinitionV2
SurfaceLiquidOwnerRestartV2
SurfaceLiquidOwnerSourceIdentityV2
SurfaceLiquidStateRecordV2
migrate_v1_to_v2
validate_surface_liquid_owner_mass_closure_v2
```

The LSE-to-owner adapter must use the accepted LSE V3 receipt to construct new
records from immutable beginning records, call `try_replace_v2_state`, run the
independent mass-closure validator, wrap the accepted envelope/restart, and
then construct the distinct `SurfaceLiquidCompleteOwnerProjectionV3`. It must
not infer ice, donate current ingress, expose ice to WB14, or use the test-only
zero-ice representation helper.

Line counts at handoff: shared `surface_liquid_owner.rs` 2947 lines (existing
WARN); new `v2.rs` 1487 lines; new `v2_restart.rs` 158 lines; new
`v2_tests.rs` 385 lines. No new file reaches the 2000-line warning threshold.

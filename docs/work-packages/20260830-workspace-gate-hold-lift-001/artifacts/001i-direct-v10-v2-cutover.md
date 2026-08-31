# WGHL-FULL-001I DirectV10 soil-thermal V2 resident cutover

Evidence class: Ran, except where explicitly labeled Static.

## Implemented boundary

- `DirectV9RealConsumerShadow` now contains exactly one
  `DirectSoilThermalResident`: V9 construction admits only V1, while
  `DirectV10RealConsumerShadow::try_new_v2` admits only a prepared native V2
  owner plus its core receipt-free seals.
- V2 residency retains one active owner and exactly one custody posture:
  receipt-free restart/checkpoint seals or the latest accepted predecessor,
  energy-credit receipt, independently supplied expected operand set, and
  orchestrator seals. No V1 snapshot/cache or V2 downgrade exists.
- `DirectSoilThermalReadView` and `DirectSoilThermalCandidate` provide typed
  V1/V2 reads. V2 reads retain the high enthalpy term and exact dyadic carry;
  V1-only accessors return a typed error on V2.
- Successor support joins compare the complete accepted V2 state, carry,
  receipt chain, parent migration identity, and predecessor transaction. They
  intentionally do not require old and newly prepared support envelopes to be
  byte-identical.
- DirectV10 exposes native `prepare_soil_thermal_support_v2`, unpublished
  `advance_soil_thermal_trial_v2`, and clone-only accepted installation. The
  installation validates accepted receipt/seals/canonical bundle and the
  complete-owner transaction join before one assignment.
- Canonical complete-owner bytes select the active resident variant. V1
  serialization remains delegated directly to `SoilThermalSnapshot`; V2 bytes
  contain the V2 owner and custody digests only.
- The unified real-hydrology candidate now retains the pre-ingress thermal
  candidates plus a canonical framed hash before any infiltration mutation.
  Validation joins topology, owner/state identity, unchanged beginning/internal
  operands, and the retained hash without reverse floating-point subtraction.
- Production resident/candidate serialization contains no microstep, solver,
  iteration, or diagnostic field.

Static limitation: this slice does not claim a persisted-restart endpoint,
runner seed, or real WAT5 production endpoint. The existing V1 V8 day solver
continues to refuse a V2 resident rather than constructing a compatibility
snapshot. Native terminal/V11 composition is integrated by the separately
owned terminal slice against the typed candidate/read view defined here.

## Focused evidence

Ran:

```text
cargo test -p openwepp-hillslope-orchestrator --lib \
  direct_v10_soil_thermal_v2_tests -- --nocapture

4 passed; 0 failed; 1,175 filtered out
```

The four cases prove:

1. one receipt-free V2 resident, V1 refusal, and V2-only canonical bytes;
2. a sub-ULP exact carry survives acceptance and is joined into the next
   support's native physical read view;
3. accepted receipt/seal substitution refuses without changing resident bytes;
4. the real `DirectV10RealConsumerShadow::try_new_v2` installs V2 only and a
   poisoned atomic installation preserves all canonical complete-owner bytes.

Ran:

```text
cargo test -p openwepp-hillslope-orchestrator --lib \
  soil_thermal_exact_carry_v2_tests -- --nocapture

6 passed; 0 failed; 1,173 filtered out
```

This includes WAT5 canonical sub-ULP carry, ordered cancellation, source and
seal substitution refusal, top-boundary custody, V1 byte freeze, and V2
configuration admission.

Ran:

```text
cargo test -p openwepp-hillslope-orchestrator --lib \
  land_surface_energy_shadow::raw_boundary_contract_tests::\
unified_ingress_updates_exact_real_receivers_and_preserves_rollback

1 passed; 0 failed; 1,178 filtered out
```

The candidate constructor calls validation after the newly retained
pre-ingress hash is sealed, so this real-receiver test traverses the new custody
path and preserves the existing rollback proof.

## Build and hygiene

Ran:

- `cargo check -p openwepp-hillslope-orchestrator --lib`: PASS. Warnings shown
  are confined to concurrent V33/v11 dormant symbols; this slice emits no
  default-build warning.
- `cargo check -p openwepp-hillslope-orchestrator --all-features`: PASS with
  the same concurrent V33/v11 warnings.
- `cargo fmt --all -- --check`: PASS.
- `git diff --check`: PASS.
- production diagnostic-key scan over the resident, canonical owner, and
  pre-ingress candidate paths: no matches.

The crate-wide warnings-denied Clippy invocation was run but is not green: it
reports 829 pre-existing/concurrent diagnostics across unrelated runtime,
hydrology, V33, and retained V8 modules. The first failures are
`snow_stage3_v11_adaptive_execution.rs` similar-name lint and dormant V33
fixed-point symbols. No clean affected-module Clippy target exists because the
crate is a single library target; this is recorded rather than misreported as
an implementation pass.

## Line counts

The new resident implementation is 575 lines and its focused test module is
282 lines. Large pre-existing include-based owner files remain above package
line-count guidance; this cutover isolates new V2 residency in its own module
instead of extending those files with the implementation body.

# Terminal rejected-pair evidence-correlation authority V4

Status: `IN REVIEW / AST-CENSUS-BOUND / NO SOURCE IMPLEMENTATION AUTHORITY`

Base: `ba7a9bfb42e49d8f0dd7db8084726a8c11e7f22a`. V3 remains an unchanged
reviewed HOLD. V4 retains its accepted ownership, role, pair/floor,
cardinality and binary64 design and replaces only its rejected wire authority.

## Additional findings

- `TDCV3-WIRE-007`: live `framed_sha256` uses `OPENWEPP\0`, version
  `u16(1)`, domain/tag lengths `u16` and value lengths `u32`, big-endian.
- `TDCV3-OWN-008`: V3 A6 mixed provider-generated amounts with terminal-
  transition deposition, sublimation, melt, refreeze and liquid outcomes.
- `TDCV3-OWN-009`: V3 A7 put accepted-event
  `TerminalSnowSoilHeatReceiptV1` inside rejected provider evidence.

V4 places provider-generated values only in carrier evidence, transition
outcomes only in coupling/terminal state and ledger, and excludes the
accepted-event receipt from rejected evidence.

## Census-first authority

Package-local `terminal_v4_census_tool` parses the exact tree with `syn` and
generates `terminal-v4-live-type-census.md`. For every required declaration it
records fully qualified type/module, source path and Git blob SHA, exact AST
fields/types or enum variants/payloads, nested collections/keys, visibility,
owner stage, native validator/digest candidates, replay class, required private
test access and normalized declaration SHA-256. Its guard regenerates and
byte-compares the census. Drift blocks implementation.

The companion exact adapter schema uses only census fields. Nested receipts
are classed as existing native replay, discarded native preimage requiring an
owner-module `#[cfg(test)]` extractor, or complete census-derived adapter. No
semantic adapter field list may be invented later.

## Ownership and correlation

Provider carrier phase owns only carrier evidence,
`TerminalSnowSoilTrialReceiptV1` and WB14 child replay. Coupling iteration owns
flux, preview, hints and convergence. Coupling selection owns ordered iteration
keys and exact selected key without arena mutation. Terminal solver owns pair
position, transition outcomes and ledger. Hydrology-complete joint appears
only in `selected_trial_v4`. Accepted-event
`TerminalSnowSoilHeatReceiptV1` is absent.

Pair positions remain `COARSE/FINE_1/FINE_2`; exact live roles remain
`FULL=0, HALF_1=1, HALF_2=2, RETRY=3, BRACKET_LOWER=4,
BRACKET_UPPER=5, ROOT=6`. Allowed adaptive mappings are COARSE+FULL or RETRY,
FINE_1+HALF_1 and FINE_2+HALF_2. An evaluated rejected pair remains
`REJECT_RETRY`; a subsequent pre-provider admission separately records
`BELOW_CARRIER_DOMAIN` and equal provider counts.

Errors retain `delta=refined-coarse`, canonical component order, live binary64
denominators/scaled values, exact left-fold maximum and first bitwise-equal
diagnostic winner. Raw f64 bits are encoded even when nonfinite; semantic
finiteness is recorded independently and never changes physics.

## Private compilation and failure boundary

A sealed private compile-time mode supplies zero-sized `NoEvidence` to every
unchanged wrapper. `CaptureEvidence` exists only in a crate-private
`#[cfg(test)]` unit path. No externally reachable generic parameter, feature,
runtime input, callback, global/thread-local recorder or internal
`catch_unwind` is allowed. Immutable keys/records travel by ordinary values.
All resolution, encoding, assertions, deliberate failures and artifact I/O
occur after physical result and beginning bytes return and are retained.

## Exact prospective file authority

Fixed-size forwarding chain:

- `snow_stage3_v11_terminal_execution.rs`;
- `hydrology/support_helpers_mod/runoff_reconciliation.rs`;
- `hydrology/support_helpers_mod/runoff_reconciliation/stage3_solver.rs`;
- `hydrology/support_helpers_mod/runoff_reconciliation/stage3_solver/support.rs`;
- `hydrology/support_helpers_mod/runoff_reconciliation/stage3_solver/evaluation.rs`;
- `hydrology/support_helpers_mod/runoff_reconciliation/stage3_solver/terminal_event.rs`.

Census-required private `#[cfg(test)]` replay/adapter owner modules:

- `snow_stage3_v11_precipitation.rs`;
- `v11_covered/carrier_phase.rs`, `v11_covered/receipt_sets.rs`,
  `v11_covered/physical_outcome_ledger.rs`;
- `land_surface_energy_shadow/covered_v8_owner.rs`;
- `v9_real_consumer_shadow.rs`;
- external LSE `owner_envelope.rs` and `solver.rs`;
- coupled-time `identity.rs` and `support.rs` where the census requires private
  preimage access.

Owner helpers may return exact test-only native preimage bytes or census
adapters only. They cannot change non-test visibility, native digest,
validation, physics, control flow or API.

## Evidence and gate

NoEvidence versus CaptureEvidence must preserve exact
`Stage3(TerminalNumerics(BelowCarrierDomain))`, calls/order/support, owners,
state, clocks, cursors, receipts, parcels, `last_*` and outputs. Every positive
call is at least 600 ms; the final admission makes zero calls; the selected
1.875/0.9375/0.9375 records reconstruct all receipts and the known energy
difference; no terminal parcel or terminal-liquid ingress exists.

Before review only package-local authority tooling, generated census, schema,
guards and docs may change. Freeze authority, adapter, census, generator and
guard hashes and obtain independent numerical/evidence/cardinality and
Rust/schema/custody/privacy reviews. Either HOLD stops before source edits.
Two GO-to-evidence reviews authorize only a subsequent exact-file diagnostic
implementation intent, not the seam, final v21 review or any temporal/Batch/
event/receiver/restart/runner/Child-3/cutover work.

# Terminal diagnostic correlation V4 exact adapter schema

Status: `IN REVIEW / CENSUS-BOUND / NO IMPLEMENTATION AUTHORITY`

Normative live declarations are generated in
`terminal-v4-live-type-census.md`. Its guarded AST fingerprints freeze every
field, Rust type, enum payload, collection and visibility. A fingerprint
mismatch blocks capture. No declaration-order, equivalent-field, digest-only
or implementation-time field selection is allowed.

## Exact framing and primitive encoding

Every diagnostic digest uses the live `framed_sha256` preimage:

```text
OPENWEPP\0
version:u16be = 1
domain_len:u16be; domain bytes
for each field in listed order:
  tag_len:u16be; tag bytes; value_len:u32be; value bytes
```

Integers retain the census Rust width and are big-endian; signed integers use
two's-complement bits. `ModelTimeNs` is `u128`. `usize` is forbidden. `f64` is
raw `to_bits():u64be`, including nonfinite values and signed zero, followed by
`semantic_finite:u8`. Boolean is `u8`. Strings/bytes are `length:u32be` plus
bytes. Options are `present:u8` plus payload. Sequences are `count:u32be`, then
`item_len:u32be` plus item. Maps use ascending Rust `Ord` key order. Tuples,
arrays and census fields use index/declaration order. Digests are 32 bytes.
Enums use `variant:u16be` in census declaration order, then exact payload.

## Nested live-type classification

The census replay class is normative:

1. Native replay bytes: embed exact type tag, native domain, native digest,
   byte length and bytes; validate natively.
2. Native digest preimage exists but bytes are discarded: a later reviewed
   implementation may add a private `#[cfg(test)]` extractor in the census
   owner module returning those exact preimage bytes.
3. No native wire: encode every census field recursively under domain
   `terminal-v4-adapter:<fully-qualified-type>`.

`TerminalSnowSoilTrialReceiptV1` is provider evidence. Exact WB14 child replay
is provider evidence. `TerminalSnowSoilHeatReceiptV1` is accepted-event-only
and absent from all rejected records. Provider records contain no hydrology-
complete joint. Parent WB14 replay is absent.

## Complete new-record wires

Each schema below lists exact framed tags in order. Its digest is SHA-256 of
the full framed preimage and is stored externally, never self-hashed.

`carrier_key_v4`, domain `terminal-carrier-key-v4`:
`schema:u16=4`, `prefix:Digest32`, `support_start:u128`, `support_end:u128`,
`live_role:u16`, `attempt:u32`, `coupling:u32`, `begin_joint:Digest32`,
`carrier_end_joint:Digest32`, `provider_call:u64`, `arena_index:u64`,
`record_digest:Digest32`.

`carrier_phase_v4`, domain `terminal-carrier-phase-v4`:
`schema:u16=4`, `key`, census adapters `CoveredTerminalTrialRequestV1`,
`CoveredProbeChildIdentityV1`, and provider-owned projection of
`CoveredCarrierPhaseResultV1`; `wb14_child_native_replay`; explicit
`terminal_parcel_absent:bool`, `terminal_liquid_hydrology_ingress:f64`,
`terminal_liquid_wb14_credit:f64`, `terminal_liquid_surface_ingress:f64` with
finiteness flags. Provider projection contains carrier evidence and
`TerminalSnowSoilTrialReceiptV1`, but excludes terminal flux/preview,
transition-generated outcomes, hydrology-complete joint and
`TerminalSnowSoilHeatReceiptV1`.

`coupling_iteration_v4`, domain `terminal-coupling-iteration-v4`:
`schema:u16=1`, `prefix`, support, `live_role:u16`, `attempt:u32`,
`coupling:u32`, `carrier_key`, census `TerminalFluxIntegral`, census
`TerminalState` preview, optional incoming and exact outgoing
`CoveredTerminalEndingSnowHintV1`; then for ice, liquid, cold content and
surface temperature: `delta`, `tolerance`, `absolute_delta` as f64 plus flags,
and `within_tolerance:bool`; `combined_converged:bool`.

`coupling_selection_v4`, domain `terminal-coupling-selection-v4`:
`schema:u16=1`, `prefix`, support, live role, attempt,
`iteration_count:u32`, ordered length-framed `iteration_keys`,
`selected_iteration_key`, `selected_carrier_key`, `selected_coupling:u32`,
`returned_flux_digest`, `returned_preview_digest`,
`returned_carrier_joint_digest`, `selected_converged:bool`. Selected occurs
once, predecessors are nonconverged, and no successor exists.

`selected_trial_v4`, domain `terminal-selected-trial-v4`:
`schema:u16=4`, `prefix`, `pair_position:Option<u16>`, `live_role:u16`,
`attempt:u32`, support, `selection_digest`, census beginning/ending
`TerminalState`, census `TerminalLedger`, `carrier_end_joint_digest`, and the
only hydrology-complete `CoveredTerminalJointTrialStateV1` adapter.

`pair_component_error_v4`, domain `terminal-pair-component-error-v4`:
`schema:u16=1`, `component:u16` (`ice=0,liquid=1,cold=2,complete_energy=3,
unallocated_energy=4`), `coarse`, `refined`, `delta`, `abs_tol`, `rel_tol`,
`denominator`, `scaled` as f64 plus flags. Exact binary64 equations are
`delta=refined-coarse`, `denominator=abs_tol+rel_tol*max(abs(coarse),
abs(refined))`, `scaled=abs(delta)/denominator`.

`pair_decision_v4`, domain `terminal-pair-decision-v4`:
`schema:u16=4`, `prefix`, `pair_ordinal:u32`, `coarse_trial_digest`,
`fine_1_trial_digest`, `fine_2_trial_digest`, `refined_state_digest`,
`refined_ledger_digest`, five ordered component-error digests,
`maximum_scaled:f64+flag`, `diagnostic_winner:u16`, `decision:u16`
(`ACCEPT=0,REJECT_RETRY=1`), `current_duration:f64+flag`,
`proposed_next:Option<f64+flag>`. Maximum is exact left fold; winner is first
bitwise-equal canonical component and never physical input.

`trial_admission_v4`, domain `terminal-trial-admission-v4`:
`schema:u16=1`, `prefix`, `admission_ordinal:u32`, proposed support,
`proposed_duration`, `required_half_duration`, `minimum_duration` as f64 plus
flags, `decision:u16` (`ADMIT=0,BELOW_CARRIER_DOMAIN=1,
DOMAIN_OR_NONFINITE=2`), `error:Option<typed_error_v4>`,
`provider_calls_before:u64`, `provider_calls_after:u64`.

`typed_error_v4`, domain `terminal-typed-error-v4`:
`schema:u16=1`, `outer_variant:u16` in census
`DirectSnowStage3EvaluationError` order, followed by its exact census payload.
`TerminalNumerics` nests the census `SnowTerminalNumericsFailure` tag;
`TerminalCustody` stores exact UTF-8. Boxed variants recursively use their
census adapter. Target final bytes are variant 2 containing
`BelowCarrierDomain` variant 2.

`rejected_prefix_v4`, domain `terminal-rejected-prefix-v4`:
`schema:u16=4`, `parent`, `prefix`; beginning snapshots for owner set, joint,
clock, provider cursor/call log, receipts, parcels, cursor and each lexically
discovered `last_*`, encoded as ascending `(qualified_name,bytes)`;
`sequence_count:u32` and ordered length-framed members tagged
`ADMISSION=0,NONPAIR_TRIAL=1,PAIR=2`; final `typed_error_v4`; ordered carrier
arena, iteration and selection record sequences; post-return snapshots in the
same names/order; `unchanged:bool`.

The final member is a zero-call `BELOW_CARRIER_DOMAIN` admission. The prior
pair remains `REJECT_RETRY`. Exactly one 1.875/0.9375/0.9375 triple resolves
complete receipts and reconstructs `27.2131278332233 J m^-2`.

# Verification Agent A — Time, Numerics, And Chronology

Status: `PASS`

Date: 2026-08-20

Verified exact checkpoint: `cf1fc326d76e9e4c0cbd4c6e1b94febf263878e0`

Verified reviewed authority content: `c53adab0a91c0ecbe853c884bfe05591826441c5`

Evidence class: `Static + Ran + adversarial executable verification`

## Independence and review closure

I independently inspected both review histories, both finding dispositions, and
the exact reviewed-to-checkpoint delta. Review A and Review B terminate in PASS;
every accepted `A-001..007` and `V11-AUTH-B-001..005` finding is represented in
the corrected canonical authority or frozen executable evidence. No waiver or
residual finding remains. The delta from `c53adab0a` to `cf1fc326d` contains
only contract-cycle review, disposition, and contract-reference records; it
does not alter reviewed authority.

## Executed evidence

- Ran strict Binding Exposure Index lint on both amended contracts: PASS, 15
  consolidated vegetation rows and 4 transaction rows.
- Ran science-contract unit-compliance lint on both contracts: PASS, no
  findings.
- Ran `reference_calculator.py`: PASS, exactly 46/46 frozen migration and
  chronology cases (16 accepted, 29 typed rejects, 1 atomic abort).
- Ran `semantic_schema_validator.py`: PASS, valid authenticated transaction,
  36/36 typed poisons rejected, before/after-event fresh restore equivalent to
  uninterrupted continuation, seven owners installed atomically, one buffered
  publication released, 13 authenticated receipts, and exact zero ending water
  bits.
- Ran `nix develop --command cargo test --test
  c3_woody_v11_authority_contract`: PASS, 5/5 tests.
- Parsed all 11 authority JSON artifacts. Draft 2020-12 meta-schema checks pass
  for all five schemas; independently constructed owner-manifest,
  parent-candidate, and both before/after-event restart instances validate with
  the explicit local schema registry.
- Recomputed every `v10-full-surface-binding.json` digest: configuration,
  state, V10 model definition, and recursive compatibility ledger all match.
  The ledger is omission-failing, recursively classifies nonidentity leaves,
  narrowly permits successor identity differences, and requires unknown-leaf
  and one-leaf-per-root mutation poisons during implementation.
- Ran `git diff --check`: PASS. The checkpoint was clean before this named
  verification artifact was written.

## Protected-boundary verification

The execution-base-to-checkpoint diff contains no production Rust edit. V10
`config.rs`, `v10_state.rs`, `water_phase.rs`, and `persistent_phase.rs` Git
object IDs exactly match execution base `d59ba76f7`; their recorded SHA-256
bindings also match. The complete `openwepp-persisted-restart-v1` and
`openwepp-coupled-time` trees exactly match the execution base, protecting
DirectV10 restart V1 and coupled-time restart V2 bytes.

## Verdict

`PASS / SC-VEGETATION-001 Version 15 V11 segmented-support authority is
verified for promotion and production implementation.`

This verification authorizes the preimplementation authority checkpoint only;
it does not claim the later Rust implementation, actual-consumer compatibility,
or terminal package gates.

## Regression verification — accepted-slab prefix cardinality

Exact checkpoint: `a7bfbbac57bd2661948ce516cd18fc34e5bd98a8`

Status: `PASS`

Evidence class: `Static + Ran + adversarial executable verification`

The correction requires `accepted_slab_receipts.len()` to equal the authenticated
`next_slab_ordinal` before prefix identity and receipt validation. The new
`checkpoint_missing_slab` poison independently removes the accepted prefix and
is rejected as `V11-RESTART`; a checkpoint can no longer omit accepted slab
chronology while retaining cursor and staged-state claims.

Regression evidence: strict BEI and unit compliance PASS for both contracts;
reference calculator 46/46 PASS; semantic validator 37/37 poisons PASS with
before/after-event restore equivalence, seven-owner atomic installation, and
delayed publication intact; authority contract tests 5/5 PASS; `git diff
--check` PASS. The vegetation, persisted-restart, and coupled-time production
trees remain byte-identical to execution base `d59ba76f7`.

Verdict: `PASS / prior Verification A remains valid at a7bfbbac5; accepted-slab
restart-prefix cardinality is now explicitly authenticated.`

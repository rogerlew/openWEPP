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

## Restart V2 amendment Verification A — Versions 18/7

Date: 2026-08-20

Status: `PASS`

Verified exact checkpoint: `6c74d866dba776189ec9bc6b8bd62901aecf4917`

Verified tree: `2295f9525ab54ba03eb951be253b3db27eba0300`

Reviewed authority content: `5918d4dbdfd0a7641d16b1f5f2040289c9893788`

Evidence class: `Static + Ran + independent adversarial verification`

Both amendment review histories terminate in PASS and explicitly close every
prior Review A and Review B restart finding without waiver. The checkpoint
delta after reviewed authority contains review records only.

Executed evidence: restart V2 canonical continuation PASS with complete
seven-owner byte equality; 54/54 declared poisons reject; 20 direct custody
probes reject; canonical restart instance validates and its Draft 2020-12
schema meta-validates; semantic authority PASS; strict BEI PASS (15 rows);
unit-compliance PASS; Rust authority suite PASS 6/6; diff hygiene PASS.

Protected-boundary object IDs match execution base `d59ba76f7` exactly:
persisted-restart tree `8c4585371f1e818eeb0d8e93d89e339b88059444`,
coupled-time tree `9206afef1574aa6051ea801560cd1514203d8531`,
V10 `config.rs` `2dd3079a4e7ede6610d954b61e563bf2c600b05b`,
`v10_state.rs` `0bbb9b3bd9242eb0d477a3731965f2de84fb177b`,
`water_phase.rs` `eec7e80b6ef7883ce0cc3e69500eeb48f5448d8d`,
and `persistent_phase.rs` `450ea5b63ec61482389ffe59420c306c8b5629ee`.
Thus DirectV10 restart V1 and coupled-time V2 remain byte-identical. The
`5dd5c1a9b..6c74d866d` authority range contains no production Rust or Cargo
edit.

Verdict: `PASS / SC-VEGETATION-001 Version 18 Restart V2 amendment is
independently verified for promotion.`

## Final disposition regression — checkpoint `1a3aa9d7953d03b2be7d7b5ddce3ce4ba9d66087`

Status: `PASS`

Verified tree: `54656a34bd391b3498726b7f11da3001051d17dc`

The disposition now explicitly enumerates and accepts without waiver every
Restart V2 family: `RA-001..004`, `TA-001`, `FA-001..003`, `RVA-001..004`,
`RVC-A-001..004`, `RVF-A-001..002`, and
`V11-RESTART-V2-B-001..006`. Their stated closures agree with the final Review
A/B PASS records and Version 18 authority.

The contracts, executable reference, and 54-poison population have exact Git
object identity with the prior verified checkpoint `6c74d866d`; only
disposition and verification records changed. As an independent regression
sample, the reference again passed complete seven-owner continuation and
54/54 poisons, the Rust authority suite passed 6/6, and range diff hygiene
passed. The prior technical Verification A PASS remains exact and no new
finding is introduced.

### Exact-checkpoint record — `081a0169634ff30f916f8af5642e5a3c03a4f922`

`PASS` (tree `ed55688c0b8a40124e191a4f7d4963f2ffe841bc`). The
transaction disposition addition only enumerates already closed finding IDs;
all reviewed contracts, schemas, reference/poisons, and authority test objects
remain exact. Prior Verification A remains unchanged.

## Sequential-debit amendment Verification A

Status: `PASS`

Verified checkpoint: `f71c36a7c186a95474f29f6470fb2b980f1311cd`

Verified tree: `da7eae817703614bc3bc0f2a32ee822f73b9cd15`

Reviewed authority: `3065c209c7d5d203a2a06fca793dc8cbc340e26e`

Both independent reviews terminate in PASS with no finding or waiver. Version
19 exactly defines accepted-order staged subtraction as authoritative owner
custody and the accepted-order exact-`+0.0` addition fold as diagnostic receipt
identity only; regrouped parent-minus-total bits cannot replace or gate the
sequential ending.

Independent evidence: reference 49/49 PASS; authority suite 7/7 PASS; strict
BEI 15 rows PASS; unit compliance PASS; direct nonassociation and reordered-
operand probes PASS; diff hygiene PASS. The authority range contains no
production Rust/Cargo edit. Protected persisted-restart and coupled-time trees
remain `8c4585371f1e818eeb0d8e93d89e339b88059444` and
`9206afef1574aa6051ea801560cd1514203d8531`; Restart V2 schema and poison
objects remain `3229ef601742d2acb403538ff7456305511edebe` and
`a9c78cf4d656ba58a88b993ffcd22381201bb964`. Protected wires are unchanged.

Verdict: `PASS / SC-VEGETATION-001 Version 19 sequential-debit amendment is
independently verified for promotion.`

## Resource-custody amendment Verification A

Status: `PASS`

Verified checkpoint: `38492e60a39d6b8d1fbfd676f3a8874c3ba9c031`

Verified tree: `53fa141b0fd6cee9907d6d5a83dca5df4cbe0427`

Reviewed authority: `e11b6c15e3daf5daaf9d4143e7ca361a4fde1a87`

Every Review A and Review B resource-custody history terminates in an explicit
PASS with all earlier HOLD findings superseded and no waiver. Version 24 closes
occupancy/shared-owner separation, aggregate authorization, one complete
candidate per owner/slab, exact V2 prefix composition, typed flux identity and
exact-once linkage, and independently rooted uninterrupted-versus-restored
suffix equality.

Independent rerun evidence: Restart V2 54/54 PASS; Restart V3 13/13 PASS with
complete suffix digest
`0b2ff7b0182c756d6d706016b164459d5d55e99e148bd776aca1c0d1d6341096`;
Rust authority 8/8 PASS; strict BEI 15 rows PASS; scoped unit compliance,
schema parsing, and range diff hygiene PASS. Direct review evidence confirms
restored-only prefix forgery, graft/support/cursor/terminal, identity/link,
candidate, and invalid flux mapping forms reject while complete seven-owner,
state, resource/material, event, reduction, and publication chronology agrees.

Protected Restart V2 objects remain pinned: reference
`13f3d009221a60cc2af094103255c5d8c3be2dbee657bb87144b2fee476bbf7c`,
schema `af9314c3f1abd70c40b849c6f466046e3c5e519583a837eefca9edbf43d02441`,
and poisons `fa5ae93f8b8e109b851f37946070bff71b5f5182b5df818c80f0d4de9990ad34`.
The resource-authority range contains no production or Cargo path. V3
reference/schema/poison hashes are respectively
`5951dcc088760cbbc2843799ae00f1170da55043879ac7940f3d438df8cac930`,
`0f8c77a67dcecdfef0f683d0d5dcda017782a550cd8fc2630613f0a8081d6efe`,
and `d3e97f2a39ce33707cf30f7289ccfdf59af6ac3ed1971aaebc2812c9bc0428ff`.

Verdict: `PASS / SC-VEGETATION-001 Version 24 resource-custody amendment is
independently verified for promotion at 38492e60a`.

### Disposition regression — `8a35ed97f139531cc7ce5ad9d975c252c5c42108`

`PASS` (tree `8a59c0f5e1c47768c5616837a0d98bdaeb1d2f83`). The
canonical disposition now enumerates and accepts all four titled Review A
checkpoint findings and `V11-RESOURCE-CUSTODY-B-001..004`, maps each to the
terminal correction at `e11b6c15e`, and records closure without waiver. Review
A and B both terminate PASS.

Only disposition and verification records changed from the verified
`38492e60a` checkpoint. Contract, V2/V3 schema, reference, poison, and authority
test objects remain exact; the previously recorded V2 54/54, V3 13/13,
authority 8/8, independent suffix, BEI, unit, schema, protected-wire, and
nonproduction-range PASS therefore remains unchanged. No residual finding is
introduced.

### LSE positive-support addendum — `3b7d40648a5543bf8e8a3936cd2b383657a9c9f2`

The V11/LSE technical dependency checks pass: oracle 15/15, schema and frozen
population consistency PASS, and protected V10/coupled-time/DirectV10 wire
paths are unchanged. Addendum verdict is **FAIL pending diff hygiene** because
the canonical LSE Review A artifact contains trailing whitespace at lines 3
and 6. No V11 authority-semantic failure was found.

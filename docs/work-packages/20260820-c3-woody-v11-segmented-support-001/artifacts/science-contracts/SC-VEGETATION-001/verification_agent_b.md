# Authority Verification B — Custody, Wire, And Restart

Status: **FAIL**

Date: 2026-08-20

Verified exact commit: `cf1fc326d76e9e4c0cbd4c6e1b94febf263878e0`

Reviewed authority commit:
`c53adab0a91c0ecbe853c884bfe05591826441c5`

Evidence class: `Static + Ran + independent adversarial execution`

Scope: transaction custody, exact owner manifest, wire closure, restart
serialization and continuation, review-finding disposition, V10 surface
binding, and protected predecessor wires. Production Rust was not reviewed or
edited.

## Mechanical and executable evidence

- Strict Binding Exposure Index lint: PASS for both contracts (15 and 4
  consolidated rows).
- Science-contract unit compliance: PASS for both contracts.
- JSON parse over every package JSON artifact: PASS.
- Independent chronology calculator: PASS, 46/46 results.
- Independent semantic validator: PASS for its frozen population, 36/36
  poisons; valid commit installs exactly seven owners and both before/after
  event checkpoints report equivalent continuation.
- `cargo test --test c3_woody_v11_authority_contract`: PASS, 5/5.
- `git diff --check`: PASS.
- Exact seven-owner manifest: PASS for `vegetation`, `snow`,
  `land_surface_energy`, `surface_liquid`, `hydrology`, `bgc`, and
  `soil_thermal` in fixed order and cardinality.
- Full V10 binding: PASS. The configuration source, state source, model
  definition, and recursive compatibility ledger recompute to their frozen
  hashes.
- Protected DirectV10 restart artifacts: PASS at
  `c29e28c4...2ad1`, `c041ab59...0ddd`, and `e01e2a93...d47c9`.
- Protected coupled-time restart V2: PASS at
  `96003072...8e8b29`; the protected base-to-verification diff is empty.

## Finding

### `V11-VERIFY-B-001` — BLOCKER — accepted slab chronology can be omitted from restart

The additive restart wire persists `accepted_slab_receipts` and
`next_slab_ordinal`, but `restore_and_continue` never requires the accepted
slab receipt count to equal the ordinal. An independent direct forgery removed
the sole accepted slab receipt from a canonical `after_event` checkpoint while
leaving `next_slab_ordinal == 1`. Restore accepted the checkpoint and returned
the same committed result.

The bypass occurs because the receipt-prefix loop accepts an empty slab prefix,
then `replay_prefix` reconstructs slab 0 directly from the trusted complete
parent candidate rather than from the persisted accepted receipt set. This
admits contradictory persisted chronology and means restart equivalence does
not authenticate every accepted support receipt. The frozen 36-poison suite
contains event/resource/material/rejected-state restart cases but no accepted
slab omission poison, so its PASS does not expose this defect.

Required correction:

1. require `len(accepted_slab_receipts) == next_slab_ordinal` and enforce the
   exact accepted prefix, uniqueness, order, identity, payload, and support
   joins;
2. reconstruct staged chronology from the authenticated persisted slab prefix,
   not an unconditionally selected full-candidate slab;
3. add direct restart poisons for slab omission, duplication, reorder, payload
   forgery, and cursor/ordinal mismatch;
4. rerun both reviews if the authority changes, then rerun both independent
   verifications at one exact corrected commit.

The broader direct-forgery matrix rejected the other 28 tested mutations,
including authority/configuration/sequence/cursor/participant/manifest,
beginning and staged owner, staged state, event/resource/material receipt,
scheduled-once, resource state, reduction, pending publication, outbox, and
parent receipt forgeries. That does not waive the accepted slab-prefix blocker.

## Verdict

**FAIL.** `INV-VEGETATION-127` and the V11 restart-equivalence obligation are
not closed while an accepted slab receipt can be deleted without rejection.
Do not promote the authority or begin production Rust from this checkpoint.

## Corrected-checkpoint re-verification

Status: **PASS**

Verified exact commit: `a7bfbbac57bd2661948ce516cd18fc34e5bd98a8`

Evidence class: `Static + Ran + independent adversarial execution`

This section supersedes the FAIL verdict above while retaining it as the
finding audit trail. `V11-VERIFY-B-001` is closed: restart now requires
`len(accepted_slab_receipts) == next_slab_ordinal`, and the exact direct probe
that removed the sole accepted slab receipt from an `after_event` checkpoint
rejects with `V11-RESTART`.

Corrected-checkpoint evidence:

- strict BEI lint: PASS for both contracts (15 and 4 rows);
- science-contract unit lint: PASS for both contracts;
- all package JSON artifacts parse: PASS;
- independent chronology calculator: PASS, 46/46;
- semantic validator: PASS, 37/37 poisons, including
  `checkpoint_missing_slab`;
- independent direct restart-forgery matrix: PASS, 29/29 rejected;
- Rust authority contract: PASS, 5/5;
- exact seven-owner manifest and full V10 four-surface binding: PASS;
- DirectV10 V1 protected hashes and coupled-time restart V2 hash: unchanged;
- protected base-to-checkpoint diff and `git diff --check`: PASS.

No residual custody, wire, restart, serialization, or protected-boundary
finding was identified.

**Superseding verdict: PASS.** Verification B authorizes authority promotion
and the exact preimplementation authority checkpoint from the custody/wire/
restart perspective, subject to the separate Verification A PASS and package
promotion procedure. This does not constitute production implementation or
terminal package acceptance.

# Verification Agent A — Ownership, Restart, And Atomicity

Status: `PASS`

Date: 2026-08-20

Verified exact checkpoint: `cf1fc326d76e9e4c0cbd4c6e1b94febf263878e0`

Verified reviewed authority content: `c53adab0a91c0ecbe853c884bfe05591826441c5`

Evidence class: `Static + Ran + adversarial executable verification`

## Review and disposition audit

Both review histories terminate in PASS at the reviewed authority commit. The
transaction findings covering lifecycle/profile integration, typed ordered
water/NH4/NO3 custody, exact seven-owner manifest, live beginning-owner joins,
one-shot commit, restart authentication, suffix continuation, rollback, and
publication ordering are all closed. Both disposition files say no waiver was
accepted, and inspection found no unresolved finding hidden by the historical
HOLD sections.

## Independent transaction verification

- Strict BEI and unit-compliance gates: PASS for both contracts.
- Independent chronology population: 46/46 PASS.
- Independent semantic transaction model: PASS with 36/36 adversarial poisons.
  The population rejects forged receipt/schema/base64/digest/body data,
  reordered or duplicate owners, wrong participants and event custody,
  overbooking and wrong ending bits, material reorder, all duration aliases,
  rejected-attempt leakage, stale clock, partial/reordered/double commit, late
  failure, premature publication, forged live/ending owners, restart replay,
  and forged cursor/state/reduction/scheduled/outbox/parent-beginning/material
  checkpoint custody.
- Before-event and after-event `OPENWEPP_C3_WOODY_V11_RESTART_V1` instances
  validate against the closed schema and independently continue to bytes equal
  to uninterrupted completion. Parent abort restores owners, clock,
  publication buffer, and consumed-set state.
- The owner manifest and complete parent candidate validate against their
  Draft 2020-12 schemas with local references resolved explicitly. The semantic
  validator independently reconstructs canonical payload bytes, framed receipt
  identities, ordered resource endings, event custody, candidate ending owners,
  parent receipt, live owner joins, and consuming publication order.
- `cargo test --test c3_woody_v11_authority_contract`: 5/5 PASS.
- All 11 JSON artifacts parse; all five schema meta-validations pass; all four
  frozen full-surface source/ledger hashes match; `git diff --check` passes.

## Boundary audit

No production Rust file differs from execution base `d59ba76f7`. The
`openwepp-persisted-restart-v1` tree and `openwepp-coupled-time` tree Git object
IDs are byte-identical to that base, so DirectV10 V1 and coupled-time V2 wire
protections remain intact. The only reviewed-authority-to-checkpoint changes
are review/disposition/reference evidence.

## Verdict

`PASS / SC-VEGETATIONTRANSACTION-001 Version 4 segmented parent-transaction
authority is verified for promotion and production implementation.`

This is a preimplementation authority verification, not implementation or
terminal package acceptance.

## Regression verification — accepted-slab prefix cardinality

Exact checkpoint: `a7bfbbac57bd2661948ce516cd18fc34e5bd98a8`

Status: `PASS`

Evidence class: `Static + Ran + adversarial executable verification`

The restart admission path now joins accepted slab receipt cardinality exactly
to `next_slab_ordinal`, then retains the existing ordered identity-prefix and
closed receipt-payload checks. The added `checkpoint_missing_slab` adversary is
rejected as `V11-RESTART`. This closes the omission alias without weakening
event, resource, material, scheduled, reduction, publication, owner, or cursor
authentication.

Regression evidence: strict BEI/unit gates PASS for both contracts; 46/46
chronology cases PASS; the expanded semantic population rejects 37/37 poisons
and preserves exact before/after-event continuation, one atomic seven-owner
install, rollback, and delayed publication; authority tests PASS 5/5; diff
hygiene PASS. All protected production trees retain their execution-base Git
object IDs.

Verdict: `PASS / SC-VEGETATIONTRANSACTION-001 Version 4 remains verified at
a7bfbbac5 with accepted-slab prefix omission rejected.`

## Restart V2 amendment Verification A — Versions 18/7

Date: 2026-08-20

Status: `PASS`

Verified exact checkpoint: `6c74d866dba776189ec9bc6b8bd62901aecf4917`

Verified tree: `2295f9525ab54ba03eb951be253b3db27eba0300`

Reviewed authority content: `5918d4dbdfd0a7641d16b1f5f2040289c9893788`

Both review histories end in PASS and disposition every restart/transaction
finding without waiver. Independent execution confirms canonical complete
continuation, 54/54 typed poison rejection, and 20 direct custody-probe
rejections. Restart schema instance/meta validation, semantic authority,
strict BEI (4 rows), unit compliance, authority tests 6/6, and diff hygiene all
PASS.

The verified transaction preserves exact segment predecessor/support chains,
terminal seven-owner equality, resource/material/event custody, scheduled-once
uniqueness, joint parent sequences, reductions, publication, and durable
outbox identities. Protected object IDs match execution base `d59ba76f7`:
persisted-restart `8c4585371f1e818eeb0d8e93d89e339b88059444` and
coupled-time `9206afef1574aa6051ea801560cd1514203d8531`; the four
protected V10 source IDs also match exactly. DirectV10 restart V1 is therefore
immutable, and the `5dd5c1a9b..6c74d866d` amendment range contains no
production Rust or Cargo edit.

Verdict: `PASS / SC-VEGETATIONTRANSACTION-001 Version 7 Restart V2 amendment
is independently verified for promotion.`

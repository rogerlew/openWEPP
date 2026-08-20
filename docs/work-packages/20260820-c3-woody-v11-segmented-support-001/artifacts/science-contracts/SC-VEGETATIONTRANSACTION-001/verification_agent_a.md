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

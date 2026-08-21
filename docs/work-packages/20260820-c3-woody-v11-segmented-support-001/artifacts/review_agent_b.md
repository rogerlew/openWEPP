# Implementation Review B — Resource Custody And Atomicity

## LSE positive-support implementation addendum — 2026-08-20

Verdict: **HOLD**.

The precheck is correctly placed before candidate cloning and nonlinear work;
the minimum, one-tick-below rollback, and structural 1 ns focused tests pass.
LSE focused Clippy with warnings denied and scoped diff hygiene pass. No
`openwepp-coupled-time`, `openwepp-persisted-restart-v1`, or vegetation/V10
implementation wire is changed.

Two release-blocking findings remain.

### `V11-LSE-IMPL-B-001` — production receipt digest does not implement the KAT

`support.rs::canonical_digest` first converts the receipt to
`serde_json::Value` and then serializes that map. The workspace uses
`serde_json` without `preserve_order`, so object members are emitted in sorted
map order. The released independent oracle and baseline KAT hash the receipt's
frozen declaration/contract field order. No production test compares an
admitted Rust receipt with
`lse-support-admissibility-baseline.json`/`ddbcf496...ba0f0`; the only LSE unit
test checks decimal/hex predicates. Consequently Rust can self-validate its own
different digest while failing the canonical authority.

Required correction: construct the exact frozen preimage directly (or serialize
an explicit digest-body struct in the frozen order), add the baseline KAT exact-
byte/digest test, canonical decode/revalidate tests, and every rehashed poison.

### `V11-LSE-IMPL-B-002` — the accepted slab does not carry or persist receipt custody

The actual V11 stack stores the receipt only in mutable diagnostic field
`last_support_receipt`. `V11ImportedV10SegmentOutput`, the accepted segment
candidate/checkpoint, and additive restart contain no support receipt. A second
segment overwrites the field, and restart cannot reconstruct or authenticate
the receipt chronology. This does not implement the released requirements that
each accepted slab carries the sealed receipt and that rollback/restart
validation consumes it.

Required correction: add the receipt to the accepted V11 segment/candidate
chronology, authenticate its parent/segment/slab/support/state joins during
acceptance, persist the ordered accepted receipts additively in the production
V11 restart target, and prove fresh restore/replay poisons. A debug accessor is
not custody.

Focused evidence executed:

- LSE support unit test: 1/1 PASS;
- actual minimum-support consumer: PASS;
- one-tick-below parent/live-stack rollback: PASS;
- structural coupled-time 1 ns identity: PASS;
- LSE all-target focused Clippy `-D warnings`: PASS;
- scoped diff hygiene and protected-tree diff: PASS.

No waiver is recommended. Both findings must close before implementation
verification or authority-conformance claims.

## LSE support receipt closure re-review — 2026-08-20

Verdict: **PASS**. This supersedes the preceding HOLD.

- `V11-LSE-IMPL-B-001` is CLOSED. Rust now serializes the declared receipt
  struct directly in the frozen field order, replaces only the digest payload
  with the authority-required empty string, prefixes the exact domain, and
  matches the amended soil-thermal-bound baseline KAT
  `419058014c851ee854a7f432e458306c67cb2f4c640dfdfd0893e521429f54ae`.
  A digest-valid segment forgery separates from the baseline.
- `V11-LSE-IMPL-B-002` is CLOSED. Every accepted V11 segment and checkpoint
  carries `V11LseSupportReceiptEnvelope` with exact canonical JSON bytes,
  canonical-bytes digest, receipt digest, support/identity/configuration/LSE and
  soil-thermal beginning joins. Acceptance and restore authenticate the closed
  receipt, reject replay/graft, and preserve the ordered envelopes. Restart V3
  admits the embedded complete checkpoint and exposes only the exact restored
  receipt projections; it does not mint a parallel identity.

Closure gates:

- LSE support KAT/forgery tests: 2/2 PASS;
- vegetation V11 chronology/replay/restore tests: 8/8 PASS, full suite reported
  272/272 PASS by the owner implementation run;
- persisted-restart library: 26/26 PASS;
- actual minimum support and below-minimum rollback: PASS;
- focused LSE/vegetation/persisted-restart Clippy with warnings denied: PASS;
- protected coupled-time, immutable V10, and DirectV10 restart V1 surfaces:
  unchanged.

Implementation Review B authorizes implementation verification. Production
activation/cutover remains outside this package.

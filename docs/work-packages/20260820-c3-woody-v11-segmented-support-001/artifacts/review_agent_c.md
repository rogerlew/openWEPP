# Implementation Review C — Restart, Wire, And Consumer

Date: `2026-08-20`

Verdict: **HOLD**.

Reviewed the production `LseSupportAdmissibilityReceiptV1`, its actual V11
pre-Newton admission, full/unequal/minimum/below-minimum behavior, rollback,
and restart custody against `SC-LANDSURFACEENERGY-001` version 6.

## Passing behavior

- Full-support physical compatibility passes without selecting a shortened or
  floored duration.
- `600+1200`, `1200+600`, three unequal supports, and forcing-order-sensitive
  execution pass through the actual stack.
- Exact `600000000 ns` support executes the unchanged nonlinear stack and
  produces seven ending owners.
- `599999999 ns` rejects as `SupportBelowMinimum` before envelope/Newton work.
- The rejected-attempt test leaves parent staged state and the live execution
  stack unchanged with no staged ending.
- Coupled-time `1 ns` structural support remains admitted independently of the
  LSE physical-domain rejection.
- The receipt uses the coupled-time conversion operation exactly,
  lower-case/decimal canonical fields, a domain-separated digest, and joins
  parent/segment/slab/ordinal/absolute support/configuration/LSE state/policies.

Command evidence:

```text
cargo test -p openwepp-land-surface-energy support -- --nocapture
3 passed

cargo test -p openwepp-hillslope-orchestrator v11_ -- --nocapture
6 passed; 1 ignored

cargo clippy -p openwepp-land-surface-energy \
  -p openwepp-hillslope-orchestrator --lib --no-deps -- -D warnings
PASS

python3 artifacts/lse_support_admissibility_reference.py
15/15 PASS

Draft 2020-12 schema meta-validation + baseline instance validation
PASS
```

## Findings

### `LSE-SUPPORT-C-001` — beginning soil-owner identity is not sealed

**Release blocker.** The contract binds the beginning LSE/soil state identity.
The Rust receipt has only `beginning_state_sha256`, populated from
`self.beginning.inner.lse_state.state_sha256`. It contains no beginning
soil-thermal owner/state digest, although the actual coupled solve consumes
that staged owner. Two attempts with identical LSE state/support but different
soil-thermal beginnings therefore produce the same admissibility receipt.

Required correction: add the canonical beginning soil-thermal owner/state
identity named by the released wire, cover it in the digest and validation,
and add an alias-separating forgery test. If the authority intended LSE state
only, correct the contract/schema through another authority cycle rather than
silently narrowing production.

### `LSE-SUPPORT-C-002` — the sealed receipt is an executor side channel

**Release blocker.** On success the receipt is assigned only to
`DirectV11RealConsumerStack.last_support_receipt`. It is absent from
`V11ImportedV10SegmentOutput`, `V11AcceptedSegmentCandidate`, accepted-segment
checkpoint/restart state, complete-owner candidates, and the final parent
candidate. Dropping the executor drops the only receipt. Consequently the
accepted physical solve does not carry the receipt through atomic acceptance,
and a mid-parent restart cannot prove that an accepted prefix carried the
sealed admission receipt represented by the authority oracle.

Required correction: give the receipt one canonical candidate/checkpoint
custody path, validate it during segment acceptance, persist accepted prefix
receipts additively in V11 restart V3 (without changing protected V1/V2
bytes), and prove uninterrupted/restored suffix equality plus replay poison.
If the receipt is intentionally attempt-local only, amend the authority and
restart oracle prospectively; the current implementation cannot claim
`INV-LANDSURFACEENERGY-117` as written.

### `LSE-SUPPORT-C-003` — production receipt validation lacks KAT/forgery tests

**Release blocker until C-001/C-002 select the final wire.** The only crate
unit test in `support.rs` checks decimal/hex helper domains. The independent
15-case Python oracle and baseline schema pass, but no Rust test asserts the
production baseline receipt digest or independently mutates parent, segment,
slab, ordinal, support bounds, duration bits, configuration, LSE/soil state,
policy digests, minimum, and receipt digest. Current consumer tests inspect
only requested duration and ordinal.

Required correction: compare the Rust-produced receipt to the frozen baseline
byte/digest KAT and run the closed forgery population against production
`validate`; use an explicit typed-error assertion for below-minimum rather than
`#[should_panic]` through a helper.

## Disposition

The minimum-support precheck itself is correctly placed and the focused
physical/rollback behavior is sound. Release remains **HOLD** because the
receipt does not bind every consumed beginning owner and is not retained by
the accepted transaction/restart chronology. No production files were edited
during this review.

## Superseding implementation Review C — PASS

This review supersedes the HOLD above after direct inspection and execution of
the corrected production tree. `LSE-SUPPORT-C-001`, `LSE-SUPPORT-C-002`, and
`LSE-SUPPORT-C-003` are closed without waiver.

- The production receipt now seals both the beginning LSE state and beginning
  soil-thermal state, and its frozen Rust KAT matches receipt digest
  `419058014c851ee854a7f432e458306c67cb2f4c640dfdfd0893e521429f54ae`.
- The actual default-off V11 consumer serializes the admitted typed receipt
  directly into `V11ImportedV10SegmentOutput`; acceptance carries the exact
  canonical bytes and digest into the accepted candidate and parent checkpoint.
- Execute, accept, and restore join the receipt's LSE configuration, beginning
  LSE state, and beginning soil-thermal state to the canonical staged owner
  payloads. Digest-valid owner-state forgeries, altered canonical bytes,
  support/lineage grafts, and receipt replay reject.
- Restart V3 embeds and authenticates the accepted V11 checkpoint, retains the
  ordered receipt envelopes, and reconstructs the same validated chronology;
  protected V1/V2 restart wires remain unchanged.
- The below-minimum actual-stack tests assert the exact typed
  `SupportBelowMinimum { requested_ns: 599999999, minimum_ns: 600000000 }`
  error and prove parent state, staged ending, and live beginning remain
  unchanged. Exact-minimum, full-support, unequal-support, forcing-order, and
  one-parent-finalization cases pass through the real constitutive stack.

Independent rerun evidence:

```text
cargo test -p openwepp-land-surface-energy support::tests
2 passed

cargo test -p openwepp-vegetation v11
9 passed

cargo test -p openwepp-persisted-restart-v1
26 passed

cargo test -p openwepp-hillslope-orchestrator v11_
6 passed; 1 ignored evidence sweep

python3 artifacts/lse_support_admissibility_reference.py
15/15 PASS

cargo clippy -p openwepp-land-surface-energy -p openwepp-vegetation \
  -p openwepp-persisted-restart-v1 -p openwepp-hillslope-orchestrator \
  --lib --no-deps -- -D warnings
PASS

cargo fmt --check
git diff --check
PASS
```

Final Implementation Review C verdict: **PASS**.

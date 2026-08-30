# WGHL-FULL-001I persisted-restart V2

Evidence mode: `Static + Ran`

## Scope and identity

This increment adds the distinct
`OPENWEPP_DIRECT_V10_REAL_CONSUMER_CHECKPOINT_V2` path without modifying any
existing V1 persisted DTO, tag, digest formula, admission routine, host, or
transaction. Work began from stable LSE core `51a9eb7fd` and declared package
source `6dce0706c`.

Owned source SHA-256 identities:

| Source | SHA-256 |
|---|---|
| `src/lib.rs` | `857e24e3b96e8230f97a523934e01dafbba37ade8897329ac8bf5ca54d1b4a4b` |
| `src/soil_thermal_v2.rs` | `d84ddb101bd1e7dd2a8c8744a773cfba1836add8c144d7343e78d8c36e196c44` |
| `src/scientific_owners_v2.rs` | `6d4427827c441aa9f67d7d5430218153d21500dbeb161f28821918e22fedf85b` |
| `src/checkpoint_v2.rs` | `655ef7fdcabb00dd4c67bd84303fa87f156652e520f147f155a952aec09c4f98` |
| `src/projection_v2.rs` | `eab1848176f8f1b1a04f31d0c4d7de1cc8df48b3cdbc11f3690f292f3564ef2b` |
| `src/host_v2.rs` | `445577e3b9761becac78a3575aa0cd87a67811a6790e4b05062315f5427b5928` |
| `src/transaction_v2.rs` | `5e0ee1d4a809748d5937e834c840a252beea50d3256b839ed2c4fd0e731c228d` |
| `src/v2_tests.rs` | `54c76b3324cf438c65453b5e127f585775f9dc36e01db2c2ca0e96a4df894d10` |

## Implemented custody

- `SoilThermalOwnerStateRestartV2` stores type-tagged canonical native frames
  for the complete owner envelope, restart seal, checkpoint seal, optional
  credit-beginning envelope, latest typed credit receipt, native independently
  sourced expected set, and native orchestrator seals. Every frame binds its
  exact canonical bytes and SHA-256 and is decoded, canonical-reencoded, and
  typed-validated on admission.
- The accepted operands and temperature projections are also persisted as a
  separately reconstructable list plus independent digests. Admission compares
  those lists to the native expected set and passes that external set—not a
  list flattened from the receipt—to native independent receipt validation.
- `from_accepted_candidate` consumes the orchestrator's authoritative expected
  set/candidate/seals and reruns full receipt, owner, restart, checkpoint, and
  orchestrator-seal validation before emitting persisted bytes.
- V1-to-V2 migration restores the V1 native snapshot, uses the canonical LSE
  migration, copies every temperature/high-term bit, installs canonical exact
  zero carry, and binds the parent V1 state and restart payload digests. The
  no-credit migration exposes a typed native seal constructor/validator
  boundary because the available native seal API requires an accepted credit
  receipt; persisted restart does not invent that missing seal hash preimage.
- V2-to-V1 downgrade always returns `DowngradeProhibited`, including zero carry.
- `ScientificOwnerStateSetV2` and `CompleteCommittedOwnerStateV2` reuse all
  unaffected V1 DTOs and substitute exactly one authoritative V2 soil owner.
- Checkpoint V2 has distinct phase/context/admission types, parent-V1 checkpoint
  binding, exact scheduler checks, owner/topology joins, and canonical payload
  sealing. V1 admission cannot reinterpret V2 bytes.
- The isolated V2 host validates a complete candidate before one assignment;
  every refusal preserves the prior host byte-equivalent state. The prepared
  V2 transaction accepts only a fully sealed successor with the exact soil
  predecessor chain and exposes checkpoint, restore, abort, and replay refusal
  boundaries without installing a second V1 soil owner.

The canonical WAT5 high term
`-34315.42154113602 J m^-2` and nonzero exact remainder
`(-1,"1dc319224e55f",-109)` survive canonical frame roundtrip exactly. No
microstep, iteration, solver, diagnostic, or carry-diagnostic metadata is
serialized by any production V2 module.

## Ran gates

| Gate | Result |
|---|---|
| `cargo nextest run -p openwepp-persisted-restart-v1 --all-features -E 'test(v2_tests)' --no-fail-fast` | PASS, run `8582eee3-d65b-4fdc-95b3-88ca1ca44c65`, 7/7; log SHA-256 `1a0acf4d8b3161bc9d188b2d7a57c84dc5aed4ceea7529facc64640823f306ab` |
| `cargo nextest run -p openwepp-persisted-restart-v1 --no-fail-fast` | PASS, run `e7f2d295-1bf7-4178-81a1-a3c025a90f7a`, 28/28; log SHA-256 `2992e9c93ae8cdc034528af28e6e85351d752d9f1c3a9fcaf1b90978ef4e5820` |
| `cargo clippy -p openwepp-persisted-restart-v1 --all-targets --all-features --no-deps -- -D warnings` | PASS; log SHA-256 `4fe893b3754ae5d32b41bdbd4239a56f98e4c5bad76bacff8b40941502316b45` |
| `cargo check -p openwepp-persisted-restart-v1` | PASS |
| `cargo fmt --all -- --check` | PASS |
| owned-path `git diff --check` | PASS |
| production-key scan for `microstep`, `iteration`, `solver`, `diagnostic`, `carry_diagnostic` | PASS, no match |

The first attempted all-feature full crate run compiled all new V2 tests but was
interrupted after 38 passes and four concurrent pre-existing Stage-3 snow
restart failures entered the unfinished v32 vapor active-set path. This is not
used as passing evidence. Exact all-feature full-crate rerun remains a parent
terminal-gate obligation after the concurrently owned snow implementation is
stable. The normal full persisted crate and exact V2 fixture suite both pass.

## Test coverage and line counts

Focused tests cover canonical framing and re-encoding, wrong type and byte
poison, V1 byte preservation, V1-to-V2 high-bit/temperature-bit/zero-carry
migration, unconditional downgrade refusal, WAT5 nonzero-carry roundtrip,
native accepted receipt/expected-set/seal replay, checkpoint admission,
atomic-host rollback, exact V1/V2 run/topology identity joins, synthetic split
immediately before and after a credit, replay refusal, cross-version schema
refusal, omission/reordering, and serialized-key exclusion. Native
LSE/orchestrator suites independently cover
operand omission, duplication, reorder, substitution, noncanonical exact
dyadics, overflow, stale receipts, exact closure, and byte-exact candidate
rollback; persisted admission invokes those same validators rather than
duplicating them.

New production modules total 1,323 lines; `v2_tests.rs` is 504 lines. Every
file is below the 2,000-line warning threshold.

## Remaining real-integration dependency

The schema and orchestrator-independent APIs are complete. Real DirectV10
projection/installation still requires the parent integration slice to supply
the accepted `SoilThermalAcceptedCandidateV2` from the physical-source builder
and to install the resulting `CompleteCommittedOwnerStateV2` as the sole soil
owner. The legacy shadow exposes only its V1 soil snapshot, so this increment
does not fabricate a V2 runtime installer or downgrade the exact owner.

Disposition: `SCHEMA IMPLEMENTED / FOCUSED PASS / REAL INSTALLATION PENDING`.

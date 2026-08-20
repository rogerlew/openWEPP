# Review_agent_a

Status: HOLD

Evidence mode: Static + Ran (non-Rust hygiene only) + Not run (Rust gates unavailable)

Reviewed exact commit: `42f88d644cf4f3c86bae0b9ae066505684699704`

Role: final independent time/numerics semantic-conformance review. This does
not replace the Phase-2A canonical contract review.

Verdict: **HOLD**. The crate is a useful API sketch and the reference chronology
demonstrates the intended happy path, but it does not enforce multiple binding
`SC-COUPLEDTIME-001` invariants. Several public APIs trust caller-provided
identity, ledger, acceptance, and commit assertions, so an adopter can bypass
the single-clock/accepted-only/atomic-publication authority. These are
closure-blocking implementation gaps, not reasons to weaken or defer the
contract.

## Findings

| Finding ID | Severity | Finding | Evidence | Recommended disposition |
| --- | --- | --- | --- | --- |
| CTA-IMPL-A-001 | Critical | Slab acceptance does not authenticate the coupled candidate or its joins. `accepted_slab_id`, `receipt_id`, `global_ledger_digest`, and every candidate `ledger_digest` are unused; no parent/segment/constraint identity is carried; `ledgers_closed: bool` substitutes for local/global ledger reconstruction. Consequently arbitrary IDs and unrelated ledgers can be installed as an accepted slab. | `crates/openwepp-coupled-time/src/transaction.rs:17-25,28-72`; the only rejection test toggles `ledgers_closed` at `crates/openwepp-coupled-time/tests/authority.rs:53-72`. | accepted; implement canonical candidate/receipt reconstruction and all owner, support, participant, flux, ledger, and beginning/ending owner-set joins before mutation; add alias/poison tests. |
| CTA-IMPL-A-002 | Critical | Event authority is not enforced. `beginning_owner_set_digest`, `event_id`, `event_context_digest`, `ledger_digest`, class/source custody, admitted mutation set, and event ordinal identity are not validated. Any owner may be mutated. Repeated same-tick calls are not bounded by the required cycle key/256-transition limit, and equal-time transition chaining is not checked. `order_events` only sorts and can miss duplicate event IDs when equal IDs are nonadjacent after sorting by other keys. | `crates/openwepp-coupled-time/src/event.rs:18-31,34-99`. | accepted; implement canonical event identity/receipt and beginning/ending joins, admitted mutation sets, deterministic same-tick chain validation, persisted replay/cycle state, and all required start/interior/end/failure/restart poison vectors against Rust. |
| CTA-IMPL-A-003 | Critical | Accepted-only reduction and publication can be bypassed by callers. `fold_accepted` accepts any finite value and arbitrary receipt (including duplicates or rejected/precommit attempts) without joining it to an accepted slab. `PublicationOutboxV1::commit` trusts a caller-supplied `parent_complete: bool`, so `commit(true)` publishes before clock/owner finalization. `finalize_parent` is repeatable and increments an arbitrary caller-supplied sequence unrelated to the staged parent transaction. | `crates/openwepp-coupled-time/src/transaction.rs:75-100,126-143`; `crates/openwepp-coupled-time/src/restart.rs:31-45`; reference consumer manually supplies the booleans/order at `crates/openwepp-hillslope-orchestrator/src/coupled_time_reference.rs:165-185`. | accepted; make reduction folding consume authenticated accepted receipts exactly once, make publication consume a committed parent receipt rather than a boolean, and bind exactly-once finalization to the staged parent sequence/identity. Add the mandated wrong-answer and rollback/restart tests against production Rust. |
| CTA-IMPL-A-004 | High | Constraint and retry semantics are incomplete. Equal-time receipt compatibility is checked only among non-adaptive facts, allowing an adaptive constraint with mismatched calendar/forcing lineage. Zero-step admission is a global boolean rather than a join to a named pending event. Retry control never validates `accepted_state_digest`, has no policy/checkpoint join or minimum-step/attempt limit, and permits endless identical proposals merely by changing an arbitrary digest. No production attempt-ID constructor exists. | `crates/openwepp-coupled-time/src/constraint.rs:27-75,78-102`; `crates/openwepp-coupled-time/src/identity.rs:22-29`. | accepted; enforce complete constraint lineage and deterministic ordered receipt, typed pending-event identity, accepted-root/policy joins, bounded progress/minimum step, and canonical attempt construction. |
| CTA-IMPL-A-005 | High | Canonical time/wire and identity authority is not represented by the public serialized types. `ModelTimeNs(pub u128)` and public `TimeSupport` fields derive ordinary Serde, permitting noncanonical numeric JSON and deserialization of `start >= end`; `duration_ns` then subtracts without a checked boundary. `framed_sha256` accepts arbitrary domains/tags/field lists and does not enforce closed V1 schemas or NFC, while no canonical constructors exist for parent, transaction, segment, slab, attempt, event, or receipt IDs. | `crates/openwepp-coupled-time/src/support.rs:7-33`; `crates/openwepp-coupled-time/src/identity.rs:31-59`; `crates/openwepp-coupled-time/src/clock.rs:36-63`. | accepted; use validated/canonical wire adapters and private invariants, validate on every deserialize/admission boundary, and provide closed typed identity constructors with Rust KAT/poison coverage. |
| CTA-IMPL-A-006 | High | Restart validation is too shallow for equivalent continuation. It checks only three top-level identities and cursor bounds; it does not validate owner/state digests, active participants/regime, ordinals, accepted event/scheduled receipts, controller checkpoint digest/history, reduction receipts, publication/outbox state, or their chronology and identity joins. Malformed public `TimeSupport` can also enter via deserialization. | `crates/openwepp-coupled-time/src/restart.rs:60-90`; `crates/openwepp-coupled-time/src/clock.rs:15-32`. | accepted; implement full semantic restart validation and uninterrupted-equivalence/poison tests against the Rust restore path. |
| CTA-IMPL-A-007 | High | The claimed independent vector population does not compare production Rust results to the independent oracle. The integration test runs Python and compares its output back to vector `expected` values/hash; crate tests cover only five narrow cases. Most of the 108 advertised cases therefore prove artifact self-consistency, not Rust conformance. | `tests/integration/coupled_time_authority_contract.rs:172-213`; `crates/openwepp-coupled-time/tests/authority.rs:15-125`; `artifacts/gate-results.md` reports `108/108` as independent-reference evidence. | accepted; add a separately authored Rust vector executor and compare Rust results to the frozen oracle for support, conversion, identities, constraints, retries, events, participant transitions, restart, reduction, publication, and poisons. Reclassify current 108/108 evidence as oracle/artifact validation until then. |
| CTA-IMPL-A-008 | Medium | Scheduled-once custody is only duplicate-receipt storage. It does not authenticate operation identity/boundary, bind the receipt to parent chronology, or prevent the same scheduled operation from replaying under a different arbitrary receipt. | `crates/openwepp-coupled-time/src/clock.rs:71-76`. | accepted; key scheduled execution by canonical operation/boundary identity and test different-receipt replay and restart behavior. |

## Gate and governance audit

- Ran `wc -l` over every touched Rust file: maximum 292 lines
  (`tests/integration/coupled_time_authority_contract.rs`), so the recorded
  line-count `PASS` is legitimate; no 2,000-line warning or 3,000-line block.
- Ran `git diff --check`: PASS at review time.
- Attempted `cargo nextest run -p openwepp-coupled-time`; the review environment
  has no `cargo` executable, so no Rust gate was rerun. The package's prior
  focused command-level results are not contradicted, but their tests do not
  establish the missing semantic conformance above.
- `gate-results.md` correctly leaves broad runner/cargo-deny/terminal gates
  pending. Those gates cannot cure the authority gaps; final implementation
  review and verification must remain incomplete until the findings are fixed
  and focused semantic/vector gates are rerun.
- The reference consumer is genuinely downstream of the public crate API, but
  it demonstrates caller discipline rather than bypass resistance. It does not
  exercise `reduce_constraints`, canonical identity construction, semantic
  restart poison rejection, authenticated event mutation, or authenticated
  accepted-only publication/reduction.

## Required closure

Disposition and correct CTA-IMPL-A-001 through CTA-IMPL-A-008, rerun the
production-Rust versus independent-oracle population, rerun focused crate and
orchestrator gates, then obtain a fresh independent implementation review.
Until then, do not mark the implementation candidate or package complete.

## FINAL RE-REVIEW — candidate `0bbd96d0ac4b593d2f7cf3ff46f990fdb8142145`

Status: **HOLD**

Evidence mode: Static + Ran

Role: independent time/numerics semantic-conformance re-review against
`SC-COUPLEDTIME-001` contract version 2. Production files were not edited.

The candidate closes substantial portions of the first review: validated time
wire types, typed identity constructors, authenticated restart chronology,
accepted-receipt reductions, durable outbox states, and the uninterrupted versus
restored reference chronology are now present. The focused build and test gates
pass. The following remaining defects prevent semantic release.

| Finding ID | Severity | Finding | Evidence | Required disposition |
| --- | --- | --- | --- | --- |
| CTA-IMPL-A2-001 | Critical | Public `Deserialize` implementations recreate authority candidates without running their constructors, while acceptance does not reconstruct all closed identities. A forged `CoupledSlabCandidateV1` can supply a self-consistent support/begin/end/ledger subset but an unrelated segment, constraint digest, slab ID, or embedded receipt; `accept_slab` never checks those fields against one another. `EventTransitionV1` and `EventQueueV1` expose the same deserialization bypass, while `apply_event` does not reconstruct the event or receipt identities. This defeats the claimed capability boundary. | `transaction.rs:114-128,350-380`; `event.rs:55-69,236-241,288-322`; all are publicly exported by `lib.rs`. | Remove unvalidated candidate deserialization or implement validating deserialization through the canonical constructors. At every acceptance boundary reconstruct and compare every segment/slab/event/receipt/clock/ledger join. Add forged-JSON poison tests. |
| CTA-IMPL-A2-002 | Critical | The parent receipt hashes the first slab **receipt ID** as `begin_owner_set`, not the parent's beginning complete-owner-set digest. Therefore every emitted parent receipt has the wrong canonical identity. In addition, `ParentCommitCandidateV1` carries no parent/clock binding and `commit_parent` checks only `complete`/`committed`; a candidate constructed from completed parent A can be committed into completed parent B, installing A's sequence and publication receipt into B. | `restart.rs:205-210,220-248,172-179,313-334`. | Hash `clock.begin_owner_set_digest`. Bind the candidate to the exact parent transaction, interval, accepted clock, ending owner set, and current sequence, and revalidate those joins atomically in `commit_parent`; add cross-parent/cross-clock poison tests. |
| CTA-IMPL-A2-003 | Critical | The required deterministic queue of multiple zero-duration events at one tick cannot execute. Every event in a queue is constructed from the same current `clock.event_ordinal` and beginning owner set. Applying the first increments/changes both; the second then necessarily fails the ordinal/beginning join. The reference consumer tests only one event, so the mandated two-events-at-one-tick case remains unproved. | `event.rs:74-76,203-224,243-284,292-320`; reference consumer `coupled_time_reference.rs:148-170`. | Represent queued event proposals separately and authenticate each transition sequentially from the preceding accepted event state/ordinal, or otherwise construct the closed queue with chained joins. Add two-event success, incompatible chain, precedence, 256-limit, and restart-between-event tests. |
| CTA-IMPL-A2-004 | High | Physical segment identity is derived with support `[accepted_until,parent_end)` for every slab. Before an interior event this falsely identifies the snow-covered segment as extending to parent end, and after more than one slab in a segment its start changes with the accepted cursor, so slab 0 and slab 1 do not share one segment identity. This contradicts a maximal physical regime segment with stable identity and exact support. | `transaction.rs:140-151`; the reference consumer constructs the same moving segment identity at `coupled_time_reference.rs:90-92,134-140`. | Persist the admitted segment's fixed start/end and stable `SegmentId`; slabs reference that identity until an event transition admits the successor segment. Add multi-slab-before-event identity and exact segment coverage tests. |
| CTA-IMPL-A2-005 | High | Constraint reduction returns only one winning constraint and slab receipts bind only that constraint digest. The authority requirement that every coincident constraint remain in a deterministic selected-boundary receipt is therefore not implemented or restartable. The zero-step argument also proves only that *some* `EventId` exists; it does not join the constraint to that pending event. | `constraint.rs:95-141`; `transaction.rs:153-160,228-230,288-290`. | Return an authenticated reduction receipt containing the complete precedence-sorted coincident set and pending-event join; bind it into attempt/slab identity and restart chronology. Add coalesced hard/adaptive/output and wrong-event zero-step poisons. |
| CTA-IMPL-A2-006 | High | Event conservation/custody is under-specified in production. The event API accepts only two equal digests and derives a ledger hash from one digest, with no typed transfer entries, units, amounts, operand lineage, or proof that `source_owner_id` belongs to the complete owner set. This cannot establish the contract's independently closing conservation-sensitive transfer ledger. | `event.rs:74-90,105-146`; compare slab `LedgerEntryV1` in `transaction.rs:8-32`. | Use authenticated event ledger entries with units and operand lineage, require the source/mutation custody joins, and add debit/credit alias-separation and non-owner-source poisons. |

### Focused gates run at the exact candidate

- `nix develop --command cargo fmt --all -- --check`: PASS.
- `nix develop --command cargo check -p openwepp-coupled-time -p openwepp-hillslope-orchestrator`: PASS.
- `nix develop --command cargo clippy -p openwepp-coupled-time --all-targets -- -D warnings`: PASS.
- `nix develop --command cargo nextest run -p openwepp-coupled-time`: PASS, 8/8.
- `nix develop --command cargo test -p openwepp-hillslope-orchestrator coupled_time_reference --lib`: PASS, 3/3 (724 filtered).

Passing gates establish build hygiene and the implemented happy paths; they do
not cover the six authority gaps above. Correct and disposition
`CTA-IMPL-A2-001` through `CTA-IMPL-A2-006`, add the named poison and multi-event
tests, then obtain another independent re-review before release.

## TERMINAL RE-REVIEW — candidate `bb8fdc7eba26dac2dcc95d981a33daee645231e5`

Status: **HOLD**

Evidence mode: Static + Ran

Role: terminal independent time/numerics semantic-conformance review against
the latest approved `SC-COUPLEDTIME-001` V2 amendments. No production files
were edited.

The correction substantially hardens the implementation and closes five of the
six prior finding families. One prior finding remains partially open, and two
new restart-equivalence defects were found during the terminal chronology
audit.

### Prior finding closure

| Prior finding | Disposition | Evidence |
| --- | --- | --- |
| `CTA-IMPL-A2-001` candidate deserialization/bypass | **CLOSED** | Authority candidates, constraints, clocks, owners, event receipts, and reductions no longer derive public `Deserialize`; slab/event acceptance reconstructs the complete candidate and compares exact equality (`transaction.rs:337-357`, `event.rs:342-368`). Canonical restart remains the validating deserialization boundary. |
| `CTA-IMPL-A2-002` parent receipt and cross-parent commit | **CLOSED** | `derive_parent_receipt` uses `clock.begin_owner_set_digest`; the commit consumes the clock, reconstructs the candidate from that exact clock, and compares all candidate identities before returning one durable clock+commit object (`restart.rs:213-262,338-368`). |
| `CTA-IMPL-A2-003` same-tick event chaining | **CLOSED** | The queue now stores ordered proposals and constructs each authenticated transition from the current accepted clock after the preceding event. The two-event chain test passes (`event.rs:277-338`; `tests/authority.rs:151-181`). |
| `CTA-IMPL-A2-004` moving/inexact segment identity | **CLOSED** | The clock retains fixed active segment start/end/ID, an explicit pre-slab boundary admission updates it, candidates require that stable ID and cannot cross its end, and restart reconstructs the active segment ID (`clock.rs:174-207`; `transaction.rs:133-150`; `restart.rs:640-655`). |
| `CTA-IMPL-A2-005` coincident constraint receipt / event join | **PARTIALLY OPEN** | The reducer now retains a sorted coincident set in an authenticated receipt and slab identity binds its digest. However a zero-step receipt accepts any caller-supplied `Some(EventId)`; there is no join to an admitted `EventProposalV1`/queue entry at the same parent, tick, class, source, and context. The test explicitly demonstrates acceptance of an arbitrary `EventId::from_digest(d(31))` (`constraint.rs:152-225`; `tests/authority.rs:213-239`). |
| `CTA-IMPL-A2-006` event conservation/custody | **CLOSED** | Event transitions require the source to be a complete-set owner and use ordered `LedgerEntryV1` values with units and operand-lineage digests; transition acceptance reconstructs the ledger and entire event candidate (`event.rs:117-134,180-191,342-368`). |

### New terminal findings

| Finding ID | Severity | Finding | Evidence | Required disposition |
| --- | --- | --- | --- | --- |
| CTA-IMPL-A3-001 | High | Restore selects `accepted_clock_digest` from the last event whenever any event exists, even when later accepted slabs follow that event. For the core chronology `slab -> event -> slab`, a committed or late-parent restore therefore retains the event-end clock rather than the final slab-end clock. This violates exact continuation state and can poison any subsequent identity rooted in the accepted clock. | `restart.rs:761-765` uses `events.last().map_or_else(...)` instead of selecting the chronologically terminal receipt. The reference chronology itself has an event at tick 60 followed by a slab ending at 100 (`coupled_time_reference.rs:148-220`), but its mid-parent restart occurs immediately after the event and does not expose the later-restore defect. | Restore the terminal clock digest from the merged slab/event chronology already validated by `validate_authenticated_chronology`, or compare terminal ticks/ordering explicitly. Add restart after an event followed by one or more slabs and assert exact accepted-clock/next-candidate identity equality. |
| CTA-IMPL-A3-002 | High | An empty diagnostic maximum does not round-trip. Serialization writes `0.0` bits when `maximum == None`, and restoration always constructs `maximum: Some(value)`. A checkpoint before the first accepted operand therefore invents an accepted zero. Subsequent all-negative accepted observations incorrectly reduce to zero rather than their true maximum. | `restart.rs:490-496` uses `maximum.unwrap_or(0.0)`; `restart.rs:713-727` always restores `Some(value)`. The existing canonical restart test serializes an empty reduction but checks only admission, not state equivalence (`tests/authority.rs:242-260`). | Encode reduction occupancy explicitly (nullable value or accepted-count-governed absence), reject inconsistent occupancy/receipt lists, and add `None` round-trip plus empty-restart-then-negative-observation tests. |

### Focused gates run at the exact candidate

- `nix develop --command cargo fmt --all -- --check`: PASS.
- `nix develop --command cargo check -p openwepp-coupled-time -p openwepp-hillslope-orchestrator`: PASS.
- `nix develop --command cargo clippy -p openwepp-coupled-time --all-targets -- -D warnings`: PASS.
- `nix develop --command cargo nextest run -p openwepp-coupled-time`: PASS, 10/10.
- `nix develop --command cargo test -p openwepp-hillslope-orchestrator coupled_time_reference --lib`: PASS, 3/3 (724 filtered).

### Verdict

**HOLD.** Close the admitted-event identity join in `CTA-IMPL-A2-005` and the
two restart-equivalence defects `CTA-IMPL-A3-001` and `CTA-IMPL-A3-002`, add the
named regression tests, and rerun terminal review. These are bounded authority
corrections; the overall architecture and the other prior closures are sound.

## FINAL VERDICT — candidate `f608dbcb49fc6e76af8917816f6621d8781789d1`

Status: **HOLD (one bounded reduction-bit defect)**

Evidence mode: Static + Ran

Role: final independent time/numerics semantic-conformance review against the
latest approved `SC-COUPLEDTIME-001` V2 amendments. No production files were
edited.

### Requested closure verification

| Finding | Result | Evidence |
| --- | --- | --- |
| `CTA-IMPL-A2-005` queue-minted zero-event join | **CLOSED** | `PendingEventJoinV1` has no public constructor or writable event ID. It can be minted only by `EventQueueV1::pending_event_join`, which canonically constructs the next transition against the supplied accepted clock. `reduce_constraints` accepts that capability rather than an arbitrary `EventId`, retains the derived ID, and binds it into the reduction digest (`event.rs:287-291,344-371`; `constraint.rs:155-225`; queue-backed test `authority.rs:217-247`). |
| `CTA-IMPL-A3-001` merged terminal clock restore | **CLOSED** | Restore now chooses the terminal receipt by comparing the last event tick to the last slab end, selecting the event only when it is chronologically at or after the slab. Thus `slab -> event -> slab` retains the final slab clock, while an event at the final slab boundary retains the event clock (`restart.rs:804-809`). |
| `CTA-IMPL-A3-002` nullable/empty diagnostic reduction | **CLOSED for occupancy and operand persistence** | `value_bits` is nullable; ordered `(receipt_id,value_bits)` operands are persisted; empty reductions restore as `None`; IDs, finiteness, duplicates, lineage, and reconstructed result are validated (`wire.rs:125-139`; `restart.rs:496-507,723-764,911-937`). The prior invented-zero defect is removed. |
| All earlier `CTA-IMPL-A-*` and `CTA-IMPL-A2-*` findings | **REMAIN CLOSED** | Candidate types remain non-deserializable, acceptance reconstructs candidates, parent commit is clock-bound, same-tick events chain from accepted state, segment identity is fixed, event ledgers/custody are typed, and the focused regression suite remains green. |

### Remaining finding

| Finding ID | Severity | Finding | Evidence | Required disposition |
| --- | --- | --- | --- | --- |
| `CTA-IMPL-A4-001` | Medium | The latest V2 amendment requires maximum/minimum reductions to retain the **first operand on numeric equality, including signed-zero equality**. Production maximum folding uses `old.max(value)`, whose signed-zero behavior does not implement that rule (for example a first `-0.0` followed by `+0.0` yields positive-zero semantics). Restart reconstruction also uses `reduce(f64::max)` and validates with numeric `Option<f64>` equality, under which `-0.0 == +0.0`; therefore a wrong declared signed-zero result can be admitted even though its bits differ from the required first operand. | Runtime fold `transaction.rs:406`; restore reconstruction and comparison `restart.rs:745-761`; binding authority text `SC-COUPLEDTIME-001.md:403-414`. | Implement the frozen equality rule explicitly (replace only when the incoming value is strictly greater; compare reconstructed and declared `to_bits()` including the nullable case). Add `[-0,+0]`, `[+0,-0]`, equal finite-value, and wrong-signed-zero restart poison tests. |

### Focused gates run at the exact candidate

- `nix develop --command cargo fmt --all -- --check`: PASS.
- `nix develop --command cargo check -p openwepp-coupled-time -p openwepp-hillslope-orchestrator`: PASS.
- `nix develop --command cargo clippy -p openwepp-coupled-time --all-targets -- -D warnings`: PASS.
- `nix develop --command cargo nextest run -p openwepp-coupled-time`: PASS, 11/11.
- `nix develop --command cargo test -p openwepp-hillslope-orchestrator coupled_time_reference --lib`: PASS, 3/3 (724 filtered).

### Final verdict

The requested queue capability, merged-clock restoration, and nullable
reduction corrections are sound. Release remains **HOLD** only for
`CTA-IMPL-A4-001`, a bounded but contract-explicit bit-identity defect. Correct
the equality fold and bitwise restart comparison, add the four named fixtures,
and rerun this final review.

## TERMINAL PASS — candidate `b3066638eb0199535b8f38c358c1798a3da5978f`

Status: **PASS**

Evidence mode: Static + Ran

Role: final closure review for time/numerics semantic conformance. No production
files were edited.

`CTA-IMPL-A4-001` is closed. Runtime maximum and minimum folds now replace the
retained value only on a strict inequality, so numeric equality preserves the
first operand's exact bits, including both signed-zero orders. Restart uses the
same frozen fold functions and compares nullable reconstructed/declared results
after `to_bits()`, preventing `-0.0`/`+0.0` substitution. Unit fixtures cover
both signed-zero orders, equal finite values, maximum, and minimum
(`transaction.rs:416-458`; `restart.rs:760-782,986-1004`).

All earlier A-series closures remain intact on this exact tree:

- authority candidates cannot be forged through public deserialization and are
  reconstructed at acceptance;
- parent receipts and atomic commits are bound to the exact parent clock;
- same-tick event proposals chain from accepted state under queue authority;
- segment identity is fixed across its slabs and exact support is enforced;
- coincident constraints retain their ordered receipt and zero-step event joins
  are queue-minted capabilities;
- event custody and conservation ledgers are authenticated;
- restart merges slab/event chronology correctly and preserves nullable ordered
  diagnostic operands without an empty-zero sentinel.

Focused gates at the exact candidate:

- `nix develop --command cargo fmt --all -- --check`: PASS.
- `nix develop --command cargo check -p openwepp-coupled-time -p openwepp-hillslope-orchestrator`: PASS.
- `nix develop --command cargo clippy -p openwepp-coupled-time --all-targets -- -D warnings`: PASS.
- `nix develop --command cargo nextest run -p openwepp-coupled-time`: PASS, 12/12.
- `nix develop --command cargo test -p openwepp-hillslope-orchestrator coupled_time_reference --lib`: PASS, 3/3 (724 filtered).

Final verdict: **PASS** for the independent time/numerics implementation review.
No open A-series finding remains.

## EXACT-COMMIT REGRESSION PASS — `9dadbe426d9b9a5bdfbf6e36d604a2b1cdc68ff7`

Status: **PASS**

Evidence mode: Static + Ran

The Sum extension preserves every previously closed A-series invariant. Its
runtime fold is an ordered IEEE-754 left fold beginning at positive zero,
rejects nonfinite operands/results before admitting the operand, and retains
the authenticated accepted receipt order. Restart persists the ordered operand
IDs and bits, reconstructs Sum with the same positive-zero left fold, rejects a
nonfinite intermediate, and compares the declared result by exact bits. Empty
Sum remains `null`, not a zero sentinel. Maximum/minimum still use strict
inequality and preserve first-operand bits on numeric equality
(`transaction.rs:388-472`; `restart.rs:15-41,731-803,986-1017`).

The added generic operand capability is constructible only from accepted slab,
event, or scheduled-once receipt types. Restart admission joins every operand
ID back to the retained accepted chronology, including event and scheduled
instants. The Sum changes do not reopen candidate authentication, parent
atomicity, event chaining, segment identity, constraint/event capability,
ledger custody, merged-clock restore, or publication buffering.

Focused gates at the exact commit:

- `nix develop --command cargo fmt --all -- --check`: PASS.
- `nix develop --command cargo check -p openwepp-coupled-time -p openwepp-hillslope-orchestrator`: PASS.
- `nix develop --command cargo clippy -p openwepp-coupled-time --all-targets -- -D warnings`: PASS.
- `nix develop --command cargo nextest run -p openwepp-coupled-time`: PASS, 13/13.
- `nix develop --command cargo test -p openwepp-hillslope-orchestrator coupled_time_reference --lib`: PASS, 3/3 (724 filtered).

Final exact-commit verdict: **PASS**. No A-series finding is open.

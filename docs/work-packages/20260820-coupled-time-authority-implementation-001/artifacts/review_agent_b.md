# Review Agent B — Rust API, Atomicity, and Bypass Resistance

Status: **FAIL — closure blocking findings**

Evidence mode: **Static + attempted run**

Reviewed exact commit: `42f88d644cf4f3c86bae0b9ae066505684699704`.

Scope: production Rust API, owner/participant joins, attempt isolation,
atomicity, bypass resistance, typed failures, orchestrator reference consumer,
negative old-path proof, gate legitimacy, and line-count governance. This is
the final implementation review and does not replace the Phase-2A contract
review.

## Findings

| ID | Severity | Finding | Evidence | Required disposition |
| --- | --- | --- | --- | --- |
| CT-RB-001 | Critical | The clock and publication state machines are publicly mutable, and publication admission trusts a caller-supplied Boolean. An adopter can directly advance `accepted_until`, replace owners/participants/receipts, set an outbox to a committed state, or call `commit(true)` before any parent commit. This defeats single clock custody, atomic owner installation, and precommit publication exclusion. | `crates/openwepp-coupled-time/src/clock.rs:16-31`; `crates/openwepp-coupled-time/src/restart.rs:25-29,32-41`; contract §§4, 9 and INV-COUPLEDTIME-005/013. | Make authority-owned mutable fields private (with read-only accessors/snapshots), admit mutations only through checked transaction methods, and bind outbox commit to an unforgeable validated parent-commit receipt/candidate rather than `bool`. Add compile-fail/API and runtime negative tests for direct clock/outbox bypass. |
| CT-RB-002 | Critical | Slab acceptance does not implement the canonical candidate/receipt/ledger join. `accepted_slab_id`, `receipt_id`, `global_ledger_digest`, and each local `ledger_digest` are never validated; the candidate carries no parent ID, slab/segment ordinal, constraint digest, beginning/ending complete-owner digest, exchanged-flux lineage, or complete owner map. A caller-selected `ledgers_closed=true` plus matching active owner bytes is sufficient to mutate accepted state. | `crates/openwepp-coupled-time/src/transaction.rs:18-25,28-72`; contract §5 and INV-COUPLEDTIME-005. | Implement the closed V1 candidate envelope and reconstruct all identities/digests and local/global ledger joins before mutation. Do not use caller truth Booleans as ledger authority. Prove wrong ID, wrong receipt, unresolved/mismatched ledger, inactive mutation, omitted carry, wrong segment/ordinal, and replay are atomic no-ops through the production Rust API. |
| CT-RB-003 | Critical | Event acceptance ignores its declared beginning owner-set digest and does not reconstruct or validate event ID, receipt ID, event context/ordinal lineage, or ledger digest. `ledger_closed=true` is again trusted. Thus an event rooted in a different owner set or carrying forged identities can mutate owners and participants. Same-tick cycle protection is only a per-call no-progress check plus a standalone sort helper; `apply_event` does not enforce event precedence or the 256-transition/repeated-cycle-key authority. | `crates/openwepp-coupled-time/src/event.rs:19-31,34-98`; contract §6 and INV-COUPLEDTIME-007. | Bind event application to the accepted owner-set digest and reconstruct event/receipt/ledger identity. Move deterministic pending-event ordering and cycle-key/limit enforcement into the authoritative transition path. Add atomic poison cases against the actual Rust API, including wrong beginning digest, forged receipt, out-of-order coincident events, and repeated semantic cycle. |
| CT-RB-004 | Critical | Parent finalization is neither exactly-once nor a complete atomic parent transaction. It accepts an arbitrary sequence unrelated to `parent_transaction_id`, can be called repeatedly, checks only `accepted_until == end`, and returns owner bytes without validating required events/scheduled operations, exact segment/slab coverage, ordered slab/event receipts, ledgers, or durable publication enqueue. The outbox is committed in a separate fallible call. | `crates/openwepp-coupled-time/src/transaction.rs:119-143`; reference consumer `crates/openwepp-hillslope-orchestrator/src/coupled_time_reference.rs:173-185`; contract §9 and INV-COUPLEDTIME-006/013. | Introduce one authoritative parent candidate/commit operation rooted in the current parent identity and accepted receipt chronology. It must atomically install the complete owner set, increment the persistent sequence once, and create the durable committed outbox receipt. Reject replay and incomplete event/scheduled/coverage/ledger joins. |
| CT-RB-005 | Critical | Production restart validation is shallow and does not enforce the semantic invariants claimed by the package. It checks only schema/model/policy equality and cursor bounds. Corrupt owner byte digests, noncanonical or invalid participant sets, replayed/duplicate receipts, inconsistent reduction lineage, invalid outbox state/joins, and publication buffers can deserialize and pass `validate`. The independent Python validator is test/evidence tooling and is not invoked by the Rust restore path. | `crates/openwepp-coupled-time/src/restart.rs:62-90`; `tests/integration/coupled_time_authority_contract.rs:232-291`; contract §8. | Implement equivalent fail-closed semantic validation in the production Rust restore boundary (or a production-safe validator it necessarily calls), with typed precedence and Rust poison tests. Make validated restart construction/restoration the only route to accepted mutable authority state. |
| CT-RB-006 | High | The orchestrator consumer demonstrates a useful chronology but bypasses key public authority surfaces, so it is not yet the required end-to-end reference proof. It never collects/reduces `StepConstraintV1`, never constructs or validates `AttemptId`, fabricates slab/event IDs from arbitrary labels, and performs reduction/outbox mutation outside slab acceptance. Its helper therefore demonstrates calls, not the closed constraint→attempt→candidate→receipt→parent-commit protocol. The consumer-path artifact also names nonexistent `run_reference_parent_v1`; the implementation exports `run_coupled_time_reference_consumer`. | `crates/openwepp-hillslope-orchestrator/src/coupled_time_reference.rs:55-185,212-244`; `artifacts/consumer-path-proof.md:12`; repository search found the only production calls at this reference module. | Route the real reference consumer through deterministic constraint arbitration, attempt identity, fully checked candidate acceptance, event receipt, restart, reduction lineage, and atomic parent publication. Correct the artifact symbol. Retain the negative proof that no physical/legacy production adopter imports this new authority; do not misstate bounded non-adoption as production cutover. |

## Validation and governance audit

- Attempted: `cargo nextest run -p openwepp-coupled-time && cargo nextest run
  -p openwepp-hillslope-orchestrator coupled_time_reference && cargo nextest run
  --test coupled_time_authority_contract`.
- Result: **not run** because `cargo` is unavailable in this reviewer
  environment (`/bin/bash: cargo: command not found`). No test PASS is claimed
  by this review.
- Static gate audit: `artifacts/gate-results.md` truthfully marks broad runner,
  cargo-deny, final review, and terminal gates pending. The package therefore
  has no terminal closure basis yet. The focused PASS counts do not exercise the
  bypasses above; many vector/poison assertions validate Python/JSON evidence
  rather than production Rust restoration and commit paths.
- Static negative-path audit: repository search found the new authority used by
  the bounded hillslope-orchestrator reference module and tests only. This is
  consistent with the package's explicit exclusion of V10/physical cutover,
  but it does not compensate for the incomplete checked reference protocol.
- Line-count governance: independently counted all touched Rust files. Maximum
  is 292 lines (`tests/integration/coupled_time_authority_contract.rs`), with
  the production reference consumer at 288 and coupled-time production modules
  at 26–143 lines. The recorded PASS is legitimate: no 2,000-line WARN or
  3,000-line block applies.

## Verdict

**FAIL.** The crate has good typed decomposition and no broad erased-error
boundary, but its public data model currently makes the acceptance invariants
advisory. Findings CT-RB-001 through CT-RB-005 are closure blocking because
callers can bypass clock custody, ledger/identity joins, exactly-once parent
commit, publication atomicity, and restart semantic validation. Re-review is
required after correction and focused Rust poison/consumer gates pass.

---

# FINAL RE-REVIEW — candidate `0bbd96d0a`

Status: **FAIL — closure blocking findings remain**

Evidence mode: **independent static review + focused execution**

Reviewed exact commit: `0bbd96d0ac4b593d2f7cf3ff46f990fdb8142145`.

Scope: Rust API closure, clock/owner custody, attempt isolation, event ordering,
exactly-once parent commit, publication atomicity, restart admission, and the
bounded orchestrator consumer. No production file was edited by this review.

## Disposition of prior findings

The candidate materially improves the first implementation: authority-owned
fields are private, constructors reconstruct identities, slab candidates carry
closed lineage, restart V2 retains accepted slab receipts, and parent commit
creates a `CommittedUndelivered` outbox in the same operation. The reference
consumer now exercises constraints, attempt identity, rejection rollback,
event transition, restart twins, reduction reconstruction, and parent commit.

However, the following bypasses keep CT-RB-001 through CT-RB-005 open in
substantive form. CT-RB-006 is partially corrected, but its required closure
gate currently fails.

## Findings

| ID | Severity | Finding | Evidence | Required disposition |
| --- | --- | --- | --- | --- |
| CT-RB2-001 | Critical | Private fields do not close the API because authority-bearing production types publicly derive `Deserialize`. A caller can deserialize a forged `CoupledClockStateV1`, `CoupledSlabCandidateV1`, `EventTransitionV1`, accepted receipt, outbox, reduction, or scheduled receipt without running its checked constructor. `accept_slab` does not reconstruct `slab_id`, `receipt_id`, receipt fields, or `end_clock`; `apply_event` likewise does not reconstruct the event/receipt envelope. Thus serde is a direct candidate/clock/receipt fabrication path and can install attacker-selected accepted-clock chronology. | `clock.rs` derives `Deserialize` for `CoupledClockStateV1`; `transaction.rs` derives it for `CoupledSlabCandidateV1` and `AcceptedSlabReceiptV1`; `event.rs` derives it for `EventTransitionV1` and `AcceptedEventReceiptV1`; `restart.rs` derives it for publication/outbox types; acceptance checks in `transaction.rs:accept_slab` and `event.rs:apply_event` validate only subsets of those envelopes. | Remove public deserialization from authority-state/candidate/receipt types, or implement validating custom deserialization through a private wire type. Independently reconstruct every candidate and receipt field again at the mutation boundary. Add hostile-serde compile/runtime tests proving private-field fabrication is impossible or rejected atomically. |
| CT-RB2-002 | Critical | `apply_event` remains a public bypass around `EventQueueV1`. Any individually valid event can be applied directly in caller-selected order, avoiding same-tick class/source/context precedence, the 256-event bound, and cycle-key history. The queue therefore is optional policy rather than the authoritative event transition path. | `event.rs` exports both `EventQueueV1::apply_next` and unrestricted public `apply_event`; `lib.rs` re-exports the module API. | Make direct event mutation crate-private and expose only the checked queue/coordinator transition, or require an unforgeable queue-arbitration receipt in event acceptance. Test that coincident events cannot be applied out of order through any public API. |
| CT-RB2-003 | Critical | Parent commit has a stale-candidate/TOCTOU bypass. `ParentCommitCandidateV1::new` snapshots receipt/output digests, but `commit_parent` only checks `committed` and end-of-support. Between construction and commit, a caller can accept an end-tick event or record a scheduled-once operation, changing owners, regime, clock and chronology; the stale candidate is still committed. Its receipt also uses the first **slab receipt ID** as `begin_owner_set`, omits scheduled-once receipts entirely, and does not bind the accepted-clock digest. | `restart.rs:ParentCommitCandidateV1::new` derives the candidate; `restart.rs:commit_parent` does not rejoin it to current clock state; the `begin` local is taken from `accepted_slab_receipts.first().id()`; `scheduled_once_receipts` are absent from parent receipt framing. | Bind the candidate to a final clock/owner/chronology digest and fully revalidate at commit, or consume/freeze the authority state when forming it. Frame the actual beginning owner-set digest and all required ordered slab/event/scheduled receipts. Add stale-after-event, stale-after-scheduled-once, and clone/replay poison tests. |
| CT-RB2-004 | High | Scheduled-once authority is forgeable/collision-prone and is not joined at finalization. `record_scheduled_once` accepts caller-provided boundary/result digests, does not reject an empty operation ID, constructs an `event-receipt` digest that omits `operation_id`, and allows distinct operations with identical supplied boundary/result material to receive the same receipt ID. Because parent finalization ignores scheduled receipts, their execution cannot affect parent identity or completeness. | `clock.rs:record_scheduled_once`; `restart.rs:ParentCommitCandidateV1::new`. | Define and reconstruct a dedicated scheduled-once receipt including parent, operation, named boundary, tick, result, and ordinal; enforce nonempty/canonical operation identity and required schedule completeness; include ordered scheduled receipts in parent finalization. |
| CT-RB2-005 | Critical | Publication/restart identity is not closed. `PublicationRecordV1` accepts `support_lineage_digest`, but `digest_record` deliberately ignores it. The wire omits lineage, `record_from_wire` silently substitutes a fixed digest, and it trusts wire `record_id` instead of reconstructing it. Outbox restore validates only the records array digest; it does not reconstruct publication receipt/parent joins or enforce state/count transitions. This permits canonical JSON to alter record identity/lineage and admit an outbox that was never produced by atomic parent commit. | `restart.rs:digest_record` parameter `_lineage`; `wire.rs:PublicationRecordWire`, `record_from_wire`; `restart.rs:validate`. | Put support lineage in the frozen record wire and identity, reconstruct `record_id`, parent receipt, records digest and publication receipt during restore, and validate outbox state/count/sequence rules. Add wrong-record-ID, wrong-lineage, forged-parent, forged-publication-receipt, invalid state/count, and uncommitted-outbox poisons against production restore. |
| CT-RB2-006 | High | The package-level focused contract gate fails at this exact candidate: the integration test reports no frozen exact framed KAT for `event-receipt-v2`. The crate and bounded consumer tests pass, but closure cannot rely on a new V2 identity absent from the frozen vector authority it claims to implement. | `nix develop --command cargo nextest run --test coupled_time_authority_contract` failed `frozen_vectors_have_separating_event_constraint_and_duration_cases`: “missing exact framed KAT for event-receipt-v2”. | Reconcile the Rust label/shape with the admitted authority and frozen vectors, then rerun the full focused suite. Do not weaken the KAT gate. |

## Focused gate evidence

- `nix develop --command cargo check -p openwepp-coupled-time` — **PASS**.
- `nix develop --command cargo clippy -p openwepp-coupled-time --all-targets -- -D warnings` — **PASS**.
- `nix develop --command cargo nextest run -p openwepp-coupled-time` — **PASS, 8/8**.
- `nix develop --command cargo nextest run -p openwepp-hillslope-orchestrator coupled_time_reference` — **PASS, 3/3 selected**.
- `nix develop --command cargo nextest run --test coupled_time_authority_contract` — **FAIL, 4 passed / 1 failed**, missing exact `event-receipt-v2` KAT.

## Verdict

**FAIL.** The candidate is substantially stronger, but it is not bypass
resistant or atomically finalizable yet. Public serde construction defeats the
private API boundary; direct event application defeats queue authority; parent
commit accepts stale chronology; scheduled-once and publication identities are
not fully joined; and a mandatory frozen-vector integration gate fails. A new
implementation candidate and independent re-review are required.

---

# TERMINAL RE-REVIEW — candidate `bb8fdc7eb`

Status: **FAIL — two restart admission blockers remain**

Evidence mode: **independent static review + focused execution**

Reviewed exact commit: `bb8fdc7eba26dac2dcc95d981a33daee645231e5`.

Scope: all findings from the prior FINAL RE-REVIEW, plus the released V2
active/committed checkpoint, scheduled-once, constraint-coincidence, segment,
and publication amendments. No production file was edited by this review.

## Prior finding closure

| Prior finding | Disposition | Evidence |
| --- | --- | --- |
| CT-RB2-001 — public serde fabrication | **CLOSED** | Authority state, owner, constraint receipt, candidate, accepted receipt, event proposal/transition, queue, reduction, scheduled receipt, publication record/outbox, parent candidate, and commit types no longer derive `Deserialize`. Restart deserializes only crate-private `deny_unknown_fields` wire DTOs and reconstructs checked production values. `accept_slab` and the private event mutator reconstruct the full candidate before mutation. |
| CT-RB2-002 — direct event-queue bypass | **CLOSED** | `apply_event` is private. Public callers submit `EventProposalV1` to `EventQueueV1`; the queue sorts proposals and owns cycle/limit history, constructs the transition against current accepted state, then calls the private mutator. |
| CT-RB2-003 — stale parent candidate / wrong parent framing | **CLOSED** | `commit_parent` consumes the clock, rebuilds the candidate from that exact final state, compares every candidate identity, and returns clock plus outbox as one `DurableParentCommitV1`. `parent-receipt-v2` now frames the actual beginning owner digest and ordered slab, event, and scheduled receipt IDs. V2 correctly keeps the retained parent sequence and separately persists its checked successor. |
| CT-RB2-004 — scheduled identity/finalization | **PARTIAL; blocker below** | Runtime construction now rejects empty operation IDs, derives `scheduled-boundary-v2` and `scheduled-receipt-v2`, and parent finalization includes scheduled receipt IDs. Restart reconstructs each receipt identity. However, restart replay exclusion is still incomplete. |
| CT-RB2-005 — publication/restart identity | **PARTIAL; blocker below** | Publication records now directly frame and restore exact accepted receipt, support, units, source, value digest and bytes; restore reconstructs record, parent, publication, records and sequence identities. Active/committed phase and durable-row cardinality are checked. However, restored delivery state/count semantics remain unchecked. |
| CT-RB2-006 — missing V2 frozen KAT | **CLOSED** | The frozen population is now 114 cases and includes the V2 additions. The package contract integration test passes 5/5, and the crate oracle executor passes as part of 10/10 crate tests. |

## Remaining findings

| ID | Severity | Finding | Evidence | Required disposition |
| --- | --- | --- | --- | --- |
| CT-RB3-001 | Critical | Restart admits scheduled-once replay for the same canonical execution key. The released V2 rule says `(parent_transaction_id, operation_id, boundary_id)` may occur exactly once regardless of result or receipt ID. Runtime `record_scheduled_once` enforces operation/tick uniqueness, but restart validation only rejects adjacent receipts that are entirely equal. Two correctly framed receipts with the same operation and boundary but different `result_sha256`/receipt IDs both pass `validate_identity`, are not equal, and are accepted; nonadjacent exact duplicates also escape the `windows(2)` check. The forged chronology is then included in parent finalization. | `clock.rs:record_scheduled_once` versus `restart.rs:validate`, where `scheduled_once_receipts.windows(2).any(|w| w[0] == w[1])` is the only replay test before per-receipt identity validation; SC-COUPLEDTIME-001 V2 §8 explicitly fixes the execution key and replay rule. | Validate global uniqueness by the canonical execution key, independent of ordering/result/receipt; reject duplicate keys and noncanonical order. Add restart poisons for same key/different result, same key/different valid receipt, adjacent and nonadjacent replay. |
| CT-RB3-002 | High | Restart does not validate the V2 outbox delivery state against `delivery_attempt_count`. A canonical `CommittedParent` can carry `CommittedUndelivered` with a nonzero count, or `DeliveredUnacknowledged`/`Acknowledged` with zero attempts, and pass all current checks because validation reconstructs parent/publication/record identities but never checks state/count coherence. The contract requires crash restoration to preserve the real transition state; admitting impossible pairs weakens exactly-once delivery/replay evidence. | `restart.rs:validate` outbox loop checks committed phase, parent, sequence, publication ID, and records digest only; no branch inspects `outbox.state` with `delivery_attempt_count`. SC-COUPLEDTIME-001 V2 §8 defines the only allowed transitions and attempt behavior. | Enforce `CommittedUndelivered => count == 0`, `DeliveredUnacknowledged => count >= 1`, and `Acknowledged => count >= 1` (plus any tighter frozen rule), with canonical restart poisons for every impossible pair. |

## Focused gate evidence

- `nix develop --command cargo check -p openwepp-coupled-time` — **PASS**.
- `nix develop --command cargo clippy -p openwepp-coupled-time --all-targets -- -D warnings` — **PASS**.
- `nix develop --command cargo nextest run -p openwepp-coupled-time` — **PASS, 10/10**.
- `nix develop --command cargo nextest run -p openwepp-hillslope-orchestrator coupled_time_reference` — **PASS, 3/3 selected**.
- `nix develop --command cargo nextest run --test coupled_time_authority_contract` — **PASS, 5/5**.

## Verdict

**FAIL.** The corrected candidate closes the public API, event ordering, stale
commit, parent framing, and frozen-KAT defects and passes every focused gate.
It is not terminally releasable because production restart admission still
accepts a prohibited scheduled-once replay and impossible durable-outbox
state/count pairs. Both defects are narrow, but they directly violate the V2
restart and exactly-once authority and require correction plus re-review.

---

# FINAL VERDICT — candidate `f608dbcb4`

Status: **FAIL — restart receipt chronology is not fully admitted**

Evidence mode: **independent static review + focused execution**

Reviewed exact commit: `f608dbcb49fc6e76af8917816f6621d8781789d1`.

## Finding-by-finding disposition

| Finding | Final disposition | Evidence |
| --- | --- | --- |
| CT-RB-001 / CT-RB2-001 — public mutable/serde bypass | **CLOSED** | Authority-bearing state and candidates remain private-field, constructor-only, and non-deserializable; restore uses private checked wire DTOs. Mutation boundaries reconstruct candidates. |
| CT-RB-002 — slab candidate/ledger/receipt joins | **CLOSED** | Slab construction and acceptance bind parent, active segment, constraint-reduction receipt, support, owner sets, ledgers, slab/receipt identity, and accepted-clock transition; acceptance reconstructs the candidate before mutation. |
| CT-RB-003 / CT-RB2-002 — event identity and queue bypass | **CLOSED** | Only `EventQueueV1` can invoke the private event mutator. It orders proposals, retains cycle/limit state, constructs transitions against current accepted owners, and now exposes a typed `PendingEventJoinV1` so callers cannot invent the event-boundary join. |
| CT-RB-004 / CT-RB2-003 — parent atomicity and stale candidate | **CLOSED** | Commit consumes the clock, reconstructs and compares the parent candidate, frames actual beginning/ending owner sets and ordered slab/event/scheduled chronology, and atomically returns the committed clock plus durable outbox. V2 retained/next sequence semantics match the released amendment. |
| CT-RB-005 / CT-RB2-005 — restart/publication joins | **PARTIAL; blocker below** | Authority digest is now an admission input; publication record, parent, outbox and reduction operands are independently reconstructed; nullable empty reductions are represented correctly. Receipt chronology bounds/order remain incomplete. |
| CT-RB-006 — reference consumer protocol | **CLOSED** | The bounded consumer exercises constraints, typed pending-event join, attempt/retry rollback, two segments with custody transfer, restart twins, accepted-only reconstruction, atomic commit, and durable delivery crash boundaries. |
| CT-RB2-004 / CT-RB3-001 — scheduled identity and duplicate execution key | **DUPLICATE-KEY PORTION CLOSED; chronology portion remains** | Runtime and restore reconstruct the V2 boundary/receipt IDs and globally reject repeated `(operation_id, boundary_id)` keys even with different results or nonadjacent placement. |
| CT-RB2-006 — frozen V2 KAT | **CLOSED** | All focused contract/oracle tests pass with the expanded frozen population. |
| CT-RB3-002 — outbox state/count coherence | **CLOSED** | Restore rejects nonzero attempts in `CommittedUndelivered` and zero attempts in `DeliveredUnacknowledged` or `Acknowledged`; transition methods preserve the admitted state machine. |

## Remaining finding

| ID | Severity | Finding | Evidence | Required disposition |
| --- | --- | --- | --- | --- |
| CT-RB4-001 | Critical | Production restart admission still does not enforce the frozen receipt chronology rules. V2 requires receipt arrays to be strictly receipt-ID ordered/unique and event/scheduled chronology to end no later than `accepted_until`. The Rust validator checks slab order and event relative ordering, plus scheduled execution-key uniqueness, but never checks scheduled receipt-ID order, never checks a scheduled tick against the accepted cursor/parent support, and never rejects an event tick after `accepted_until`/parent end. A canonical checkpoint can therefore contain a validly reframed future scheduled receipt or future event, or a noncanonical scheduled receipt ordering, and be admitted; parent finalization then authenticates that invalid chronology. | `restart-schema-v2.json` semantic rules 3 and 6; `restart.rs:validate` scheduled block only compares execution keys then validates individual identity; `wire.rs:validate_authenticated_chronology` orders events relative to each other but does not bound event ticks by `accepted_until_ns`/parent support. | Enforce canonical receipt ordering/uniqueness and `parent.start <= receipt.tick <= accepted_until` for every accepted event/scheduled receipt at the production restore boundary. Add production Rust poisons for future scheduled tick, future/out-of-parent event, reordered scheduled IDs, and nonadjacent duplicate receipt IDs. |

## Focused gate evidence

- `nix develop --command cargo check -p openwepp-coupled-time` — **PASS**.
- `nix develop --command cargo clippy -p openwepp-coupled-time --all-targets -- -D warnings` — **PASS**.
- `nix develop --command cargo nextest run -p openwepp-coupled-time` — **PASS, 11/11**.
- `nix develop --command cargo nextest run -p openwepp-hillslope-orchestrator coupled_time_reference` — **PASS, 3/3 selected**.
- `nix develop --command cargo nextest run --test coupled_time_authority_contract` — **PASS, 5/5**.

## Final verdict

**FAIL.** Every previously identified API, atomicity, duplicate-key, outbox,
and frozen-vector defect is corrected, and all focused gates pass. Terminal
release is still blocked because the production restart path admits future and
noncanonical event/scheduled receipt chronology contrary to the frozen V2
schema. This is a bounded restore-validation correction; no broader API
redesign is required.

---

# TERMINAL PASS — candidate `b3066638e`

Status: **PASS**

Evidence mode: **independent static review + focused execution**

Reviewed exact commit: `b3066638eb0199535b8f38c358c1798a3da5978f`.

## Final closure

| Finding family | Disposition | Evidence |
| --- | --- | --- |
| Public API and serde bypass resistance | **CLOSED** | Authority-bearing mutable state, candidates, transitions, receipts, reductions, outboxes, and commits remain constructor-only and non-deserializable. Private wire DTO admission and mutation-boundary reconstruction remain intact. |
| Slab/event ownership and ledger atomicity | **CLOSED** | Slabs bind the admitted active segment and typed constraint-reduction receipt; events can mutate only through the ordered/cycle-bounded queue; inactive-owner and ledger joins are reconstructed before mutation. |
| Parent commit and publication atomicity | **CLOSED** | Commit consumes and revalidates the final clock/candidate, installs the durable `CommittedUndelivered` outbox in the same returned state, and prevents stale-candidate and replay paths. Parent/interval, owner, slab, event, scheduled and output identities remain joined. |
| V2 restart and retained/next transaction sequence | **CLOSED** | Restore checks model, authority and policy identities, active/committed phase, retained transaction identity, checked next sequence, authenticated slab/event/scheduled chronology, reductions, records, outbox joins and canonical reserialization. |
| Scheduled-once exact execution key | **CLOSED** | Runtime derives the boundary and receipt identity, rejects repeated operation/tick execution, and stores receipts in strict receipt-ID order. Restore enforces strict ordering, global uniqueness of `(operation_id, boundary_id)` independent of result/receipt, reconstructs each identity, and bounds the tick to parent support and accepted cursor. |
| Event chronology order and bounds | **CLOSED** | Restore enforces consecutive event ordinals, deterministic tick/class/source/context precedence, beginning/end clock-owner chaining, and `parent.start <= event.tick <= accepted_until <= parent.end`. Future and out-of-parent event receipts fail before state installation. |
| Outbox state/count and reduction lineage | **CLOSED** | Restore rejects impossible delivery state/count pairs. Accepted operand IDs and exact finite operand bits are retained, duplicate operands fail, and maximum/minimum reductions are independently reconstructed with first-equal-bit retention. |
| Frozen oracle and bounded consumer | **CLOSED** | The production crate matches the expanded frozen population, and the orchestrator consumer exercises typed event joins, retry rollback, segmentation, restart twins, reduction reconstruction, atomic commit and durable delivery crash boundaries. |

## Focused gate evidence

- `nix develop --command cargo check -p openwepp-coupled-time` — **PASS**.
- `nix develop --command cargo clippy -p openwepp-coupled-time --all-targets -- -D warnings` — **PASS**.
- `nix develop --command cargo nextest run -p openwepp-coupled-time` — **PASS, 12/12**.
- `nix develop --command cargo nextest run -p openwepp-hillslope-orchestrator coupled_time_reference` — **PASS, 3/3 selected**.
- `nix develop --command cargo nextest run --test coupled_time_authority_contract` — **PASS, 5/5**.

## Terminal verdict

**PASS.** Candidate `b3066638e` closes CT-RB4-001 and preserves every prior B
closure. I found no remaining Rust API, ownership, atomicity, scheduled/event
chronology, restart, publication, or bypass-resistance blocker within this
package's bounded 2A scope.

---

# EXACT-COMMIT REGRESSION VERDICT — candidate `9dadbe426`

Status: **PASS**

Evidence mode: **independent static regression review + focused execution**

Reviewed exact commit: `9dadbe426d9b9a5bdfbf6e36d604a2b1cdc68ff7`.

The Sum correction preserves every prior B closure. It does not reopen public
deserialization, clock/owner mutation, direct event application, stale parent
commit, scheduled replay/order/bounds, event chronology, publication joins, or
outbox state/count paths. Cross-class reduction operands are exposed only as
typed capabilities derived from accepted slab, event, or scheduled receipts;
duplicate receipt admission remains rejected. Restore retains ordered operand
IDs and exact finite operand bits, reconstructs Sum in accepted order with
checked finite accumulation, and distinguishes an empty reduction (`null`)
from an admitted numeric zero. Maximum/minimum first-equal-bit behavior remains
unchanged.

Focused gates:

- `nix develop --command cargo check -p openwepp-coupled-time` — **PASS**.
- `nix develop --command cargo clippy -p openwepp-coupled-time --all-targets -- -D warnings` — **PASS**.
- `nix develop --command cargo nextest run -p openwepp-coupled-time` — **PASS, 13/13**.
- `nix develop --command cargo nextest run -p openwepp-hillslope-orchestrator coupled_time_reference` — **PASS, 3/3 selected**.
- `nix develop --command cargo nextest run --test coupled_time_authority_contract` — **PASS, 5/5**.

Final exact-commit verdict: **PASS.** No B-scope regression or new API,
atomicity, ownership, restart, chronology, reduction, publication, or bypass
blocker was found at `9dadbe426`.

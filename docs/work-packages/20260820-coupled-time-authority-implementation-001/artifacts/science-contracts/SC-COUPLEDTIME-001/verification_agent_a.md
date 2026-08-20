# Authority Verification A

Verdict: **FAIL**

Evidence mode: **Static + Ran.** Verified candidate commit
`93dc4e97b2417f6fec443ddc420c35ff6c517049` directly. The worktree contained
an unrelated post-commit edit to this contract-cycle directory; all reviewed
authority inputs matched the candidate commit, and this verification does not
rely on that edit.

Reviewer scope: independent verification of A-001 through A-007 and overlapping
B findings after disposition. This artifact does not verify production Rust and
does not authorize promotion or implementation.

## Focused gates

| Gate | Result | Evidence |
| --- | --- | --- |
| independent Python reference | PASS | Ran `python3 .../reference_model.py`; output SHA-256 was the frozen `c4b2281fa6fa98c6ad080f9c0927360daaf1441da25cda36d1fc90c38529e92d` |
| Binding Exposure Index, strict | PASS | Ran `python3 tools/check_sc_binding_exposure.py --strict .../SC-COUPLEDTIME-001.md`; 3 rows consolidated |
| SC unit compliance | PASS | Ran `python3 tools/release/check_sc_unit_compliance.py --path .../SC-COUPLEDTIME-001.md` |
| JSON syntax: model/time/restart/vector artifacts | PASS | Ran `python3 -m json.tool` on all four files |
| candidate diff hygiene | PASS | Ran `git diff --check 93dc4e97b^ 93dc4e97b` |
| Rust contract comparison | NOT RUN | `cargo nextest run --test coupled_time_authority_contract` could not start because `cargo` is unavailable in this verifier environment (`cargo: command not found`) |

The passing reference hash proves deterministic execution of the current
population. It does not cure missing cases or make tautological `forced_error`
cases independent semantic evidence.

## Finding closure

| Finding | Result | Verification |
| --- | --- | --- |
| A-001 | **CLOSED** | Accepted chronology is separated from ephemeral `RetryControlV1`; rejection may advance diagnostic retry state while accepted state stays byte-identical, restart discards ephemeral retry state, and retry/restart/exhaustion vectors execute. |
| A-002 / B-003 | **CLOSED** | Bookkeeping ordinals/receipts are excluded from progress. The contract defines a semantic cycle key and a 256-transition budget; both repeated-key and budget-exhaustion vectors execute. |
| A-003 / B-002 | **STILL OPEN** | Framing is substantially specified, but the released vector population contains no executable canonical hash/identity known-answer case at all. It has no expected `ParentIntervalId`, `ParentTransactionId`, `SegmentId`, `AcceptedSlabId`, `AttemptId`, `EventId`, receipt ID, ambiguous-length separation, field reorder/omission, wrong-version, or transaction-successor overflow result. Several receipt domains are named without complete field lists. Independent implementations can therefore still disagree without failing this gate. |
| A-004 | **STILL OPEN** | Closed class orders and a coincidence paragraph were added, and the compatible tie vector executes. The incompatible tie is only `forced_error` with no constraints or compatibility inputs, so neither the oracle nor Rust test decides incompatibility. “Compatible custody preconditions” remains undefined as a closed predicate. Exact coincident event receipts/state are also absent. |
| A-005 | **STILL OPEN** | Exact rational event quantization and the common duration conversion are now specified and the available one-bit/large-duration cases execute. Required hard-case closure is incomplete: there is no exact halfway-tie vector, no event-quantization case above `2^53` ns, no quantization near `u128::MAX`, and no quantization-magnitude/addition overflow case. `u128_overflow` tests decimal parsing, not the conversion algorithm. |
| A-006 / B-006 | **STILL OPEN** | The oracle now visits all present cases and the Rust source compares its output structurally, but the population still does not implement the contract's release gate. Most ownership/atomicity/publication failures are tautological `forced_error` inputs. There are no canonical identity/digest answers, complete candidate/ledger joins, uninterrupted-vs-restart owner/receipt/publication equivalence, restart-before/after-event cases, rejected-attempt reduction aliases, pre/post-restart maximum aliases, volume/nominal-duration alias, duplicate scheduled output reconstruction, or independently reconstructed publication order. The Rust test checks the oracle against expected values stored in the same vector file; it does not separately implement these semantics. |
| A-007 | **CLOSED** | The model definition now uses receipt-bound run-relative zero consistently with the contract. Calendar and forcing receipts are named identity inputs. Distinguishing receipt identity vectors are still required under A-003/A-006, but the origin contradiction itself is removed. |
| B-001 | **STILL OPEN** | The restart schema now names the previously omitted continuation surfaces and includes bounded substructures, including complete owner bytes and outbox state. However, executable poison/equivalence vectors for altered run/calendar/forcing/model/policy fields, accepted events, scheduled-once receipts, reduction/publication state, and before/after-event restart remain absent. Schema annotations alone do not prove continuation equivalence. |
| B-004 | **STILL OPEN** | A durable outbox state machine and idempotent receipt semantics are now authoritative prose. Only two happy publication states and precommit/rollback error labels exist; crash boundaries, acknowledgement, duplicate delivery, restart reconstruction, and parent/outbox atomicity are not executable. |
| B-005 | **STILL OPEN** | Schemas are closed and bounded, but the decimal regex still admits 39-digit values above `u128::MAX`; ordering, range, and support predicates exist only in `x-semantic-validation` annotations. No independent schema/semantic validator gate or comprehensive malformed collection vectors enforce them. |
| B-007 | **STILL OPEN** | Event ledger shape exists inside restart schema, but canonical slab-candidate, slab-receipt, parent-candidate, and parent-receipt schemas and complete owner/cardinality join vectors are absent. The inactive-owner and partial-install cases merely request their expected error. |

## Disposition

The two direct chronology contradictions (A-001 and A-002/B-003) and the origin
contradiction (A-007) are corrected. The candidate is nevertheless not a
releasable authority checkpoint because canonical identity, machine-decidable
constraint compatibility, numeric boundary population, restart equivalence,
candidate/ledger joins, and independent output/publication evidence remain
unclosed. These are accepted release-blocking findings, not optional production
implementation work.

Do not promote/index `SC-COUPLEDTIME-001` and do not begin production Rust.
Complete the missing schemas and executable independent vectors, rerun all
invalidated gates (including the Rust comparison in a Cargo-capable
environment), and submit a new exact authority candidate for dual verification.

---

## Rerun — 2026-08-20 — corrected candidate `756b08461`

Final rerun verdict: **FAIL**

Evidence mode: **Static + Ran.** Reverified exact commit
`756b08461b252fc58a0d0511713b6abf2d2f66b3`. The prior verification above is
retained as historical evidence for `93dc4e97b`; this section supersedes its
candidate-specific closure assessment only.

### Rerun gates

| Gate | Result | Evidence |
| --- | --- | --- |
| independent Python reference | PASS | Ran `python3 .../reference_model.py`; exact output SHA-256 `c57e0532427929d7d3dc45182290d0c65a23f41a36d6d2e34d2e8018741fdeb8` matched the frozen vector identity |
| Rust/reference contract comparison | PASS | Ran `nix develop --command cargo nextest run --test coupled_time_authority_contract`; 4/4 passed |
| Binding Exposure Index, strict | PASS | 3 binding rows fully consolidated |
| SC unit compliance | PASS | no findings |
| JSON syntax | PASS | model definition, time schema, restart schema, vectors, and receipt/candidate/ledger schema parsed |
| candidate diff hygiene | PASS | `git diff --check 756b08461^ 756b08461` |

These gates prove that the 57-case population is deterministic and that Rust
compares the Python result with the frozen expected JSON. Release still depends
on the population covering the accepted review corrections rather than only
passing its present cases.

### Rerun finding closure

| Finding | Result | Rerun verification |
| --- | --- | --- |
| A-001 | **CLOSED** | `RetryControlV1` remains correctly separated from accepted chronology. Executable retry reduction, repeated/exhausted retry, policy mismatch, rejected-state identity, and fresh restart-at-accepted-boundary behavior are present. |
| A-002 / B-003 | **CLOSED** | The physical/pending-event cycle key excludes ordinals and receipts; repeated semantics and the same-tick budget are independently evaluated by the oracle. |
| A-003 / B-002 | **STILL OPEN** | The contract now closes field lists and adds exact preimage/digest KATs for parent interval, parent transaction, and event receipt plus ambiguous-length separation and sequence overflow. It still lacks the review-required exact KATs for `SegmentId`, `AcceptedSlabId`, `AttemptId`, `EventId`, slab receipt, parent receipt, publication receipt, wrong version, field reorder, and omission. The generic oracle hashes caller-supplied fields, so it does not itself enforce each named domain's closed field list. |
| A-004 | **CLOSED** | Compatibility is now a closed predicate over parent/cursor/calendar/forcing and compatibility-group identity, with explicit event custody chaining. Compatible and incompatible equal-time constraints are real inputs evaluated by the oracle; event precedence produces exact ordered receipts/state. |
| A-005 | **CLOSED** | The exact rational conversion is retained and executable cases now include exact halfway ties-to-even, one-bit neighbors, above-`2^53` nanoseconds, `u128::MAX`, addition overflow, and proposal-magnitude overflow. |
| A-006 / B-006 | **STILL OPEN** | Tautological `forced_error` cases were removed and the present inputs are semantically evaluated. However, the contract's mandatory release population is still materially incomplete: no executable restart immediately before/after an event, event replay after restore, uninterrupted-vs-restored receipt/owner/reduction/publication equivalence, altered restart run/calendar/forcing/model/policy fields, missing/duplicate/failed ledger joins, omitted/extra owner candidate, partial installation, malformed/canonical-reserialization cases, accepted-plus-rejected versus volume/nominal-duration reduction aliases, pre-restart-only/post-restart-only maxima, duplicate scheduled output in publication, outbox crash/acknowledgement boundaries, or rollback-retained publication case. The new semantic-poison document lists these as **required**, but listing is not execution evidence. The Rust test still compares the Python implementation to expected values from the same vector file rather than separately implementing these absent semantics. |
| A-007 | **CLOSED** | Receipt-bound run-relative origin remains consistent. Parent identity KAT now includes distinct calendar and forcing receipt fields, removing the prior origin/identity ambiguity for the admitted case. |
| B-001 | **STILL OPEN** | Restart schema now retains controller and pending-publication bytes rather than digest alone and declares the full continuation surface. No executable restart constructor/validator, poison population, or uninterrupted/restored equivalence case proves those fields, replay barriers, active segment, scheduled-once state, reduction state, and publication state are sufficient or enforced. |
| B-004 | **STILL OPEN** | Durable outbox semantics remain authoritative, but executable cases only compute an accepted-value maximum and record order. They do not model `CommittedUndelivered -> DeliveredUnacknowledged -> Acknowledged`, crash boundaries, durable parent/outbox atomicity, acknowledgement, duplicate/redelivery rules, or restart recovery. The case named `publication_redelivery` has no outbox state or delivery transition. |
| B-005 | **STILL OPEN** | The authority now explicitly requires a semantic validator beyond JSON Schema. That validator is not implemented or exercised in the reference gate. The schemas' decimal regex still admits 39-digit values above `u128::MAX`, and ordering, relational support, cardinality, digest reconstruction, and canonical-reserialization rules remain annotations/documented requirements without executable poison coverage. |
| B-007 | **STILL OPEN** | A closed structural schema now names candidate/receipt/ledger fields and the oracle evaluates several owner joins. There is still no semantic validator or positive complete candidate/ledger reconstruction, and no executable missing/extra/duplicate owner, support/duration mismatch, missing/duplicate/failed ledger, exchanged-flux lineage, parent receipt, or partial-install case. Structural schema plus prose does not close the required exact join algorithm. |

### Rerun disposition

The corrected candidate closes A-004 and A-005 in addition to the chronology
and origin findings previously closed. It also makes substantial, useful
progress on identity framing, restart wire structure, and candidate/ledger
shape. It does **not** close the authority release gate because several items
explicitly recorded as required poison/equivalence populations remain only
prose, schema annotations, or unexecuted structures.

Do not promote/index this candidate and do not begin production Rust. Add the
remaining exact identity KATs and implement an independent semantic validator
plus executable restart, candidate/ledger, reduction, and durable-outbox
populations. Then rerun both reference and Rust gates and submit the new exact
checkpoint for verification.

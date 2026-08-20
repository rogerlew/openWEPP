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

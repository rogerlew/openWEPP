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

---

## Final rerun — 2026-08-20 — candidate `c2d900bfa`

Final verdict: **FAIL**

Evidence mode: **Static + Ran.** Verified exact commit
`c2d900bfac8f8cbce6bc09e189d3fe52dd11136d`. Earlier verdicts remain the
historical record for their named commits; this section is the controlling
assessment for `c2d900bfa`.

### Final focused gates

| Gate | Result | Evidence |
| --- | --- | --- |
| independent reference population | PASS | 96/96 cases evaluated; 52 accepted and 44 rejected; output SHA-256 `dbaa037d8004fd03b17c2ce5e6fad8c28df0eaa42354b87bd8a4e66f97fe7322` matched the frozen vector identity |
| independent semantic-schema population | PASS | 31/31 cases evaluated by `semantic_schema_validator.py`; 4 valid documents accepted and 27 poisons rejected |
| focused Rust contract gate | PASS | `nix develop --command cargo nextest run --test coupled_time_authority_contract`; 5/5 passed |
| Binding Exposure Index, strict | PASS | 3 rows consolidated |
| SC unit compliance | PASS | no findings |
| exact candidate diff hygiene | PASS | `git diff --check c2d900bfa^ c2d900bfa` |

### Final closure assessment

| Finding | Result | Final verification |
| --- | --- | --- |
| A-001 | **CLOSED** | Retry control, rejection no-op, policy mismatch, exhaustion, and accepted-boundary restart are executable. |
| A-002 / B-003 | **CLOSED** | Same-tick physical progress, replay, semantic-cycle, and budget behavior are executable and ordinal-independent. |
| A-003 / B-002 | **STILL OPEN** | Exact KATs now cover every domain declared by `identity_domain_fields`, and the Rust gate proves tag/type/cardinality agreement with that declaration. The accepted review finding also explicitly required separating vectors for reordered fields, omitted fields, and wrong authority version. None of the 96 reference cases exercises an identity with reordered/omitted fields or a non-V1 framing version, and the generic `identity()` oracle accepts caller-supplied field lists/version without validating them against the closed domain definition. Canonical-JSON field-order poisons do not test canonical identity-field order or hash authority version. |
| A-004 | **CLOSED** | Equal-time compatibility, conflict, ordering, event custody, and event precedence are machine-decidable and executable. |
| A-005 | **CLOSED** | All named conversion hard cases, ties, neighbors, large values, and overflow classes execute. |
| A-006 / B-006 | **STILL OPEN** | The expanded reference and independently implemented semantic validator remove the former presence-only/`forced_error` defect and now cover restart identity/state, joins, candidate cardinality, ledgers, accepted-only reduction, and outbox chronology. The accepted finding and canonical test-vector obligations still require executable wrong-answer cases for pre-restart-only maximum, post-restart-only maximum, duplicate scheduled output, and publication retained after parent rollback. None appears in the 96 reference cases or 31 semantic cases. `publication-and-reduction-operand-lineage.md` continues to name these as required wrong-answer fixtures, confirming they were not dispositioned out of scope. |
| A-007 | **CLOSED** | Receipt-bound run-relative origin and calendar/forcing identity mapping are consistent and identity-bound. |
| B-001 | **CLOSED** | The additive restart schema, reference restart/equivalence cases, and independent restart poisons cover complete owner bytes, accepted event/scheduled receipts, active chronology, policies/controller bytes, reductions, publication state, and before/after-event continuation. DirectV10 hashes remain protected. |
| B-004 | **STILL OPEN** | Parent commit, delivery, post-delivery crash, idempotent redelivery, and acknowledgement transitions execute. The accepted publication finding also requires crash/restart behavior at every commit/delivery boundary and rollback removal. There is no `CommittedUndelivered + crash/restart` case, no acknowledged-state crash/restart/no-redelivery case, and no parent-rollback case proving both staged publication and outbox are absent. The lineage artifact explicitly retains those crash-boundary and rollback fixtures as required. |
| B-005 | **CLOSED** | The separate semantic validator now enforces checked `u128`, relational support/cursor bounds, ordered uniqueness, owner/cardinality joins, byte/digest reconstruction, accepted-only lineage, outbox checks, and canonical JSON serialization with an independent 31-case gate. |
| B-007 | **CLOSED** | The structural receipt/candidate/ledger schema plus executable reference joins and independent semantic poisons cover complete owner cardinality, disposition, inactive mutation, common support/duration, state bytes/digests, ledger resolution/failure, and atomic no-op failures. |

### Final disposition

This candidate closes the substantive restart, schema, owner/ledger, and most
publication/reduction gaps from the prior rerun. It is close, but every accepted
finding is not yet closed. Authority release still requires:

1. identity-field reorder, omission, and wrong-version poison vectors evaluated
   against the closed per-domain field definition; and
2. executable pre-/post-restart maximum aliases, duplicate scheduled output,
   parent-rollback publication removal, and the remaining outbox crash/restart
   boundary cases already required by the contract and operand-lineage artifact.

Do not promote/index `SC-COUPLEDTIME-001` or begin production Rust at this
commit. Add those bounded cases, rerun the same three executable gates, and
submit the resulting exact checkpoint for final verification.

---

## Release verification — 2026-08-20 — candidate `11d520330`

Final verdict: **PASS**

Evidence mode: **Static + Ran.** Verified exact commit
`11d520330e6020f560f63ae64ef34732ce82afc4`. This section supersedes the
candidate-specific verdicts above; those remain historical evidence for their
named commits.

| Gate | Result | Evidence |
| --- | --- | --- |
| independent reference population | PASS | 108/108 executable cases; 56 accepted and 52 rejected; output SHA-256 `4540951f70f9de0846669f8f955e7eeca425dd831108997f50009d6ec002df95` matched the frozen identity |
| independent semantic-schema population | PASS | 31/31 cases; 4 valid documents accepted and 27 poisons rejected |
| focused Rust contract gate | PASS | `nix develop --command cargo nextest run --test coupled_time_authority_contract`; 5/5 passed |
| Binding Exposure Index, strict | PASS | 3 rows consolidated |
| SC unit compliance | PASS | no findings |
| exact candidate diff hygiene | PASS | `git diff --check 11d520330^ 11d520330` |

All A-001 through A-007 and overlapping B-001 through B-007 findings are now
**CLOSED**. In particular, the final population independently rejects identity
field reorder, omission, and wrong framing version; rejects pre- and
post-restart-only maxima and duplicate scheduled output; rejects retained
publication without parent commit; proves parent rollback removes the buffered
publication/outbox; and covers committed-undelivered and acknowledged
crash/restart/redelivery boundaries.

No accepted finding remains open. `SC-COUPLEDTIME-001` passes authority review
verification A at this exact checkpoint and may proceed to the remaining dual-
verification, promotion/index, and exact authority-checkpoint steps required by
the package. This PASS authorizes that authority-release sequence; it does not
independently attest later production Rust.

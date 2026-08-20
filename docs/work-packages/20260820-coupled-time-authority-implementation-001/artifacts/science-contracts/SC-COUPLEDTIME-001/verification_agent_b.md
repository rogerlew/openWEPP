# Authority Verification B — Ownership, Restart, Wire, And Publication

Status: complete — FAIL (latest rerun at `756b08461`)

Evidence mode: Static + Ran

Candidate verified: `93dc4e97b241` (`admit coupled time authority candidate`).
Scope was independent closure verification of accepted findings B-001 through
B-007. There were no rejected findings or rejected rationales to verify. I
inspected the corrected canonical contract, disposition, schemas, vectors,
independent reference model, and separately authored Rust contract test. I did
not edit any authority or implementation file.

## Focused gates

- Ran `python3 .../reference_model.py`: PASS. Emitted result SHA-256
  `c4b2281fa6fa98c6ad080f9c0927360daaf1441da25cda36d1fc90c38529e92d`,
  exactly matching `expected_reference_results_sha256`.
- Ran strict JSON parsing for `model-definition.json`, `time-wire-schema.json`,
  `restart-schema.json`, `coupled-time-vectors.json`,
  `runtime-descriptor.json`, and `error-precedence.json`: PASS.
- Ran `git diff --check`: PASS.
- Attempted `cargo nextest run --test coupled_time_authority_contract`: NOT RUN;
  this verification environment has no `cargo` executable on `PATH` or in the
  user cargo bin directory. The recorded package gate says 4/4 PASS, but that
  historical statement is not substituted for independent execution here.
- Recomputed the three protected DirectV10 artifact hashes: PASS:
  `c29e28c4...2ad1`, `c041ab59...0ddd`, and `e01e2a93...d47c9`.
  Exact diff from execution base `f48100538` over those artifacts and
  `crates/openwepp-persisted-restart-v1` is empty.

## Finding closure

| Finding | Result | Verification |
| --- | --- | --- |
| B-001 | **STILL OPEN** | The contract and restart schema now name the required continuation state and distinguish complete owner bytes from their digest. However, the required poison population for omitted/altered restart fields and restart immediately before/after event, scheduled-once, reduction, and outbox states is absent. `restart_after_rejection` only returns constants and does not serialize, validate, restore, or compare a restart value. The Rust test only searches the schema text for three names. |
| B-002 | **STILL OPEN** | Canonical domain-separated framing and checked parent sequence semantics are now specified in prose. No vector or reference-model operation constructs any parent, transaction, segment, slab, attempt, event, or receipt identity; there is no exact expected identity digest, ambiguous-length/reordered/omitted/wrong-version separator, or transaction-overflow execution. Thus independent implementations are not yet demonstrated to share the identity. |
| B-003 | **CLOSED** | The contract excludes ordinals, IDs, and receipts from event progress, defines a physical/pending-event cycle key and a 256-event bound. The reference model executes repeated semantic keys and budget exhaustion as `EventNoProgressCycle`; `event_no_progress_cycle_new_ordinals` no longer relies on ordinal progress. |
| B-004 | **STILL OPEN** | The contract defines atomic durable enqueue and the three-state idempotent outbox, and the restart schema represents those states. The vectors exercise commit and unacknowledged redelivery only; they do not execute receipt identity, acknowledgement, restart at commit/delivery crash boundaries, loss/duplicate prevention, or outbox continuation through actual restart serialization. `publication_after_rollback` is a forced requested error rather than evaluated rollback behavior. |
| B-005 | **STILL OPEN** | The schemas now declare closed shapes, collection bounds, and semantic validation notes, and the reference rejects one over-`u128` tick. No executable schema/semantic validator is invoked. There are no noncanonical decimal, support-order, collection-order/duplicate, receipt-order/bound, base64/digest, or canonical-reserialization poison cases. A JSON parser accepting the documents does not enforce their instance semantics. |
| B-006 | **STILL OPEN (CRITICAL)** | Population increased to 49 cases and the Python output is hash-pinned, but 12 cases use `forced_error`, which simply raises the error supplied by the vector. Key owner joins, partial installation, inactive mutation, replay, scheduled-once, rollback, and policy mismatch therefore cannot detect an incorrect authority algorithm. There is no identity/receipt computation, restart round trip, schema validation, reduction reconstruction, or canonical owner/ledger result. The Rust test compares Python output back to expected values embedded in the same vector and performs presence/string checks; it does not supply an independent implementation of the missing semantics. |
| B-007 | **STILL OPEN** | The restart schema closes an event-receipt/ledger shape, but no canonical slab-receipt or parent-receipt schema exists and no executable owner-candidate map/cardinality/ledger join algorithm is present. Omitted/duplicated owner, inactive-owner mutation, partial installation, and wrong beginning-owner cases are forced errors rather than constructed candidates tested by the oracle. |

## Regression and release disposition

The protected DirectV10 restart artifacts and crate are unchanged from the
execution base, and the focused executable reference gate remains internally
stable. I found no DirectV10 regression. That does not close the accepted review
findings: B-001, B-002, B-004, B-005, B-006, and B-007 remain open, including
the critical executable-authority defect B-006.

**Verdict: FAIL.** Do not promote/index `SC-COUPLEDTIME-001`, create the exact
authority release checkpoint, or begin production Rust. Replace forced-error
and presence-only evidence with independently evaluated identity, restart,
schema, owner/ledger, outbox, and receipt vectors; then rerun both independent
verifications and the Rust focused gate.

## Rerun Against Corrected Commit `756b08461`

Rerun status: complete — FAIL

Evidence mode: Static + Ran

This section supersedes the candidate assessment above while retaining it as
historical evidence. Exact corrected commit:
`756b08461b25` (`close coupled time authority review findings`). There remain
no rejected findings or rejected rationales.

### Rerun gates

- Ran `python3 .../reference_model.py`: PASS. Emitted result SHA-256
  `c57e0532427929d7d3dc45182290d0c65a23f41a36d6d2e34d2e8018741fdeb8`,
  exactly matching the corrected vector manifest.
- Inspected all 57 corrected cases: 57 use evaluated operations and zero use
  `forced_error`.
- Ran `nix develop -c cargo nextest run --test
  coupled_time_authority_contract`: PASS, 4/4 tests.
- Ran `git diff --check`: PASS.
- Recomputed protected DirectV10 hashes: PASS, unchanged at
  `c29e28c4...2ad1`, `c041ab59...0ddd`, and `e01e2a93...d47c9`.
  Exact diff from `f48100538` over the protected artifacts and
  `crates/openwepp-persisted-restart-v1` remains empty.

### Corrected finding closure

| Finding | Rerun result | Verification |
| --- | --- | --- |
| B-001 | **STILL OPEN** | `restart-schema.json` is materially improved: it now closes accepted controller bytes, event/scheduled receipts, accepted reduction operands, pending publication records, complete outbox records/state, and semantic chronology/digest requirements. But no vector has a restart document and the oracle has no serialize/validate/restore operation. The mandatory uninterrupted/mid-parent, immediately-before/after-event, scheduled-once, reduction, publication, omitted/altered-field, and rejected-iterate restart poisons remain requirements in prose rather than executable evidence. `restart_after_rejection` exercises retry state only. |
| B-002 | **CLOSED** | Five executable framed-hash KATs now include parent interval, parent transaction, event receipt, and ambiguous-length separators with exact preimages/digests. Checked transaction successor overflow is executable. The framing algorithm, domains, integer encodings, sequence semantics, and wrong-partition separation are sufficiently frozen for authority release. |
| B-003 | **CLOSED** | The non-bookkeeping physical/pending-event cycle key and finite same-tick budget remain authoritative and are executed by the no-progress and budget cases. Replay, ledger failure, event ordering, and start/interior/end boundaries are evaluated rather than requested errors. |
| B-004 | **STILL OPEN** | The contract, expanded restart schema, and publication-lineage artifact now define the crash-safe durable outbox protocol and required crash fixtures. The executable `publication` operation only filters accepted samples, computes a maximum, and returns input record order after testing `parent_committed`. `publication_redelivery` does not model outbox state or idempotency. There are no commit/delivery/acknowledgement crash-restart transitions, stable receipt-on-redelivery check, acknowledgement-order poison, rollback case, or committed-owner/outbox atomicity execution. |
| B-005 | **STILL OPEN** | `semantic-validation-and-poison-requirements.md` now precisely freezes the missing semantic validator and poison obligations, and structural schemas are much stronger. The reference model still does not admit an instance of either schema or execute canonical reserialization, collection ordering/bounds, decoded-byte/digest, support/cursor, receipt chronology, or altered restart-field poisons. The Rust gate parses only the vector file and performs schema string checks. Naming the mandatory validator is authority progress, not evidence that its wire predicate is enforceable. |
| B-006 | **STILL OPEN (CRITICAL)** | The forced-error bypass is fully removed; all present cases execute reference-model logic and are compared structurally by the Rust gate. Identity, event, constraint, retry, candidate, scheduled-once, publication maximum, transaction overflow, and DirectV10 cases are genuine KAT improvements. However, the contract's own canonical vector obligations still require restart equivalence/poisons, partial owner acceptance and ledger failure, rejected-state reduction/publication aliases, pre/post-restart maxima, nominal-duration alias, duplicate output, rollback, and complete authority tuples. Those populations are absent. Therefore the corrected 57-case set is not yet the **complete** executable authority vector gate required by B-006 and `OBL-COUPLEDTIME-005`. |
| B-007 | **STILL OPEN** | `receipt-candidate-ledger-schema.json` now freezes structural slab/event/parent/publication kinds, complete/active owner IDs, owner candidates, candidate dispositions, ledgers, accepted child receipts, and publication digest. The semantic artifact freezes exact-cardinality and ledger joins. But the schema makes support/tick/ordinal/duration optional for every kind and delegates kind-conditional requirements to a validator that is not executable here. The oracle's four candidate cases do not construct this schema, test complete candidate cardinality, exchanges/local/global ledgers, missing/duplicate/failed ledgers, extra/missing owners, partial installation, or slab/event/parent receipt reconstruction. |

### Latest release disposition

The corrected candidate substantially closes B-002 and confirms B-003, removes
the tautological `forced_error` mechanism, adds real identity KATs and receipt
schema authority, expands restart/outbox authority, and preserves DirectV10
exactly. No regression was found in the focused gates.

**Latest verdict: FAIL.** B-001, B-004, B-005, B-006, and B-007 remain open;
B-006 remains critical because the canonical contract says the vector
population **must** distinguish the still-absent restart, ledger, reduction,
publication, rollback, and authority-tuple poisons. Do not promote/index or
begin production Rust until those required semantics are executable, the
focused gates rerun, and both independent verifiers pass the exact corrected
checkpoint.

## Final Rerun Against Corrected Commit `c2d900bfa`

Final rerun status: complete — **PASS**

Evidence mode: Static + Ran

This final section supersedes both historical FAIL dispositions above while
retaining them as the finding/correction audit trail. Exact corrected commit:
`c2d900bfac8f` (`complete coupled time executable authority evidence`). There
remain no rejected findings or rejected rationales.

### Final gates

- Ran the independent reference model: PASS. All 96 cases executed with zero
  `forced_error`; emitted SHA-256
  `dbaa037d8004fd03b17c2ce5e6fad8c28df0eaa42354b87bd8a4e66f97fe7322`
  exactly matches the frozen vector manifest.
- Ran the independent semantic validator against
  `semantic-schema-poisons.json`: PASS. All 31 cases matched their expected
  disposition: four admitted controls and 27 fail-closed poisons.
- Ran `nix develop -c cargo nextest run --test
  coupled_time_authority_contract`: PASS, 5/5 tests.
- Ran `git diff --check`: PASS.
- Recomputed protected DirectV10 hashes: PASS, unchanged at
  `c29e28c4...2ad1`, `c041ab59...0ddd`, and `e01e2a93...d47c9`.
  Exact diff from `f48100538` over the protected authority artifacts and
  `crates/openwepp-persisted-restart-v1` is empty.

### Final finding closure

| Finding | Final result | Verification |
| --- | --- | --- |
| B-001 | **CLOSED** | Twelve executable restart cases now cover immediately-before/after-event restoration, run/calendar/forcing/model/constraint/controller identity poison, accepted event and scheduled-once receipt poison, reduction poison, and publication-outbox poison. Two uninterrupted/restored equivalence cases compare ending owners, slab/event/scheduled receipts, reduction state, and outbox across both event boundaries. The independent semantic validator additionally admits a complete restart control and rejects altered required/digest/chronology state. |
| B-002 | **CLOSED** | Fourteen identity cases cover every closed framed domain used by the authority, including parent interval/transaction, segment, accepted slab, attempt, event, constraint, owner set, event/slab/parent/publication receipts, and ambiguous-length separation. Exact preimages/digests are frozen, and checked parent-transaction successor overflow remains executable. |
| B-003 | **CLOSED** | Event start/interior/end, same-tick ordering, failure, replay, physical no-progress cycle, and finite same-tick budget are executable. Ordinal-only progress cannot satisfy the cycle check, and every rejected event path proves accepted-state no-op. |
| B-004 | **CLOSED** | The outbox operation now executes atomic parent enqueue, delivery, crash retention, idempotent redelivery with stable receipt identity, and acknowledgement. Restart vectors preserve outbox state across continuation; publication-before-commit remains fail-closed. Independent schema poisons reject record-digest corruption, invalid state, and sequence overflow. Together these cover the specified crash lifecycle without loss, identity change, or precommit visibility. |
| B-005 | **CLOSED** | The independent semantic validator consumes real valid restart/receipt controls and rejects 27 poisons covering `u128` range/canonical form, support/cursor bounds, closed required shape, owner/participant order and digests, controller bytes, future/invalid receipts, outbox records/state/sequence, receipt candidate cardinality/disposition/base64/state digest/support/duration, unresolved ledgers, and canonical JSON field-order/whitespace/duplicate-field violations. This is executable wire admission rather than schema-string presence. |
| B-006 | **CLOSED** | The 96-case oracle population now executes identity, support, conversion, event, constraint, retry, restart equivalence/poisons, owner-ledger joins, scheduled-once, reductions and wrong aliases, publication/outbox lifecycle, complete authority tuples, and DirectV10 protection. The separately authored Rust gate requires all named closure cases, structurally compares every result, hash-pins the population, rejects any `forced_error`, and independently invokes the semantic poison validator. The previously missing mandatory populations are present. |
| B-007 | **CLOSED** | The canonical receipt/candidate/ledger schema is exercised by valid receipt controls plus owner/ledger join KATs and independent poisons for omitted/wrong/duplicate candidates, wrong disposition, corrupt candidate bytes/digest, support/duration mismatch, inactive/cardinality violations, and unresolved/failed ledgers. Exact KATs cover slab, event, parent, and publication receipt identities. Complete-owner cardinality, beginning join, aggregate ledger closure, and ending owner/ledger outputs are now executable. |

### Final release disposition

All accepted findings B-001 through B-007 are closed at exact commit
`c2d900bfac8f`. The executable evidence agrees with the corrected contract and
schemas, the independent poison gate fails closed, restart/outbox/reduction and
receipt chronology are reconstructable, and the released DirectV10 wire remains
byte-identical.

**Final verdict: PASS.** Verification B authorizes the Phase-2A authority
release checkpoint from the ownership/restart/wire/publication perspective,
subject to the separate Verification A result and the package's required
promotion/index/checkpoint procedure. This PASS does not authorize behavior
beyond `SC-COUPLEDTIME-001` or waive later production implementation gates.

## Regression Verdict At `11d520330`

**PASS.** Exact commit `11d520330e60` preserves closure of B-001 through
B-007. The expanded independent oracle executed 108/108 cases with zero
`forced_error` and matched frozen SHA-256
`4540951f70f9de0846669f8f955e7eeca425dd831108997f50009d6ec002df95`.
The semantic-schema suite matched all 31 expected dispositions (four admitted
controls, 27 fail-closed poisons), and `nix develop -c cargo nextest run --test
coupled_time_authority_contract` passed 5/5. `git diff --check` passed; all
three protected DirectV10 hashes and the exact protected-tree diff remain
unchanged. The final fixture additions introduce no B-scope regression.

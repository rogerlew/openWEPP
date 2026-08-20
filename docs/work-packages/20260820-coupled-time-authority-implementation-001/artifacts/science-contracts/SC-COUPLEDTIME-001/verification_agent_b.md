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

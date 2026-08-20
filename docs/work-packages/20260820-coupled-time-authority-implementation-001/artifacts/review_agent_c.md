# Final implementation review C

Status: FAIL

Evidence mode: Static + attempted focused gate

Reviewed exact commit: `42f88d644cf4f3c86bae0b9ae066505684699704`

Role: independent canonical-serialization, restart-determinism, publication,
and dependency/security review. This review does not replace the Phase-2A
canonical contract review.

## Verdict

Closure is blocked. The artifact schema and independent Python validator define
a substantially richer canonical restart wire than the production Rust type
implements. The current tests exercise those artifacts separately and therefore
cannot prove that a serialized production restart is admitted by the mandatory
validator. Parent-owner installation and durable outbox enqueue are also two
separate caller operations, so the required atomic publication crash boundary
is not implemented.

## Findings

| Finding ID | Severity | Finding | Evidence | Recommended disposition |
| --- | --- | --- | --- | --- |
| CT-C-001 | BLOCKER | Production `CoupledTimeRestartV1` is not `OPENWEPP_COUPLED_TIME_RESTART_V1` as frozen by the approved schema. It uses ordinary serde JSON (including numeric `u128` and byte arrays), has only six top-level fields, and omits run/calendar/forcing lineage, transaction sequence, next ordinals, active segment bounds/ID, accepted owner-set digest, boundary modes, constraint-policy identity, and the schema's structured receipts. `validate` checks only schema/model/controller policy and cursor bounds. There is no canonical serializer/parser or production admission through the mandatory semantic validator. | `crates/openwepp-coupled-time/src/restart.rs:62-90`; `crates/openwepp-coupled-time/src/support.rs:7-14`; `artifacts/restart-schema.json` required-field list and string-u128 definitions; contract lines 335-364. The integration test at `tests/integration/coupled_time_authority_contract.rs:216-230` checks only that the artifact text contains three phrases; lines 232-290 run the Python validator on its own poison documents, never on Rust-produced bytes. | accepted: implement the versioned canonical wire and fail-closed admission path, then prove Rust serialization is accepted and every malformed/noncanonical poison is rejected by the production boundary. |
| CT-C-002 | BLOCKER | Atomic owner commit plus durable publication enqueue is not expressible by the API. `finalize_parent` merely returns cloned owners; the caller separately invokes `outbox.commit(bool)`. A crash between those calls can install owners without an outbox or vice versa. The Boolean is caller-asserted and is not joined to a parent receipt/candidate. | `crates/openwepp-coupled-time/src/transaction.rs:119-143`; `crates/openwepp-coupled-time/src/restart.rs:24-41`; reference consumer lines 173-185. This contradicts contract lines 368-379 and the operand-lineage artifact lines 27-41. | accepted: provide one durable transaction candidate/commit boundary binding parent receipt, final owners, ordered records, outbox sequence, and `CommittedUndelivered`; add crash-boundary tests before/after the single commit. |
| CT-C-003 | HIGH | The production outbox does not retain enough state for the specified crash/replay protocol. It lacks `parent_receipt_id`, records digest, delivery-attempt count, and record-level accepted-receipt/support/value-digest lineage. `mark_delivered` changes only an enum, and the restart permits a `Staged` outbox although the canonical durable-outbox schema admits only committed states. | `crates/openwepp-coupled-time/src/restart.rs:7-60` versus `artifacts/restart-schema.json` definitions `publicationRecord` and `outbox`; contract lines 356-364 and 366-386; operand-lineage artifact lines 27-49. | accepted: align production types and transitions with the canonical schema; persist attempt bookkeeping without changing identity; test same-key redelivery and acknowledged no-redelivery across actual serialize/restore cycles. |
| CT-C-004 | HIGH | Restore validation does not establish equivalent continuation or replay exclusion. Accepted event and scheduled receipts are bare IDs with no tick/ordinal/context, reduction receipts may duplicate, owner/participant collections and stored owner digests are not revalidated, outbox transitions/joins are not checked, and canonical reserialization equality is absent. | `crates/openwepp-coupled-time/src/clock.rs:16-32,71-76`; `crates/openwepp-coupled-time/src/transaction.rs:75-100`; `crates/openwepp-coupled-time/src/restart.rs:72-90`. Required validation is contract lines 337-364. | accepted: make restore admission reconstruct and validate every digest, ordering/uniqueness/cardinality rule, receipt chronology, reduction lineage, outbox join, and canonical byte equality before any state installation. |
| CT-C-005 | HIGH | Publication/reduction acceptance remains static design text, not result-bearing acceptance. The Rust reference executes only the restarted chronology; it does not execute an uninterrupted twin and compare accepted receipt IDs, owner bytes, reduction state, publication order, and terminal parent identity. No production test independently reconstructs the reduction/publication records or exercises the listed wrong-answer aliases and four outbox crash boundaries. | `crates/openwepp-hillslope-orchestrator/src/coupled_time_reference.rs:143-201,247-287`; `artifacts/publication-and-reduction-operand-lineage.md` remains `Status: authority candidate`, `Evidence mode: Static`; `artifacts/restart-and-rollback-evidence.md:7-12` claims an equivalent result without an uninterrupted production run. | accepted: implement a separately written reconstructor and uninterrupted/restored twin runs against production outputs; add alias-separating and crash-boundary fixtures with real magnitude/closure evidence. Self-consistency and the independent artifact calculator alone are insufficient. |
| CT-C-006 | MEDIUM | Event restore lineage is weakened because `apply_event` never checks the supplied `beginning_owner_set_digest`, `event_id`, context digest, or ledger digest against the current accepted state/receipt identity. A transition with arbitrary lineage values can mutate accepted owner state and then be persisted as only a receipt ID. | `crates/openwepp-coupled-time/src/event.rs:18-87`. | accepted: reconstruct and validate canonical event/receipt identity and beginning/ending owner-set joins before mutation; persist the complete accepted event receipt required by the schema. |

## Protected wire, dependencies, and security

- Static diff from `f48100538` through the reviewed commit contains no file
  under `crates/openwepp-persisted-restart-v1/**`; this supports the narrow claim
  that DirectV10 production source was not edited. The submitted evidence does
  not show the claimed recomputed released schema/vector/manifest hashes, so
  byte-protection still needs command-level terminal evidence.
- The new crate uses ordinary registry dependencies already represented in the
  workspace lock (`serde`, `sha2`, `thiserror`, `num-bigint`, `num-traits`, and
  dev-only `proptest`). Static inspection found no network, credential, unsafe,
  shell, or external-delivery path in the new crate.
- Focused gate attempt: `cargo nextest run -p openwepp-coupled-time && cargo deny
  check` could not start because `cargo` was not available in this review
  process's `PATH` (`/bin/bash: cargo: command not found`). This review does not
  claim those gates ran. The package's existing `gate-results.md` also records
  cargo-deny and broad closure as pending.

## Line-count and gate-legitimacy check

Ran `wc -l` on all touched Rust files. The largest production consumer is 288
lines, the integration contract test is 292 lines, and the new crate modules are
26-143 lines. No touched Rust file reaches the 2,000-line WARN threshold.

The focused PASS counts recorded in `gate-results.md` are plausible for the
surfaces named, but they do not close the findings above: the schema test and
semantic poison runner validate standalone artifacts, while the production
restart uses a different serde representation. Cargo-deny, broad workspace
gates, final reviews, exact DirectV10 byte hashes, and result-bearing
publication/reduction acceptance remain pending or absent.

---

# FINAL RE-REVIEW

Status: FAIL

Evidence mode: Static + focused executable gates

Reviewed exact commit: `0bbd96d0ac4b593d2f7cf3ff46f990fdb8142145`

Role: independent canonical-V2-serialization, restart-determinism,
DirectV10/V1-protection, and publication/outbox crash-semantics review.

## Verdict

The candidate makes substantial progress: it introduces a closed V2 wire,
canonical reserialization equality, authenticated slab/event chronology,
uninterrupted/restored consumer twins, and an additive byte-preserving V1
envelope. The focused crate and consumer tests pass and the DirectV10 persisted
restart crate has no diff.

Closure is nevertheless blocked. The production V2 admission path does not
reconstruct or authenticate scheduled-once, reduction, publication-record,
parent-receipt, or outbox identities/joins. In addition, owner installation and
durable outbox persistence remain separate from the in-memory `commit_parent`
operation. A canonical but forged outbox can therefore be admitted, and a real
storage crash can still persist only one half of the intended atomic commit.
The focused contract gate also fails at this exact candidate.

## Finding disposition from the first review

| Finding | Re-review disposition |
| --- | --- |
| CT-C-001 | Partially closed. V2 now has a dedicated closed serde wire, string integers, canonical byte equality, and authenticated slab/event admission. It remains open because the full V2 semantic surface is not reconstructed by production admission. |
| CT-C-002 | Open BLOCKER. `commit_parent(&mut clock, candidate)` mutates the clock and returns an outbox value; no durable storage transaction atomically installs owner/clock bytes and enqueues that outbox. |
| CT-C-003 | Partially closed. The outbox now carries parent receipt, records digest, sequence, state, and attempt count, but restore does not authenticate those fields against reconstructed parent/publication identities. |
| CT-C-004 | Partially closed. Slab/event chains and canonical bytes are validated, but scheduled/reduction/publication/outbox state remains under-validated. |
| CT-C-005 | Substantially closed for the bounded consumer: uninterrupted/restored twins and an independent maximum reconstructor exist. The crash fixture serializes a bare outbox with ordinary serde, not the canonical full restart/storage boundary. |
| CT-C-006 | Closed for accepted event identity and merged chronology by the V2 receipt reconstruction in `wire.rs`. |

## New and remaining findings

| Finding ID | Severity | Finding | Evidence | Required disposition |
| --- | --- | --- | --- | --- |
| CT-C-RR-001 | BLOCKER | Canonical V2 restore accepts unauthenticated publication/outbox lineage. `record_from_wire` verifies only payload hash; it does not reconstruct `record_id`, validate accepted-receipt membership/support, or retain authenticated support-lineage. `from_wire` copies arbitrary `publication_receipt_id` and `parent_receipt_id`; `validate` checks only that the parent ID is nonzero and that `records_sha256` matches the records. It never reconstructs the parent receipt, publication receipt, outbox sequence join, record IDs, or record-to-accepted-receipt support joins. Thus an attacker can alter these fields, reserialize canonically, and pass production admission. | `restart.rs:606-673,714-734`; `wire.rs:record_from_wire`; compare V2 schema semantic requirements and Python validator outbox checks. | Reconstruct every record, parent, publication, sequence and accepted-receipt join in Rust; add canonical forged-field poisons through `CoupledTimeRestartV2::from_canonical_json`. |
| CT-C-RR-002 | BLOCKER | The claimed atomic crash boundary is still only an in-memory API operation. `commit_parent` mutates `clock.committed` and its sequence, then returns `ParentCommitV1` containing the outbox. No API emits or durably installs one canonical aggregate containing the final owner/clock state and outbox. A caller/storage crash between persisting the mutated clock and returned outbox remains possible. | `restart.rs:313-337`. The consumer commits, then serializes only `ParentCommitV1`; its crash test separately serializes only `PublicationOutboxV1` with ordinary serde (`coupled_time_reference.rs:453-472`). | Provide one canonical durable commit aggregate/storage transaction boundary and test crash-before/after that exact write. |
| CT-C-RR-003 | HIGH | Scheduled-once and reduction restore state is copied without identity reconstruction. Scheduled receipt IDs, boundary identity/tick/result joins, reduction receipt membership/order/uniqueness, units, and independently reconstructed maximum are not validated in `from_wire`/`validate`. This permits replay suppression to be forged or a reduction to resume from invented operands/value. | `restart.rs:619-650,697-734`; production `validate_authenticated_chronology` covers only slabs/events. | Authenticate scheduled receipts and reduction state against the accepted receipt population and exact reduction operands before installing state; add production poison tests. |
| CT-C-RR-004 | HIGH | The focused canonical contract gate fails at the exact reviewed commit: the frozen-vector test reports `missing exact framed KAT for event-receipt-v2`. A final implementation candidate cannot be released with a mandatory focused contract gate failing. | Command and result below; `tests/integration/coupled_time_authority_contract.rs:112`. | Add the frozen `event-receipt-v2` exact framed KAT and rerun all invalidated authority gates before re-review. |
| CT-C-RR-005 | MEDIUM | The legacy `CoupledTimeRestartV1::preserve_complete_parent` proves only parseable JSON with the V1 schema tag. It does not enforce canonical bytes or prove a complete-parent boundary despite its method name. This is acceptable only as an opaque preservation envelope that is never treated as admitted/resumable state; the API/documentation should make that limitation impossible to misunderstand. | `restart.rs:13-29`. | Rename/narrow or explicitly document/test it as opaque pass-through only; never use it as V1 semantic admission. |

## Focused gate evidence

Executed from `/workdir/openWEPP` at exact `0bbd96d0a`:

```text
nix develop --command cargo nextest run -p openwepp-coupled-time
PASS: 8/8

nix develop --command cargo test -p openwepp-hillslope-orchestrator coupled_time_reference --lib
PASS: 3/3 (724 filtered)

nix develop --command cargo test --test coupled_time_authority_contract
FAIL: 4/5 passed; frozen_vectors_have_separating_event_constraint_and_duration_cases failed:
missing exact framed KAT for event-receipt-v2
```

DirectV10/V1 protection check:

```text
git diff 42f88d644..0bbd96d0 -- crates/openwepp-persisted-restart-v1
PASS: empty diff
```

No production file was edited by this reviewer.

---

# TERMINAL VERDICT — `9dadbe426`

Status: PASS

Evidence mode: Static + focused executable gates

Reviewed exact commit: `9dadbe426d9b9a5bdfbf6e36d604a2b1cdc68ff7`

## Verdict

All C review findings are closed at this exact candidate. Empty Sum now remains
`None`, serializes canonically as `value_bits: null`, restores as `None`, and
rejects a hexadecimal-zero empty sentinel. Nonempty Sum retains the required
positive-zero seed, checked finite ordered fold, first `-0.0` to positive-zero
bit behavior, typed slab/event/scheduled receipt membership, and exact
cross-class canonical-restored `8.25` result.

The broader canonical V2, restart-determinism, publication/outbox crash,
authority binding, scheduled-once, event chronology, and DirectV10/V1
protections reviewed in prior rounds remain intact. No new C finding was found.

## Final finding closure

| Finding set | Disposition |
| --- | --- |
| CT-C-001 through CT-C-006 | CLOSED |
| CT-C-RR-001 through CT-C-RR-005 | CLOSED |
| CT-C-TR-001 and CT-C-TR-002 | CLOSED |
| CT-C-FV-001 | CLOSED |
| CT-C-TP-001 | CLOSED |
| CT-C-450-001 | CLOSED |
| CT-C-484-001 | CLOSED: `new_sum` no longer installs zero; empty reconstruction returns `None`; canonical null roundtrip and zero-sentinel poison execute. |

## Focused gate evidence

Executed from `/workdir/openWEPP` at exact `9dadbe426`:

```text
nix develop --command cargo nextest run -p openwepp-coupled-time
PASS: 13/13

nix develop --command cargo test -p openwepp-hillslope-orchestrator coupled_time_reference --lib
PASS: 3/3 (724 filtered)

nix develop --command cargo test --test coupled_time_authority_contract
PASS: 5/5

nix develop --command cargo clippy -p openwepp-coupled-time --all-targets -- -D warnings
PASS
```

DirectV10/V1 protection:

```text
git diff 484133dc1..9dadbe426 -- crates/openwepp-persisted-restart-v1
PASS: empty diff
```

No production file was edited by this reviewer.

---

# FINAL RE-REVIEW — `484133dc1`

Status: FAIL

Evidence mode: Static + focused executable gates

Reviewed exact commit: `484133dc1aff93ac1f14889bdfe9b5947cc3e885`

## Verdict

The previously reported nonempty-sum defects are closed. Live and restore sum
now use a positive-zero seed, reject nonfinite intermediate results, and compare
exact result bits. The typed slab/event/scheduled `8.25` reduction traverses
canonical V2 serialization and restore, and a direct first-operand `-0.0` test
proves a `+0.0` result. All focused gates pass and DirectV10 remains unchanged.

One exact empty-state defect remains. `new_sum` initializes the stored result to
`Some(+0.0)`, and restart reconstruction returns `Some(+0.0)` for an empty sum.
The released authority explicitly requires an empty reduction to have no
operands and `value_bits = null`; positive zero is the fold seed once at least
one operand exists, not an empty-result sentinel. The implementation therefore
serializes an empty sum as hexadecimal positive zero and will reject the
schema-authoritative null empty sum on reconstruction.

## Closure status

| Finding | Disposition |
| --- | --- |
| All findings through CT-C-TP-001 | Closed. Typed operands, Sum, authority binding, canonical joins, crash semantics, and all prior restart protections remain satisfied. |
| CT-C-450-001 nonempty sum | Closed. Positive-zero-seeded ordered folding, `-0 → +0`, finite-intermediate checks, and canonical restored `8.25` bits are demonstrated. |
| CT-C-450-001 empty nullable state | Open as CT-C-484-001 below. |

## Remaining finding

| Finding ID | Severity | Finding | Evidence | Required disposition |
| --- | --- | --- | --- | --- |
| CT-C-484-001 | BLOCKER | Empty Sum violates the released nullable-result authority. `DiagnosticReductionV1::new_sum` sets `maximum = Some(0.0)`, so `to_wire` emits `value_bits = "0000000000000000"` with zero operands. `reconstruct_reduction(Sum, [])` also returns `Some(0.0)`. SC-COUPLEDTIME-001 requires empty reductions to use `value_bits = null` and explicitly prohibits zero as the empty sentinel. | `transaction.rs:433-441`; `restart.rs:18-31,500-519`; SC-COUPLEDTIME-001 reduction amendment. | Keep an empty Sum result as `None`. On the first operand compute checked `+0.0 + value`; on restore return `None` for no operands and otherwise perform the positive-zero-seeded checked fold. Add an empty-sum canonical roundtrip asserting `value_bits: null`, plus a poison proving zero-with-no-operands is rejected. |

## Focused gate evidence

Executed from `/workdir/openWEPP` at exact `484133dc1`:

```text
nix develop --command cargo nextest run -p openwepp-coupled-time
PASS: 12/12

nix develop --command cargo test -p openwepp-hillslope-orchestrator coupled_time_reference --lib
PASS: 3/3 (724 filtered)

nix develop --command cargo test --test coupled_time_authority_contract
PASS: 5/5

nix develop --command cargo clippy -p openwepp-coupled-time --all-targets -- -D warnings
PASS
```

DirectV10/V1 protection:

```text
git diff 450820fda..484133dc1 -- crates/openwepp-persisted-restart-v1
PASS: empty diff
```

No production file was edited by this reviewer.

---

# TERMINAL VERDICT — `450820fda`

Status: FAIL

Evidence mode: Static + focused executable gates

Reviewed exact commit: `450820fdab69fe40511fcae93e79c1007a5102ab`

## Verdict

The prior typed-membership and missing-operator gaps are substantially closed.
`AcceptedReductionOperandV1` is a closed token minted from accepted slab,
event, or scheduled receipts; `Sum` exists; live sum rejects nonfinite inputs
and intermediates; restart recognizes all three receipt populations; and the
reference consumer computes the requested cross-class `7.75` value. All
focused gates pass and DirectV10 remains untouched.

One bit-exact sum defect remains. The released contract requires sum to be a
left fold beginning at positive zero. Production initializes the first operand
with `map_or(value, ...)`, and restore reconstruction uses iterator `reduce`, so
neither path applies the required initial `+0.0`. This is observably different
for a first operand of `-0.0`: the contract result is the bits of `+0.0 + -0.0`,
whereas production retains the first operand's `-0.0` bits. The positive-value
`7.75` fixture cannot separate this alias. Additionally, the cross-class sum is
not included in the restart object's reduction vector, so the fixture does not
exercise its canonical serialize/restore admission.

## Prior closure status

| Finding | Disposition |
| --- | --- |
| All findings through CT-C-FV-001 | Closed, including authority binding, canonical V2 joins, crash phases, signed-zero maximum/minimum, and restart recognition of slab/event/scheduled receipt IDs. |
| CT-C-TP-001 typed operand portion | Closed. The token has private identity and public constructors only from accepted receipt types. |
| CT-C-TP-001 Sum presence/finite intermediate portion | Closed. `Sum` is admitted and live folding rejects a nonfinite result before mutation. |
| CT-C-TP-001 exact ordered sum/restart fixture portion | Open as CT-C-450-001 below. |

## Remaining finding

| Finding ID | Severity | Finding | Evidence | Required disposition |
| --- | --- | --- | --- | --- |
| CT-C-450-001 | BLOCKER | Sum does not begin from positive zero as required, and the cross-class fixture is not persisted/restored. Live `fold_accepted_operand` uses `self.maximum.map_or(value, ...)`; restart reconstruction uses `.reduce(|left,right| left + right)`. Both skip the mandated `+0.0` seed. A first `-0.0` operand therefore yields `-0.0` rather than the contract's bit-exact positive-zero-seeded result. The consumer's `cross_class_sum` is calculated and reported but `CoupledTimeRestartV2::new` still receives only the separate `reduction`, so `7.75` never traverses V2 canonical restart. | `transaction.rs:447-468`; `restart.rs:760-780,991-1012`; `coupled_time_reference.rs:183-205` versus restart construction around lines 210-220. | Implement sum as `try_fold(+0.0, checked_add)` in both live and restore validation, retaining nullable empty semantics separately. Add first-negative-zero, nonfinite-intermediate, empty-sum, and exact ordered-roundoff cases. Persist the slab/event/scheduled `7.75` reduction through canonical V2 and prove restored operator, operand IDs/bits, and result bits. |

## Focused gate evidence

Executed from `/workdir/openWEPP` at exact `450820fda`:

```text
nix develop --command cargo nextest run -p openwepp-coupled-time
PASS: 12/12

nix develop --command cargo test -p openwepp-hillslope-orchestrator coupled_time_reference --lib
PASS: 3/3 (724 filtered)

nix develop --command cargo test --test coupled_time_authority_contract
PASS: 5/5

nix develop --command cargo clippy -p openwepp-coupled-time --all-targets -- -D warnings
PASS
```

DirectV10/V1 protection:

```text
git diff b3066638e..450820fda -- crates/openwepp-persisted-restart-v1
PASS: empty diff
```

No production file was edited by this reviewer.

---

# TERMINAL PASS/FAIL

Status: FAIL

Evidence mode: Static + focused executable gates

Reviewed exact commit: `b3066638eb0199535b8f38c358c1798a3da5978f`

Role: final closure review of all prior C findings, with emphasis on typed
slab/event/scheduled reduction membership and bit-exact signed-zero behavior.

## Verdict

The signed-zero defect is closed: maximum and minimum now use explicit ordered
comparisons, retain the first operand on equality, compare reconstructed result
bits, and have direct `[-0,+0]` / `[+0,-0]` unit coverage. Restart validation
also recognizes accepted slab, event, and scheduled receipt IDs as valid
reduction lineage. All focused gates pass and DirectV10 remains untouched.

The released V2 reduction surface is still not completely implemented. The
canonical schema and contract admit `sum`, but Rust rejects it and exposes no
sum constructor/fold. Further, the only public live reduction fold accepts an
`AcceptedSlabReceiptV1`; there is no typed public path to fold an accepted event
or scheduled-once receipt. Restore can recognize such IDs, but valid runtime
state containing those operands cannot be constructed through the authority
API. Consequently typed slab/event/scheduled membership is validation-only,
not an end-to-end admitted operation.

## Closure status

| Finding | Final disposition |
| --- | --- |
| All CT-C, CT-C-RR, and CT-C-TR findings | Closed. Authority/model/policy binding, canonical V2 admission, chronology, scheduled identity, record/parent/publication/outbox joins, crash phases, atomic aggregate, and DirectV10/V1 protection remain satisfied. |
| CT-C-FV-001 signed-zero portion | Closed. `retain_maximum`/`retain_minimum` explicitly retain the first equal operand and compare exact result bits. |
| CT-C-FV-001 typed membership portion | Partially closed. Restore membership recognizes slab/event/scheduled receipt populations, but public live folding remains slab-only. Superseded by CT-C-TP-001. |

## Remaining finding

| Finding ID | Severity | Finding | Evidence | Required disposition |
| --- | --- | --- | --- | --- |
| CT-C-TP-001 | BLOCKER | The released V2 reduction authority is not end-to-end implemented. `restart-schema-v2.json` admits operators `maximum`, `minimum`, and `sum`, and SC-COUPLEDTIME-001 mandates ordered reconstruction for all three over accepted slab/event/scheduled receipts. Rust `from_wire` accepts only maximum/minimum, `ReductionOperatorV1` has no Sum, and `DiagnosticReductionV1::fold_accepted` accepts only `&AcceptedSlabReceiptV1`. Thus a schema-valid sum checkpoint is rejected, and event/scheduled operands cannot be legitimately produced through the public authority API even though restore membership recognizes their IDs. | Schema reduction definition at this candidate; SC-COUPLEDTIME-001 reduction amendment; `restart.rs:733-781,930-1003`; `transaction.rs:379-438`. No event/scheduled/sum production fixture exists. | Implement `Sum` with ordered positive-zero left fold and checked finite intermediates/results. Provide a typed accepted-operand API (enum/trait or explicit methods) for slab, event, and scheduled receipts while preserving replay protection. Add live-to-canonical-to-restored fixtures for each receipt class, sum, empty nullable value, and nonfinite intermediate rejection. Alternatively narrow the schema/contract through the mandatory authority amendment cycle before release. |

## Focused gate evidence

Executed from `/workdir/openWEPP` at exact `b3066638e`:

```text
nix develop --command cargo nextest run -p openwepp-coupled-time
PASS: 12/12

nix develop --command cargo test -p openwepp-hillslope-orchestrator coupled_time_reference --lib
PASS: 3/3 (724 filtered)

nix develop --command cargo test --test coupled_time_authority_contract
PASS: 5/5

nix develop --command cargo clippy -p openwepp-coupled-time --all-targets -- -D warnings
PASS
```

DirectV10/V1 protection:

```text
git diff f608dbcb4..b3066638e -- crates/openwepp-persisted-restart-v1
PASS: empty diff
```

No production file was edited by this reviewer.

---

# FINAL VERDICT

Status: FAIL

Evidence mode: Static + focused executable gates

Reviewed exact commit: `f608dbcb49fc6e76af8917816f6621d8781789d1`

Role: terminal independent canonical-V2-serialization,
restart-determinism, DirectV10/V1-protection, reduction reconstruction, and
publication/outbox crash-semantics review.

## Verdict

The two prior terminal defects are substantially corrected. V2 admission now
requires the expected authority digest and rejects mismatch. Diagnostic
reductions now persist ordered `(receipt_id, value_bits)` operands, require the
ID projection to match, reject nonfinite values, reconstruct the declared
maximum, and reject duplicate receipts. All focused gates pass and DirectV10
persisted-restart production files remain unchanged.

Release is still blocked by one exact conformance defect in the newly amended
reduction authority. The contract admits accepted slab, event, and
scheduled-once receipts as reduction operands and mandates first-operand
retention on numeric equality, including signed zero. Production validation
recognizes only slab receipts, and both live folding and restart reconstruction
use `f64::max`, whose signed-zero tie behavior does not implement the required
first-operand rule. The amendment therefore is not yet faithfully implemented,
despite the ordinary positive-value fixtures passing.

## Prior finding closure

| Finding | Final disposition |
| --- | --- |
| CT-C-001 through CT-C-006 | Closed except insofar as reduction semantic conformance is carried forward in CT-C-FV-001. |
| CT-C-RR-001 | Closed. Publication record, parent receipt, publication receipt, sequence, and outbox joins are reconstructed. |
| CT-C-RR-002 | Closed at the package boundary. The consuming durable aggregate and canonical committed checkpoint provide the required atomic handoff. |
| CT-C-RR-003 | Closed for scheduled identity, operand persistence, duplicate rejection, accepted-slab membership, and ordinary maximum reconstruction; superseded by CT-C-FV-001 for event/scheduled membership and exact tie semantics. |
| CT-C-RR-004 | Closed. Exact framed KAT and contract gates pass. |
| CT-C-RR-005 | Closed. V1 remains opaque/non-resumable and DirectV10 production bytes are untouched. |
| CT-C-TR-001 | Closed. `from_canonical_json(bytes, model, authority, policy)` and `validate` compare the expected authority; the Rust test proves wrong-authority rejection. |
| CT-C-TR-002 | Partially closed. Ordered operand values are now persisted and the maximum is reconstructed, but the implementation is narrower/different than the released amendment as detailed below. |

## Remaining finding

| Finding ID | Severity | Finding | Evidence | Required disposition |
| --- | --- | --- | --- | --- |
| CT-C-FV-001 | BLOCKER | Reduction admission and numeric tie semantics do not implement the released amendment. The contract requires each operand ID to name an accepted slab, event, **or scheduled** receipt and requires maximum/minimum to retain the first operand on numeric equality, explicitly including signed-zero equality. Production `accepted_support` searches only `accepted_slab_receipts`, so valid event/scheduled operands are rejected. Live `fold_accepted` and restore reconstruction use `f64::max`; that operation does not express the mandated first-on-equality rule and can canonicalize a `-0.0, +0.0` tie to `+0.0` rather than retaining the first operand's bits. | SC-COUPLEDTIME-001 reduction amendment, lines 403-414 at this candidate; `restart.rs:864-925`; `transaction.rs:397-409`; restore reconstruction at `restart.rs:746-764`. | Either narrow the authority through the mandatory amendment cycle or implement typed accepted event/scheduled reduction operands and exact operator-specific folds. For maximum use an explicit ordered comparison (`if candidate > retained { candidate } else { retained }`) so equality retains the first bits. Add event, scheduled, `[-0,+0]`, and `[+0,-0]` production roundtrip fixtures. |

## Focused gate evidence

Executed from `/workdir/openWEPP` at exact `f608dbcb4`:

```text
nix develop --command cargo nextest run -p openwepp-coupled-time
PASS: 11/11

nix develop --command cargo test -p openwepp-hillslope-orchestrator coupled_time_reference --lib
PASS: 3/3 (724 filtered)

nix develop --command cargo test --test coupled_time_authority_contract
PASS: 5/5

nix develop --command cargo clippy -p openwepp-coupled-time --all-targets -- -D warnings
PASS
```

DirectV10/V1 protection:

```text
git diff bb8fdc7eb..f608dbcb4 -- crates/openwepp-persisted-restart-v1
PASS: empty diff
```

No production file was edited by this reviewer.

---

# TERMINAL RE-REVIEW

Status: FAIL

Evidence mode: Static + focused executable gates

Reviewed exact commit: `bb8fdc7eba26dac2dcc95d981a33daee645231e5`

Role: independent canonical-V2-serialization, restart-determinism,
DirectV10/V1-protection, and publication/outbox crash-semantics review against
all prior C findings and the latest V2 phase/scheduled amendments.

## Verdict

The corrected candidate closes the previously reported publication-record,
parent-receipt, publication-receipt, sequence, scheduled-once, active/committed
phase, and canonical crash-roundtrip gaps. All requested focused gates pass,
including the formerly missing `event-receipt-v2` KAT. DirectV10 persisted V1
source remains untouched.

Terminal release is still blocked by two fail-closed restart defects. The public
V2 admission API has no expected authority digest and therefore accepts a
canonically reserialized checkpoint with an arbitrary `authority_sha256`.
Diagnostic reduction `value_bits` is likewise copied and re-emitted without
reconstruction from retained operands. This directly contradicts the V2
schema's mandatory semantic rules and permits a forged peak to survive restart.

## Finding-by-finding closure

| Finding | Terminal disposition |
| --- | --- |
| CT-C-001 | Closed for the V2 structural wire and canonical reserialization path. Superseded by CT-C-TR-001/002 for two remaining semantic bindings. |
| CT-C-002 | Closed at the authority API boundary. `commit_parent` now consumes the clock and returns one `DurableParentCommitV1` containing the committed clock and outbox-bearing commit; the canonical committed checkpoint enforces `CommittedParent`, one outbox, no pending buffer, and the successor sequence. |
| CT-C-003 | Closed. Outbox parent/publication receipts, sequence, records digest, state, attempt count, and canonical crash transitions are retained and validated. |
| CT-C-004 | Partially closed. Slab/event/scheduled chronology, phases, owner set, record support, and outbox joins are validated. Authority digest and reduction value remain open below. |
| CT-C-005 | Closed for the bounded reference consumer. Uninterrupted/restored twins, independent maximum reconstruction, rejected/nominal aliases, and full canonical committed/delivered/acknowledged crash restores execute. |
| CT-C-006 | Closed. Event identity and merged owner/clock chronology remain authenticated. |
| CT-C-RR-001 | Closed. Record IDs/support membership and parent/publication/outbox joins are reconstructed in production Rust. |
| CT-C-RR-002 | Closed at the package abstraction boundary by the consuming durable aggregate and canonical committed checkpoint/crash tests. Actual backing-store atomic-write mechanics correctly remain an adopter/storage concern. |
| CT-C-RR-003 | Partially closed. Scheduled receipt identity and reduction receipt membership are checked; reduction value reconstruction is still absent (CT-C-TR-002). |
| CT-C-RR-004 | Closed. The exact framed `event-receipt-v2` KAT is present and the focused contract gate passes 5/5. |
| CT-C-RR-005 | Closed narrowly: V1 remains an explicitly byte-preserving, non-resumable opaque envelope, while V2 is the admitted chronology. DirectV10 production files remain unchanged. |

## Remaining terminal findings

| Finding ID | Severity | Finding | Evidence | Required disposition |
| --- | --- | --- | --- | --- |
| CT-C-TR-001 | BLOCKER | V2 restart admission does not bind `authority_sha256` to an expected authority. `from_canonical_json` accepts only expected model and controller policy arguments; `validate` compares those two but never compares `self.authority_sha256`. Because `from_wire` stores the untrusted authority value and `to_wire` emits the same value, canonical reserialization equality does not reject a forged but well-formed authority digest. | `restart.rs:570-579,803-814`; public signature `from_canonical_json(bytes, model, policy)`. V2 schema requires `authority_sha256`, and restart admission is required to bind policy/authority identity before installation. | Add an expected authority digest to the public admission boundary and reject mismatch; add a Rust poison that mutates only `authority_sha256`, canonically serializes, and proves rejection. |
| CT-C-TR-002 | BLOCKER | Production restore does not reconstruct diagnostic reduction `value_bits`. `from_wire` accepts an arbitrary finite-or-nonfinite f64 bit pattern and pairs it with accepted receipt IDs; `validate` checks only that each ID names an accepted slab. It does not require uniqueness/order and cannot recompute the maximum because operand values are not retained in the wire. A forged peak therefore admits and affects published diagnostics after restart. | `restart.rs:671-688,854-863`; `DiagnosticReductionV1` stores only final maximum plus receipt IDs (`transaction.rs:379-418`). The V2 schema explicitly requires that reduction operands reconstruct `value_bits` in declared order. | Persist authenticated operand values/bits (or an equivalent reconstructable accepted operand ledger), validate order/uniqueness/finite values, independently recompute the maximum during admission, and add forged-value/duplicate/reordered production poison tests. This may require an authority/schema amendment because the current V2 wire lacks operand values. |

## Focused gate evidence

Executed from `/workdir/openWEPP` at exact `bb8fdc7eb`:

```text
nix develop --command cargo nextest run -p openwepp-coupled-time
PASS: 10/10

nix develop --command cargo test -p openwepp-hillslope-orchestrator coupled_time_reference --lib
PASS: 3/3 (724 filtered)

nix develop --command cargo test --test coupled_time_authority_contract
PASS: 5/5

nix develop --command cargo clippy -p openwepp-coupled-time --all-targets -- -D warnings
PASS
```

DirectV10/V1 protection:

```text
git diff 0bbd96d0..bb8fdc7eb -- crates/openwepp-persisted-restart-v1
PASS: empty diff
```

No production file was edited by this reviewer.

# B01 WB14 captured witness: predicate correction

DRAFT FOR OWNER ADOPTION. No execution is authorized by this document alone.

## Active checkpoint, exact source, scope, checks and stops

**Same checkpoint:** B01-WB14-AVAILABLE-SOURCE-RED-WITNESS.
**Owning record:**
`docs/work-packages/20260912-b01-wb14-available-source-red-witness-001/package.md`.
**Deliverable:** finish the existing offline checker/test inventory, apply the accepted checker to the retained current-run evidence, and obtain both independent affected-scope verdicts. No new package or runtime repair.

**Immutable starting revision:** `76d6d17cf09ddb2a2a6d150c91eb409128305af7` in `rogerlew/openWEPP`.
The checker and tests below are in the owning package's `artifacts/`:

- `check_witness.py`: SHA-256 `e7f74c26201949b9cf428600001d2c6324109d3a34adc98fe05cc55ea544025e`.
- `test_check_witness.py`: SHA-256 `70caea902057fd10a1aac5490cc13760e1b701707fa33cad051dff057816689a`.
- `current-run-boundary-packet.json`: SHA-256 `d8f1fe07d73ac09a6eb002b1a49a714cad7a0af1bb58ffab7612282ea002cc76`.
- Current raw observation: SHA-256 `7a944817acf43be2ba018f670efbd3490a0cf1f3629b0011ae26449de2332ba9`, 3,699,473,934 bytes.

**Actual runtime source:** unchanged `reconciled-available145-r1`, actual 927-entry map SHA-256 `c7aea0707d0c6a0876ec4bee5a7fcf15e751a092e086f517ced7edba8282b71e`.
**Frozen binary:** SHA-256 `958e61764d373a2c183764e7f7ff41607a32ef74a2af5cd7b3f299bf1123906f`.
These identities are different from the repository evidence revision. Historical test substitutions and their UNKNOWN semantic deltas remain unchanged.

**Permitted changes:** the existing offline checker/tests, their necessary evidence, the owning record and affected locator. Raw packet/corpus, extraction, source, binary, fixtures, collector, prior findings and authority are immutable. Preserve the failed cut through existing Git history and retain new verification outputs before another edit.

**Acceptance/checks:** the full existing named positive/negative inventory, exact intended refusal predicates, source-defined identity/state relationships, current packet/receipt/input/selection joins, Python syntax/tests, final checker application, and distinct independent correctness and QA fix verification. All earlier unmet acceptance remains binding. Reuse unchanged raw-custody and build/selection evidence; do not re-export the packet or repeat corpus hashing solely because checker tests changed.

**Runtime allowance: ZERO.** Original invocation is exhausted at 1/1. No Rust builds/tests, binary executions/listings, collectors, simulations, checkpoint operations, cadence edits, source adoption, or cleanup. Offline Python tests and evidence adjudication only.

**Proposed owner amendment: cycle ceiling only.** Raise this SAME checkpoint's cumulative unsuccessful-cycle ceiling from seven to nine. Add NO active time. Carry 74m20s cumulative active time, including 26m39s consumed from the existing 60-minute adjudication supplement: 33m21s remains at the reference cut. Deduct all newly consumed reading, edits, checks and reviews from that balance; exclude only recorded pure waits. Preserve all seven failures and the historical overrun against four. Neither this document nor a new session resets consumption.

Stop at the remaining active-time limit, nine cumulative failures, a new downstream/out-of-scope defect, inaccessible necessary evidence/review, material identity drift, or any need to weaken frozen acceptance. Preserve and return; do not keep editing after failed verification without reporting its consumed cycle to Astra.

## Finite corrections

### B01-CONT-011: isolate the intended negative predicate

The failed `wrong_boundary_valid_json` changes only the guard's serialized input. `check()` first compares guard/caller bytes and correctly rejects `input_pair` before checking the boundary. Keep that ordering.

For a semantic input-boundary mutation, decode the common input once, change the intended numeric value, encode it once, and install identical copies in both guard and caller. Preserve valid surrounding serialization. Update redundant diagnostic copies when needed to keep the test focused. Prove the intended value changed, not merely the formatting. Keep a separate one-copy mutation expecting `input_pair`.

Apply the same reasoning to every existing case, not just the first failure:

- `u128_out_of_domain` currently changes only one input copy and is masked by `input_pair`.
- Beginning-cumulative mutations currently change only the guard beginning and are masked by the raw beginning-pair comparison.
- Persistent/candidate cumulative mutations encounter whole-state `parent` equality before the later cumulative checks.
- The test named `wrong_ofe` mutates the guard OFE, whose earlier predicate is `guard`, not the authority-map `ofe` predicate.

Separate coherent cumulative-domain tests from deliberately inconsistent owner-copy tests. For the latter, name and assert the earliest intended parent/state identity refusal. Do not relabel a masked test as domain coverage. Shared nonzero, negative, nonfinite and wrong-type cumulative cases must still prove the initial-state requirement independently. Retain coordinate coverage and the original full reviewer inventory.

Do not weaken an expected code to whatever happened, reorder checker guards merely to satisfy a test, accept any exception, or drop cases. Use readable named cases. Test mutation helpers must check real changes; binary64 cases must distinguish signed zero by bits where required.

Use standard test facilities such as `unittest` subtests to report the complete finite case table in a verification invocation rather than terminate at its first assertion. Missing/unexecuted cases remain explicit. This is not a new testing framework or permission to conceal multiple corrective edit/verification cycles as one.

### B01-CONT-012: require every identity join

At `check_witness.py:109`, the expression `a != b != c` is not an all-equal check. It rejects only when both adjacent inequalities hold. It misses an authority-only mismatch and a binding-only mismatch.

Read the three mandatory `coupled_parent_transaction_sha256` values explicitly with the existing typed 32-byte validator. Reject unless ALL THREE are equal. Independently test authority-only, prefix-only and binding-only mutations, with otherwise valid payloads and a changed byte that is guaranteed different.

The outstanding correctness review also requires a present, nonzero `accepted_slab_sha256`. Iterating over keys that happen to exist does not enforce mandatory digest presence. Explicitly require the source-defined mandatory fields and the nonzero accepted-slab identity; add missing, malformed and all-zero negatives. Do not compare that accepted-slab identity to an unrelated owner digest.

Preserve the reviewer-established distinction: authority beginning-owner digest joins prefix beginning-owner digest; prefix ending-owner digest joins binding parent-beginning complete-owner digest. Those two owner groups are not generally equal. No full-prefix cryptographic authentication, rollback or conservation claim is added.

## Execution and review

Astra coordinates; one implementer edits. Before editing, trace the full existing case table against the current check ordering. Correct the finite defect set together, retaining every original acceptance obligation. At most one corrective cycle is delegated at a time; failed verification returns immediately with its log and updated ledger before another edit. Routine syntax/test fixes count.

Reuse `/root/witness_correctness_replacement` and `/root/witness_qa_replacement` for their affected fix verification. Finished sessions are not assumed unavailable; use supported continuation. No new reviewer wave is required or authorized. Their prior findings and accepted custody portions remain attributable. An actual inaccessible required slot stops acceptance; never fabricate a signoff.

QA's completed stream/membership verification is reusable for the unchanged packet/corpus. It does not certify the new checker. Correctness must verify the final semantics and identity checks; QA must verify actual named-case execution and the final witness evidence. This adviser conversation satisfies neither independent role.

After complete checker tests, apply the checker to the existing current packet and actual retained run/receipt/binary/listing inputs. Keep its internal pending-review result distinct from witness acceptance. Witness PASS requires both independent scopes and every retained predicate. The runner remains FAIL with `execution_valid=false`; missing `outputs.json` is retained, not inferred into rollback proof.

## Return

Return the exact candidate diff/hash, named per-case results (including any unexecuted cases), current-evidence checker result, both independent dispositions, unchanged custody references, and consumed time/cycles in the SAME package record. Carry existing scoped evidence commit/push permission and verify changed remote files. No new simulation, source adoption, cadence repair, full-parent/restart/conservation acceptance, or production readiness follows.

Reference evidence at the pinned revision: `checker-submission-7-result.json`, `checker-submission-7-execution.txt`, `correctness-replacement-review.txt`, `qa-replacement-current-custody.txt`, and the existing checker/test/authorization records. Read applicable current instructions and relevant canonical authority; do not repeat accepted historical review without an affected change.

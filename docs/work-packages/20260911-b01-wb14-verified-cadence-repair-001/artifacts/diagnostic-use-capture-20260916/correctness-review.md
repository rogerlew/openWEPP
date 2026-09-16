# Correctness review — diagnostic-use capture corpus

**Reviewer:** `/root/diagnostics_correctness` (Sol/high), independent of the
capture/export writer, orchestrator, and QA reviewer. This is an attributable
review of the new corpus and does not claim continuity with a prior reviewer.

**Reviewed identity:** frozen candidate tree
`ecc48d5235d8d53af496856459e3eaa29cca2dda2511697350a67bac172f9592`,
cumulative patch
`4559c7d21fbcdb9314845e6a5c19940d051d0ef861e48c7cb2a4ae7e51020c5d`,
supplemental input custody
`2e9e27eeb0a355672e7a74c8fd645a2ce95182ae88bf78741bc8f98ecac9b082`,
and frozen executable
`1c733d1ca2fea700b2f34f06cd29a89a6f7d66977a042ddfb371816a320a0955`.

**Evidence class — Static:** I inspected the frozen production recorder path,
observer lifecycle and selector, the applicable inactive-prefix authority, the
offline selector/export/inspection method, and the retained source/build/input
receipts. **Ran-readlogs:** the build, exact-harness listing, one authentic model
invocation, collector, full-stream inventory, selected export, and offline
inspectors were run by the orchestrator or implementer; I inspected their terminal
records and payloads. **Ran (reviewer):** I used bounded read-only JSON/event-shard
parsing and byte/digest comparisons over selected exports to verify the joins
reported below. I did not run Cargo, Rust tests, a model, a native importer, a
probe, or a second full-stream hash.

## Findings

### High — the captured workload retains the unresolved WB14 authority mismatch

- **Location:**
  `terminal/run.json`, exported records 123091–123093, and
  `docs/specifications/science-contracts/contracts/SC-SURFACELIQUID-001.md`
  `INV-SURFACELIQUID-035` / `SC-COUPLEDTIME-001.md`
  `INV-COUPLEDTIME-031`.
- **Evidence:** the authentic runner ended `101`, `execution_valid=false`, with
  `SURFACELIQUID-E-008 IngressCandidate: WB14 day or interval continuation
  mismatch`. The captured input and parent working state are day 4 / interval 22,
  while the WB14 guard expects day 0 / interval 0. The guard reports
  `accepted_parent_local_projection=false`, `initial=true`,
  `finalize_parent_interval=false`, and `state_mutated=false`. The caller repeats
  the same typed E008. The contract requires a complete authenticated inactive
  prefix, no inactive WB14 physics or receipt, and physical child ordinal zero.
- **Disposition:** this is the intended preserved scientific blocker, not a
  recorder or acquisition regression. Fail-closed behavior is visible and no
  mutation is reported, but the corpus does not make the supplied outer authority
  acceptable. Native reconstruction, cadence repair, and scientific qualification
  remain blocked.

### Low — the first inspector's archive day field is a literal label

- **Location:** `inspect-summary.json`, each archive row's
  `archive_integrity.day_index`.
- **Evidence:** the field contains the string `"day_index"` rather than 0, 1, 2,
  or 3. This makes that one summary field unsuitable as archive/day binding
  evidence.
- **Disposition:** nonblocking for this acquisition. `export-summary.json` binds
  the rows to top-level day scalars, and I independently read the archive-entry
  event shards and verified actual days 0–3 plus their owner/root/chain joins.
  `structural-findings.json` records the erratum. No claim below relies on the bad
  literal.

No new high- or medium-severity correctness, safety, security, numerical,
serialization, typed-error, or science-contract defect was found in the recorder
placement, bounded acquisition, export, or selected-payload custody.

## Static recorder and selector assessment

The production snow-free hook is outside test-only scaffolding at
`snow_stage3_v11_real_parent_execution.rs:154`. It records the actual parent,
consumer, clock, transformed prepared support, owner/history inputs, reduction,
ledger, provisional receipt and binding before construction and execution of the
provisional real-consumer stack at line 182. It therefore observes the pre-child
state without advancing that physical child or publishing a result. The separate
test-only live-input oracle remains gated by `#[cfg(test)]`; production recording
itself is retained by `physical_enabled()`.

`snow_stage3_v11_current_context_capture.rs:20-23,1360-1363` selects exactly day 4,
interval 22 and half-open support
`[385920000000000,385980000000000)`. Its snow-free capture records the actual
ordered prefix inputs/result, deferred soil custody, ending snow owner, pending
parcels and prepared support; its declared posture explicitly supplies no
installable capability. The runner enables the observer only for exact
`OPENWEPP_ACCURACY_PHYSICAL=1`, starts the spooled observer before model execution,
and finishes it after the returned error, preserving the failing path. I found no
science arithmetic, clamp, unit conversion, solver, admission, or publication
change in this acquisition. The frozen source remained unchanged after the run.

## Acquired corpus and custody

The terminal receipts bind B01 / `gradual_warm_tail7`, one OFE, the original case
SHA-256 `c48cc560dff31102085ebcf852225b2063a1629ad96cfa9f7283364df44fbf3f`,
physical observation enabled, a 3,600-second child timeout and no retry. The
collector exited zero; the runner exited 101 after 1,436.636 seconds, with no
timeout or infrastructure error. `observation_complete=true` and 123,094 physical
rows are explicit. `post-model-custody.json` confirms the source tree, patch,
binary, original inputs and all 13 generated fixture files remained unchanged.

The whole-stream inventory is COMPLETE over all 8,701,419,029 observation bytes,
SHA-256
`1555efe1b592081a3dd67639d71a18e4e67f518e24b93a95ddfe18de2023d87c`,
123,094/123,094 records, valid syntax and EOF, with zero anomalies. The selected
export independently streamed to the same byte count, hash, row count and EOF and
is COMPLETE. It contains exactly the required eight rows: archives
27431/54078/79222/107936, provider/day 108033, target 123091, guard 123092, and
caller 123093. Selection is semantic by kind, phase, day, interval and support,
not by a historical ordinal. Total selected payload is 1,796,372,827 bytes under
the 4-GiB limit: 1,330,153,960 archive bytes, 466,164,750 compact JSON bytes, and
54,117 event bytes. All eight records carry their own physical ordinal and the
same process 996446, session 1, and `ThreadId(2)`; capture ordinal equals the
inventory ordinal.

I independently verified these decisive structural joins:

- The four archive entries are actual days 0, 1, 2 and 3. Their accepted endpoints
  are one through four days, their next parent sequences are 48/96/144/192, and
  each next entry's beginning owner, previous archive root and previous ordered
  chain exactly equal its predecessor's ending/resulting values. The final day-3
  owner, record, parent receipt, archive root and ordered chain exactly equal the
  prepared day-4 prefix. The four external canonical-record files have the lengths
  and SHA-256 values declared by their archive entries. These are byte/hash and
  chronology joins; the canonical record contents remain semantically unvalidated.
- The day-4 provider has 48 positive, contiguous 1,800-second supports spanning
  `[345600000000000,432000000000000)`. Provider support 22 is
  `[385200000000000,387000000000000)`, which contains the captured transformed
  child. Its provider/GSI receipt is day 4 and its forcing, beginning/ending cursor
  and two destination forcing receipts are present. Archives, provider and target
  share exact run identity
  `59bce9a19ec0402075d3ca88c00594f5bea65e82df3efde86a21cc8db91680e9`
  and topology identity
  `a8890ab71b502ee9afe95b2cc14681ba9627fd946276c524668142c757801a75`.
- Target 123091 is the unique `snow_free` success at day 4 / interval 22 with
  support `[385920000000000,385980000000000)`. Its prepared support is identical,
  `covered_projection` is null, and no destination capability is installed. The
  coupled clock and parent checkpoint both decode with accepted-until
  385920000000000. Seven clock owners, six current consumer owners, the complete
  owner ledger, parent checkpoint, native lane/soil/hydrology/consumer constructor
  operands, reduction, provisional receipt, provider cursor, pending parcel and
  deferred native soil custody are all retained.
- The target's phase diagnostics contain eight contiguous ordered owner joins from
  parent start 385200000000000 through prefix end 385920000000000, followed by the
  captured child start. The current stage-3 state, ending snow owner, one pending
  terminal parcel, parent support and authenticated inactive-prefix fields are
  present. This is structurally consistent with the inactive-prefix authority; it
  is not a semantic proof that all contract obligations hold.
- The target's coupled binding and the caller's binding are equal after exact
  numeric normalization; their raw JSON bytes differ only because one transport
  encodes the time/bit integers as JSON numbers and the other as exact decimal
  strings. Guard and caller input bytes are exactly equal, guard and caller
  beginning bytes are exactly equal, and caller beginning and working bytes are
  exactly equal. Target, guard and caller are consecutive observer ordinals in the
  same process/session/thread, so no intervening physical observation separates
  the pre-child capture from the typed failure.

The local export, decoded context, and member-shard directories are durable and
source-bound. The compact publication manifest retains source/member positions,
byte counts and hashes. The four archive binaries and the 457,913,228-byte native
consumer constructor payload remain durable local-only artifacts with explicit
manifest entries rather than compact published copies.

## Evidence boundary and residual risk

- **Present/exported:** all eight selected records and every target top-level
  member are retained with original physical ordinal and member position.
- **Decoded:** bounded canonical/typed byte members for the clock, checkpoint,
  configuration, ledger, provisional receipt and guard/caller states were decoded
  into separate source-bound files. Large native/archive values remain exported
  without whole-object semantic decoding.
- **Structurally checked:** full-stream size/hash/EOF/row count, selector
  multiplicity, export budgets, archive byte length/hash, archive-prefix joins,
  provider support continuity, target support/clock joins, record metadata and
  guard/caller exact-byte joins were checked. This review also checked the target
  versus caller binding after exact numeric normalization.
- **Semantically unvalidated:** no native importer, restored-context constructor,
  successor execution, restart round trip, conservation reconstruction, WB14
  physical child, finalization, native probe, or model replay was run. The capture
  cannot establish scientific validity, installability, uninterrupted/split
  restart equality, absence of all inactive physics, or a successful physical
  ordinal-zero child.

The very large raw stream, archive binaries and native-consumer operand remain
local; their loss would remove unique reconstruction inputs even though hashes and
compact provenance are published. The two complete full-stream passes and selected
member checks are sufficient for bounded acquisition acceptance, but they are not
a remote backup or release artifact.

The original regression remains **FAIL**. Strict Clippy remains **FAIL**. The
separately adopted complete matched-diagnostic criterion remains **PASS**. None of
those outcomes is changed by this corpus.

## Verdict

**Bounded corrected-recorder acquisition and custody: PASS.** Exact source,
binary, input, observer activation, terminal outcome, complete raw-stream custody,
unique target selection and the required provider/archive/history/guard/caller
operands are preserved and structurally coherent. The one low-severity inspector
label defect is independently bypassed by source-bound row evidence and does not
block this acquisition.

**Native restoration: NOT RUN / INCOMPLETE. Scientific qualification: NOT
ESTABLISHED / HOLD. Authentic runner: FAIL. Original regression: FAIL. Complete
recorder preparation and overall package: HOLD.** The authentic model still fails
closed on the day-4/interval-22 versus 0/0 authority mismatch. The authorized
acquisition deliberately did not run a native importer or probe, and it produced
no successful physical successor, restart, conservation, or new regression
evidence.

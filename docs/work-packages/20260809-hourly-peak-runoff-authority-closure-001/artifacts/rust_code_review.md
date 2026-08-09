# Rust Code Review

Status: `complete`

Review target: exact commit
`949349e7055c5d19277eeb708401c4614a52cd77`.

## Findings

### Critical -- daily local-liquid debits are projected onto a merged hourly source ledger

Routed melt is now correctly seeded as hourly WB14 supply, and inter-OFE runon
is added to the same `hourly_additional_supply_m` array
(`crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/00c_day_input_builder_impl.rs:348-408`,
`crates/openwepp-hillslope-orchestrator/src/direct_runtime/runoff.rs:657-690`).
The ledger stops preserving source ownership after that merge, but two later
debits are local-liquid-only:

- On an active pure-melt day,
  `resolve_r4m_same_pass_infiltration_m` reconstructs a daily infiltration
  amount from local `liquid_input_m` (`runoff.rs:514-565`). The caller removes
  the difference from the earliest positive bins of the merged melt-plus-runon
  excess (`runoff.rs:231-239`, `:1899-1919`). With earlier runon and later melt,
  this debits runon and leaves melt as the residual runoff even though the daily
  scalar closes. The modeled peak hour can therefore be swapped silently.
- `frost_retained_local_liquid_m` is a daily local-liquid amount, but
  `reconcile_hourly_partition_runoff_profile` removes it proportionally from
  every positive merged WB14 runoff bin (`runoff.rs:1432-1474`). No hourly
  frost-retention producer or cited process authority establishes that
  proportional timing. It can debit runon as well as local melt and changes the
  production peak magnitude by assumption.

Amending `SC-WATBAL-001#INV-WATBAL-103` to prescribe proportional allocation
does not supply the missing physical clock; the package prohibits proxy or
synthetic production timing. Preserve source-tagged hourly supply/partition
operands through these debits and remove local liquid in its modeled producer
hours. If the frost owner cannot produce that timing, material daily-only frost
retention must hard-fail the hourly peak claim. Add a real mixed
melt-plus-runon winter vector with sources in opposing hours and a material
frost-retention vector.

### High -- threshold and tolerance precedence can erase positive sources or missing timing

Positive runon is not admitted when its combined total is at or below
`1e-12 m`, and positive runon with no WB14 producer also returns success without
admission (`runoff.rs:657-667`). WB14 likewise uses the additional supply only
when its raw sum is greater than `1e-12 m` (`runoff.rs:1805-1818`); the input
validator never checks the 24 additional-supply values (`runoff.rs:1910-1959`),
and the binning helper masks negatives with `max(0.0)` (`runoff.rs:1743-1761`).
Thus small positive timed melt/runon can disappear, while malformed negative or
NaN supply can be ignored instead of failing at this boundary.

The downstream reconciliation broadens the loss: with an empty hourly ledger,
any positive `partition_runoff_m` up to
`24 * 1e-9 m * max(1, scale)` is changed to exact zero
(`runoff.rs:1451-1464`). This bypasses the earlier source-free limit of
`1e-12 m` (`runoff.rs:1409-1421`) and uses `TOL-WATBAL-009` to absorb missing
hourly timing, although the contract explicitly says that tolerance cannot
absorb a missing source (`SC-WATBAL-001:870,873`). A source-free positive value
between `1e-12 m` and `24e-9 m` should reach the WB16 missing-timing hard error,
not become a dry day.

Validate every additional-supply bin before summation, admit every finite
positive source, reject positive runon without a WB14 producer, and remove the
positive-empty-ledger canonicalization. `TOL-WATBAL-009` may reconcile two
present ledgers; it cannot stand in for one that is absent. Add just-below/above
threshold vectors for melt and runon, NaN/negative vectors, and an empty-hourly
ledger with positive daily runoff at `2e-12 m` and near `24e-9 m`.

### Medium -- WB16 guard taxonomy is only partially wired and duplicates its authority

`map_wb16_peak_guard` repeats the three WB16 code strings and stores the code in
the generic `DirectKernelGuardFailure.phase` field (`runoff.rs:1557-1569`,
`direct_runtime/04_audit_error_helpers.rs:254-269`). The existing typed
`Wb11HydrologyKernelGuardError` already owns the phase-to-code mapping
(`hydrology/02_guard_errors.rs:168-240`), but the WB16 operator does not use it.
Additionally, missing R4A and R4O producers are returned before the adapter and
therefore still emit generic `MissingDirectUpstream` messages
(`runoff.rs:833-859`). The focused test proves only the missing-hourly-shape
route to `E-001` (`direct_runtime_dc01.rs:159-170`); it does not prove all actual
span inputs or `E-002`/`E-003`.

Centralize WB16 errors in the existing typed guard family and adapt the entire
R7D6 span once. Remove the duplicate string map and assert code plus boundary
class for missing producers, non-finite depths, and domain/closure failures.

### Medium -- the public zero-basis guard is asymmetric

The publication helper now correctly rejects a positive public runoff basis
when the peak shadow has zero runoff. The reverse mismatch remains accepted:
when `q_runoff_m == 0` and the shadow runoff is positive, the helper scales the
peak rate to zero but returns the shadow's positive duration
(`crates/openwepp-hillslope-orchestrator/src/direct_runtime/01_publication.rs:589-635`).
That publishes zero run volume/peak with a nonzero rectangular-equivalent
duration. The current producer sets `q_ofe_m == q_runoff_m`, but the publication
seam is responsible for failing closed on basis drift. Add the symmetric
zero/nonzero closure guard and test it; the existing test covers only the other
direction (`01_publication.rs:970-1023`).

### Medium -- peak arithmetic is duplicated, and key tests exercise the copy

The production closing-depth operator at `runoff.rs:1616-1649` and the
test-only weight operator at `runoff.rs:1652-1702` independently implement the
same maximum-hour, earliest-tie, rate, duration, and 24-hour-support algorithm.
The concentrated/spread and malformed-weight tests call the test-only copy
(`direct_runtime_dc01.rs:320-353`), so they can remain green if production
arithmetic drifts. This is substantial mirrored numerical logic and lacks an
intentional-duplication justification. Retain one peak operator and make the
vectors feed its real closing-depth input; keep weight closure as a separate
small validator if it is still needed.

### Medium -- resumable records do not validate expected output cardinality

The census now has good atomic replacement and binds reusable records to record
schema, case, plan, binary, and input hashes. However, `record_matches` accepts
any common array length, including zero or a consistently truncated calendar
(`docs/work-packages/20260809-hourly-peak-runoff-authority-closure-001/tools/topanga_openwepp_census.py:130-154`).
`validate_event_rows` also accepts an empty event set (`:356-425`), while the
completion flag checks only plan identity/inventory (`:496-508`). Ran evidence
confirmed that a provenance-correct record containing four empty arrays returns
`record_matches=True` and that empty event validation succeeds. Such records
can therefore be reused and reported as a complete 1,088-trial cohort without
proving that any daily output was retained.

Bind each receipt to the expected climate/output calendar or at minimum a
nonzero expected row count and calendar digest. Reject shortened, duplicated,
or empty records, and require positive event-pair evidence before the package's
continuity claim. Add valid-atomic-reuse, empty, truncated, wrong-calendar, and
interrupted-write tests.

## Evidence

- Static: reviewed the exact commit and its delta from `c7dbfefe7`, including
  runtime, publication, runner, output/HBP consumers, contracts, tests, and
  census harness. The checkout was at the requested commit; unrelated untracked
  package logs were not modified.
- Ran: `git diff --check c7dbfefe7..949349e7055c5d19277eeb708401c4614a52cd77`
  -- PASS.
- Ran: 17 focused orchestrator runoff/peak tests -- PASS, 17/17.
- Ran: focused public peak-boundary test -- PASS, 1/1.
- Ran: census provenance/schema unit tests -- PASS, 4/4.
- Ran: canonical plan identity check -- SHA-256 matched the embedded value;
  1,088 unique eligible trials and 280 baselines were present.
- Ran: an isolated empty-record probe reproduced the semantic resume gap:
  `record_matches=True`, empty validation accepted.
- Not run: heavy workspace quick/full/doctest gates or the full Topanga cohort;
  those remain separate package closure evidence.

## Resolved Prior Findings And Residual Risk

- The original gross-melt double count is removed: routed melt enters WB14 as
  additional liquid once, and peak/transfer/publication consume only
  post-partition WB14 runoff plus WB19 saturation return.
- The explicit uniform surface/lateral runon fallback is removed; material
  positive runon with a zero shape now returns a typed missing-upstream error.
- `DirectPeakRunoffInputs` and the WB16 `ealpha` production derivation are gone.
  Retained `tstar`/`qpstar`/`vstar` fields are fixed-zero, unconsumed historical
  diagnostics and do not control the production peak.
- The normal public path applies area once and the real p61/p102 tests now
  reconstruct HBP peak from 24 hourly volumes. Those tests do not cover mixed
  winter sources, daily-only frost debits, the widened empty-ledger tolerance,
  or the reverse zero publication basis.
- Atomic NPZ replacement and hash-bound resume provenance are materially
  improved, but the semantic record-shape finding prevents treating reuse alone
  as terminal evidence.

## Verdict

`HOLD` -- the Critical mixed-source debit can silently retime the production
peak while daily runoff still closes, and the High threshold/tolerance path can
erase positive source-backed or missing-timing runoff. Commit `949349e7055c...`
is not acceptable for Critical closure until those paths and their real
consumer tests are corrected.

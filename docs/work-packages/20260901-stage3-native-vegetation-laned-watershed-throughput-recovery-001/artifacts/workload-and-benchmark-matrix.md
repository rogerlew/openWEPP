# Workload and benchmark matrix

Status: `MEASURED PRIOR-SOURCE MATRIX FAIL — CURRENT QUALIFICATION NOT COMPLETE`

Evidence mode: `Static + Ran`

Early expected-red rows are historical intake. The completed 1/10/19 matrix
and later diagnostic measurements are recorded below; the matrix failed
time/RSS and predates the latest bounded increments.

| Workload | Input/fixture | Required proof | Current result |
| --- | --- | --- | --- |
| Frozen/mixed DFF day | `disturbed_burn/forest_high_severity_loam`, ksatadj-on timing arm | release completion, counters, closure, non-CoE Stage 3 | FAIL at `1800..1860 s`, 33.56 s optimized test |
| Snow-free/wet-canopy day | package-owned vectors derived from native forest fixtures | native vegetation/ET owner and no legacy/PMET-only substitution | NOT RUN |
| Complete 365-day snow season | parameterized `stage3_runner_qualification` | accumulation, persistence, meltout, support/evaluation histogram | NOT RUN |
| Native 45-year climate | `dff_ws1_native_forest/hjandrews_conifer_forest` (`1980..2024`, management `nyears=45`) | exact day/year assertion, native owner, release CLI completion | FAIL before day zero: typed missing Stage-3 owner seed; this fixture cannot carry a century claim |
| 1/10/19-OFE Lane D | package-owned positive native Stage 3/Lane-D scale fixture | real production selection, completed-run batches, per-OFE route/publication checks, `T10/T1`, `T19/T10`, CPU/wall/RSS, hashes and counters | RAN complete 5-warm-up + 30-measured-batch release matrix: science/scaling PASS; time, Stage-3 fraction, 10/19-OFE peak RSS, and aggregate RSS return FAIL |
| 10+-OFE year | `representative-10ofe-100year-workload.json` + `representative_ten_ofe_complete_year_real_runner` | complete 365 days, outputs, ledgers, exact owners | RECIPE/INTAKE PASS; release run pending |
| 10+-OFE 100-year hillslope | same checked-in recipe + `representative_ten_ofe_hundred_year_real_runner` | assert exactly 36,525 days and 10+ OFEs, complete run under 210 s wall, hashes, counters, closure | RECIPE/INTAKE PASS; release run pending |

No existing committed fixture combines native vegetation, 10+ OFEs, exact
Stage 3 owner seed, real Lane D, and 100 years. Phase 2 must add one checked-in
package-owned fixture from existing authoritative components. Any repeated
climate construction must record the source interval, exact repetition/leap
policy, asserted final day count, and hashes; it is a deterministic throughput
stress case, not a 100-year independent climate-science validation. H2637 shadow or
separate single-OFE native evidence cannot be relabeled as the combined claim.

The checked-in recipe SHA-256 is
`f47d249abdc61b9fdce3b581996d8be7461c02412990af34327498270330e633`.
Its executable generator uses the 365-day authenticated
`ReappearanceRoutingBgc` forcing, proleptic Gregorian calendar years, and an
explicit February 29 policy that repeats February 28 forcing while preserving
the distinct date. The 2001 year arm has exactly `365` rows; the 2000--2099 arm
has exactly `36,525` rows and `25` February 29 rows. Native management contains
one explicit slot for every `(rotation repetition, OFE)` pair (`1,000` for the
century arm). Static recipe and full parser/static-frame/exact-owner-sidecar
intake gates passed `2/2`; no simulation result is claimed yet.

Every result records source commit and dirty hash, exact binary hash, all input
hashes, repetition count, CPU affinity, CPU/wall/RSS, OFE-days, adaptive
receipts, candidate trials and rejection categories, support-width histogram,
physical-map/iteration counts, native vegetation/ET and Stage 3 attribution,
Lane D route calls/OFE ordering, and independent closure operands/residuals.
Unavailable counters are emitted as `null` with an explanation and make the
final qualification `INCOMPLETE`; they are never synthesized or relabeled.
Failed/incomplete runs report no extrapolated pass.

The ignored release-only matrix is
`hillslope::tests::stage3_laned_release_qualification_matrix_1_10_19_ofe` in
`stage3_long_run_qualification.rs`. Every timed iteration still validates the
sealed Stage 3 snapshot, exact one-day/owner/OFE cardinality, parent-support
chronology, production Lane-D day counters, HBP OFE count and positive event,
upstream-to-downstream WAT OFE order, independent WAT/PASS/HBP/manifest
closure, and routed public-output custody. Batches contain only completed
production-run intervals; preparation and those assertions are untimed. Each
OFE surface runs in a fresh child process. Every iteration records current RSS
before fixture allocation, samples it every 1 ms across fixture construction
and execution, and records it after output cleanup. The hard peak is the maximum
of sampled active RSS and isolated-process `VmHWM`; return uses the documented
`8 MiB + 1 MiB/OFE` before/after tolerance. All 30 measured batches must have
identical input hashes, result/closure identity, counters, width histogram, and
Lane-D public evidence before summary construction. The full mixed-regime
runner day is not labeled as a pure snow-free measurement. Final PASS is
impossible until the runner boundary exposes the
canonical physical-map evaluation distribution, complete native/Stage 3/Lane
D/remaining-runner attribution, and internal Lane-D route-call count/order.
The authoring increment initially ran only fast deterministic statistics,
decision, schema, RSS-boundary, Linux `/proc/self/stat`, and CPU-affinity
parser tests. The later isolated run completed the full performance matrix;
its exact disposition appears below and in `long-run-throughput-and-closure.md`.

## 2026-09-03 current-harness audit

Static inspection found that the current ignored entry points are authentic
workload producers but are not yet sufficient terminal evidence:

- the representative year and century execute `365 * 10 = 3,650` and
  `36,525 * 10 = 365,250` real OFE-days, respectively, but each runs once,
  lacks a release guard, does not read the counters it resets, and omits the
  declared CPU/peak-and-return-RSS, source/binary/input/output hash,
  bounded-telemetry, five-sample, and independent WAT5/PASS/HBP closure
  protocol;
- the release matrix's raw output-tree hash includes nondeterministic manifest
  invocation times and unique paths, so byte-identical cross-batch result
  identity cannot pass as written. Scientific outputs and deterministic
  semantic manifest fields must be compared separately from invocation/path
  metadata;
- the matrix retains every detailed iteration until the 250 ms batch ends and
  uses process-lifetime `VmHWM`. At faster runtimes this retains more telemetry
  and JSON evidence, contaminating the runtime memory measurement. Bounded
  streaming aggregates and isolated runtime-peak measurement are required;
- its 10-OFE areas are `100, 200, ..., 1000 m2`, not the representative recipe
  areas `100, 125, ..., 325 m2`;
- the two actual 365-day snow-season fixtures remain distinct from the
  representative year. One is native single-OFE; the other is two-OFE legacy
  P102 management. The representative-year test does not assert accumulation,
  persistence, meltout, frost, or reappearance and cannot substitute for the
  season gate. The `year/season` budget label must therefore be separated.

The checked-in JSON is also only partially bound: execution consumes the area
vector and selected century count while hardcoding other dates/counts and
leaving most policy/provenance/output fields unauthenticated. These are harness
and evidence defects, not kernel throughput results. They must be corrected
before Phase-4 claims, after the bounded one-day measurement becomes viable.

Static correction 2026-09-03: the package-owned 365-row season generator was
changed from leap year 2000 to nonleap year 2001, including the positive
one-day derivative fixture. A new exact guard requires 365 rows from
`1 January 2001` through `31 December 2001` for both season profiles. This
closes the previously observed December-30 truncation in source construction;
the test has not yet run because the independent CoupledTime-v17 expected-red
module intentionally keeps the orchestrator test build red until its production
capability is implemented.

Static terminal-harness repair design 2026-09-03:

- before the 1/10/19 matrix can carry evidence, split raw scientific-output
  hashes from a fail-closed normalized semantic-manifest projection, consume
  each iteration into a constant-space batch accumulator, isolate `VmHWM` in
  one fresh process per batch, and derive all surface areas from the checked-in
  representative recipe;
- before season qualification, retain the two distinct lifecycle fixtures and
  give each the release/CPU/RSS/hash/counter protocol rather than substituting
  the representative year;
- before year/century qualification, use one warm-up plus five independent
  measured processes, bounded aggregate Stage-3/Lane-D telemetry, streaming
  hashes and independent WAT5/WAT/PASS/HBP reconstruction, retained-output
  custody, and a typed `deny_unknown_fields` recipe that consumes or validates
  every field; and
- the existing runner qualification hosts are already near the 3,000-line
  ceiling. New terminal protocol, output-evidence, season, recipe, and bounded-
  telemetry modules plus an all-HBP-payload fold must be added to the exact
  write set prospectively before implementation. The HBP parser extension is
  not in the present write set.

These repairs are sequenced after the current one-day viability measurement.
The diagnostic one-day probe may guide performance work, but it cannot be
promoted to matrix or long-run qualification evidence.

## Continued one-day viability checkpoint 2026-09-03

Ran the exact pinned one-OFE release probe repeatedly after allocation,
same-call validation, Stage-3 identity-anchor Jacobian, exact leaf-maximum,
the rejected hydraulic-probe experiment, and the rejected shared-custody
experiment. The latest post-revert retained result is
`4,920,992 us/OFE-day`, down about 42.0% from the
approximately `8.487 s`
continuation reference and exactly equal in public water closure and workload
counts. This full mixed-regime one-OFE day is not the pure snow-free surface
governed by the `250 us/OFE-day` micro budget and is retained as a diagnostic,
not as that budget's qualification result. The applicable prior-source 10-OFE
complete-day and 19-OFE per-OFE matrix surfaces remain hard throughput fails.

Diagnostic attribution places about `3.46 s` in terminal direct/composed
trials and only about `0.45 s` in the complete snow-free native vegetation/ET
envelope. Within the covered terminal provider, approximately `1.73 s` is the
physical prefix and about `0.88 s` is complete-owner continuation. The atomic
validated V4 construction/byte handoff reduced the snow-free frozen core from
about `296.9 ms` to `201.2 ms`; eliminating the discarded same-call V3
round-trip reduced it again to about `153.7 ms`; direct typed V4 construction
without immediate self-serialization/reparse reduced it again to about
`110 ms`. Reusing the already validated V9-to-V8 projection inside the same
covered physical trial reduced physical-evidence work from about `1.385 s` to
`1.280 s`. These are
single-probe engineering diagnostics, not substitutes for the package's five-
warm-up/30-measured-batch protocol. That 1/10/19-OFE matrix completed at
`5.333934174/12.4533514695/22.8885138375 s` median per run. Scaling and exact
science passed; time, Stage-3 fraction, 10/19-OFE peak RSS, and aggregate RSS
return failed. Year/season/century runs remain not run while the one-day
viability gate fails by orders of magnitude.

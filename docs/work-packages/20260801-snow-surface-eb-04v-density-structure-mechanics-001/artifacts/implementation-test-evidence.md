# Implementation And Test Evidence

Status: `PASS`.

Evidence mode: `[Static] + [Ran]`.

`SnowDensityProcessDiagnostics` now travels from the authoritative density
runtime through `SnowDensityRuntimeOutcome` and `DirectSnowLiquidPartition` to
the real direct-production JSONL consumer. The implementation records direct
operands at their computation sites and preserves the original combined dry
compaction mutation arithmetic. PTM and POC reporting allocates the exact
uncapped bulk-density increment in proportion to their same-state raw
tendencies; the separate internal-cap term records realized minus uncapped
movement. Reporting does not feed back into state mutation.

All 36 frozen B/L/S/LS cells completed from release binary
`fb670d086937a7785a2549339832f71b96fc98f3c8992ec8d24961123b33826f`.
The real consumer emitted schema `openwepp-r7h-direct-production-snow-trace-v2`
for every row. Value comparison of all 36 public WAT tables against the retained
EB-04R predecessor passed exactly across 574,196 rows. A parsed comparison of
all 36 JSONL traces likewise passed across 574,196 common rows after excluding
only the schema identifier and new `density_process_*` fields.

The first cohort was invalidated after review found local-density-space cap
attribution. A corrected pre-terminal cohort was then superseded because it
predated all-field finite guards and used the wrong modeled-snow-free pairing
operator. Both are retained and explicitly non-decision-eligible; the hash
above identifies the only terminal cohort.

No selector, coefficient, cap, equation, public WAT column, phase, melt,
canopy, radiation, longwave, sublimation, or forcing path changed.

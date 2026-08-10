# Independent Hydrology/Science Re-Review B

Status: `executed`

Evidence class: `Static: base a65cc3973 through exact reopened authority/test commit 669269ee4fff3aab89ba2d5c72e4fdd34b12b7c2; ADR-0036, contracts, runtime source, focused and integration tests, publication consumers, and package evidence. Ran: prior implementation/consumer suites through 33831787 plus the reopened ADR source guard and ADR Markdown validation at 669269ee.`

Verdict: `PASS`

Reviewer independence: Reviewer A's report was not consulted in reaching this verdict.

## Findings

No closure-blocking science, hydrology, unit, or claim-boundary finding remains
in the reviewed implementation.

## Prior Critical Recheck — `PASS`

The local-only daily same-pass infiltration correction is fully retired, not
merely protected by a mixed-source guard. The R4K span now writes WB14's own
`cumulative_infiltration_m` through as the same-pass infiltration operand
(`crates/openwepp-hillslope-orchestrator/src/direct_runtime/runoff.rs:231-246`).
The former snow-derived daily reconstruction method, additional-infiltration
calculation, source-custody guard, and post-WB14 earliest-bin debit call are
deleted. The source-level authority test also rejects reintroduction of the
former `snow_reconstructed_same_pass_infiltration_m` path
(`tests/integration/peak_hourly_authority_contract.rs:49-56`). The sole
`f9082926..2d8367f0` delta adds a real R4I -> R4J -> R4K regression with
`0.010 m` of pure routed melt in producer hour 5 and limited infiltration
capacity. It proves that WB14 publishes a positive infiltration strictly below
the melt supply, percolation consumes that exact WB14 value without a daily
snow override, the residual closes to `melt - infiltration`, and every residual
bin except producer hour 5 remains exact zero
(`crates/openwepp-hillslope-orchestrator/src/tests/tests_mod/direct_runtime_r7g_frost.rs:723-798`).
There is no production-source or contract change in that test-only delta.

The later `2d8367f0..d934ab9b` delta also makes no production change. Contract
v170 correctly reclassifies historical `GAP-WATBAL-005` as
`closed — superseded`: the rainfall-envelope/APPMTH `ealpha` chain is not an
active peak producer, and retained manifest values are explicitly limited to
`false`/`retired_not_applicable` schema lineage. This reconciles stale
provenance without reviving or broadening the retired peak authority.

The terminal `d934ab9b..df41f352` delta is test-only and does not weaken an
assertion. EROD16 now recognizes PASS `peakro` as `m^3 s^-1`, divides by the
same `fwidth * efflen` area exactly once, and applies the legacy passby gate and
Wave-1 fixture operands on the required internal `m s^-1` basis. The H2637
routing test changes only temperature/dewpoint fields in its copied climate
fixture, leaving precipitation and routing inputs unchanged, so the unrelated
partial-frost missing-clock guard no longer preempts the routing-authority
assertions. This is valid test isolation, not suppression of the production
frost guard.

The `df41f352..33831787` authority/runtime sequence aligns the adjacent erosion
contract with the implemented consumer. SC-SED rev61/63 names the
source-complete maximum-hour depth rate in `m s^-1`, public-only area
conversion to `m^3 s^-1`, rectangular-equivalent duration, and the prohibition
on uniform/rainfall-window/analytical fallback. The active erosion guard
independently reconstructs `Q / peakro_depth` and applies the contract-matching
absolute `1.001e-9 s` custody threshold. This replaces both the stale sediment
mass tolerance and rev62's dimensionally invalid relative expression. Tests
prove sub-threshold acceptance and supra-threshold failure at `0.25 s`, `10 s`,
and `80,000 s`; the allowance does not grow with event duration.

The warmed H2637 evidence assertion is internally consistent: the copied
fixture changes temperature/dewpoint only, the shadow expects all 731 days seen
and routed, and all three uniform-shape counters remain zero. Those counters
support source-complete shape custody for this isolated evidence vector; they
do not erase the production frost guard or generalize to an unwarmed climate.

The reopened `33831787..669269ee` delta closes the remaining decision-authority
contradiction. ADR-0036 now states that a current hourly-surface payload has one
native peak authority: `max_h(q_hourly(h) / 3600 s)` internally and
`max_h(V_h / 3600 s)` publicly after exactly one area conversion. It explicitly
retires `vave * qpstar`, rainfall-envelope operands, and APPMTH branches from
the native production peak; it also prohibits rescaling the hourly profile
toward a separate estimator.

The retained scalar/triangular path is narrowly and correctly bounded to
legacy shards that lack the paired hourly water surface. It is labeled a
compatibility fallback/diagnostic and cannot establish native WB16 authority or
a current hourly-peak acceptance claim. This preserves historical readability
without weakening current minor-1 hillslope authority. The ADR's maximum-hour
claim remains an hourly mean, not an instantaneous, sub-hourly, channel, or
watershed peak.

The binding source guard requires the reconciled native formulas and
compatibility boundary and rejects the three prior contradictory formulations:
the separate analytical estimator, the statement that hourly maximum mismatch
is not an error, and the rescale-to-independent-peak alternative. This is a
useful regression guard for the exact authority seam and does not substitute
for the semantic contract/runtime review recorded above.

The surviving `remove_depth_from_hour_bins_earliest` call is not the retired
daily correction. It executes inside WB14's own chronological infiltration and
depression-storage producer and removes the bucket-filling depression-storage
depth from the earliest producer excess (`runoff.rs:1867-1888`). For a
non-draining daily depression-storage bucket, that chronological ownership is
physically consistent and does not reconstruct timing after partition.

## Source and Closure Review

### WB14 rain, routed melt, and runon ownership — `PASS`

Local rain retains its explicit hyetograph. Producer-timed routed melt enters
`hourly_additional_supply_m`; surface and lateral runon require nonempty
producer shapes and are added to that same hourly supply before WB14 executes
(`runoff.rs:592-680`). When additional supply exists, WB14 constructs a shared
24-hour liquid-supply basis, performs infiltration interval by interval, and
emits the residual excess into the corresponding hour
(`runoff.rs:1744-1754,1781-1864`). These supplies are not later appended as
runoff limbs. Tests establish melt-only timed runoff, complete melt
infiltration, runon-driven infiltration, missing-shape failure, and preservation
of tiny positive source-backed supply.

### Frost retention — `PASS`

No positive frost residual is tolerance-cleared. Complete daily-only frost
retention clears the hourly series only when `partition_runoff_m == 0.0`
exactly. Every positive residual, including `5e-13 m`, requires producer-timed
hourly frost custody and otherwise hard-fails
(`runoff.rs:1423-1438`; `direct_runtime_dc01.rs:277-326`). The aggregate WB14
roundoff tolerance only compares independently accumulated ledgers and does not
mutate a positive hourly bin.

### WB19 return and peak closure — `PASS`

The closing hourly series is WB14 post-depression excess plus the WB19
producer's same-hour saturation return. Each operand is validated finite and
nonnegative, its sum must close to positive daily runoff, and missing or
materially mismatched timing yields a typed WB16 hydrology guard. Normalized
weights are derived only after closing depths exist. Dry runoff publishes zero;
positive runoff cannot use an empty, uniform, rainfall-duration, or daily-scalar
fallback.

### Units, area, consumers, and claim boundary — `PASS`

The internal peak remains maximum hourly-mean depth rate in `m s^-1`. Public
publication reconciles the peak to the event run-volume depth basis and
multiplies by the same positive lane area exactly once to obtain `m^3 s^-1`
(`crates/openwepp-hillslope-orchestrator/src/direct_runtime/01_publication.rs:576-640`).
Erosion consumes depth rate. The multi-OFE integration independently proves
outlet `sum(V_h) = runvol`, reconstructs `max(V_h) / 3600 s`, and compares that
value with both HBP and pass output. Contracts accurately limit the claim to
maximum hourly mean hillslope-event discharge and label duration as
rectangular-equivalent, not instantaneous/channel/watershed peak or physical
hydrograph duration.

## Executed Evidence

Ran at exact commit `f9082926f369036cbb5ab5a51a21c284599285f7`:

```text
cargo nextest run -p openwepp-hillslope-orchestrator direct_runtime_dc01
24 tests run: 24 passed, 447 skipped
```

Ran at exact terminal commit
`2d8367f0ea03f6b21a41dda5efcd4f02595a2adf`:

```text
cargo nextest run -p openwepp-hillslope-orchestrator \
  r7h_pure_melt_r4k_preserves_wb14_capacity_and_residual_hour
1 test run: 1 passed, 471 skipped
```

Ran at exact terminal commit
`d934ab9b033b245502ecf91b57e6df5edd583528`:

```text
cargo nextest run --test peak_hourly_authority_contract
4 tests run: 4 passed, 0 skipped
```

Ran at exact reopened authority/test commit
`669269ee4fff3aab89ba2d5c72e4fdd34b12b7c2`:

```text
cargo nextest run --test peak_hourly_authority_contract
4 tests run: 4 passed, 0 skipped

markdown-doc lint --path \
  docs/decisions/0036-hydrograph-resolved-sediment-transport-and-routing.md \
  --format plain
1 file validated, 0 errors, 0 warnings
```

Ran at exact terminal commit
`df41f3526dd61eb801d2c9a244bef197c1f169ed`:

```text
cargo nextest run --test erod16_wave1_continuity_fixture_conservation \
  --test laned_shadow_h2637
9 tests run: 9 passed, 2 skipped
```

Ran at exact terminal commit
`33831787b7029b28b0716c8458f08a11899db446`:

```text
cargo nextest run -p openwepp-hillslope-orchestrator \
  hb01_g_duration_custody_uses_absolute_seconds_at_multiple_scales
1 test run: 1 passed, 472 skipped

cargo nextest run --test peak_hourly_authority_contract
4 tests run: 4 passed, 0 skipped
```

## Evidence Status

The remaining cohort and broader gate work is package evidence status, not an
implementation science defect. It must still be reconciled to the exact release
anchor with the provenance required by the package before closure; this review
does not convert pending evidence into a pass claim.

## Verdict Rationale

Exact reopened authority/test commit `669269ee4fff3aab89ba2d5c72e4fdd34b12b7c2` retains the removal of the last
unauthorized daily-to-hourly infiltration allocation and leaves WB14 as the
single owner of hourly supply, infiltration, depression storage, and residual
excess. Frost, subsurface-return timing, positive-source preservation, area
conversion, and consumer semantics remain fail-closed and contract-consistent.
The implementation therefore passes independent hydrologic science review.

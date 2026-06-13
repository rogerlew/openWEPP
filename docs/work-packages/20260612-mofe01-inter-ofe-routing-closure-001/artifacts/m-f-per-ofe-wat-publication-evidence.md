# M-F Per-OFE WAT Publication Evidence

Status: M-F-REDO executed-hold; active per-OFE handoff and anti-clone
publication fixed, but `QOFE` geometry scaling remains blocked

Evidence mode: Ran + Static

## Summary

M-F-REDO corrected the two defects that made the original M-F output hollow:
multi-OFE lanes now run on OFE-local soil/slope/management surfaces instead of
aggregate clones, and public WAT rows now carry active nonzero surface and
lateral handoff values. H1/H6/H9/H11 final smoke under
`/tmp/openwepp_mofe01_mfredo_final` exits zero, row keys remain aligned, active
surface/lateral transfer residuals close at `0.0`, and the anti-clone audit
reports no active all-OFE-identical days.

M-F-REDO is still not complete. The candidate still publishes `QOFE == Q` on
all four real multi-OFE smoke surfaces (`max_abs_qofe_minus_q=0.0`). The pinned
legacy-clean ladder proves that is not the authoritative per-OFE WAT shape:
legacy max `abs(QOFE-Q)` is `362.13991` mm on H1, `177.51694` mm on H6,
`185.89531` mm on H9, and `84.64425` mm on H11. Static baseline source shows
why: legacy public `Q` uses cumulative-length `efflen/totlen` scaling, while
public `QOFE` uses OFE-length `efflen/slplen` scaling.

The next corrective increment must port the geometry-scaled per-OFE `QOFE`
publication rule while preserving the M-F-REDO active handoff, anti-clone, and
single-OFE anchor gates.

## M-F-REDO Implementation

Static changes in M-F-REDO:

- Multi-OFE persistent lanes now build static runtime surfaces from their own
  soil, slope, management, and PMET slices instead of cloning the aggregate
  runtime surface.
- Lane runtime surfaces publish `mofe.static_lane.contributor_ofe_count`, so
  downstream MOFE carry and Wave-2 activation still see the contributor count
  even though the lane-local kernel surface has `nelem=1`.
- Current surface carry now includes the WB14 partition runoff contribution so
  `MOFE_HOURLY_CURRENT_SATURATION_RUNOFF_ROOT` feeds active per-OFE `QOFE` and
  downstream `UpStrmQ`.
- WB19 lateral writeback adds to existing current carry instead of replacing
  it.
- WB12 storage reconciliation now includes same-pass runon input; tests with
  explicit runon fixtures were reconciled to that contracted equation.
- `SC-WATBAL-001` version 157 and `SC-SYSTEM-001` version 80 add anti-clone and
  active handoff publication invariants.

## M-F-REDO Runtime Smoke

Fresh final smoke ran directly, without the comparator subagent, under:

- `/tmp/openwepp_mofe01_mfredo_final`

Runtime execution:

| H | OFEs | Rows | Days | Exit | Elapsed seconds |
| ---: | ---: | ---: | ---: | ---: | ---: |
| 1 | 5 | 10960 | 2192 | 0 | 203 |
| 6 | 3 | 6576 | 2192 | 0 | 121 |
| 9 | 4 | 8768 | 2192 | 0 | 156 |
| 11 | 2 | 4384 | 2192 | 0 | 88 |

Direct WAT publication audit:

| H | `max_upstrmq` | downstream nonzero `UpStrmQ` rows | surface active days | surface residual | `max_subrin` | downstream nonzero `SubRIn` rows | lateral residual | max `abs(QOFE-Q)` | active clone days |
| ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| 1 | 516.6511489326965 | 4195 | 1056 | 0.0 | 4.044897921229842 | 341 | 0.0 | 0.0 | 0 |
| 6 | 259.52557446634825 | 2073 | 1052 | 0.0 | 5.385048904125066 | 132 | 0.0 | 0.0 | 0 |
| 9 | 388.38836169952253 | 3144 | 1058 | 0.0 | 6.002857116832274 | 180 | 0.0 | 0.0 | 0 |
| 11 | 132.26278723317412 | 990 | 990 | 0.0 | 4.02642906688712 | 79 | 0.0 | 0.0 | 0 |

The original M-F zero-on-zero surface-handoff blocker is retired: active
surface edges have nonzero operands. The remaining exact-zero value is
different and now blocking: `QOFE` is exactly equal to `Q` on all smoke rows.

Pinned legacy direct audit:

| H | Legacy rows | Legacy max `abs(QOFE-Q)` |
| ---: | ---: | ---: |
| 1 | 10960 | 362.13991 |
| 6 | 6576 | 177.51694 |
| 9 | 8768 | 185.89531 |
| 11 | 4384 | 84.64425 |

Static provenance:
`/workdir/wepp-forest_260430_baseline/src/watbal.for` writes public `Q` as
`runoff(iplane)*1000.*efflen(iplane)/totlen(iplane)` and public `QOFE` as
`runoff(iplane)*1000.*efflen(iplane)/slplen(iplane)` in the non-`contrs` path.

## M-F-REDO Local Comparison

Local semantic comparisons ran with
`.venv/bin/python tools/owcmp/semantic_wat.py --candidate-year-offset 1999`.
No comparator subagent was used.

| H | Common rows | Candidate-only | Baseline-only | Semantic pass | `Q` fail count | `QOFE` fail count | `UpStrmQ` fail count | `SubRIn` fail count |
| ---: | ---: | ---: | ---: | --- | ---: | ---: | ---: | ---: |
| 1 | 10960 | 0 | 0 | false | 5619 | 5627 | 4284 | 7868 |
| 6 | 6576 | 0 | 0 | false | 3135 | 3136 | 2064 | 3556 |
| 9 | 8768 | 0 | 0 | false | 4228 | 4231 | 3121 | 5359 |
| 11 | 4384 | 0 | 0 | false | 2004 | 2004 | 984 | 1811 |

Row-key alignment is complete for the smoke set. Semantic failure remains
value-family failure, not structure.

## M-F-REDO Single-OFE Anchor

Single-OFE anchors were rerun under:

- `/tmp/openwepp_mofe01_mfredo_single_final`

H8/H15/H19/H20/H22/H23/H28 exited zero and are byte-identical to M-E2 outputs
for `.hbp`, `.loss.json`, `.plot.parquet`, and `.wat.parquet` (28/28 PASS in
`single-ofe-anchor-cmp.tsv`).

## M-F-REDO Gates

| Gate/check | Result | Notes |
| --- | --- | --- |
| Public per-OFE row cardinality | PASS | H1/H6/H9/H11 row counts equal `day_count * contributor_ofe_count`. |
| Active surface handoff | PASS | Downstream `UpStrmQ` is nonzero on active routed days and adjacent residuals close at `0.0`. |
| Active lateral handoff | PASS | Downstream `SubRIn` is nonzero on active lateral days and adjacent residuals close at `0.0`. |
| Anti-clone gate | PASS | Active all-OFE-identical day count is zero for H1/H6/H9/H11. |
| No `QOFE=Q` alias | FAIL | Candidate `max_abs_qofe_minus_q=0.0`; pinned legacy-clean ladder requires non-alias geometry scaling. |
| Semantic comparison | FAIL | Commands exited zero and row keys align, but value families fail. |
| Single-OFE anchor | PASS | 28/28 byte comparisons pass against M-E2 outputs. |
| Rust closure loop | PASS | `cargo fmt --check`, clippy with warnings denied, `cargo test --workspace`, `cargo deny check`, and `git diff --check` pass. |
| Line-count governance | WARN | `scheduler_seed_and_runtime.rs` is 2122 lines; below 3000 but above the 2000-line warning threshold. |

## M-F Historical Summary

M-F wired the multi-OFE runner to publish WB13/WAT rows from the internal
per-OFE WB13 records produced by M-E4-REDO. The candidate now emits one public
row per OFE per day, and manifest provenance reports
`publication_ofe_policy=per-ofe-dynamic-water-balance-state`. M-F's apparent
`QOFE` non-alias conclusion is superseded by M-F-REDO, which proves public
`QOFE` still aliases public `Q` after active handoff is fixed.

M-F is not complete. The real-run WAT audit shows the surface transfer producer
still publishes zero downstream `UpStrmQ` on representative multi-OFE runs.
This violates the M-F gate "no downstream `UpStrmQ=0`" and leaves semantic
comparison failing on the same surface-carry family. The lateral handoff path
has nonzero downstream `SubRIn` rows; the surface carry path remains the
blocking producer defect.

## Implementation

Static changes in this increment:

- Multi-OFE daily execution now appends public WB13/WAT rows from
  `DailyInternalPerOfeWb13Collection` instead of the prior aggregate row path.
- Single-OFE execution keeps the aggregate publication path.
- Public WB13 provenance now validates per-day OFE row grouping, monotonic
  `(sim_day_index, OFE)` keys, `day_count * contributor_ofe_count` cardinality,
  per-OFE storage lineage, and first/last OFE keys.
- Per-OFE row construction accepts an explicit `QOFE` source from the current
  transfer output; aggregate rows still require the old `QOFE == Q` guard.
- The summary accumulator keeps the aggregate `QOFE == Q` guard but allows
  explicitly marked per-OFE publication rows.
- Watershed manifest validation accepts and checks the new per-OFE publication
  metadata shape.
- Contract-derived tests require public per-OFE WAT publication source tokens.

## Runtime Smoke

Local H smoke ran directly, without the comparator subagent, under:

- `/tmp/openwepp_mofe01_mf`

Runtime execution:

| H | OFEs | Rows | Days | Exit | Elapsed seconds |
| ---: | ---: | ---: | ---: | ---: | ---: |
| 1 | 5 | 10960 | 2192 | 0 | 244 |
| 6 | 3 | 6576 | 2192 | 0 | 136 |
| 9 | 4 | 8768 | 2192 | 0 | 185 |
| 11 | 2 | 4384 | 2192 | 0 | 90 |

H1 manifest audit:

- `publication_ofe_policy`: `per-ofe-dynamic-water-balance-state`
- `storage_lineage_policy`: `per-ofe-dynamic-wb-state`
- `per_ofe_state_policy`: `published-per-ofe-wb13-records`
- `row_count`, `per_ofe_record_count`, `per_ofe_expected_record_count`: `10960`
- `per_ofe_internal_day_count`: `2192`
- first row key: `Y=1`, `J=1`, `OFE=1`, `sim_day_index=1`
- last row key: `Y=6`, `J=365`, `OFE=5`, `sim_day_index=2192`
- `sim_day_index_monotonic`: `true`

Direct WAT publication audit:

| H | `max_upstrmq` | downstream nonzero `UpStrmQ` rows | `max_subrin` | downstream nonzero `SubRIn` rows | max `abs(QOFE-Q)` | surface handoff residual | lateral handoff residual |
| ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| 1 | 0.0 | 0 | 3.313221035829904 | 356 | 129.16278723317413 | 0.0 | 0.0 |
| 6 | 0.0 | 0 | 5.331580804504631 | 128 | 129.76278723317412 | 0.0 | 0.0 |
| 9 | 0.0 | 0 | 5.772356765746494 | 180 | 129.46278723317417 | 0.0 | 0.0 |
| 11 | 0.0 | 0 | 4.002334964818551 | 79 | 132.26278723317412 | 0.0 | 0.0 |

The exact-zero surface handoff residual is not acceptance evidence here:
candidate `current UpStrmQ == previous QOFE` holds only because both surface
terms are zero. The nonzero lateral `SubRIn` rows show the audit is capable of
observing active transfer; the surface producer is the remaining blocker.

## Local Comparison

Local semantic comparisons ran with `.venv/bin/python tools/owcmp/semantic_wat.py`
and `--candidate-year-offset 1999`. No comparator subagent was used.

| H | Common rows | Candidate-only | Baseline-only | Semantic pass | `UpStrmQ` fail count | max `UpStrmQ` diff | `SubRIn` fail count | `QOFE` fail count | max `QOFE` diff |
| ---: | ---: | ---: | ---: | --- | ---: | ---: | ---: | ---: | ---: |
| 1 | 10960 | 0 | 0 | false | 730 | 342.5 | 7864 | 1645 | 452.6749 |
| 6 | 6576 | 0 | 0 | false | 39 | 155.6289 | 3553 | 269 | 266.2754 |
| 9 | 8768 | 0 | 0 | false | 82 | 131.4431 | 5354 | 400 | 247.8604 |
| 11 | 4384 | 0 | 0 | false | 6 | 64.79408 | 1811 | 133 | 169.2885 |

The structural row-key gap is retired for these smoke runs: candidate and
baseline share all row keys. Semantic failure now lands on actual value
families, especially surface run-on/run-off transfer.

## Gates

| Gate/check | Result | Notes |
| --- | --- | --- |
| Public per-OFE row cardinality | PASS | H1/H6/H9/H11 row counts equal `day_count * contributor_ofe_count`; CLI03 M-F integration test passes. |
| Manifest per-OFE publication metadata | PASS | New policy, storage lineage, state policy, row counts, and first/last OFE keys are present. |
| No `QOFE=Q` alias | SUPERSEDED / FAIL | M-F's apparent non-alias finding is superseded by M-F-REDO; candidate H1/H6/H9/H11 now show `max_abs_qofe_minus_q=0.0` once active handoff is fixed. |
| Surface handoff `current UpStrmQ == previous QOFE` | STRUCTURAL PASS / ACCEPTANCE FAIL | Residual is 0.0 only because current surface carry is zero throughout the candidate smoke set. |
| No downstream `UpStrmQ=0` | FAIL | `max_upstrmq=0.0` and downstream nonzero `UpStrmQ` rows are `0` for H1/H6/H9/H11. |
| Lateral handoff `current SubRIn == previous latqcc` | PASS | Residual is 0.0 and downstream nonzero `SubRIn` rows are observed. |
| Semantic comparison | FAIL | Row keys align, but `UpStrmQ`, `SubRIn`, and `QOFE` value failures remain. |
| Single-OFE anchor | PARTIAL | Single-OFE code path remains aggregate and focused tests pass; substrate byte-identity anchor was not rerun in M-F because the multi-OFE surface gate failed. |
| `cargo fmt --check` | PASS | Final post-edit run. |
| `cargo clippy --workspace --all-targets -- -D warnings` | PASS | Final post-edit run. |
| `cargo test --workspace` | PASS | Full Rust closure loop passed. |
| `cargo deny check` | PASS | `advisories ok, bans ok, licenses ok, sources ok`. |
| `git diff --check` | PASS | Final whitespace check passed. |

## Blocking Finding

The M-F acceptance blocker is below publication shape: the current surface
transfer producer still emits zero current surface carry (`QOFE`/`ui_SCrunf`
family) on real WA Cascades multi-OFE runs where legacy has active downstream
`UpStrmQ` and `QOFE`. Publishing the internal WB13 records made the surface
defect visible; it did not close it.

Do not repair this by setting public `UpStrmQ` or `QOFE` from local `Q` in the
WAT writer. That would be surrogate publication and would reintroduce the
M-C/M-E4 class of hollow closure. The corrective increment must implement a
contract-backed current surface export producer that feeds the same transfer
state consumed by downstream OFEs.

## Next Corrective Increment

M-F-REDO should:

- identify the authoritative runoff/surface export producer for
  `MOFE_HOURLY_CURRENT_SATURATION_RUNOFF_ROOT` and per-OFE `QOFE`;
- add red tests on real or fixture multi-OFE runs requiring downstream nonzero
  `UpStrmQ` on active surface-runoff days;
- keep the public per-OFE row cardinality and `QOFE != Q` checks;
- prove `current UpStrmQ == previous QOFE` with nonzero active surface carry,
  not zero-on-zero equality;
- preserve lateral handoff and M-E4-REDO internal identity closure; and
- rerun the H1/H6/H9/H11 smoke comparisons and single-OFE anchors before
  M-G/M-H proceeds.

## Claude review (2026-06-13) — hold CONFIRMED and DEEPENED; root cause is aggregate-duplicated records

Evidence mode: Ran (legacy `H*.wat.dat` parse + duckdb on the M-F cohort
`/tmp/openwepp_mofe01_mf/output/`).

**Codex's hold is correct, and the defect is real (not an inadequate
fixture)** — but it is deeper and more important than the "surface export
producer" localization.

1. **The block is a genuine defect, confirmed against the legacy-clean
   ladder.** Legacy routes surface runoff downstream on all four smoke
   hillslopes: H1 OFE2-5 `maxUpStrmQ` 55–342 mm on 13/56/157/506 days;
   H6/H9/H11 similar. openWEPP publishes `UpStrmQ=0` everywhere. This is
   exactly the like-for-like comparison the M-A calibration (legacy clean at
   1–5 OFEs) was built to enable — and it flags a real openWEPP defect.

2. **Root cause is bigger than UpStrmQ: the per-OFE records are
   aggregate-duplicated, not distinct per-OFE hydrology.** On H1, every
   hydrology column is **identical across all 5 OFEs** — `Es`=0.004246,
   `Ep`=1.856852, `Dp`=0.244799, `SoilWaterTotal`=274.93, `P`=130, and `Q`
   shows **max 2 distinct values per day across 5 OFEs** (genuine routing
   would give up to 5). `QOFE=0` everywhere; `Q`=129.16 identical. Distinct
   per-OFE soil/slope/cover with real run-on cannot produce identical
   ET/percolation/storage/runoff — so the published per-OFE rows are
   aggregate values stamped onto OFE-keyed rows, with routing scalars
   (`SubRIn` partially, `UpStrmQ=0`) bolted on. `UpStrmQ=0`/`QOFE=0` are
   symptoms of this root issue.

3. **The critical lesson: M-E4-REDO's genuine conservation identities are
   necessary, not sufficient.** They closed at ~1e-13 on these *clone*
   records — because each aggregate-duplicated record is internally
   water-balanced, so the per-element identity cannot detect duplication or
   mis-routing. **Conservation closure proves no water is lost; it proves
   neither distinct per-OFE physics nor correct routing.** The legacy
   comparator caught what conservation could not — a live validation of
   ADR-0017 (comparator-as-flag) even with conservation as the acceptance
   authority. Both checks are load-bearing.

4. **Latent M-E3 gap exposed.** M-E3's no-bleed test proved OFE lane states
   don't contaminate each other; it never proved the lanes produce
   **distinct** outputs from distinct per-OFE inputs. The identical-across-OFE
   columns suggest either the M-E3 lanes are not genuinely differentiated
   (static per-OFE soil/slope/cover slices not applied, or run-on not
   reaching the lane kernel) or M-F publishes from the aggregate path rather
   than the per-OFE records. This is the M-F-REDO root-cause question.

## M-F-REDO scope expansion (Claude)

M-F-REDO must root-cause the aggregate-duplication, not just the surface
`UpStrmQ` symptom:

- **Trace whether the M-E3 per-OFE lane records contain distinct values
  before publication** (read the seam; cite lines). If the lanes produce
  clones, the defect is upstream in M-E3 lane differentiation (static slice
  application / run-on delivery to the lane kernel). If the lanes produce
  distinct values but M-F publishes aggregate, the defect is in the M-F
  publication source.
- **Acceptance gate (genuineness, mirroring the M-E4-REDO lesson):** on a
  multi-OFE fixture and the H1/H6/H9/H11 cohort, per-OFE `Q`/`Es`/`Ep`/`Dp`/
  `SoilWaterTotal` must be **distinct across OFEs** (not clones), `UpStrmQ`/
  `QOFE` nonzero on the days legacy shows surface routing, and the genuine
  per-element + transfer identities still close at `TOL-WATBAL-007`. A
  comparator flag against legacy's per-OFE UpStrmQ/QOFE on the smoke set
  (legacy clean here) is the like-for-like check.
- Add a **distinctness assertion** to the per-OFE contract surface
  (`SC-WATBAL-001`/`SC-SYSTEM-001`): multi-OFE published records on a routed
  hillslope must not be all-OFE-identical in the hydrology columns — an
  anti-clone gate, the structural analog of the anti-tautology TOL addendum.

This is the load-bearing routing-physics question the whole rung exists to
answer; it cannot close until per-OFE rows carry genuinely distinct, correctly
routed hydrology.

## Claude review (2026-06-13) — M-F-REDO: anti-clone fix is INCOMPLETE (runoff still cloned)

Evidence mode: Ran (duckdb on `/tmp/openwepp_mofe01_mfredo_final/output/H1.wat.parquet`).

M-F-REDO made real progress — `UpStrmQ` now routes downstream (4195 nonzero
rows, was 0), accumulation works (OFE outlet = self + upstream), and `Ep`
differs slightly per OFE — but the anti-clone fix **did not reach runoff**:

- On the peak-runoff day (year 4, julian 293) **all 5 OFEs are still identical
  clones**: local runoff (`Q − UpStrmQ`) = 129.1628 for every OFE, and `Es`,
  `Ep`, `SoilWaterTotal` identical too.
- Across runoff days, **968 of 1057 (92%) have all-5-OFE-identical local
  runoff**; only 89 show any distinction.
- The clean `Q = 129.16 × ofe_id` ladder is therefore the **clone artifact
  accumulating**, not genuine per-OFE routing. ET (`Ep`) differentiated;
  runoff generation did not. That ET-distinct-but-runoff-cloned split is the
  diagnostic clue for where the lanes are/aren't differentiating.

**The anti-clone gate must bite on the runoff columns, not only ET.** M-F-REDO
remains executed-hold for the runoff clone, ahead of (and gating) the QOFE
geometry work.

## Legacy Q vs QOFE semantics — pinned from `watbal.for` (operator clarification 2026-06-13)

The non-contoured `else` write path (`watbal.for:1094`, `:1099`) renders the
**same** routed `runoff(iplane)` as **two different depths**:

- **Q** (the "runoff" WAT column) = `runoff(iplane) · efflen/totlen`
  (`:1094`) — normalized to **cumulative** length (totlen, to the OFE bottom).
  Source comment: *"use cumulative length (totlen) because efflen may span
  OFEs."* A hillslope-cumulative depth; this is why it grows downslope and
  reads as confusing.
- **QOFE** = `runoff(iplane) · efflen/slplen` (`:1099`) — normalized to **this
  OFE's own** slope length. **The sane reading: QOFE is the runoff depth
  *through this OFE*, in the OFE's own geometry** (operator framing, confirmed
  against source).

So it is one routed runoff volume, two normalizations: local (÷slplen → QOFE)
vs cumulative (÷totlen → Q). openWEPP currently publishes `Q == QOFE == raw
accumulating ladder` (neither normalization applied) on top of the cloned
runoff. The geometry fix is necessary but downstream of the clone fix —
applying ÷slplen to a cloned runoff just renders equal local depths.

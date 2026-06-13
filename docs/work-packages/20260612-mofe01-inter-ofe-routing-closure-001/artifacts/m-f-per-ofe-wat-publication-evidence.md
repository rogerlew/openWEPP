# M-F Per-OFE WAT Publication Evidence

Status: M-F executed-hold; public per-OFE row publication exists but surface
handoff acceptance failed

Evidence mode: Ran + Static

## Summary

M-F wired the multi-OFE runner to publish WB13/WAT rows from the internal
per-OFE WB13 records produced by M-E4-REDO. The candidate now emits one public
row per OFE per day, manifest provenance reports
`publication_ofe_policy=per-ofe-dynamic-water-balance-state`, and per-OFE
`QOFE` is no longer mechanically aliased to local `Q`.

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
| No `QOFE=Q` alias | PASS | Direct audit reports max `abs(QOFE-Q)` above 129 mm on all smoke runs. |
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

# D3 Increment C1b Capacity/Overflow Landing

Status: executed-hold; C1b gates pass, D3 acceptance still open
Evidence mode: Ran + Static
Date: 2026-06-12

## Scope Executed

Increment C1b was run without the comparator subagent because the user reported
the GPT-5.3-Codex-Spark weekly quota was exhausted. The parent model ran local
focused tests, the p1/p43 starter trace, and the 43-prefix `algebraic-radium`
cohort comparison with DuckDB/CLI tooling.

The landed implementation adds:

- fine-layer liquid and ice capacity checks using the total pore capacity
  form `(thetdr + ul/dg)`;
- capacity-limited `frwatc(1)` ingress, freeze-path ice formation, and
  downward fine-layer liquid routing;
- `frost.runtime_watpdg_m` and `frost.runtime_watbtm_m` overflow surfaces,
  with `watbtm` entering WB13 `Dp`;
- bounded WB18/WB13 deep-percolation publication-dust canonicalization
  (`1e-11 m`) and WB18 scalar/layer storage roundoff rebalance (`2e-11 m`);
  and
- trace guard diagnostics that prefer `wb19_dg_####` layer geometry when
  validating WB18 frozen-depth terms.

## Focused Evidence

Ran:

- C1b CLIM06 vectors reject persisted fine ice over capacity, enforce
  freeze-path pore capacity, use `ul` as active storage above residual, and
  route overflow to `watbtm` while closing the shadow identity.
- WB13 publication tests prove `Dp` includes `D + frost.runtime_watbtm_m`
  and zeroes bounded source dust before publication.
- WB18 tests prove bounded deep-percolation dust is restored before state
  debit, positive deep loss remains in the scalar ledger, no-flux layer/scalar
  roundoff is rebalanced, and zero root uptake preserves the incoming scalar.
- Runner trace test proves WB18 guard text uses preferred `wb19_dg_####`
  geometry when both legacy `dg_####` and WB19 geometry are present.

## Starter Capacity Gate

Ran: p1 and p43 traced through the first 100 simulation days at
`/tmp/fdhp01_increment_c1b_starter_trace_final14_20260612T035756Z`.

Persisted artifact:
`fdhp01_increment_c1b_starter_capacity_20260612.json`.

Result:

- p1: `1700` trace rows and `15192` layer checks scanned; zero
  `frzw > ul` rows; minimum capacity margin `0.020482917898791884 m`.
- p43: `1700` trace rows and `15192` layer checks scanned; zero
  `frzw > ul` rows; minimum capacity margin `0.020378509421531917 m`.

The starter trace payload does not expose the shadow `frwatc` residual field.
The residual closure portion of the C1b starter concern is therefore covered
by the focused CLIM06 shadow-identity tests plus the full cohort storage
closure below, not by a named trace column.

## Cohort Gate

Ran: release `openwepp-cli-hill` over the 43-prefix `algebraic-radium` cohort
without subagent delegation.

- Run root: `/tmp/fdhp01_increment_c1b_cohort_final14_20260612T035618Z`
- Reports:
  - `fdhp01_increment_c1b_run_status_20260612.tsv`
  - `fdhp01_increment_c1b_execution_summary_20260612.json`
  - `fdhp01_increment_c1b_annual_closure_residuals_20260612.csv`
  - `fdhp01_increment_c1b_depth_metrics_20260612.csv`
  - `fdhp01_increment_c1b_frozwt_frdp_ratio_20260612.csv`
  - `fdhp01_increment_c1b_starter_capacity_20260612.json`

Result:

- `43/43` clean exits; `43/43` WAT outputs.
- No capacity guard trips on valid cohort input.
- Years 2-6 `Total-Soil + frozwt` max abs residual:
  `1.5347723092418164e-12 mm`.
- Year 7 boundary watch item: max abs residual
  `6.963318810448982e-13 mm`.
- Year 1 initialization residual remains outside the staged gate:
  max abs `1.0505061950725292 mm`.
- Profile-bound pinning remains removed: `0/43` prefixes pinned; minimum
  margin `5.824859208653152 mm`.
- `frozwt/frdp` decorrelation does not regress versus Increment B: max
  per-prefix correlation `0.9860178382757524`, below Increment B's
  `0.9861968090242198`.

## Hold Evidence

C1b passes the water-side capacity/overflow increment, but it does not close
D3. The watch expectation did not move in the desired direction: mean maximum
depth is `1791.9747961835646 mm`, worse than the Increment B mean
`1782.265765656973 mm` and still near the `1800 mm` profile bound. Frozen
activity remains long in absolute terms (`45782` frozen days across the
cohort; `42556` days above `200 mm`).

This means the next increment is not publication polish or capacity loosening.
C2 must port the thaw arms and sandwich/thaw-through state machine against the
C1b capacity/overflow path, and it must also explain why the freeze-side
energy/resistance path still drives depth toward the profile bound.

## Disposition

Increment C1b lands. FDHP01 remains `executed-hold` because D3 depth/duration
acceptance is still open. The next executable increment is C2 from
`d3-staged-increment-plan.md`.

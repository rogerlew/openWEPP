# Timing Comparator Evidence

Status: COMPLETE. Evidence mode: Ran.

## Commands

- Materialization:
  `/home/workdir/wepppy/.venv/bin/python docs/work-packages/20260707-laned-router-d16-hybrid-noharm-selector-solvecost-hold-lift-001/artifacts/materialize_selected_cohort.py`
- Execution:
  `/home/workdir/wepppy/.venv/bin/python docs/work-packages/20260707-laned-router-d16-hybrid-noharm-selector-solvecost-hold-lift-001/artifacts/run_active_suite.py`
- Summary:
  `/home/workdir/wepppy/.venv/bin/python docs/work-packages/20260707-laned-router-d16-hybrid-noharm-selector-solvecost-hold-lift-001/artifacts/summarize_active_suite.py`

Primary evidence files:

- `artifacts/active-suite-command-log.json`
- `artifacts/active-suite-run-summary.md`
- `artifacts/active-suite-summary.json`
- `artifacts/active-suite-summary.md`
- `artifacts/active-suite-run-logs/*.time.log`

The raw `selected-cohort-runs/` directories are generated working fixtures and
are intentionally not committed in this package. Recreate them with
`artifacts/materialize_selected_cohort.py`, then rerun
`artifacts/run_active_suite.py` to regenerate raw manifests and outputs.

## Timing Summary

| Member | Plain user | Hybrid-request user | Delta | Selector result |
|---|---:|---:|---:|---|
| `h2637` | `40.05 s` | `33.62 s` | `-6.43 s` (`-16.05 %`) | selected hybrid `11590/11590` |
| `mn_corn_h4` | `0.52 s` | `0.54 s` | `+0.02 s` | fallback `209/209` |
| `n_idaho_forest_h1` | `0.95 s` | `0.96 s` | `+0.01 s` | fallback `185/185` |
| `wa_cascades_forest_h1` | `15.49 s` | `15.46 s` | `-0.03 s` | fallback `6905/6905` |
| Aggregate | `57.01 s` | `50.58 s` | `-6.43 s` (`-11.28 %`) | selected-cohort no-harm lifted |

## Profile Counters

| Member/mode | Implicit steps | Map evals | Branch evals | Alpha evals |
|---|---:|---:|---:|---:|
| `h2637` plain | `0` | `0` | `0` | `173774272` |
| `h2637` hybrid request | `980804` | `0` | `20110873` | `119746445` |
| `mn_corn_h4` hybrid request | `0` | `0` | `0` | `611120` |
| `n_idaho_forest_h1` hybrid request | `0` | `0` | `0` | `1423970` |
| `wa_cascades_forest_h1` hybrid request | `0` | `0` | `0` | `46177740` |

The non-bare members' hybrid-request profile counters match active plain
because all requested lane-days fell back to plain.

## Closure Surfaces

Hybrid-request max closure residuals:

| Member | Cascade rel | Seam rel | Identity rel |
|---|---:|---:|---:|
| `h2637` | `4.58e-13` | `4.08e-14` | `4.44e-13` |
| `mn_corn_h4` | `1.04e-14` | `3.59e-15` | `8.75e-14` |
| `n_idaho_forest_h1` | `7.73e-15` | `5.21e-15` | `5.26e-14` |
| `wa_cascades_forest_h1` | `1.45e-14` | `2.59e-14` | `5.93e-14` |

All are far below active closure tolerances.

## Output Deltas

- H2637 retains the known hybrid exact-bare-skin publication movement:
  outlet `-0.43957 %`, `tdet` sum `-1.8883 %`, `sedcon_1..5` sums
  `-6.4742 %`. These remain diagnostic for future promotion tolerance
  ratification and are not closed here.
- `mn_corn_h4`, `n_idaho_forest_h1`, and `wa_cascades_forest_h1` are
  active-plain identical under hybrid request for the summarized outlet and
  pass-sediment surfaces; HBP and pass parquet hashes are equal for each.

## Disposition

`GAP-OFEHYB-003` selected-cohort timing no-harm is resolved for opt-in hybrid
request at current mesh. Non-bare solve-cost viability and
`INV-OFEHYB-008` default-promotion/tolerance ratification remain open.

# Timing Profile Adjudication

Status: EXECUTED. Evidence mode: Ran.

Sources:

- `../20260707-laned-router-d16-rowcrop-canhgt-active-runtime-publication-001/artifacts/active-suite-command-log.json`
- `../20260707-laned-router-d16-rowcrop-canhgt-active-runtime-publication-001/artifacts/active-suite-summary.json`
- `../20260707-laned-router-d16-rowcrop-canhgt-active-runtime-publication-001/artifacts/active-suite-run-logs/*.time.log`

## Endpoint Timing

| Member | Plain user | Hybrid user | User delta | Plain wall | Hybrid wall |
|---|---:|---:|---:|---:|---:|
| `h2637` | `40.17 s` | `33.62 s` | `-16.31 %` | `0:40.21` | `0:33.66` |
| `mn_corn_h4` | `0.56 s` | `0.60 s` | `+7.14 %` | `0:00.58` | `0:00.62` |
| `n_idaho_forest_h1` | `0.96 s` | `1.23 s` | `+28.13 %` | `0:00.97` | `0:01.25` |
| `wa_cascades_forest_h1` | `15.65 s` | `24.50 s` | `+56.55 %` | `0:15.68` | `0:24.52` |

Aggregate selected user time:

- Plain: `57.34 s`.
- Hybrid: `59.95 s`.
- Hybrid aggregate delta: `+4.55 %`.

## Profile Counters

| Member | Step delta | Hybrid implicit steps | Hybrid map evals | Map evals / implicit step | Alpha eval delta |
|---|---:|---:|---:|---:|---:|
| `h2637` | `-29.56 %` | `980804` | `0` | `0.00` | `-31.09 %` |
| `mn_corn_h4` | `-13.67 %` | `2996` | `1209254` | `403.62` | `-17.30 %` |
| `n_idaho_forest_h1` | `-5.16 %` | `1984` | `2707529` | `1364.68` | `-6.55 %` |
| `wa_cascades_forest_h1` | `-33.11 %` | `500560` | `98192634` | `196.17` | `-38.98 %` |

Interpretation:

- H2637 wins because the exact bare-skin evaluator applies: hybrid removes
  about `29.6 %` of steps and pays zero implicit map-evaluation cost.
- WA Cascades loses despite a `33.1 %` step reduction because the selected
  implicit cells require generic non-bare map iteration: `98.2 M` map
  evaluations.
- The small MN and N Idaho cases show the same generic-map pattern at smaller
  scale: explicit steps fall, but setup/solve overhead dominates.
- The current selector does not predict when implicit work is cheap enough to
  beat the plain explicit path.

## Closure And Deltas

| Member | Outlet delta | HBP hash equal | Pass hash equal | Identity residual plain/hybrid |
|---|---:|---:|---:|---:|
| `h2637` | `-0.43957 %` | false | false | `2.45e-13` / `4.44e-13` |
| `mn_corn_h4` | `-0.014109 %` | true | true | `8.75e-14` / `8.75e-14` |
| `n_idaho_forest_h1` | `-0.005746 %` | false | false | `5.26e-14` / `5.26e-14` |
| `wa_cascades_forest_h1` | `-0.012918 %` | false | false | `5.93e-14` / `5.93e-14` |

The closure surfaces remain machine-scale. The promotion concern is fidelity
tolerance and timing no-harm, not active closure failure.

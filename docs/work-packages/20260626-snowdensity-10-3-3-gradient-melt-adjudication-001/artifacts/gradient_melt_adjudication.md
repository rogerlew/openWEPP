# SNOWDENSITY-10.3.3 Gradient Melt Adjudication

Evidence mode: Ran.

- Schema: `snowdensity10-3-3-gradient-melt-adjudication-v1`
- Contract: `SC-SNOWFREEZE-001 INV-SNOWFREEZE-050 INV-SNOWFREEZE-057 INV-SNOWFREEZE-063`
- Runtime coupling: `diagnostic snowbench replay only; no production activation`
- No site constants: `True`
- No coefficient retuning: `True`
- Disposition: `LOW-CANOPY-NON-PROMOTION`
- Answer: The shortwave/albedo CoE modernization does not earn low-canopy value on the current verdict-bearing evidence. It should stay opt-in/diagnostic while later packages diagnose partition, rain-on-snow heat, and canopy longwave mechanisms.

## Verdict Summary

| Scope | Fail delta | Score delta | Candidate fails | Legacy fails | Candidate score | Legacy score | Earns value |
|---|---:|---:|---:|---:|---:|---:|---|
| `verdict_bearing` | -1 | 0 | 8 | 7 | 92 | 92 | `False` |
| `verdict_bearing_low_canopy` | -1 | 0 | 7 | 6 | 70 | 70 | `False` |

## Regime Summary

| Regime | Fail delta | Score delta | Candidate fails | Legacy fails | Candidate score | Legacy score | Scope note |
|---|---:|---:|---:|---:|---:|---:|---|
| `conifer` | 0 | 0 | 1 | 1 | 22 | 22 | Marcell conifer exact binding. |
| `mixed` | -1 | -1 | 6 | 5 | 32 | 33 | Diagnostic unweighted aggregate only; no mixed-stratum verdict. |
| `deciduous` | -1 | 0 | 4 | 3 | 34 | 34 | Marcell deciduous + Harvard hardwood exact bindings. |
| `open_pasture` | 0 | 0 | 3 | 3 | 36 | 36 | Marcell open + Harvard open exact bindings. |

## Comparison Set

| Comparison | Regime | Fixture | Stratum | Scope | Rows |
|---|---|---|---|---|---:|
| `marcell_conifer` | `conifer` | `marcell_conifer_mn` | `conifer` | `verdict_bearing` | 237 |
| `marcell_deciduous` | `deciduous` | `marcell_deciduous_mn` | `deciduous` | `verdict_bearing` | 255 |
| `marcell_open` | `open_pasture` | `marcell_open_mn` | `open` | `verdict_bearing` | 214 |
| `harvard_hardwood` | `deciduous` | `harvard_deciduous_ma` | `hardwood` | `verdict_bearing` | 821 |
| `harvard_open` | `open_pasture` | `harvard_open_ma` | `open` | `verdict_bearing` | 821 |
| `marcell_mixed_aggregate` | `mixed` | `marcell_mixed_mn` | `unweighted_site_strata` | `diagnostic_only` | 226 |
| `harvard_mixed_aggregate` | `mixed` | `harvard_mixed_ma` | `unweighted_site_strata` | `diagnostic_only` | 402 |

## Model Summaries

| Model | Verdict paired | Verdict robust fails | Verdict robust score | Low-canopy fails | Low-canopy score | Robust counts |
|---|---:|---:|---:|---:|---:|---|
| `legacy_coe` | 1229 | 7 | 92 | 6 | 70 | `{"fail": 7, "marginal": 6, "pass": 10, "strong": 22, "unavailable": 15}` |
| `coe_shortwave_albedo_v1` | 1229 | 8 | 92 | 7 | 70 | `{"fail": 8, "marginal": 6, "pass": 7, "strong": 24, "unavailable": 15}` |

## Unbound Observations

| Source | Stratum | Rows | Status | Reason |
|---|---|---:|---|---|
| `harvard_hf237` | `hemlock` | 821 | `unbound_no_pure_conifer_fixture` | Harvard delineation has no pure hemlock/conifer hillslope; mixed proxy is not a verdict-bearing binding. |

Promotion/default activation is not authorized by this diagnostic package.
Mixed-regime rows are diagnostic unweighted aggregates and do not carry canopy-stratum verdicts.

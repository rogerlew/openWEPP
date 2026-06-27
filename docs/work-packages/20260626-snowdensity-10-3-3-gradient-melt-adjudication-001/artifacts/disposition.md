# Disposition

Status: complete.

Closure: `COMPLETE-10-3-3-GRADIENT-MELT-ADJUDICATED`.

The package scaffolded and executed a diagnostic canopy-gradient CoE melt
adjudication. It added
`tools/snowfreeze_observed/cancov_gradient_melt_adjudication.py`, a focused
guard test, and committed JSON/Markdown report artifacts.

## Result

Disposition: `LOW-CANOPY-NON-PROMOTION`.

`coe_shortwave_albedo_v1` does not earn low-canopy value relative to
`legacy_coe` under the current verdict-bearing Harvard/Marcell stratified
evidence:

| Scope | Legacy robust fail | Opt-in robust fail | Legacy score | Opt-in score |
|---|---:|---:|---:|---:|
| Verdict-bearing | 7 | 8 | 92 | 92 |
| Low-canopy verdict-bearing | 6 | 7 | 70 | 70 |

Regime summary:

- Conifer exact binding: neutral (`fail 1 -> 1`, score `22 -> 22`).
- Deciduous exact bindings: worse (`fail 3 -> 4`, score `34 -> 34`).
- Open/pasture exact bindings: neutral (`fail 3 -> 3`, score `36 -> 36`).
- Mixed aggregates: diagnostic-only, worse (`fail 5 -> 6`, score `33 -> 32`).

Harvard hemlock remains observation-installed but unbound to a pure
hemlock/conifer modeled hillslope and is excluded from verdict-bearing evidence.

## Boundary

No production activation, default, parser/runfile/user CLI selector, output
schema, coefficient, radiation, canopy, albedo, density, snow/rain partition,
frost, or fixture-input change was made.

Next route: §10.3.4 Maritime Over-Accumulation Diagnosis.

# Closure Evidence

Evidence mode: Ran.

Primary artifact:

- `artifacts/legacy_snow_comparison.json`
- `artifacts/legacy_snow_comparison.md`

All-site result:

- Route counts: `{'BOTH-FAIL-LEGACY-CLOSER-FLAG': 2, 'BOTH-FAIL-OPENWEPP-CLOSER-FLAG': 1, 'NO-PAIRED-OBSERVED-SNOW-DEPTH': 2}`
- Legacy closer by mean absolute observed-depth residual: Site 1 Sleepers South
  and Site 4 Morris.
- openWEPP closer by mean absolute observed-depth residual: Site 2 Sleepers W9.
- Sites 3 and 5 have no paired observed snow-depth rows.

Paired snow-depth control summary:

| Site | openWEPP mean abs m | Legacy mean abs m | openWEPP failures | Legacy failures | Disposition |
| --- | ---: | ---: | ---: | ---: | --- |
| Site 1 Sleepers South | `0.414590` | `0.319415` | `322/384` | `296/384` | Both fail; legacy closer flag. |
| Site 2 Sleepers W9 | `0.348239` | `0.371766` | `143/193` | `148/193` | Both fail; openWEPP closer flag. |
| Site 4 Morris | `0.068716` | `0.053343` | `28/83` | `23/83` | Both fail; legacy closer flag. |

openWEPP-vs-legacy model-surface summary:

- Depth mean absolute deltas by site: `0.044829 m`, `0.010797 m`,
  `0.006906 m`, `0.002599 m`, `0.004521 m`.
- SWE mean absolute deltas by site: `0.013429 m`, `0.002940 m`,
  `0.001900 m`, `0.000665 m`, `0.001222 m`.
- Legacy daily-winter capture had no missing nonzero-SWE dates on any site.

Disposition:

- Complete.
- Legacy WEPP is not consistently better at snow-depth observations and both
  models fail snow-depth control on the paired sites.
- Current openWEPP SWE is close to legacy SWE, so SWE lineage is not the leading
  suspect.
- The next package remains snow-depth producer/carry/input/settlement
  adjudication, with legacy source used as guide/flag evidence but not a
  bit-match target.

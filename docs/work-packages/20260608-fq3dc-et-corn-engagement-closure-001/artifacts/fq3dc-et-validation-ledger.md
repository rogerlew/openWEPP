# FQ3-DC ET Validation Ledger

Status: complete

Evidence mode: Static + Ran.

## Spot Validation

Ran: clean output root `/tmp/fq3dc_et_after4`.

| Prefix | Rows | Ep total | Ep nonzero days | Interception total | Interception nonzero days | Es total | Er total | Q total | Q nonzero days |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| p8 Corn | 2557 | 1938.103398211248 | 1085 | 615.0133788383012 | 572 | 2432.099428023835 | 0.0 | 320.73667698020574 | 137 |
| p1 Tah perennial | 2557 | 5475.201811235968 | 2557 | 643.3614332068395 | 840 | 19.9936848344507 | 0.0 | 138.177034433567 | 201 |

Static: p1 remains on the previously working perennial path with nonzero `Ep`
and interception. The prior openWEPP p1 `Ep` reference was approximately
`5511 mm`; post-fix p1 `Ep=5475.201811235968`, a `-36 mm` over-7-year delta
(`-0.65%`). This is classified as a non-blocking corrected-calendar perturbation:
the scheduler now publishes Julian `day` for all management paths instead of
day-of-month, while interception remains stable and closure holds. `Er=0`
matches upstream expected-config-zero classification.

## Corn Population Validation

Ran: 36 Corn runfiles from the FQ-3 upstream classification under
`/tmp/fq3dc_et_population`.

- Prefix count: `36`
- Missing or failed prefixes: `[]`
- Zero `Ep`: `[]`
- Zero `Interception`: `[]`
- Nonzero `Er`: `[]` (`Er=0` is accepted expected-config-zero from upstream
  legacy/openWEPP classification)
- `Ep` total: `69048.60615122155 mm`
- `Interception` total: `22140.48163817884 mm`
- `Q` total: `11337.38078494163 mm`
- Min prefix `Ep`: `p4`, `1820.4776976239411 mm`
- Max prefix `Ep`: `p38`, `1987.2911530664057 mm`

Population output summary file:
`/tmp/fq3dc_et_population/corn_population_summary.csv`.

## Closure Validation

Ran: annual residual ledger
`/tmp/fq3dc_et_population/annual_closure_residuals.csv`.

Identity:
`RM + Irr - Interception - Q - Ep - Es - Er - Dp - latqcc - Tile - delta(Total-Soil)`.

- Annual rows: `216` (`36` prefixes x years `2..7`)
- Max absolute residual: `3.1604940886609256e-11 mm`
- Mean absolute residual: `1.4051114181644085e-11 mm`
- Worst row: `p43`, year `6`, residual `-3.1604940886609256e-11 mm`

## Acceptance

The Corn annual-crop `Ep` and canopy interception defect is closed. The original
package wording also named `Er`, but upstream evidence showed `Er=0` is not an
observed defect for this population because legacy `Er` is also zero.

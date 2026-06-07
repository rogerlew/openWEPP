# FQ4 Frost Validation Ledger

Status: complete

Evidence mode: Ran.

## Population

Ran: 43-prefix algebraic-radium single-OFE population under
`/tmp/fq4_population`.

- Prefix count: `43`
- Failures: `[]`
- WAT outputs: `43`
- `frsoil.active=false`: `[]`
- Zero `frozwt` prefixes: `[]`
- `max(frozwt)` min/max across prefixes:
  - min: `27.499999999999993 mm`
  - max: `31.000000000000007 mm`
- `Q` total min/max across prefixes:
  - min: `377.4596796181719 mm`
  - max: `932.4370340257645 mm`

Population summary:
`/tmp/fq4_population/activation_summary.csv`.

## p8 Spot

Pre-fix `/tmp/fq4_pre`:

- WAT rows: `2557`
- `sum(frozwt)=0`
- `max(frozwt)=0`
- `nonzero_frozwt=0`
- `Ep=1938.1033982112465`
- `Interception=615.0133788383013`
- `Q=320.7366769802057`

Post-fix `/tmp/fq4_after`:

- WAT rows: `2557`
- `sum(frozwt)=28902.293333333757 mm-day`
- `max(frozwt)=30.399999999999995 mm`
- `nonzero_frozwt=1017`
- `Ep=1897.926319484048`
- `Interception=615.0133788383013`
- `Q=714.0252915305779`
- `Dp=625.9498077438285`
- `latqcc=268.55779256823564`

## Paired p8 On/Off

Ran: `/tmp/fq4_pair`.

| Metric | Frost on | Frost off (`wintRed=0`) | On - off |
|---|---:|---:|---:|
| `sum(frozwt)` | 28902.293333333757 | 0.0 | 28902.293333333757 |
| `max(frozwt)` | 30.399999999999995 | 0.0 | 30.399999999999995 |
| `Q` | — | — | 393.2886145503722 |
| `Dp` | — | — | 0.0 |
| `latqcc` | — | — | -62.19133256957724 |

## Annual Closure

Ran: `/tmp/fq4_population/annual_closure_residuals.csv`.

Identity:
`RM + Irr - Interception - Q - Ep - Es - Er - Dp - latqcc - Tile - delta(SoilWaterTotal)`.

Using `SoilWaterTotal` includes `frozwt` in storage.

- Annual rows: `258` (`43` prefixes x years `2..7`)
- Max absolute residual: `3.2173375075217336e-11 mm`
- Mean absolute residual: `1.2140145908367492e-11 mm`
- Worst row: `p39`, year `6`, residual `-3.2173375075217336e-11 mm`

## Acceptance

Frost activation is closed across the repaired single-OFE population. The old
FROSTVAL01 `frost-break` verdict is withdrawn as a defective-ledger artifact;
the corrected full-WAT identity closes with frost engaged.

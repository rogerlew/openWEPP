# Sanity Results

Status: `SANITY-PASS-WITH-FINDING`

Evidence mode: `Ran`

## Fresh Debug Suite

Ran:

`cargo nextest run -p openwepp-runner --test
mt3_hbp_hourly_consumer_contract --no-capture`

Result: `7/7 PASS`, nextest `13.283 s`, wall `13.92 s`, run ID
`7431c048-2070-4ddd-bf50-1fc5d09f17c4`. The suite emitted 15 KW/CREAMS result
rows and four KW timestep comparisons. It emitted no `W11C_FINDING` row.

## Representative Current Results

| Branch/grid/scenario | Outlet m3 | Storage m3 | Peak m3/s | Peak/input |
|---|---:|---:|---:|---:|
| KW / 3,600 / early spike | 7199.999986904 | 0.000013096 | 0.999951840 | 0.499975920 |
| KW / 600 / early spike | 7199.999996721 | 0.000003279 | 1.999993817 | 0.999996908 |
| KW / 3,600 / early spread | 7199.999978507 | 0.000021493 | 0.499996248 | 0.999992497 |
| KW / 600 / early spread | 7199.999995122 | 0.000004878 | 0.500000000 | 1.000000000 |
| KW / 3,600 / late spike | 7134.526047370 | 65.473952630 | 0.992440232 | 0.496220116 |
| KW / 600 / late spike | 7089.739831820 | 110.260168180 | 1.999993817 | 0.999996908 |
| CREAMS / early spike | 7200.000000000 | 0 | 1.954502253 | 0.977251127 |

All printed KW/CREAMS zero rows were exact zero. The four MC zero controls
executed with peak and outlet volume within `1e-12`; their other unasserted
fields are not claimed as exact. Across printed KW rows, storage was finite and
nonnegative in `[0, 110.260168180] m3`, peak/input was at most `1.0`, and
terminal volume never materially exceeded 7,200 m3. Maximum printed absolute
channel-balance residual was approximately `1.779e-12 m3`; maximum absolute
sediment residual was approximately `4.83e-13 kg`.

Uniform KW carries the authorized steady initial/final hydraulic storage
`10.168594800 m3`, so the raw diagnostic `input - outlet - final storage` is
`-10.168594800 m3`; the initial-storage-aware public channel ledger closes at
roundoff. This is the declared INV-ROUTE-021 state, not generated water.

## W11C Before/After Comparator

| Historical W11C defect | Before | Fresh current result |
|---|---|---|
| KW 3,600-second early spike | outlet `7265.192021`, storage `-65.192021 m3` | outlet `7199.999986904`, storage `0.000013096 m3` |
| KW 600-second early spike | outlet `7261.723300`, storage `-61.723300 m3` | outlet `7199.999996721`, storage `0.000003279 m3` |
| CREAMS serial publication | `14,400 m3`, element 1, spike sediment `0.133333 kg` | `7,200 m3`, element 2, `240 kg` |
| active W11C MC grids | published amplified/timestep-sensitive results | 16 active cases typed E003 before publication; 4 zero controls execute |
| admitted MC | not demonstrated | both 60-second static/dynamic real CLI routes pass finite/passive/balance assertions |
| zero-count sidecar | aliased requested 600 seconds to default 60 seconds | parsed 600-second result matches positive-count 600 control and differs from 60-second candidate |

## Finding W11E-F001 — KW Timestep Sensitivity

Severity: `Medium` classification/evidence finding, not a demonstrated
canonical defect.

The corrected KW route still has a material grid response: early-spike peak
changes `0.999951840 -> 1.999993817 m3/s`, late-spike peak changes
`0.992440232 -> 1.999993817 m3/s`, and late storage changes
`65.473952630 -> 110.260168180 m3` from 3,600 to 600 seconds. Spread and uniform
forcing are nearly grid-insensitive. Every observed result remains finite,
nonnegative, passive, terminal-volume bounded, and ledger-consistent, so no
INV-ROUTE-021/022 violation is shown. The result is nevertheless too material
to call unqualified `SANITY-PASS` or physical timestep convergence.

Current debug classification: `SANITY-PASS-WITH-FINDING`, conditional on exact
release reproduction and remaining closure/review gates. Future validation
claims should adjudicate W11E-F001 against independent routing/timestep
authority; W11E does not label it an open production defect.

## Exact Release Reproduction

The delegated runner rebuilt/confirmed the exact watershed release binary and
ran the same seven-test consumer suite through its absolute path. Result:
`7/7 PASS`, run ID `65342801-a814-43a7-be38-b5234c4ceeff`, nextest `2.553 s`,
wall `3.13 s`. It emitted the same 15 result rows and four timestep rows, no
`W11C_FINDING` row, and the same W11E-F001 peak/storage deltas.

Release binary SHA-256:
`f82cc9fa539d26cdf9a6797d3e272bca22a7a19dc4b9988a3a95e7cd4c38d792`.
Exact path, stat, source-baseline distinction, and command are bound in
`release-binary-provenance.md`.

Terminal result classification: `SANITY-PASS-WITH-FINDING`. Heavy gates and
both same-agent verifications pass.

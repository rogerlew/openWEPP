# FQ3-DC Runoff Validation Ledger

Status: complete

Evidence mode: Ran.

## Evidence Locations

- Spot rerun outputs: `/tmp/fq3dc_after2/outputs/{p8,p1}`
- Population rerun outputs: `/tmp/fq3dc_population_after2/outputs`
- Run status: `/tmp/fq3dc_population_after2/run_status.tsv`
- Runoff summary CSV: `/tmp/fq3dc_population_after2/runoff_summary.csv`
- Annual closure CSV: `/tmp/fq3dc_population_after2/annual_closure_residuals.csv`

## Spot Results

| prefix | before Q mm | after Q mm | after QOFE mm | after max daily Q mm |
| --- | ---: | ---: | ---: | ---: |
| p8 | 3.930232875259954e-15 | 513.5200235860505 | 513.5200235860505 | 38.27585635657911 |
| p1 | 3.0643619152587176e-13 | 138.17703443356697 | 138.17703443356697 | 22.371848095086023 |

Spot annual closure:

- after max abs residual: `1.4949819160392508e-11 mm`
- before max abs residual: `3.512923285597935e-11 mm`

## Population Results

Population: 42 runnable single-OFE prefixes, excluding protected p11.

- before nonzero `Q > 1e-6 mm`: `7/42`
- after nonzero `Q > 1e-6 mm`: `42/42`
- after min annual-run sum `Q`: `73.84564909923932 mm`
- after max annual-run sum `Q`: `988.3316769592636 mm`
- `QOFE` equals `Q` on the single-OFE outputs.

Annual closure over years 2..7:

- rows: `252`
- max abs residual: `2.808064891723916e-11 mm`
- mean abs residual: `1.2249418410820432e-11 mm`

Worst rows are numerical noise only; largest observed row was `p5` year 4
`-2.808064891723916e-11 mm`.

## Acceptance

Accepted. The correction makes runoff measurable on the affected population and
preserves WAT annual conservation closure. Magnitudes are not accepted by legacy
matching; legacy was used only as the nonzero-runoff reachability flag.

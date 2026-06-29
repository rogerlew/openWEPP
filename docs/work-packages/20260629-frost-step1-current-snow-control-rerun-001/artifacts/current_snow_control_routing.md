# FROST STEP 1 Current-Snow Control Routing

Evidence mode: Ran.

- Schema: `frost-step1-current-snow-control-routing-v1`
- Contract: `SC-SNOWFREEZE-001 INV-SNOWFREEZE-047/048/050`
- Route counts: `{'BLOCKED': 1, 'FORCING-LIMITED': 2, 'INCONCLUSIVE-NO-PAIRED-SNOW': 2}`
- Step 2 unblocked sites: `['site1_sleepers_south_field_vt', 'site2_sleepers_w9_hardwood_vt']`

## Per-Site Routing

| Site | Route | Snow Gate | Pairs | Failures | Cover Agreement | Snow Timing Fails | Mean signed m | Max abs m | Attributable Frost Signatures |
| --- | --- | --- | ---: | ---: | ---: | ---: | ---: | ---: | --- |
| site1_sleepers_south_field_vt | FORCING-LIMITED | magnitude-only snow-depth residual | 384 | 218 | 0.940 | 2 | 0.156066 | 0.789709 | frost timing attributable; magnitude carries snow forcing uncertainty |
| site2_sleepers_w9_hardwood_vt | FORCING-LIMITED | magnitude-only snow-depth residual | 193 | 72 | 0.959 | 5 | 0.081377 | 0.582594 | frost timing attributable; magnitude carries snow forcing uncertainty |
| site3_scan_mandan_nd | INCONCLUSIVE-NO-PAIRED-SNOW | snow control cannot be established | 0 | 0 | n/a | 0 | n/a | n/a | frost timing report-only until independent snow control exists |
| site4_ggd498_morris_mn | BLOCKED | forcing-robust snow-control defect remains | 83 | 20 | 0.759 | 5 | 0.0440007 | 0.223233 | no frost attribution; snow defect would alias into frost |
| site5_reynolds_creek_us_rls_id | INCONCLUSIVE-NO-PAIRED-SNOW | snow control cannot be established | 0 | 0 | n/a | 0 | n/a | n/a | frost timing report-only until independent snow control exists |

## GAP-SNOWFREEZE-002 Disposition Input

- Status: `open_narrowed`
- Summary: Frost timing attribution is unblocked at paired sites whose only remaining snow-control failure is forcing-limited absolute magnitude; frost magnitude attribution is Step 2. Sites without paired snow depth remain inconclusive for snow control.

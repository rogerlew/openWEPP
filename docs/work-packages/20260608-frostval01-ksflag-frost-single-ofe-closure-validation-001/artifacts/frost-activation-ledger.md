# Frost Activation Ledger

Status: executed-hold
Evidence mode: Ran + Static

Milestone 1 gate verdict:
- Not satisfied for the full 43-single-OFE population.
- 37/43 are blocked before hydrology/frost evaluation by `HS-RUNTIME-E-062`.
- The 6 runnable prefixes (`p8`, `p13`, `p22`, `p23`, `p26`, `p28`) show no frost activation signature under current telemetry.

ksflag authority confirmation:
- Source soils for runnable prefixes contain `1 1` at the `ntemp ksflag` line.
- Paired off-soils were generated with a strict single replacement of standalone `1 1 -> 1 0`.
- Verification file: `/tmp/frostval01/full/off_ksflag_checks.txt`.

Runnable-prefix activation metrics (`activation_summary.csv`):
- `max_frozwt_on_mm = 0` and `max_frozwt_off_mm = 0` for all 6.
- `sum_frozwt_on_mm_day = 0` and `sum_frozwt_off_mm_day = 0` for all 6.
- `frozwt_nonzero_days_on = 0` and `frozwt_nonzero_days_off = 0` for all 6.
- `delta_dp_on_minus_off_mm = 0`, `delta_latq_on_minus_off_mm = 0`, `delta_q_on_minus_off_mm = 0` for all 6.

Interpretation:
- On currently runnable hillslopes, the ksflag on/off paired runs are numerically identical for the tracked flux terms and frost water telemetry. This is classified as `frost-inactive` (no measurable frost gate bite) for those 6.
- For blocked hillslopes, activation is undetermined due to runtime failure before activation evidence can be observed.

Per-single-OFE classification:

| Prefix | Evidence state | Activation evidence summary | Classification |
| --- | --- | --- | --- |
| p1 | blocked | CLIHILL-E-011 runtime surface failure for soil | activation-undetermined-blocked |
| p2 | blocked | CLIHILL-E-011 runtime surface failure for soil | activation-undetermined-blocked |
| p3 | blocked | CLIHILL-E-011 runtime surface failure for soil | activation-undetermined-blocked |
| p4 | blocked | CLIHILL-E-011 runtime surface failure for soil | activation-undetermined-blocked |
| p5 | blocked | CLIHILL-E-011 runtime surface failure for soil | activation-undetermined-blocked |
| p6 | blocked | CLIHILL-E-011 runtime surface failure for soil | activation-undetermined-blocked |
| p7 | blocked | CLIHILL-E-011 runtime surface failure for soil | activation-undetermined-blocked |
| p8 | ran | ksflag=1 honored, frost inactive (frozwt 0; on-off deltas 0) | frost-inactive |
| p9 | blocked | CLIHILL-E-011 runtime surface failure for soil | activation-undetermined-blocked |
| p10 | blocked | CLIHILL-E-011 runtime surface failure for soil | activation-undetermined-blocked |
| p11 | blocked | CLIHILL-E-011 runtime surface failure for soil | activation-undetermined-blocked |
| p12 | blocked | CLIHILL-E-011 runtime surface failure for soil | activation-undetermined-blocked |
| p13 | ran | ksflag=1 honored, frost inactive (frozwt 0; on-off deltas 0) | frost-inactive |
| p14 | blocked | CLIHILL-E-011 runtime surface failure for soil | activation-undetermined-blocked |
| p15 | blocked | CLIHILL-E-011 runtime surface failure for soil | activation-undetermined-blocked |
| p16 | blocked | CLIHILL-E-011 runtime surface failure for soil | activation-undetermined-blocked |
| p17 | blocked | CLIHILL-E-011 runtime surface failure for soil | activation-undetermined-blocked |
| p18 | blocked | CLIHILL-E-011 runtime surface failure for soil | activation-undetermined-blocked |
| p19 | blocked | CLIHILL-E-011 runtime surface failure for soil | activation-undetermined-blocked |
| p20 | blocked | CLIHILL-E-011 runtime surface failure for soil | activation-undetermined-blocked |
| p21 | blocked | CLIHILL-E-011 runtime surface failure for soil | activation-undetermined-blocked |
| p22 | ran | ksflag=1 honored, frost inactive (frozwt 0; on-off deltas 0) | frost-inactive |
| p23 | ran | ksflag=1 honored, frost inactive (frozwt 0; on-off deltas 0) | frost-inactive |
| p24 | blocked | CLIHILL-E-011 runtime surface failure for soil | activation-undetermined-blocked |
| p25 | blocked | CLIHILL-E-011 runtime surface failure for soil | activation-undetermined-blocked |
| p26 | ran | ksflag=1 honored, frost inactive (frozwt 0; on-off deltas 0) | frost-inactive |
| p27 | blocked | CLIHILL-E-011 runtime surface failure for soil | activation-undetermined-blocked |
| p28 | ran | ksflag=1 honored, frost inactive (frozwt 0; on-off deltas 0) | frost-inactive |
| p29 | blocked | CLIHILL-E-011 runtime surface failure for soil | activation-undetermined-blocked |
| p30 | blocked | CLIHILL-E-011 runtime surface failure for soil | activation-undetermined-blocked |
| p31 | blocked | CLIHILL-E-011 runtime surface failure for soil | activation-undetermined-blocked |
| p32 | blocked | CLIHILL-E-011 runtime surface failure for soil | activation-undetermined-blocked |
| p33 | blocked | CLIHILL-E-011 runtime surface failure for soil | activation-undetermined-blocked |
| p34 | blocked | CLIHILL-E-011 runtime surface failure for soil | activation-undetermined-blocked |
| p35 | blocked | CLIHILL-E-011 runtime surface failure for soil | activation-undetermined-blocked |
| p36 | blocked | CLIHILL-E-011 runtime surface failure for soil | activation-undetermined-blocked |
| p37 | blocked | CLIHILL-E-011 runtime surface failure for soil | activation-undetermined-blocked |
| p38 | blocked | CLIHILL-E-011 runtime surface failure for soil | activation-undetermined-blocked |
| p39 | blocked | CLIHILL-E-011 runtime surface failure for soil | activation-undetermined-blocked |
| p40 | blocked | CLIHILL-E-011 runtime surface failure for soil | activation-undetermined-blocked |
| p41 | blocked | CLIHILL-E-011 runtime surface failure for soil | activation-undetermined-blocked |
| p42 | blocked | CLIHILL-E-011 runtime surface failure for soil | activation-undetermined-blocked |
| p43 | blocked | CLIHILL-E-011 runtime surface failure for soil | activation-undetermined-blocked |

Comparator note:
- Comparator parity was not used as acceptance authority (ADR-0017). This package uses conservation/activation authority only.

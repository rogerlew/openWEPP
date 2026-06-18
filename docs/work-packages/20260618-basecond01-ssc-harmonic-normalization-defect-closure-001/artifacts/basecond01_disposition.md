# BASECOND01 Disposition

Evidence class: Static + Ran

Status: complete.

Verdict: `COMPLETE-WITH-CORRECTION`.

Closed defect:

- `BASECOND01-SSC-HARMONIC-NORMALIZATION` is closed.
- Vertical `ssc` / `wb18_perc_ssc_####` projection now follows
  `SC-INFILE-SOIL-001` v0.1.11:
  - top normalized 200 mm interval uses the baseline top source-layer `ksat`
    rule;
  - below the top interval, split-source vertical conductivity is
    inverse-conductivity/harmonic;
  - hourly `wb19_lateral_ssh_####` remains arithmetic from `ksat*anisotropy`.

H2637 disposition:

- H2637 no-UI rerun completed successfully.
- Aggregate WAT/PASS metrics are unchanged within recorded precision:
  `runvol_pct_precip` remains `71.0036550031206`.
- Therefore BASECOND01 does not close the remaining FARPOINT01 H2637 magnitude
  flag.

Routing:

- Do not reopen vertical `ssc` as an unresolved defect.
- Do not make hourly `ui_ssh` harmonic to chase the aggregate comparator flag.
- The next package should adjudicate the remaining H2637 magnitude after
  source-intent conductivity corrections, with the starting premise that
  `REFINTENT001` and `BASECOND01` landed and were H2637 aggregate-inert.

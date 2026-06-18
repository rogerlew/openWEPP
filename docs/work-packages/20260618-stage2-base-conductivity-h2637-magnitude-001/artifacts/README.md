# Artifacts

Status: complete 2026-06-18.

Verdict: `OPENWEPP-DEFECTIVE`.

Completed deliverables:

- `base-cond-sensitivity-probe.md`
- `base-cond-lineage.md`
- `base-cond-source-intent-check.md`
- `base-cond-plausibility.md`
- `base-cond-per-step-verdict.md`
- `base-cond-handoff.md`
- `base-cond_disposition.md`

Summary:

- H2637 base soil conductivity is byte-live: `ksat_x0.9` changed WAT/PASS
  checksums, aggregate `latqcc`, PASS `runvol`, and peak WAT `latqcc`.
- Raw `.sol` `ksat` parsing and H2637 hourly `wb19_lateral_ssh` consumption are
  correct for this fixture.
- Vertical `wb18_perc_ssc` 200 mm normalization is defective: split-layer
  vertical `ssc` must be harmonic/inverse-conductivity normalized, while hourly
  `ui_ssh` remains arithmetic from `ssc2 * ui_anisrt`.

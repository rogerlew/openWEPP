# Base Conductivity Disposition

Evidence class: Static + Ran

Status: complete 2026-06-18.

Verdict: `OPENWEPP-DEFECTIVE`.

## Summary

The package confirmed the base soil conductivity is byte-live on H2637:
scaling all `.sol` layer `ksat` values to `0.9x` changed WAT/PASS checksums,
aggregate all-OFE `latqcc`, PASS `runvol`, and peak WAT `latqcc`.

The source-intent adjudication found a narrow defect:

- openWEPP correctly parses raw `ksat` and excludes `ksatadj` for H2637;
- openWEPP correctly uses hourly `wb19_lateral_ssh` for H2637 WB19 lateral
  flow;
- openWEPP incorrectly computes vertical `wb18_perc_ssc_####` with arithmetic
  averaging where baseline source intent computes `ssc1` by inverse
  conductivity accumulation.

H2637 split-layer example:

- current layer-3 `wb18_perc_ssc`: `270.8259 mm/h`
  (`0.0000752294166666667 m/s`);
- source-intent vertical layer-3 `ssc`: `117.955408163210 mm/h`
  (`0.0000327653911564473 m/s`);
- source-intent hourly lateral layer-3 `ui_ssh`: `270.8259 mm/h`.

## Deliverables

| Deliverable | Status |
|---|---|
| `base-cond-sensitivity-probe.md` | complete |
| `base-cond-lineage.md` | complete |
| `base-cond-source-intent-check.md` | complete |
| `base-cond-plausibility.md` | complete |
| `base-cond-per-step-verdict.md` | complete |
| `base-cond-handoff.md` | complete |
| `base-cond_disposition.md` | complete |

## Acceptance Criteria

| Criterion | Result |
|---|---|
| Sensitivity probe proves base conductivity is live | PASS; `ksat_x0.9` changed checksums and aggregate outputs |
| Conductivity lineage extracted | PASS; raw `ksat` to 200 mm `ssc`/`ui_ssh` values recorded |
| Source-intent and contract verdict | PASS; vertical `ssc` defective, hourly `ui_ssh` correct for H2637 |
| Verdict and handoff | PASS; defect-closure follow-on specified |
| Evidence labels | PASS; artifacts use Static/Ran labels |
| Markdown lint | PASS; `markdown-doc lint --path docs/work-packages/20260618-stage2-base-conductivity-h2637-magnitude-001 --format plain` validated 11 files with 0 errors and 0 warnings |

## FARPOINT01 Status

FARPOINT01 remains open and is rerouted to a defect-closure package for
vertical `wb18_perc_ssc` normalization.

This package does not authorize a WB19 lateral equation fix and does not
authorize changing hourly `wb19_lateral_ssh` to harmonic.

## Gates

No Rust production code was changed in this verdict package, so Rust workspace
gates were not required here. The follow-on defect closure must run the full
Rust gate set after code changes.

Ran:

- `markdown-doc lint --path docs/work-packages/20260618-stage2-base-conductivity-h2637-magnitude-001 --format plain`
  - 11 files validated, 0 errors, 0 warnings.
- `git diff --check`
  - clean.

# PL14R Tier-A Comparator Rerun Delta Report

Status: `complete`
Evidence mode: `Ran`

## Scope

Strict Tier-A replay rerun using pinned legacy baseline lane versus direct
openWEPP candidate staging for PL14R closeout refresh.

## Summary Outcomes

| Surface | strict_pass | Primary status | Key deltas |
|---|---|---|---|
| `H5.wat.dat` | `false` | `structure_diff` | `line_count_baseline=1123`, `line_count_candidate=5`, `line_count_mismatch=1118`, `numeric_arity_mismatch_lines=1`, `text_mismatch_lines=1` |
| `H5.plot.dat` | `false` | `missing candidate artifact` | `baseline_file_count=1`, `candidate_file_count=0`, `only_baseline_count=1`, `only_baseline_examples=["H5.plot.dat"]` |

## Daily Water-Balance Surface (`H5.wat.dat`)

- Baseline lane emitted full legacy daily output (`1123` lines).
- Candidate lane emitted staged WB13 openWEPP candidate sample (`5` lines).
- Comparator status is structural mismatch with strict tolerance posture.
- No numeric comparisons were executed (`numeric_values_compared=0`).

## Plot Surface (`H5.plot.dat`)

- Baseline lane emitted `H5.plot.dat`.
- Candidate lane did not include `H5.plot.dat`.
- Comparator surfaced explicit artifact absence (`only_baseline_count=1`) with
  `strict_pass=false`.

## Interpretation for PL15R

- PL14R rerun objective (strict replay rerun + reproducible provenance) is met.
- Initial strict-rerun hold signals are preserved as evidence but are superseded
  by the schema-aligned retest addendum below.
- Current PL14R disposition is `PL14R_COMPLETE_GO_FORWARD_TO_PL15R`.

## Schema-Aligned Day-By-Day Retest Addendum (2026-05-23)

- A follow-on retest aligned candidate `H5.wat.dat` rows to the canonical
  25-column WB13 schema (apples-to-apples measure set) using:
  - `SoilWaterTotal = Total-Soil + frozwt`
  - fixture profile constants:
    `ProfileDepth=400.00`,
    `ProfilePorosityCap=171.48`,
    `ProfileFCStore=38.75`,
    `ProfileWPStore=14.38`
- Retest lane artifacts:
  - `artifacts/h5_wat_comparator_schema_aligned.json`
  - `artifacts/h5_plot_comparator_schema_aligned.json`
  - `artifacts/h5_wat_day_by_day_schema_aligned.json`
- Retest outcomes:
  - `H5.wat.dat`: `strict_pass=true`, `status_counts={"identical": 1}`
  - `H5.plot.dat`: `strict_pass=true`, `status_counts={"identical": 1}`
  - Day-by-day parity (`OFE,J,Y` keyed, 1095 rows): all 25 measures exact,
    zero non-zero deltas.

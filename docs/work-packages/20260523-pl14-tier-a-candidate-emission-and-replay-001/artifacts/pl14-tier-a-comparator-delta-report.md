# PL14 Tier-A Comparator Delta Report

Status: `complete`
Evidence mode: `Ran`

## Scope

Strict Tier-A replay lane comparison using pinned legacy baseline lane versus
direct openWEPP candidate staging for PL14 closeout.

## Summary Outcomes

| Surface | strict_pass | Primary status | Key deltas |
|---|---|---|---|
| `H5.wat.dat` | `false` | `structure_diff` | `line_count_baseline=1123`, `line_count_candidate=5`, `line_count_mismatch=1118`, `numeric_arity_mismatch_lines=1`, `text_mismatch_lines=2` |
| `H5.plot.dat` | `false` | `missing candidate artifact` | `baseline_file_count=1`, `candidate_file_count=0`, `only_baseline_count=1`, `only_baseline_examples=["H5.plot.dat"]` |

## Daily Water-Balance Surface (`H5.wat.dat`)

- Baseline lane emitted full legacy daily output (`1123` lines).
- Candidate lane emitted staged WB13 openWEPP candidate sample (`5` lines).
- Comparator status is structural mismatch with strict tolerance posture;
  no numeric comparisons were executed (`numeric_values_compared=0`).

## Plot Surface (`H5.plot.dat`)

- Baseline lane emitted `H5.plot.dat`.
- Candidate lane did not include `H5.plot.dat`.
- Comparator surfaced explicit artifact absence (`only_baseline_count=1`) with
  `strict_pass=false`.

## Interpretation for PL15

- PL14 objective (strict replay execution + reproducible provenance) is met.
- Residual strict Tier-A deltas remain open and are forwarded to PL15 for
  closure/risk disposition.

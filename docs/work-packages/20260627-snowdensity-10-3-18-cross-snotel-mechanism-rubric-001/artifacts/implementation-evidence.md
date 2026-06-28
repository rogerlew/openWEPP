# Implementation Evidence

Static:

- Added `tools/snowfreeze_observed/cross_snotel_mechanism_rubric.py`.
- Added focused guard
  `tests/integration/snowdensity10_3_18_cross_snotel_mechanism_rubric.rs` and
  registered it in `Cargo.toml`.
- Added package scaffold and README pointer.

Ran:

- `.venv/bin/python tools/snowfreeze_observed/cross_snotel_mechanism_rubric.py`
  executed the real direct-production WAT path for 50 supported
  site/model combinations: 5 supported current direct-runtime models across 5
  SNOTEL sites and 5 bound `cancov_forest` SWE/depth/density strata.
- The diagnostic also scored archival rejected candidates as explicit
  unavailable profiles and folded the SNOWFROST-FIDELITY-H PySnobal SNOTEL flag
  profile where available.
- Output artifacts:
  `artifacts/cross-snotel-mechanism-rubric.{json,md}`.

Key output:

- Disposition: `DIAGNOSTIC-COMPLETE-NO-PROMOTION-DECISION`.
- Activated bundle: robust fail count `17`, robust score `172`.
- Supported next-lever read: `harder_pomeroy_partition` ranked first among
  supported current direct-runtime candidates (`+7` robust score delta, `+2`
  robust fail delta, `9` better robust cells, `2` worse robust cells vs
  activated).
- 10.3.17 shallow guard remains non-promoted: no aggregate robust-score or fail
  improvement vs activated (`0` score delta, `0` fail delta).
- 10.3.16 sublimation remains worse in this cross-corpus profile (`-19` robust
  score delta, `-3` robust fail delta).
- Humid-New-England cancov residuals are `NOT-REPRESENTATIVE` of the mountain
  SNOTEL activated-bundle fail signature set (`fail_cell_jaccard = 0.2`).

Boundary evidence:

- No production/default/cap/schema/fixture/frost change.
- No parser/runfile/user selector change.
- No site calibration or observed-row-conditioned runtime behavior.
- Legacy and PySnobal remain ADR-0017 flag profiles, not targets.

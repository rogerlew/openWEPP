# Review Disposition

Evidence mode: Static review.

Review A: data provenance and anti-tuning

- Finding: SNOTEL normalized observations are checked in with station triplet,
  source URL, access date, unit normalization, provenance files, and row counts.
  Disposition: accepted.
- Finding: observed-density SSD values are derived from peak-SWE-period density
  before residual comparison. Disposition: accepted; no residual-fit SSD sweep
  was introduced.
- Finding: raw AWDB downloads remain under `target/` cache. Disposition:
  accepted.

Review B: comparator and rubric authority

- Finding: H now emits `snotel-density-three-way-comparison-v2` and binds the
  report to `INV-SNOWFREEZE-049`, `INV-SNOWFREEZE-050`, and
  `TOL-SNOWFREEZE-011`. Disposition: accepted.
- Finding: legacy WEPP and PySnobal are scored as diagnostic profile overlays,
  not correctness targets. Disposition: accepted.
- Finding: CSS Lab PySnobal WY2017 fails inside PySnobal despite finite exported
  forcing. Disposition: hold; do not mark H complete.

Final disposition: executed-held at
`HOLD-PYSNOBAL-CSS-WY2017-SNOBAL-CORE-FAILURE`.

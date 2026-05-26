# MOFE13 Disposition

Status: complete
Evidence mode: mixed (Static + Ran)
Disposition: GO-WB14-KSATADJ-ALIGNED

Disposition date: 2026-05-26

Static:
- Scope completed within declared write set: contract authority, contract tests,
  WB14/runtime implementation, parity rerun evidence.
- Implementation follows canonical migration posture for the scoped regimes
  (9001/9002/9003) with typed domain guards.

Ran:
- Required gates passed (`fmt`, `clippy`, workspace tests, `cargo deny`).
- H324 lane rerun completed with emitted candidate outputs and manifest.
- Canonical baseline comparator remains width-limited (26-column dat parsing);
  normalized investigation run confirms residual row-key mismatch posture.

Decision:
- MOFE13 objective is complete: openWEPP now executes baseline-authoritative
  `ksatadj` WB14 regime logic and produces successful H324 candidate output.
- Semantic parity is not closed in this package due existing comparator
  row-identity mismatch (calendar-year + multi-OFE baseline versus canonicalized
  candidate publication keys).

Follow-on required:
- Comparator/publishing alignment package for shared row-key identity across
  baseline and candidate surfaces so semantic pass/fail can be evaluated on
  common rows.

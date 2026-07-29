# Terminal Verification B — Operator-Adjudicated Exact Tree

Evidence class: `Ran: exact-tree gates and independent arithmetic; Static:
operator authorization, Rust/Python, lifecycle, write-set, contract, and LFS
review`

Verifier disposition: `PASS`

Package scientific disposition:
`COMPLETE / READINESS PASS / SOURCE AUTHORITY SUCCESSOR REQUIRED`

## Findings

No blocking QA finding remains.

The former `HIGH` Incident 002 governance finding is `AUTHORIZED / LIFTED`.
`artifacts/operator-governance-adjudication.md` records the operator's explicit
2026-07-28 authorization: “I authorize the retrospective analysis.” The
authorization is sufficient because the analysis uses only the already frozen,
exhaustively executed grid and ridge and changes no input, axis, observation,
objective, tolerance, stopping rule, result, or parameter selection.

The method remains explicitly labeled retrospective; authorization does not
make it prospective. The exact package consistently limits its effect to the
named direct-runtime surface source/rate operator, classified
`IMPLEMENTED / CALIBRATION_READY_DATA_LIMITED / PARTIALLY_IDENTIFIABLE`.

## Stop-loss and Claim-boundary Verification

- Native leaf transfer remains implemented, but native leaf-only source
  sufficiency is `NOT_CALIBRATION_READY / NOT_ASSESSED` because authenticated
  CAL-04B member-level source traces are unavailable.
- Recurring needle and fine-woody sources remain
  `AUTHORITY_MISSING / NOT_CALIBRATION_READY / NOT_ASSESSED`; unavailable
  components are not assigned zero values or hidden in decay.
- The source-composition system remains
  `AUTHORITY_MISSING / NOT_CALIBRATION_READY / NONIDENTIFIABLE`.
- Empirical decomposition fitting remains
  `NOT_CALIBRATION_READY / NOT_ASSESSED / AUTHORITY_BLOCKED`. No preferred
  source/rate value, carbon-to-dry-mass conversion, or
  organic-horizon-to-modeled-pool substitution is introduced.
- Bounded contract-first `CANOPY-LITTER-SOURCE-AUTHORITY-01` remains required
  before roadmap Orders 6-8. The readiness pass does not authorize its science
  or implementation.

## Exact-tree Evidence

- Primary evidence contains 16 members and 116,800 rows; ridge evidence
  contains five members and 36,500 rows. Every member retains the exact
  `frame_day_index=0..7299` sequence and complete carried
  surface/interrill/rill state.
- All 16 primary and five ridge reconstructions pass. `S020-K050` is the sole
  zero-SSE daily truth, and all five terminal-ridge members remain within the
  frozen tolerance.
- Independent recomputation reproduced all eight retrospective sensitivities,
  covariance, and correlation with zero delta. All 16 boundary/failure cases
  and 28 Harvard custody-bound diagnostics pass the terminal validator.
- Normal validation passes; optimized Python fails closed with exit `1`.
  Focused runner nextest
  `26d2c2ee-c5c4-4083-8d97-dfe1a1fbc93c` passed one test with 220 skipped.
- Package-local Rust format, warnings-denied Clippy, zero-test harness, and
  dependency policy pass. Python compilation, 30-file Markdown lint, and
  `git diff --check` pass.
- The three predecessor authority hashes match. Git LFS attributes and
  `git lfs fsck --objects` pass for primary object
  `e0778f36e0286a30fe17523cd19e0f204928c8c17211409de17b7af4cb970e63`
  and ridge object
  `4ee2c2e0c757a7aa61c37ff3c9627f74a98f78e489d615e05a4acbe720421d1d`.
- The package contains exactly 51 retained non-build files, including the
  operator adjudication. CAL-05-attributable writes remain within the declared
  package/docs set; no production crate, canonical contract, protected
  fixture, canonical test, management, default, or scientific parameter
  changed.

## Non-blocking Debt and Follow-ups

- A reusable prospective successor should encode the sensitivity stencil and
  denominators as design data rather than fixed script constants.
- The package-local Rust harness has zero unit tests; reusable follow-on
  tooling should test multi-day identity and typed failure serialization.
- `tools/analyze.py` reports one mixed-dimension reconstruction maximum under a
  mass-unit label. Split dimensional tolerances before reuse.

## QA Statement

`PASS`. The operator authorization truthfully lifts the sole retrospective
governance hold while preserving the retrospective label and all independent
source-authority, native-sufficiency, material-mapping, and empirical-fit
barriers. The exact tree supports
`COMPLETE / READINESS PASS / SOURCE AUTHORITY SUCCESSOR REQUIRED`.

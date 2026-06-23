# Release Readiness

Status: blocked-by-R7F.

## R7H Evidence

- Direct-mode runtime contract: not release-ready. Default-candidate and
  rollback mechanics exist, but production direct has a counted hot
  compatibility edge.
- Rollback window: explicit compatibility rollback is implemented through API
  policy and CLI `--compatibility-runtime`.
- Operator CLI/API docs: not completed; release docs would be premature while
  R7F is red.
- Manifest expectations: partially implemented through top-level
  `runtime_selection` provenance and corrected direct compatibility-edge
  counters.
- Anti-evasion checks: not implemented; release checks must be added after R7F
  is closed.
- R7A-R7G catalog links: not complete.

## Disposition

R7H remains blocked. No release cutover readiness claim is valid until the
direct day-input builder no longer constructs compatibility surfaces in the
production direct hot loop and R7G performance/fixture gates pass.

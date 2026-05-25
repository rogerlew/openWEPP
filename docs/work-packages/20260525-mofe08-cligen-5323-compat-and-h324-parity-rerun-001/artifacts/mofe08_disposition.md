# mofe08_disposition

Status: HOLD
Evidence mode: mixed (Static + Ran)

Disposition date: 2026-05-25

Static:
- Scoped CLIGEN `5.323` compatibility objective is implemented.

Ran:
- Climate parser accepts `5.323` and canonicalizes to `5.3` (test-verified).
- Carved-letter `H324` parity lane was rerun after compatibility change.

HOLD rationale:
- MOFE parity comparator remains blocked because candidate generation now fails
  at a downstream runtime soil surface requirement:
  - `HS-RUNTIME-E-003: primary soil layer missing required theta_r_rosetta (thetdr)`
- This is a post-climate blocker outside the scoped CLIGEN parser compatibility
  change and prevents semantic comparator execution.

Follow-on required:
- New package to close `HS-RUNTIME-E-003` for carved-letter/7778 soil runtime
  surface projection, then rerun semantic comparator for `H324`.

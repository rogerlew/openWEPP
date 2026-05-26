# MOFE09 Disposition

Status: HOLD
Evidence mode: mixed (Static + Ran)

Disposition date: 2026-05-25

Static:
- Scoped runtime seam objective is implemented with canonical contract authority.

Ran:
- `HS-RUNTIME-E-003` blocker is closed for legacy `7778` measured-theta soils.
- Carved-letter `H324` parity lane rerun executed.

HOLD rationale:
- Comparator remains blocked because candidate generation now fails at a
  downstream management projection guard:
  - `HS-RUNTIME-E-050: PL projection field gddmax ... out of domain (0, allowed >0.0)`

Follow-on required:
- New package to close `HS-RUNTIME-E-050` for carved-letter management runtime
  projection, then rerun semantic comparator for `H324`.

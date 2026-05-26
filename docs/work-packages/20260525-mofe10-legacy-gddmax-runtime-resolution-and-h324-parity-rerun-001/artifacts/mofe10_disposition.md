# MOFE10 Disposition

Status: HOLD
Evidence mode: mixed (Static + Ran)

Disposition date: 2026-05-25

Static:
- Scoped `gddmax=0` legacy sentinel behavior is implemented with canonical
  contract authority and typed runtime guard posture.

Ran:
- `gddmax` sentinel projection and resolution path is active in runtime.
- Carved-letter `H324` parity lane rerun executed.

HOLD rationale:
- Comparator remains blocked because candidate generation now fails at a
  downstream management projection guard:
  - `HS-RUNTIME-E-050: PL projection field oratea ... out of domain (0, allowed >0.0)`

Follow-on required:
- New package to close `oratea` zero-domain compatibility behavior for carved-letter
  management runtime projection, then rerun semantic comparator for `H324`.

# Disposition

Status: HOLD

Evidence mode: ran

Static:

- HPHYS0306 is a diagnostic/evidence package and does not authorize production
  physics edits.
- Inactive fixed-baseline melt hours are not zero-imputed.
- Downstream WB13/WB17/WB18/WB19/WB12 compensation remains prohibited.

Ran:

- Generated branch-active paired ledger rows: `9`.
- Branch-active status counts: `{'branch-active-mask-gap': 8, 'branch-active-mask-closed': 1}`.
- Trace authority snapshot: `post_wb13`.
- Branch-active parser conflict counts: `{0: 9}`.
- First-source counts: `{'melt-call-mask': 8, 'same-hour-multi-source:cmelt,snodpt': 1}`.
- Route counts: `{'branch-active-mask-hold': 8, 'same-hour-multi-source-hold': 1}`.
- Production edit authorized rows: `0`.

## Rationale

HPHYS0306 closed the HPHYS0305 all-row `paired-surface-gap:amelt` ambiguity by
using the correct branch-active comparison domain. Eight windows still have
baseline/openWEPP melt-call mask mismatches, so numeric melt-term correction is
not yet authorized. H39 first-2013 has matching active masks but the first
chronological active-domain divergence occurs at the same hour for `cmelt` and
`snodpt`; that requires a same-hour source-ordering package before any producer
correction.

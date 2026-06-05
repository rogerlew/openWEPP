# Implementation/Test Evidence

Status: complete

Evidence mode: ran

Static:

- Added package-local diagnostic runner
  `hphys0306_branch_active_observe_semantics.py`.
- The runner reads HPHYS0305 fixed-baseline observe logs and openWEPP traces,
  compares baseline melt-call keys against openWEPP
  `snow_hourly_melt_branch_active`, and compares numeric surfaces only on the
  active domain.
- No production physics code was changed.

Ran:

- `python -m py_compile .../hphys0306_branch_active_observe_semantics.py` passed.
- `python .../hphys0306_branch_active_observe_semantics.py` generated:
  - `branch-active-melt-term-ledger.json`
  - `branch-active-melt-term-summary.md`
  - `branch-active-observe-method.md`
- Ledger rows: `9`.
- Branch-active status counts: `{'branch-active-mask-gap': 8, 'branch-active-mask-closed': 1}`.
- Trace authority snapshot: `post_wb13`.
- Branch-active parser conflict counts: `{0: 9}`.
- First-source counts: `{'melt-call-mask': 8, 'same-hour-multi-source:cmelt,snodpt': 1}`.
- Route counts: `{'branch-active-mask-hold': 8, 'same-hour-multi-source-hold': 1}`.
- Production edit authorized rows: `0`.

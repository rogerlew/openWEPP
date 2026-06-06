# HPHYS0308 Snowd Branch State-Ordering Method

Ran:

- Loaded HPHYS0306 `branch-active-melt-term-ledger.json` to recover target
  window years and Julian-day bounds.
- Loaded HPHYS0307 `melt-call-branch-activation-ledger.json` to restrict this
  package to baseline-extra/openWEPP-extra branch activation rows.
- Parsed HPHYS0305 fixed-baseline observe logs for `H305_T_*`, `H305_F_*`,
  `H305_S_OUT`, and `H305_M_POST` records.
- Parsed openWEPP final `post_wb13` trace rows for branch-active, snow
  depth/density, forcing, and melt-term hourly maps.
- Rebuilt branch-extra key sets from paired active masks and emitted key-level
  route classifications.

Static:

- Classification is evidence-only; no production edit is authorized by this
  package.

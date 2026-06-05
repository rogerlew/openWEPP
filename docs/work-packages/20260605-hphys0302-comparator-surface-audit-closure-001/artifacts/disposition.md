# Disposition

Status: executed-hold

Evidence mode: Ran

Static:

- HPHYS0302 is a comparator-surface audit package and made no production
  physics edits.
- Canonical authority is now:
  - `SC-SNOWFREEZE-001#INV-SNOWFREEZE-033`
  - `SC-WATBAL-001#INV-WATBAL-077`

Ran:

- Comparator-surface audit completed over all nine H1/H7/H39 target windows.
- `RM` is a valid like-for-like daily WB13/WAT output comparison surface.
- `Snow-Water` is a valid daily output comparison surface, not melt producer
  authority.
- Raw `hrmlt` and post-raw `wmelt` comparisons are valid aggregate cut-point
  surfaces but not term-level producer authority.
- Term-level melt correction remains blocked because paired baseline
  `amelt`/`bmelt`/`cmelt`/`dmelt` and state/forcing surfaces are absent.
- `production_edit_authorized=false`.

Decision:

- HOLD.
- Do not patch forcing, snow producer, WB17, WB18, WB19, or WB13 from the
  current aggregate/output residuals.
- Continue with paired baseline/openWEPP melt term/state instrumentation for
  `amelt`, `bmelt`, `cmelt`, `dmelt`, `hrrain`, `hrtemp`, `tdpt`, `hrad`,
  `cloudC`, `vwind`, `snodpt`, and `densgt`.

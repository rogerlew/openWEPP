# Disposition

Status: hold

Evidence mode: ran

Static:

- HPHYS0313 remains `HOLD`, not production-closed.
- Settling-route rows have a concrete branch-gated follow-up candidate:
  pinned-baseline 2013 day 11 hour 11 executes the positive-`hrsnow` snowing
  branch at `snowd.for:166-172` with `hrsnow = 0.0007454545120708644 m`,
  while the homologous openWEPP hourly snowfall field is present and `0.0 m`.
  The inferred `driftf + driftg` contribution is approximately
  `5.878973752260208e-9 m`, so no snow-drift migration is authorized from this
  route.
- Year-start-route rows remain inherited before 2014: H1/H7/H39 rows targeted
  at 2016 are already materially divergent at 2014 day 1 hour 1, so the next
  diagnostic must recurse into 2013 terminal carry feeding 2014 day 1 hour 1.
- No downstream compensation is authorized in WB13, WB17, WB18, WB19, WB12,
  melt terms, or branch predicates.

Ran:

- HPHYS0313 diagnostic runner represented all six HPHYS0312 inherited
  terminal groups and all `57` carried HPHYS0309 rows.
- Route counts: `3` `hourly-snowfall-input-lineage-hold`, `3`
  `recursive-year-start-inherited-state-hold`.
- Production edits authorized: `0`.

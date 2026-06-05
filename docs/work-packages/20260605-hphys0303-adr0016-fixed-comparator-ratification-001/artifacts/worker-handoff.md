# Worker Handoff

Status: complete

Evidence mode: static

Static:

- Active comparator is fixed `wepp_260430`:
  `47ac4c32faeea81bb99081f955a14c38b815ef4d`.
- Use regenerated baseline parquets from
  `/tmp/hphys0303_adr0016_1780691036/reports/hillslope/fixed_baseline_partitions`
  for follow-on H1..H39 comparisons.
- Use `artifacts/fixed-vs-original-output-delta.json` to see where the fixed
  comparator differs from archived `dac3c950` outputs.
- Use `artifacts/observe-identity-fixed-comparator.json` as observe identity
  evidence for H1/H7/H39 only.
- Do not use archived original `dac3c950` negative-melt behavior as active
  comparator authority except for archaeology/delta proof.
- HPHYS0302 HOLD remains active: paired baseline/openWEPP term/state surfaces
  for `amelt`, `bmelt`, `cmelt`, `dmelt`, `hrrain`, `hrtemp`, `tdpt`,
  `hrad`, `cloudC`, `vwind`, `snodpt`, and `densgt` are still required before
  production snow/melt or downstream WB edits.

Ran:

- No remote refs were pushed.
- Host smoke helper is not applicable to the HPHYS fixture root used here; rely
  on the full H1..H39 replay and observe identity artifacts.
- Before producer instrumentation, rerun the H1..H39 semantic suite against the
  fixed baseline parquets and reclassify snow/`RM` windows under ADR-0011.

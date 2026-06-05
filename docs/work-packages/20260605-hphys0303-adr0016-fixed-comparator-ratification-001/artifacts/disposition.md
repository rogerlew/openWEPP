# Disposition

Status: accepted-ready

Evidence mode: static + ran

Static:

- ADR-0016 is ratified locally as `Accepted`.
- ADR-0012 is amended to make the fixed `wepp_260430` comparator the active
  regenerated H1..H39 comparator anchor and preserve `dac3c950` as archived
  original bug context.
- `SC-SNOWFREEZE-001`, `SC-WATBAL-001`, and the SC registry are amended to
  make fixed-comparator negative-melt authority explicit.
- HPHYS0302 production-edit `HOLD` remains active. This package does not
  authorize production melt, forcing, WB17, WB18, WB19, or WB13 edits.

Ran:

- HPHYS0303 runner passed all ratification checklist gates and produced
  `artifacts/comparator-ratification-ledger.json`.
- Fixed comparator commit:
  `47ac4c32faeea81bb99081f955a14c38b815ef4d`.
- Fixed comparator tag:
  `wepp_260430_negmeltfix_comparator_47ac4c32faee`.
- Fixed H1..H39 baseline parquets:
  `/tmp/hphys0303_adr0016_1780691036/reports/hillslope/fixed_baseline_partitions`.
- Fixed H1..H39 baseline parquet year/key validation passed.
- H1/H7/H39 observe identity passed; broader H1..H39 observe identity was not
  claimed.
- Host smoke helper failures were dispositioned as non-applicable to this HPHYS
  fixture root, not treated as hidden passes.
- Next required work is paired melt-term/state instrumentation against the
  fixed comparator after the H1..H39 openWEPP-vs-fixed-baseline semantic rerun;
  no downstream compensation is authorized.

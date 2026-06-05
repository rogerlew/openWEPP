# Contract Implementation Evidence

Status: complete

Evidence mode: static + ran

Static:

- ADR-0016 is amended to `Accepted` with exact fixed comparator commit,
  binary hashes, regenerated H1..H39 parquet manifest with year/key
  validation, H1/H7/H39 observe identity artifact, and passing SC lint
  artifact.
- ADR-0012 is amended by ADR-0016 and now distinguishes archived original
  `dac3c950` from active fixed comparator commit
  `47ac4c32faeea81bb99081f955a14c38b815ef4d`.
- `SC-SNOWFREEZE-001` negative-melt refs now bind active comparator authority
  to fixed `wepp_260430` branch/tag while retaining archived original bug
  context.
- `SC-WATBAL-001` negative-melt water-balance invariants now refer to
  fixed-comparator lineage and archived-original bug context.
- `SC-SNOWFREEZE-001` and `SC-WATBAL-001` Variables/Units and Symbol Alias Map
  tables now satisfy the registry-backed unit lint for touched contract
  surfaces.
- `docs/specifications/science-contracts/index.md` records the HPHYS0303
  registry note and fixed comparator identity.

Ran:

- `/workdir/wepppy/.venv/bin/python docs/work-packages/20260605-hphys0303-adr0016-fixed-comparator-ratification-001/artifacts/hphys0303_adr0016_ratification.py`: pass, `ratification_status=accepted-ready`.

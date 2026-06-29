# Authority Provenance

Evidence class: Static.

Stage 3 authority is drawn from existing ratified sources and prior Paradigm 2
packages:

- `SC-SNOWFREEZE-001` v110, `REF-SNOWFREEZE-PARADIGM2-STAGE3`,
  `INV-SNOWFREEZE-080`, and `OBL-SNOWFREEZE-P-055`.
- ADR-0029 commits openWEPP to Paradigm 2 multilayer snow after ADR-0028
  observed-data admission authority.
- Stage 0 added pure `openwepp-meteorology` surface-energy-balance primitives.
- Stage 1 added persistent `DirectSnowLayerState` / `DirectSnowLaneState.layers`
  and proved real layer persistence/conservation.
- Stage 2 verified that the Stage 1 layer stack forms a real basal-denser-than-
  surface density gradient, but did not promote frost insulation coupling.
- 10.3.8 holding-capacity authority provides the per-layer retained-liquid
  bound.
- `REF-SNOWFREEZE-LIBSNOBAL-CC0` records local PySnobal/libsnobal provenance:
  `/home/workdir/pysnobal` commit `bf8b41c71e3e54ae654ae04005ddf72566c47ee6`;
  `setup.py` declares `license="CC0 1.0"` and `deny.toml` permits `CC0-1.0`.

No constants or thresholds in this package may be tuned to openWEPP fixtures.

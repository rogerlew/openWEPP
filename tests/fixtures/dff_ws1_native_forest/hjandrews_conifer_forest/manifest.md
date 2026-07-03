# DFF-WS1 native forest CLI fixture — HJ Andrews conifer

End-to-end verification fixture for the openWEPP-native forest `lanuse` mode
(`ow-lanuse-1`), DFF-WS1 Increment-2. Exercised by
`tests/integration/dff_ws1_native_forest_cli.rs`.

## Provenance
Derived from `tests/fixtures/cancov_forest/hjandrews_conifer_or/` (unburned
`forest` / `loam`). The `.sol`, `.slp`, `.cli`, `pmetpara.txt`, `snow.txt`, and
`gwcoeff.txt` are copied verbatim from that fixture. `p2.man` is **converted** from
the cropland-encoded (`landuse=1`) masquerade to a first-class native forest
`.man`:
- datver `ow-lanuse-1`, `landuse=3` forest sentinel, `PlantScenarioData::Forest`
  block with `forest_class = forest`;
- the Tier-A growth operands carry the same values as the source cropland block;
  the lookup-owned quartet (`xmxlai=14, rdmax=2, decfct=1, dropfc=1`) equals the
  authoritative `(forest, loam)` land-soil lookup row;
- plant name kept `Tah_4899` so it resolves the explicit `Tah_4899` PMET record
  (no compatibility first-row fallback).

## What the run proves
- native forest `.man` parses and projects the growth-symbol surface;
- `.man` forest class (`forest`) reconciles with the `.sol` `DisturbedPolicy`
  luse (`forest`) — fail-closed guard passes;
- the PMET lookup resolves the explicit forest record (no fallback);
- the hillslope runs on the direct production executor and emits HBP + loss +
  wat parquet outputs.

`p2.run.toml` is the openWEPP-native runfile (`openwepp-hillslope-runfile-v1`).
Run outputs (`output/`) are generated to a temp copy by the test and are not
committed.

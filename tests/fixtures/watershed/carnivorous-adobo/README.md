# Carnivorous-Adobo Watershed Fixture

Status: committed development fixture for WSHED-FIXTURE01.

## Provenance

- Source substrate: `/wc1/runs/ca/carnivorous-adobo/wepp`.
- Source substrate inspected: 2026-07-01 local session.
- Source substrate directory mtime: `2026-05-09 08:01:43.199344558 -0700`.
- Source run directory mtime: `2026-05-09 08:02:20.216559065 -0700`.
- Soil-file comments were sanitized to remove embedded absolute source-soil
  paths and helper implementation names from the committed fixture. WEPP data
  records were not intentionally changed.
- Daily climate `rad` values were normalized on 2026-07-01 for WSHED-W3 after
  the direct-runtime hillslope child path failed `SC-CLIMATE-001#INV-CLIMATE-013`
  on over-bound `radly` rows. The correction clamps only fixture data: `39`
  unique daily records, copied across `p1.cli` through `p32.cli` and `pw0.cli`,
  were reduced to `floor(baseline sunmap horizontal r3)` for latitude `48.25`.
  Production runtime guards still fail closed and do not mutate `radly`.
  Detailed clamp evidence is recorded in
  `docs/work-packages/20260701-wshedw3-bounded-worker-pool-001/artifacts/scaling/carnivorous-adobo-radly-clamp-manifest.json`.
- Committed text fixture files were normalized to LF line endings, with trailing
  spaces/tabs and trailing blank EOF lines stripped for repository diff hygiene.
- Adopting package:
  `docs/work-packages/20260701-wshedfixture01-committed-watershed-fixture-adoption-001/`.
- Scope: near-term watershed development fixture for future W2/W3 runtime
  packages. It is not a performance result or output-identity claim by itself.

## Inventory

The fixture commits the input/runfile set under `runs/` and records checksums in
`input-manifest.sha256`.

- Total committed input files: `208`.
- Hillslope count: `32`.
- Hillslope input sets: `p1` through `p32`, each with `.run`, `.man`, `.slp`,
  `.cli`, and `.sol`.
- Hillslope launch files: `p1.source.run` through `p32.source.run`, schema
  `openwepp-hillslope-runfile-v1`; these bind committed hillslope inputs for
  generated-pass watershed supervisor runs.
- Watershed input set: `pw0.run`, `pw0.str`, `pw0.chn`, `pw0.imp`, `pw0.slp`,
  `pw0.cli`, `pw0.sol`, and `pw0.man`.
- Watershed launch file: `case.run`, schema `openwepp-watershed-runfile-v1`.
- Shared sidecars: `chan.inp`, `chntyp.txt`, `gwcoeff.txt`, `pmetpara.txt`,
  `snow.txt`, `tc.txt`, and `wepp_ui.txt`.
- Excluded generated/transient files: source `.err` files and `plots/*.tif`.

## Topology Summary

- Watershed structure file: `runs/pw0.str`.
- Structure datver: `99.1`.
- Structure rows: `15`.
- Channel elements: `15`, derived element ids `33..47`.
- Impoundments: `0`.
- Maximum hillslope reference: `32`.
- Channel file `runs/pw0.chn` declares `15` channels and `ipeak=4`.
- `runs/chan.inp` is present and lists all `15` channel element ids.

## Gate Intent

This fixture is the committed `strict-committed-fixture` substrate required by
ADR-0032 and the watershed runtime architecture fixture ladder. Persistent
openWEPP gates must read this committed path, not `/wc1`, `/tmp`, scratch
outputs, or wepppy files.

WSHED-FIXTURE01 adopts the source input/runfile substrate and parser contract
fixture. WSHED-W6 adds schema-versioned TOML watershed and hillslope launch
files so `openwepp-cli-watershed` can execute the committed fixture directly
without `/wc1`, `/tmp`, scratch, or wepppy inputs.

# Fixture Provenance

Status: `EXECUTED`

Static:

- Fixture path: `tests/fixtures/watershed/carnivorous-adobo/`.
- Input/runfile path: `tests/fixtures/watershed/carnivorous-adobo/runs/`.
- Checksum manifest: `tests/fixtures/watershed/carnivorous-adobo/input-manifest.sha256`.
- Fixture README: `tests/fixtures/watershed/carnivorous-adobo/README.md`.

Ran:

- `stat -c '%n %y %s bytes' /wc1/runs/ca/carnivorous-adobo/wepp /wc1/runs/ca/carnivorous-adobo/wepp/runs /wc1/runs/ca/carnivorous-adobo/wepp/runs/pw0.str /wc1/runs/ca/carnivorous-adobo/wepp/runs/pw0.run`
- `find tests/fixtures/watershed/carnivorous-adobo/runs -maxdepth 1 -type f | awk ...`
- `rg -n "/wc1|wepppy" tests/fixtures/watershed/carnivorous-adobo/runs || true`
- `cargo nextest run --test infile_watershed_structure_parser_contract carnivorous_adobo_committed_fixture_is_repo_local_32_hillslope_gate`

## Source Substrate

- Source substrate: `/wc1/runs/ca/carnivorous-adobo/wepp`.
- Source substrate inspected: 2026-07-01 local session.
- Source substrate directory mtime: `2026-05-09 08:01:43.199344558 -0700`.
- Source run directory mtime: `2026-05-09 08:02:20.216559065 -0700`.
- Source `pw0.str` mtime: `2026-05-09 08:02:07.897487453 -0700`.
- Source `pw0.run` mtime: `2026-05-09 08:02:08.048488329 -0700`.

The source substrate is orientation and capture substrate only. Persistent gates
must read `tests/fixtures/watershed/carnivorous-adobo/`.

## Adopted Inventory

- Total committed input files: `175`.
- Hillslope runfiles: `32`.
- Hillslope `.cli` files: `32`.
- Hillslope `.slp` files: `32`.
- Hillslope `.sol` files: `32`.
- Hillslope `.man` files: `32`.
- Source `.err` files committed: `0`.
- Excluded generated/transient source files: `.err` files and `plots/*.tif`.
- Soil-file comment provenance was sanitized to remove embedded absolute
  source-soil paths and helper implementation names from the committed fixture.
  WEPP data records were not intentionally changed.
- Committed text fixture files were normalized to LF line endings, with trailing
  spaces/tabs and trailing blank EOF lines stripped for repository diff hygiene.
  `input-manifest.sha256` was regenerated after normalization.

## Boundary

Static:

- This package adopts the committed source input/runfile substrate and parser
  contract fixture.
- It does not claim current `openwepp-cli-watershed` end-to-end execution from
  this fixture.
- The current `docs/contracts/openwepp-watershed-runfile-contract.md` watershed
  CLI surface requires TOML schema `openwepp-watershed-runfile-v1` with
  `hillslopes_block[].pass_file` HBP bindings.
- The source substrate did not provide HBP pass shards. Source `output/`
  contained `H*.plot.dat` files, not pass-like HBP files.

## Required Inputs

Hillslope input sets:

- `p1` through `p32`, each with `.run`, `.man`, `.slp`, `.cli`, and `.sol`.

Watershed input set:

- `pw0.run`
- `pw0.str`
- `pw0.chn`
- `pw0.imp`
- `pw0.slp`
- `pw0.cli`
- `pw0.sol`
- `pw0.man`

Shared sidecars:

- `chan.inp`
- `chntyp.txt`
- `gwcoeff.txt`
- `pmetpara.txt`
- `snow.txt`
- `tc.txt`
- `wepp_ui.txt`

## Topology Summary

- Structure datver: `99.1`.
- Structure rows: `15`.
- Hillslope count: `32`.
- Channel elements: `15`, derived element ids `33..47`.
- Impoundments: `0`.
- Maximum hillslope reference: `32`.
- Channel file `pw0.chn` declares `15` channels and `ipeak=4`.
- `chan.inp` is present and lists all `15` channel element ids.

## wepppy Orientation

Static:

- `/home/workdir/wepppy/docs/work-packages/20260701_wshed_fixture01/package.md`
  was not present.
- `/home/workdir/wepppy/tests/topo/test_wshed_fixture01.py` was not present.

No openWEPP closure claim relies on wepppy files.

# Onshore-Xenophobia Watershed Fixture

Status: committed full large-watershed fixture for WSHED-W6.

## Provenance

- Source substrate: `/wc1/runs/on/onshore-xenophobia/wepp`.
- Source substrate inspected: 2026-07-02 local session.
- Source substrate directory mtime: `2026-04-28 13:28:19.178908480 -0700`.
- Source run directory mtime: `2026-04-28 13:26:59.951307444 -0700`.
- Source `pw0.str` mtime: `2026-04-28 12:24:22.673099700 -0700`.
- Source `pw0.run` mtime: `2026-04-28 12:25:15.377446220 -0700`.
- Soil-file comments were sanitized to remove embedded absolute source-soil
  paths and project-specific helper names from the committed fixture. Generic
  source-utility provenance comments remain. WEPP data records were not
  intentionally changed.
- Adopting package:
  `docs/work-packages/20260702-wshedw6-publication-large-watershed-scaling-001/`.
- Scope: full large-watershed strict committed fixture for W6 publication and
  scaling evidence.

## Inventory

The fixture commits the input/runfile set under `runs/` and records checksums in
`input-manifest.sha256`.

- Total committed input and launch entries: `7847`.
- Regular files: `6541`.
- Symlinks: `1306`.
- Hillslope count: `1305`.
- Hillslope input sets: `p1` through `p1305`, each with `.run`, `.man`, `.slp`,
  `.cli`, `.sol`, and `.source.run`.
- Watershed launch file: `runs/case.run`, schema
  `openwepp-watershed-runfile-v1`.
- Hillslope launch files: `runs/p1.source.run` through
  `runs/p1305.source.run`, schema `openwepp-hillslope-runfile-v1`; these bind
  the source WEPP inputs and request only the required pass/loss outputs that
  the watershed supervisor consumes.
- Watershed input set: `pw0.run`, `pw0.str`, `pw0.chn`, `pw0.imp`, `pw0.slp`,
  `pw0.cli`, `pw0.sol`, and `pw0.man`.
- Shared sidecars: `chan.inp`, `chntyp.txt`, `gwcoeff.txt`, `pmetpara.txt`,
  `snow.txt`, `tc.txt`, and `wepp_ui.txt`.
- Excluded generated/transient files: source `.err` files and generated output
  directories.

## Shared Climate Representation

The source run stores `p1.cli` through `p1305.cli` and `pw0.cli` as hard links
to one byte-identical climate file. Git does not preserve hard-link identity, so
this fixture commits one canonical file at `runs/shared/onshore-xenophobia.cli`
and uses relative symlinks for the legacy filenames.

This is a file-system representation choice only. The full watershed topology,
hillslope set, run horizon, management, slope, soil, watershed channel,
structure, impoundment, and sidecar inputs are preserved. No watershed subsetting
or fixture physics transformation was applied.

## Topology Summary

- Watershed structure file: `runs/pw0.str`.
- Structure datver: `99.1`.
- Structure rows: `544`.
- Channel elements: `544`, derived element ids `1306..1849`.
- Impoundments: `0`.
- Maximum hillslope reference: `1305`.
- Channel file `runs/pw0.chn` declares `544` channels and `ipeak=4`.
- `runs/chan.inp` is present.
- Source watershed manifest reports `1305` hillslope parquet rows and `544`
  channel parquet rows.

## Run Intent

This fixture is the W6 full large-watershed acceptance substrate. Persistent W6
publication and scaling gates must read this committed path, not `/wc1`, `/tmp`,
scratch outputs, or wepppy files.

Run recipe:

```sh
openwepp-cli-watershed \
  --run-dir tests/fixtures/watershed/onshore-xenophobia/runs \
  --run-file case.run \
  --output-dir /tmp/openwepp-onshore-xenophobia-out \
  --policy compat \
  --jobs 1
```

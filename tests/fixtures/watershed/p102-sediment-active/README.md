# P102 Sediment-Active Watershed Fixture

Status: committed WSHED-W7R sediment-active watershed fixture.

## Provenance

- Hillslope source substrate:
  `tests/fixtures/erosion_multi_ofe_p102/`, a real 2-OFE WSHED-W7DC01
  (`insensible-aliquot` H102) disturbed-forest hillslope.
- Watershed wrapper: generated one-channel, one-hillslope topology for WSHED-W7R
  publication closure. The wrapper is a complete watershed fixture for this
  acceptance substrate; it is not a slice of a larger watershed.
- The wrapper does not edit HBP/pass sediment values. `openwepp-cli-watershed`
  launches `openwepp-cli-hill`, which generates `H1.hbp` and `H1.pass.parquet`
  from the committed p102 source inputs.
- WSHED-W11D changed the wrapper-only channel routing selector from static
  Muskingum-Cunge (`ipeak=4`) to kinematic wave (`ipeak=3`) on 2026-07-11.
  The historical 600-second MC recurrence is numerically inadmissible under
  `SC-ROUTE-001#INV-ROUTE-022`; KW preserves this fixture's intended hourly
  HBP sediment/publication and `--jobs` identity coverage without weakening
  the production MC guard. Hillslope source inputs and HBP values are unchanged.
- Source p102 climate remains the 10-year truncated CLIGEN series documented by
  `tests/fixtures/erosion_multi_ofe_p102/README.md`.

## Inventory

- Hillslopes: `1`
- Channels: `1`
- Impoundments: `0`
- Source input links: `H1.*` and `pw0.*` resolve to the committed p102
  hillslope files.
- Watershed topology and sidecars:
  `pw0.str`, `pw0.chn`, `pw0.imp`, and `chan.inp`.
- Launch files:
  `runs/case.run` for `openwepp-cli-watershed` and
  `runs/H1.source.run` for generated hillslope execution.

## Run Recipe

```sh
openwepp-cli-watershed \
  --run-dir tests/fixtures/watershed/p102-sediment-active/runs \
  --run-file case.run \
  --output-dir /tmp/openwepp-p102-sediment-active-out \
  --policy compat \
  --jobs 1 \
  --hillslope-binary target/release/openwepp-cli-hill
```

WSHED-W7R acceptance runs compare `--jobs 1` with `--jobs 4` decoded parquet
schemas and rows.

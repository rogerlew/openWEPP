# W-C Routing and Output Finding

Status: W-C executed-hold

Evidence mode: Ran + Static

## Hard-Stop Classification

W-C started from the W-B hard stop:

- `CLIWAT-E-020 watershed dispatch reported failure`
- wrapped kernel status `WKERNEL-WS10-CHANNEL-E-003`
- zero watershed output files

The first failing surface was not a bad channel topology or invalid positive
sediment route. It was a valid zero-sediment hillslope contributor rejected by
the WS10 channel payload guard: complete HBP sediment fields were present, but
`total_detachment - total_deposition <= 0`, concentration support was zero,
and particle-flow fractions were also zero. That is a valid no-mass sediment
payload, not missing particle support.

W-C also exposed a second valid-state guard on the same path: `nchnum=0`
represents channel detail output disabled in the `chan.inp`/fallback runtime
surface. It is not a positive routing operand and must not block routing.

## Contract Amendment

`SC-ROUTE-001` was amended to version `45`:

- complete zero-sediment contributor payloads are valid when all sediment mass
  and concentration support is zero;
- positive sediment mass or concentration still requires positive
  particle-fraction support;
- `nchnum=0` is valid as an output-disabled state and is not a routing domain
  violation.

## Implementation Result

Routing fixes:

- `read_hillslope_sediment_payload` distinguishes zero-mass sediment payloads
  from positive sediment payloads requiring particle fractions.
- incoming sediment capacity assembly skips fraction normalization for
  zero-mass hillslope payloads.
- channel runtime validation accepts `nchnum >= 0`.

Publication fixes:

- watershed output writing now accepts multiple daily row seeds.
- the watershed CLI builds daily `totalwatsed*` row seeds from sibling
  hillslope WAT parquet files when present.
- `totalwatsed3.parquet` water-balance fields are sourced from WAT operands
  instead of writer defaults for real routed runs.

## Real-Run Evidence

Configured run:

```bash
cargo run -q -p openwepp-runner --bin openwepp-cli-watershed -- \
  --run-dir /tmp/openwepp_wshed01_wa/watershed/run \
  --run-file case.run \
  --output-dir /tmp/openwepp_wshed01_wc_final_configured/output \
  --policy compat
```

Legacy-discovery run:

```bash
cargo run -q -p openwepp-runner --bin openwepp-cli-watershed -- \
  --run-dir /tmp/openwepp_wshed01_wa/watershed/run \
  --run-file case.run \
  --output-dir /tmp/openwepp_wshed01_wc_final_legacy/output \
  --policy compat \
  --legacy-sidecar-discovery
```

Observed:

- Both runs exited `0`.
- Both runs emitted all `14` watershed parquet outputs.
- `totalwatsed3.parquet` row count: `2192` for both runs.
- `max(abs(runvol - Q * Area / 1000.0))`: `0.0 m^3`.
- First-row configured values:
  - `P`: `32.717215206680784`
  - `RM`: `13.203340055286729`
  - `SoilWaterTotal`: `335.10212226223916`

The exact zero residual here is only the W-C volume-depth mapping check for
published `runvol` and `Q`/`Area`; it is not claimed as W-D water-balance
closure. W-D must still run the totalwatsed3 audit with independent operands.

## Disposition

W-C met the scoped routing/output publication gates and remains held only
because the package-level acceptance surface is W-D totalwatsed3 closure.

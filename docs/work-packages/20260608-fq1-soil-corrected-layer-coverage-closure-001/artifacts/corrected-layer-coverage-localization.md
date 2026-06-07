# Corrected-Layer Coverage Localization

Evidence mode: `Static:` source inspection plus `Ran:` legacy/comparator status inspection.

## Symptom

Static: `/tmp/frostval01/full/run_status.tsv` records `37/43` algebraic-radium
single-OFE prefixes failing before hydrology with `HS-RUNTIME-E-062`, for example:

- `p1`: layer `6`, parser interval `1100..2000 mm`, covered `700 mm`.
- `p2`/`p4`: layer `4`, parser interval `760..2000 mm`, covered `1040 mm`.
- many prefixes: layer `4`, parser interval `1270..2000 mm`, covered `530 mm`.

The six controls were `p8,p13,p22,p23,p26,p28`.

## Mechanism

Static: `map_corrected_layer_runtime_symbols_to_parser_layers` built fixed
normalized corrected intervals from `0..1800 mm` and then required every parser
layer interval to be fully covered. Valid parser profiles with a final layer
ending at `2000 mm` therefore exposed a tail gap from the normalized corrected
grid bottom to parser bottom. The parser was reading the profile depths; the
missing behavior was a deepest-corrected-layer extension for parser-layer
diagnostic/constitutive symbols.

Static: hydrology seed-grid authority remains separate. `legacy_cumulative_depths_mm`
and normalized WB11/WB18/WB19 seed aliases remain governed by `INV-SOIL-015`; FQ1
does not change their normalized grid.

## Ownership

Ran: legacy `wepp_260606_hill` evidence in
`/wc1/runs/al/algebraic-radium/wepp/runs/p1.err` ends with
`WEPP COMPLETED HILLSLOPE SIMULATION SUCCESSFULLY`. `p8.err` also completes. This
flags representative blocked and control soils as valid legacy inputs.

Conclusion: root cause is in the OpenWEPP soil corrected-layer mapping envelope,
not an invalid soil input and not a downstream hydrology-kernel mechanism.

# Implementation

The public `parse_soil` coordinator now delegates its unchanged ordered stages
to private helpers: preamble parsing, OFE collection, restrictive-footer
closure, and trailing-record rejection. `parse_ofe_block` similarly delegates
header/policy, layer sequence, and optional per-OFE restrictive row handling.

`parse_layer_row` remains the datver dispatcher and delegates only the existing
row bodies to four cohesive private implementations: base (`97.5`/`2006.2`),
`7777`, `7778`, and Rosetta (`9002`/`9003`/`9005`). Token positions, conversion
sequence, validation order, units, exact error construction, and `SoilLayer`
field mappings remain in their original branch order.

No grammar, datver acceptance, thresholds, public API, typed error taxonomy,
serialization, or physics calculation changed. Added tests are target-local,
within the module's embedded test block.

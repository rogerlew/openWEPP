# Publication Operand Lineage

Status: `passed`

Evidence mode: `Static:`

W7 did not change sediment-sensitive publication formulas, schemas, units, or
normalization bases. W6 lineage remains authoritative for the public typed
publication writer:

- `sediment_yield_kg`: routed channel sediment yield from typed routed state.
- `total_detachment_kg`: sum of pass-backed latest-event detachment over
  contributing hillslopes.
- `total_deposition_kg`: sum of pass-backed latest-event deposition over
  contributing hillslopes.
- `sediment_class_deposition_kg` and
  `sediment_volume_concentration_m3_m3`: unavailable in current W6/W7 public
  watershed frames and emitted null.

Rejected W7 closure aliases:

- zero-fill as acceptance sediment;
- manually edited pass/HBP sediment values;
- legacy `loss.dat` soil-loss summaries as a substitute for
  production-generated openWEPP HBP sediment;
- producer self-consistency without an independently nonzero produced signal.

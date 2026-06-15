# WAT Publication Boundary

Static: protected behavior surfaces were preserved:

- WAT field names and aliases remain unchanged, including `ofe_id|OFE` and
  `Total-Soil Water|Total-Soil`.
- Required-column lookups remain fail-closed through the existing typed error
  constructors.
- Optional absent/all-null handling remains routed through existing
  `optional_*` helpers.
- `Area <= 0.0` still returns `WatershedWatPublicationError::InvalidValue` with
  the absolute row index.
- Aggregation and daily row publication formulas were not edited.
- Public output row fields and units were not edited.

Ran: focused reader tests cover alias/default/error behavior:

```text
cargo test -p openwepp-runner watershed_wat::tests -- --nocapture
```

Disposition: publication boundary preserved by private helper extraction only.

# Contract-Test Implementation Evidence

Status: completed

Evidence mode: static+ran

## Contract-Derived Test

Static: Added
`hphys0261_trace_row_captures_ep_initialization_magnitude_lineage` in
`crates/openwepp-runner/src/hillslope/mod.rs`.

Static: The test requires serialized trace rows to expose:

- `pl_pltol`
- `pl_swu_effective_pltol`
- `wb18_ul_layers_m`
- `wb17_swu_stress_threshold_layers_m`
- `wb17_swu_storage_to_threshold_layers`
- existing aggregate/layer `Etp`, `Ep`, `UPi`, and `Ui` lineage

Ran: `cargo test -p openwepp-runner hphys0261_trace_row_captures_ep_initialization_magnitude_lineage -- --nocapture`
failed before production trace implementation with the expected missing-field
assertion:

```text
assertion `left == right` failed
  left: Null
 right: Number(0.33)
```

Ran: `cargo test -p openwepp-runner hphys -- --nocapture` passed after
implementation: `33 passed; 0 failed`.

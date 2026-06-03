# Implementation and Test Evidence

Status: completed

Evidence mode: static+ran

## Implementation

Static: Bumped the additive opt-in HPHYS0245 trace schema to
`openwepp-hphys0245-wb11-wb18-wb19-wb17-ep-init-trace-v5`.

Static: Added trace fields in `crates/openwepp-runner/src/hillslope/mod.rs`:

- `pl_pltol`
- `pl_swu_effective_pltol`
- `wb18_ul_layers_m`
- `wb17_swu_stress_threshold_layers_m`
- `wb17_swu_storage_to_threshold_layers`

Static: Added helper derivations for `pltol*ul(i)` thresholds and
`theta(i)/(pltol*ul(i))` storage ratios. The change is trace-only and does not
modify hydrology equations or default-off runtime behavior.

## Focused Validation

Ran: `cargo test -p openwepp-runner hphys -- --nocapture`

Result:

```text
33 passed; 0 failed
```

Ran: `docs/work-packages/20260603-hphys0261-wb17-ep-magnitude-initialization-lineage-closure-001/artifacts/hphys0261_diagnostics.py --run-root /tmp/hphys0261_20260603T042648Z`

Result: targeted H1/H7/H39 trace diagnostics and the full H1..H39 semantic
suite completed.

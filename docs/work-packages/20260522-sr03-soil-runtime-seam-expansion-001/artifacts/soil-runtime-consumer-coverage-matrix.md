# Soil Runtime Consumer Coverage Matrix

Status: `complete`
Evidence mode: `Static + Ran`

Static:
- Mapped SR03 projected runtime symbols to soil/hydrology consumer contract obligations for seam-scope closure.

Ran:
- Coverage matrix validated against passing projection tests in `parser_runtime_seam_integration.rs` and `runtime_inputs` unit tests.

| consumer_contract | required_symbols | projected_symbols | coverage_status | notes |
|---|---|---|---|---|
| `SC-SOIL-001` | Layer hydraulic-state substrate for conductivity/erodibility pathways (`Ksi/Ksai` inputs, layer depth structure, residual/field capacity) | `nsl`, `solthk`, `solthk_{j:04}`, `dg`, `dg_{j:04}`, `thetdr`, `thetdr_{j:04}`, `thetfc`, `thetfc_{j:04}`, `ssc`, `ssc_{j:04}`, plus OFE-indexed forms | `covered (seam scope)` | SR03 closes parser-to-runtime substrate export; dynamic Chapter-7 state evolution remains downstream. |
| `SC-WATBAL-001` | Per-layer field-capacity/residual/thickness/conductivity prerequisites for ET/percolation eligibility (`Theta_r`, `FCi`, `Ksi/Ksai`) | `thetdr*`, `thetfc*`, `dg*`, `solthk*`, `ssc*`, `nsl` | `covered (seam scope)` | Provides required static per-layer surfaces for water-balance consumers; runtime volumetric water state remains kernel-owned. |
| `SC-SUBHYD-001` | Drainable-layer conductivity/field-capacity/thickness prerequisites for lateral-flow and drawdown branches | `thetfc*`, `ssc*`, `dg*`, `solthk*`, `nsl` | `covered (seam scope)` | Supplies boundary substrate inputs; subsurface flux/state evolution is out of SR03 seam ownership. |
| `SC-INFILE-SOIL-001` | Parser canonical fields and topology/layer closure (`ntemp`, `nsl`, `solthk`, `ksat`, `theta_r`, `fc`) | `ntemp`, `ofe{i}_nsl`, `ofe{i}_solthk`, `ofe{i}_{solthk|dg|thetdr|thetfc|ssc}_{j:04}` and first-OFE aliases | `covered` | SR03 projection is sourced directly from parsed canonical layer fields with strict typed guard enforcement. |

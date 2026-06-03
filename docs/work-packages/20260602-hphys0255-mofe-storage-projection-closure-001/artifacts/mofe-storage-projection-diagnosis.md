# MOFE Storage Projection Diagnosis

Status: complete
Evidence mode: static + ran

Static: current runtime projection behavior

- `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/02_soil_slope.rs`
  builds unqualified `wb11_nsl`, `wb19_*`, `wb18_perc_*`, and
  `wb13_profile_*` storage surfaces from the primary OFE only.
- The same projection preserves per-contributor parser/corrected-layer
  diagnostics as scoped symbols such as `ofe2_nsl`, `ofe2_dg_0002`, and
  `ofe3_solthk`.
- No `ofeN_wb19_*` dynamic hydrology state vector exists today.

Static: current publication behavior

- `crates/openwepp-runner/src/hillslope/mod.rs` derives MOFE publication
  `Area` from summed OFE geometry.
- WB13 storage fields consume simulation-owned unqualified runtime storage:
  `wb11_soil_water -> Total-Soil`, `Total-Soil + frozwt -> SoilWaterTotal`,
  and unqualified `wb13_profile_*` profile surfaces.
- Therefore aggregate `Area` and storage lineage are separate dimensions.

Ran: asymmetric MOFE projection test

- Command: `cargo test --test wb11_storage_projection_kernel_contract hphys0255 -- --nocapture`
- Result: pass after contract/test authoring.
- Finding: asymmetric OFE2/OFE3 parser profiles remain scoped; WB11 hydrology
  aliases are not overwritten by later OFE rows.

Conclusion

- The code did not have a storage math defect that could be fixed locally.
- The gap was missing explicit provenance telling downstream consumers not to
  infer dynamic area-weighted storage from aggregate `Area`.
- Dynamic per-OFE aggregate storage remains a future migration scope and should
  stay `HOLD` until per-OFE hydrology state vectors are contract-authored and
  implemented.

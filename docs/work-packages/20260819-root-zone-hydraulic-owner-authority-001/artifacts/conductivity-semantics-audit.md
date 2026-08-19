# Conductivity Semantics Audit

Historical HOLD intake evidence. The audited `Ksat` lineage remains binding;
the missing current-conductivity disposition is superseded by the selected
Brooks--Corey operator in `SC-ROOTZONEHYDRAULICS-001@1.0.0-rc1`.

Evidence class: `Static`

Verdict: `DirectSubsurfaceLayerState.conductivity_m_s` is not an admitted
current unsaturated soil-root conductivity.

Evidence:

- `DirectSubsurfaceLayerInputs.conductivity_m_s` is copied unchanged into
  `DirectSubsurfaceLayerState` and copied again when parameters refresh.
- Percolation later names a depth-weighted result
  `saturated_conductivity_m_s` and separately multiplies effective/base
  conductivity by the moisture-dependent `fx`.
- `SC-INFILE-SOIL-001` traces the source to input `ksat` and normalized vertical
  conductivity projection.
- `SC-SUBHYD-001` separately defines disturbed-soil/frost effective
  conductivity for WB14. That surface is not the V10 root-zone conductivity.

Consequently the proposed direct `1000 * conductivity_m_s` conversion would
silently use saturated/base conductivity as current unsaturated conductivity.
Authority must first admit a Clapp-Hornberger/Brooks-Corey unsaturated
conductivity relation with exact parameters and deterministic operation order,
or expose a different live state field with proved semantics.

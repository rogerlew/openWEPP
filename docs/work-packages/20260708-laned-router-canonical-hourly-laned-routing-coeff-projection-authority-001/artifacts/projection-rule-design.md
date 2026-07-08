# Projection Rule Design

Status: queued placeholder.

Execution must either define a deterministic projection rule or hold.

Minimum design requirements:

- explicit `routing_coefficients` override projected coefficients;
- disturbed/native forest table authority remains separate;
- legacy cropland projection must produce all five static operands or fail
  closed for Lane D production:
  - `k_o`;
  - `form_C_d`;
  - `D_r_m`;
  - `lambda`;
  - `vegetation_C_d`;
- provenance must surface in run manifests and/or output metadata;
- mixed explicit/projected authority is allowed only if contract-ratified and
  every scheduled OFE resolves to a complete coefficient set;
- unsupported landuse classes fail closed when Lane D production is required.

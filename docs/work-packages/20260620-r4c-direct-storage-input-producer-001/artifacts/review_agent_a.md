# R4C Review Agent A

Status: complete.
Evidence mode: Static local review.

Review focus:

- contract authority and operand lineage;
- storage-input source selection and guard completeness;
- anti-alias tests for precipitation and initial storage;
- no-publication/no-default/no-scheduler boundary.

## Findings

No blocking findings.

Review notes:

- R4C sources `storage_initial_m` from direct `water.soil_water_m` and
  `precip_input_m` from R3A direct precipitation, matching the package lineage.
- Anti-alias tests reject transfer input, total accounted input, R4A liquid
  input, publication storage, and R3B diagnostic-ledger storage as the selected
  storage-input authorities.
- R4C is shadow-only and does not edit publication, output schemas, scheduler,
  compatibility runtime, or default activation.
- R4B now fails closed when R4C has not run, then separately fails closed when
  R4A runoff has not run.

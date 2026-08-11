# Diagnostic Contract Test Evidence

Status: `PASS`

Evidence mode: `Ran`

Final focused contract/wiring command:

    cargo nextest run --test peak_hourly_authority_contract --test subhourly_generation_contract --test subhourly_generation_properties --test subhourly_water_output_roundtrip

Result: 10/10 passed, nextest run `45ec1b74-61af-461a-bece-9b524fc0cc2d`.
The four peak-authority cases remained green. Tests bind the post-peak seam,
source-completeness errors, separate output target, schema ID, unit catalog,
null erosion fields, and absence of HBP/sediment coupling.

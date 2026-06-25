# Kickoff Prompt

Execute
`docs/work-packages/20260625-snowfrost-fidelity-c-sfcc-frozen-k-diagnostics-001/package.md`.

Add diagnostic-only SFCC/unfrozen-water and frozen hydraulic-conductivity
comparison surfaces. Keep all code outside production runtime physics. Prove
monotonic/bounded diagnostic behavior, salinity/impedance sensitivity, model
provenance labels, and no production `crates/` coupling.

Do not tune observed residuals, do not promote a selected model, do not add a
runtime switch, do not enable `Qwet`, and do not change direct/compatibility
execution.

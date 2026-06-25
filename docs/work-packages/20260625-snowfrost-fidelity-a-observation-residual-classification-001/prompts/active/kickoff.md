# Kickoff Prompt

Execute
`docs/work-packages/20260625-snowfrost-fidelity-a-observation-residual-classification-001/package.md`.

Run all five `tests/fixtures/snowfreeze_observed/` pilot sites through the
observed frost-depth harness, classify residuals under
`SC-SNOWFREEZE-001#INV-SNOWFREEZE-047`, and record whether any site is eligible
for frost-model defect attribution under the `TOL-SNOWFREEZE-009` snow-control
gate.

Do not change snow/frost physics, do not enable `Qwet`, do not tune thresholds,
and do not default-activate direct runtime. If modeled snow depth is absent,
classify the site as blocked or inconclusive rather than defective.

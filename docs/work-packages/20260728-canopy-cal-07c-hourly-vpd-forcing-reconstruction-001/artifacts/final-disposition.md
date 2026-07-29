# CAL-07C Final Disposition

Evidence class: `Static + Ran`

Status: `COMPLETE / BOUNDED EXECUTION / ORDER 7 HOLD RETAINED`

CAL-07C admitted the Alerce hourly-product daily-mean VPD operand for a
package-local bounded execution and regenerated the full frozen CAL-07
Southern Hemisphere evaluation. The immediate Alerce negative daily VPD input
blocker is lifted for CAL-07C only.

Order 7 remains held. The result is not a production forcing-operator change
and does not replace `SC-PLANT-001` OBL-PLANT-P-013. The deciduous transition
chronology cell is contradicted, signed-latitude/seasonal-direction combined
status is contradicted, and amplitude/floor/decomposition cells remain
not-evaluated.

Primary evidence:

- `artifacts/source-admission.md`;
- `artifacts/executor-path-proof.md`;
- `artifacts/science-summary.md`;
- `artifacts/verdict-matrix.csv`;
- `artifacts/figures/`; and
- `artifacts/gate-evidence.md`.

Terminal review:

- `artifacts/review-agent-a.md`: PASS for bounded evidence/claim calibration;
- `artifacts/verification-agent-a.md`: PASS with retained hold boundaries;
- `artifacts/review-agent-b.md`: PASS for bounded-execution closure; and
- `artifacts/verification-agent-b.md`: PASS / verified.

Final local gates:

- CAL-07C independent validation: PASS;
- package-local Rust `cargo fmt --check`: PASS;
- package-local Rust `cargo check`: PASS;
- Python syntax: PASS;
- Markdown lint for package, roadmap, and catalog: PASS;
- SVG XML/render checks: PASS; and
- `git diff --check`: PASS.

No accepted prospective or terminal finding remains open.

Terminal worktree status before handoff is limited to the intended CAL-07C
package plus the roadmap/catalog updates:

```text
## main...origin/main
 M docs/planning/canopy-phenology-assurance-roadmap.md
 M docs/work-packages/README.md
?? docs/work-packages/20260728-canopy-cal-07c-hourly-vpd-forcing-reconstruction-001/
```

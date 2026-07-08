# Gate Results

Status: EXECUTED-HOLD-DX-REFERENCE-ADEQUACY
Evidence mode: Ran.

| Gate | Status | Evidence |
|------|--------|----------|
| `git diff --check` | PASS | Ran, no output. |
| Markdown/doc lint for touched docs | PASS | `markdown-doc lint --path ...` -> 27 files, 0 errors, 0 warnings. |
| Required-reading budget | PASS | `artifacts/required-reading-map.md`; AGENTS and contract docs read before edits. |
| Contract/profile/BEI checks for touched `SC-*` contracts | PASS | `python3 tools/release/check_sc_unit_compliance.py --path docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md` passed; `python3 tools/check_sc_binding_exposure.py ...` exited 0 with known science-review-follow-on rows. Full all-contract unit compliance still fails on unrelated pre-existing contracts. |
| Case-4 dimensionless cells-per-reach convergence ladder | PASS | Full nextest includes Case-4 tests; focused 3-test Case-4 command also passed. |
| Fine-reference adequacy halving check | FAIL | `mesh-ladder-summary.md`: `mn_corn_h4` and `n_idaho_forest_h1` adequate; `wa_cascades_forest_h1` fine rungs failed active closure; H2637 stress failed shape/sediment adequacy. |
| Candidate-vs-reference and baseline-vs-reference error tables | PASS | `mesh-ladder-summary.md` and `.json` generated from trace outputs. |
| Predeclared counted residual-class surfaces | PASS | Trace + manifest fields include end-window storage, tail-fold, uniform-shape, and degenerate-shape classes. |
| `min_cells` scheme-regime rationale and short-OFE floor rung | PASS | `mesh-baseline-inventory.md`; H2637/mn/WA floor rungs recorded. |
| Fixed `dt` caps across mesh ladder | PASS | Contract rev 39 and code keep `900 s` sample / `300 s` max substep. |
| Shadow-lane policy decision if production mesh changes | PASS | No production mesh change; shadow remains separate fixed `10 cells/OFE`. |
| Selected-cohort active plain timing/fidelity | FAIL | Full ladder ran 24 rungs; WA fine-reference rungs failed closure and promotion is held/rejected. |
| H2637 synthetic stress timing/fidelity | FAIL | H2637 ran all rungs; retained as synthetic stress only and failed adequacy on shape/sediment. |
| Exact runner-binary provenance | PASS | Release SHA256 `9a4f9c2755723c2e312dea460ed714bb183e283968fef2f003cf7690a71d48b8`. |
| Protected default/off byte identity if touched | NOT RUN | No production/default mesh flip; trace output is opt-in only. |
| Active-mode closure evidence | FAIL | Fixed/default rungs close; WA diagnostic fine rungs fail active closure and hold/reject promotion. |
| DC01/no-double-feed proof if production active routing changes | NOT RUN | No production active ownership change. |
| Routed-hydrograph consumer proof if production active routing changes | NOT RUN | No production active ownership change; trace rows record diagnostic D13 routed hourly weights but are not a new cutover proof. |
| Focused Lane D / `ofe_routing` tests | PASS | Mesh-policy selector tests, trace selector test, trace-only startup regression, and Case-4 focused tests passed. |
| `cargo fmt --check` | PASS | Ran after final edits. |
| `cargo clippy --workspace --all-targets -- -D warnings` | PASS | Ran after final edits. |
| `cargo nextest run --workspace --profile full` | PASS | 1418/1418 passed, 3 skipped. |
| `cargo deny check` | PASS | advisories/bans/licenses/sources ok. |
| Anti-evasion guard if triggered | NOT RUN | No required-case binding, cohort fixture authority, or external-authority suite posture changed. |
| `.rs` line-count governance | PASS | `line-count-governance.md`; one pre-existing 2749-line builder WARN recorded. |

# Secondary QA review — recorder fixture isolation

**Reviewer:** `/root/wrapper_custody_qa`, independent secondary QA reviewer

**Evidence class:** Static: inspected the terminal candidate/reference deltas, preservation receipts, command records, compiler diagnostic, configuration continuity, and correctness review. Ran evidence inspected: final candidate `cargo check --lib --tests` passed; the detached-reference equivalent failed. I ran no Rust, model, reader, or corpus command. A read-only scoped whitespace check of the three terminal changed files in each tree found no whitespace errors.

## Findings

### HIGH — detached reference cannot compile, blocking all required behavioral and quality gates

**Paths:** [reference-check.json](reference-check.json), [reference-check-errors.json](reference-check-errors.json), `crates/openwepp-hillslope-orchestrator/src/land_surface_energy_shadow/mod.rs:78`

The reference check exits 101 because its copy omits `tests/integration/land_surface_energy_real_hydrology_shadow_contract.rs`, which the orchestrator test module includes. The input exists with the same SHA-256 in the producing cut and candidate but is absent from the reference. The hard stop was reached at 27/27, so no repair or rerun is allowed.

This blocks compiled listing, native reference/candidate fixture equality, the original 540-second regression, recorder mode controls, terminal affected-file `rustfmt --check`, owning/runner Clippy, and matched lint comparison. The final candidate check passing does not establish these workflow gates.

### HIGH — fixture acceptance remains unproved

**Paths:** [correctness-review.md](correctness-review.md), [stop.json](stop.json)

No final execution compares disabled, non-target, enabled, capture-failure, wrong-phase, and omission recorder modes against the required canonical result/custody payload. The original timing regression and matched native characterization pair are also not run. The package must remain **HOLD / NO APPROVAL**.

### MEDIUM — test-oracle maintenance debt remains

**Paths:** `crates/openwepp-hillslope-orchestrator/src/v9_real_consumer_shadow_wb14_tests.rs`, `crates/openwepp-hillslope-orchestrator/src/snow_stage3_v11_adaptive_production_tests.rs`

The independent, manually projected complete-owner and accepted-segment test payloads avoid using recorder output as their own oracle, which is appropriate. They duplicate evolving owner structures, however. Future field additions require mutation-sensitive updates to both projection surfaces before a completeness claim can be retained. The prior dynamic-frame limitation also remains: immutable constructor inputs are independently checked, but there is no independent expected payload for future dynamic hydrology output fields.

## Non-blocking debt and follow-ups

- [execution-config-continuity.json](execution-config-continuity.json) resolves the preliminary configuration custody concern: `.cargo` is absent and `.config/nextest.toml` is identical in producing, candidate, and reference trees. Future detached-copy tooling should include the `tests/` tree whenever crate modules include it, and record it in source membership.
- [final-source.json](final-source.json) and [reference-final-source.json](reference-final-source.json) bind the terminal candidate (`719ba2…`, patch `4378d…`) and reference (`996e87…`, patch `e8a700…`) to zero-fuzz reconstructions; [preservation-checks.json](preservation-checks.json) binds both command snapshots. [terminal-delta.json](terminal-delta.json) limits post-compile-cut changes to three test/harness files, records equality of both shared harness files and the native-characterization block, and preserves the original caller and selector module.
- The candidate library/test-target check passed on the final candidate with the source unchanged. This is compile evidence only. Terminal affected-file `rustfmt --check`, Clippy comparisons, test listing, and all behavioral tests remain unperformed.

## QA disposition

**HOLD / NO APPROVAL.** Static custody for the frozen terminal source is adequate and the scoped whitespace check found no issue, but the reference-copy preparation defect exhausted the authorized correction budget before required behavior and quality gates could run.

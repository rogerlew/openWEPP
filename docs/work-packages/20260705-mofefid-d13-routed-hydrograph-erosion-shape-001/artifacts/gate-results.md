# Gate Results

Status: **COMPLETE** (Static + Ran).

| Gate | Evidence | Status | Notes |
|---|---|---|---|
| `git diff --check` | Ran | PASS | No whitespace errors. |
| Markdown lint | Ran | PASS | Final `markdown-doc lint --path ...` validated 30 files, 0 errors, 0 warnings. |
| Contract/profile/BEI checks | Ran | PASS-WITH-DISPOSITION | `check_sc_binding_exposure.py SC-OFEROUTE-001.md` = `PASS-DEFERRED`, matching existing science-review-follow-on rows. `SC-SED-001` has no BEI section; markdown and SC unit lint passed. |
| Unit-governance checks | Ran | PASS-WITH-DISPOSITION | `check_sc_unit_compliance.sh` passed on both touched contracts; `check_unit_registry.sh` passed 21 tests. Raw conversion scan on broad touched files reported pre-existing unrelated literals; D13 added no raw dimensional conversion. |
| Focused D13 tests | Ran | PASS | `cargo test -p openwepp-hillslope-orchestrator wave1_span_routed_hydrograph_shape -- --nocapture`: 3 passed. |
| Adjacent Wave-1 suite | Ran | PASS | `cargo test -p openwepp-hillslope-orchestrator direct_runtime_wave1_continuity -- --nocapture`: 28 passed. |
| Size-bound regression gate | Ran | PASS | `r7b_constructor_type_size_layout_is_bounded`: `DirectDayConstructorInputs=4088 <= 4096` after boxing routed shape. |
| H2637/Lane D evidence | Ran | PASS | `cargo test -p openwepp --test laned_shadow_h2637 h2637_native_shadow_classifies_uniform_shape_after_d12 -- --ignored --nocapture`: 1 passed, 325.24s. |
| `cargo check --workspace` | Ran | PASS | Workspace checked after final boxed-shape fix. |
| `cargo fmt --check` | Ran | PASS | Clean. |
| `cargo clippy --workspace --all-targets -- -D warnings` | Ran | PASS | Clean. |
| `cargo nextest run --workspace --profile full` | Ran | PASS | Final run: 1381 passed, 2 skipped, 579.721s. Initial run found the day-constructor size regression; fixed by boxing the routed shape. |
| `cargo deny check` | Ran | PASS | `advisories ok, bans ok, licenses ok, sources ok`. |
| Anti-evasion guards | Static | NOT REQUIRED | D13 did not touch external-authority suite posture, cohort fixtures, or required-case bindings. |

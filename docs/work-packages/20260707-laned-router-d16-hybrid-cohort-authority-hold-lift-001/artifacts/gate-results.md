# Gate Results

Status: EXECUTED-HOLD-ROUTE-COEFFICIENT-AUTHORITY. Evidence mode: Static + Ran.

| Gate | Status | Evidence |
|---|---|---|
| `git diff --check` | PASS | Final post-review run exit `0`; package-local no-index whitespace check over new files also exit `0`. |
| Markdown/doc lint | PASS | Package path: `20` Markdown files, 0 errors/warnings. README path: `1` file, 0 errors/warnings. |
| owcmp env preflight | PASS | All three selected manifests pass `owcmp env --manifest`; see `owcmp-preflight.md`. |
| owcmp executable suite preflight | BLOCKED | All three selected manifests exit `1` under `manifest run` because they are inventory-only. |
| Active-runnable input preflight | BLOCKED | Selected roots have `0` native route-coefficient matches and `0` `*.run.toml` files. |
| Active missing-coefficients guard | PASS | `cargo test -q --test laned_shadow_h2637 h2637_active_fails_closed_without_routing_coefficients`: `1` passed. |
| Contract/profile/BEI checks | NOT RUN | No contract edits landed. |
| Focused Lane-D / `ofe_routing` tests | NOT RUN | No code/contract edits landed beyond the focused active guard test. |
| Anti-evasion guards | NOT RUN | No required-case binding, cohort fixture, or external-authority suite posture changed. |
| `cargo fmt --check` | PASS | Final post-review run exit `0`. |
| `.rs` line-count governance | PASS | `git diff --name-only -- '*.rs'` returned no files. |
| Full Rust closure loop | NOT RUN | No Rust, fixture, suite-posture, or contract implementation landed. |

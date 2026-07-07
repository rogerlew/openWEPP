# Gate Results

Status: EXECUTED-HOLD-ROUTE-COEFFICIENT-BRIDGE-AUTHORITY. Evidence mode: Static + Ran.

| Gate | Status | Evidence |
|---|---|---|
| `git diff --check` | PASS | Post-review run exit `0`; see `verification-local-gates.md`. |
| Markdown/doc lint | PASS | Final package path: `18` Markdown files, 0 errors/warnings. README path: `1` file, 0 errors/warnings. |
| Current source-authored input scan | BLOCKED | Ran over selected D16 roots: `157` `.man` files, zero native datver or `routing_coefficients` matches, zero `*.run.toml` active inputs. |
| Bridge-authority audit | BLOCKED | Static audit found no current contract/provenance mapping for all five route coefficients; D11 evidence rejects legacy-field inference. |
| Active missing-coefficients fail-closed guard | PASS | `cargo test -q --test laned_shadow_h2637 h2637_active_fails_closed_without_routing_coefficients`: `1` passed. |
| Contract/profile/BEI checks | NOT RUN | No `SC-*` contracts were touched. |
| Native parse/projection tests | NOT RUN | No native input-authoring or bridge code landed. |
| Active plain/hybrid preflight | BLOCKED | Source-authorized active inputs were not produced. |
| Anti-evasion guards | NOT RUN | No suite posture, cohort fixture, external-authority binding, or required-case binding changed. |
| `cargo fmt --check` | PASS | Post-review run exit `0`; see `verification-local-gates.md`. |
| `.rs` line-count governance | PASS | `git diff --name-only -- '*.rs'` returned no files. |
| Full Rust closure loop | NOT RUN | No Rust, contract, fixture, or suite-posture implementation landed. |

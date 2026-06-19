# PERFDEEP09 Remediation Iteration Log

Status: complete.
Evidence class: Static + Ran.

| Iteration | Hypothesis | Files | Validation | Screening timing | Decision |
|---|---|---|---|---|---|
| 0 | No-edit current branch still fails gate | none | release build; H2637 control | `682.65 s`, RSS `228924 KB` | blocker reproduced |
| 1 | Replace private `SymbolRegistry` reverse lookup `BTreeMap` with `HashMap` | `crates/openwepp-kernel-contract/src/lib_mod/core_types/00_symbol_registry_and_indexed_surfaces.rs` | `cargo test -p openwepp-kernel-contract symbol_registry`; `cargo fmt --check` | `689.30 s`, RSS `229352 KB` | rejected and reverted; slower than control and not identity-clean |
| 2 | Collapse perennial decomposition overflow validation from seven full scans to one slot/crop pass | `crates/openwepp-hillslope-orchestrator/src/hydrology/07_decomposition_equations.rs`; `crates/openwepp-hillslope-orchestrator/src/tests/tests_mod/boundaries.rs` | `cargo test -p openwepp-hillslope-orchestrator pl12_contract_conformance`; `cargo test -p openwepp-hillslope-orchestrator decomposition`; `cargo fmt --check` | `634.61 s`, RSS `228856 KB` | retained; ran final three-rep gate |

Final retained code:

- `perfdeep09_decomp_overflow_one_pass_rep1`: `634.61 s`, RSS `228856 KB`
- `perfdeep09_decomp_overflow_one_pass_rep2`: `635.65 s`, RSS `228280 KB`
- `perfdeep09_decomp_overflow_one_pass_rep3`: `636.58 s`, RSS `228168 KB`

Median: `635.65 s` (`<= 676.67 s`).

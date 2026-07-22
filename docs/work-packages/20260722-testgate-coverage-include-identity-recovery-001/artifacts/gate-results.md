# Gate Results

| Gate | Status | Evidence |
| --- | --- | --- |
| Direct characterization | PASS | Nextest run `37ce16ae-19be-4607-bf77-8297fce225c2`; 1 passed. |
| Real verifier consumer | PASS | Nextest run `e3ce5d89-b78d-4766-93df-bb3ed164c895`; 1 passed in 209.652s. |
| Focused full-source LLVM coverage | PASS | Exact moved `SF` and owned FN/FNDA; LCOV SHA `3f77c88b...`. |
| Focused cargo-crap | PASS | Target CRAP 5; helpers CRAP 2; JSON SHA `df93664b...`. |
| `cargo fmt --all` / check | PASS | Ran before focused evidence. |
| Planner all-target Clippy, warnings denied | PASS | Exit 0. |
| CRAP production-filter unit | PASS | 1 passed in 0.035s. |
| `git diff --check` | PASS | Exit 0. |

Ran: the successful coverage command used the pinned cargo-llvm-cov control
`--no-default-ignore-filename-regex`. The default excludes both `/tests/`
components and `*_tests.rs`; disabling that reporting filter exposed the exact
test source without changing executed code.

Retained evidence:

- `/tmp/cqr-b02-verifier-full-source.lcov`, SHA-256
  `3f77c88bbfb17b57facbec60739dcba8cc135acfaba1588c6d50481c6a7571a0`;
- `/tmp/cqr-b02-verifier-full-source-crap.json`, SHA-256
  `df93664b01e5f95897d3061661324cdc04ee4b644bbe2a1b3bac7b9e742f423e`.

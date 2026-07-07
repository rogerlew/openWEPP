# Gate Results

Status: EXECUTED-HOLD-FIDELITY-TOLERANCE. Evidence mode: Static + Ran.

| Gate | Status | Evidence |
|---|---|---|
| `git diff --check` | PASS | Ran after review-response artifacts; exit `0`. |
| Markdown/doc lint for touched docs | PASS | `markdown-doc lint --path <package>`: 13 files, 0 errors/warnings. `markdown-doc lint --path docs/work-packages/README.md`: 1 file, 0 errors/warnings. |
| Contract/profile/BEI checks | NOT RUN | No contract amendment landed; hold occurred before contract/code edits. |
| Case-4 hybrid ladder | PASS | Ran focused nextest command; 1 passed in `144.949 s`. |
| H2637 active endpoint/profile timing | PASS | Active plain `39.73 s`; explicit hybrid `33.45 s`; counters recorded. |
| H2637 fidelity/delta audit | BLOCKED | Material publication deltas lack named default-promotion tolerance authority. |
| Protected-output byte identity | NOT RUN | No code/default flip landed; default/off pre-change baseline recorded. |
| Active-mode closure evidence | PASS | Explicit hybrid manifest closure residuals remain machine-scale. |
| Selector provenance proof | NOT RUN | No implementation landed; current env opt-in semantics unchanged. |
| Focused Lane-D / `ofe_routing` tests | NOT RUN | Broader focused suite not run because package held before code; Case-4 subgate is recorded separately as PASS. |
| `cargo fmt --check` | PASS | Ran after review-response artifacts; exit `0`. |
| `cargo clippy --workspace --all-targets -- -D warnings` | NOT RUN | No Rust implementation landed; package held before code. |
| `cargo nextest run --workspace --profile full` | NOT RUN | No Rust implementation landed; package held before code. |
| `cargo deny check` | NOT RUN | No dependency/Rust implementation change; package held before code. |
| `.rs` line-count governance | PASS | `git diff --name-only -- '*.rs'` returned no files; no Rust line-count exposure changed. |

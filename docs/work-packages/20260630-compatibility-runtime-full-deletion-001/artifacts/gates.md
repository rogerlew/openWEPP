# Gate Evidence

Evidence mode: Static/Ran.

| Gate | Status | Evidence |
|---|---|---|
| ADR-0031 ratified | PASS | Static: ADR-0031 status is Accepted with this package as ratification provenance. |
| Runtime selector removed | PASS | Static: no `HillslopeRuntimeSelection::Compatibility`, `HillslopeDefaultRuntimeActivation::Disabled`, or `--compatibility-runtime` remains under `crates/` or `tools/`; CLI help no longer advertises the flag; observed harness only accepts `direct-production-executor`. |
| Focused runner regression | PASS | Ran: `cargo check -p openwepp-runner`; `cargo clippy -p openwepp-runner --all-targets -- -D warnings`; `cargo test -p openwepp-runner`; `cargo fmt --check`; `git diff --check`. |
| Output identity | NOT-RUN | Full H2637/multi-OFE/Wave-2 byte/value identity was not run because Stage 2 deletion held before a full deletion candidate existed. |
| No compatibility runtime | HOLD | Static: public selector is gone, but `scheduler.rs`, `day_frame.rs`, scheduler lifecycle helpers, and carrier types remain compiled and test-backed; static carrier scan found approximately 1100 symbol-map references. |
| RSS/time | NOT-RUN | Not measured because no full runtime deletion candidate existed after the Stage 2 hold. |
| Scoped doc lint | PASS | Ran: `markdown-doc lint --path docs/decisions/0031-delete-compatibility-runtime-single-authority-terminal.md --path docs/decisions/README.md --path docs/work-packages/20260630-compatibility-runtime-full-deletion-001 --path docs/work-packages/README.md --path docs/ROADMAP.md --path docs/architecture/array-native-runtime-specification.md` (`10 files validated`). |
| Full closure gates | NOT-RUN | Full workspace gates were not run because the package held before full deletion. Focused runner/doc gates are recorded above. |

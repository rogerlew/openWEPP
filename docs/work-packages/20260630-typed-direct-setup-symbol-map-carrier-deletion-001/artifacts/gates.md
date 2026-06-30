# Gates

Evidence mode: Static/Ran.

| Gate | Status | Evidence |
|---|---|---|
| Stage 1 typed setup without production symbol-map seed surfaces | HOLD | Not met. Static inventory shows direct setup still uses `HillslopeWritebackSurface` seed authorities for lane constructor and day-input authority seeding. |
| Stage 1A direct symbol-registry removal | PASS | Production direct setup now skips `SymbolRegistry` / `HotSymbolTables`; H2637 byte identity and zero compatibility edges verified. |
| Stage 2 carrier deletion | BLOCKED | Blocked until Stage 1 removes setup authority. |
| Stage 3 no-compatibility proof | BLOCKED | Awaiting setup-carrier deletion and counters/static audit. |
| H2637 identity | PASS | HBP/loss/plot/WAT/PASS byte-identical against clean `5b139058` baseline. |
| Multi-OFE/Wave-2 focused gate | PASS | `cargo test -p openwepp --test cli03_runner_contract_derived_tests cli03_mofe03_multiofe_runfile_executes_wave2_without_manual_symbol_injection -- --nocapture`. |
| RSS no-regression | PASS | H2637 full-output RSS `91796 KiB`, below the clean `5b139058` baseline `110916 KiB` and the prior streaming-sink envelope (`112652 KiB`). |
| Focused build/check | PASS | `cargo fmt --check`; `cargo check -p openwepp-runner`; release build. |
| Scoped Markdown lint/validate | PASS | `markdown-doc lint --path docs/work-packages/README.md --path docs/work-packages/20260630-typed-direct-setup-symbol-map-carrier-deletion-001 --format json`; `markdown-doc validate --path docs/work-packages/README.md --path docs/work-packages/20260630-typed-direct-setup-symbol-map-carrier-deletion-001 --format json`. |
| Full Rust/doc gates | NOT RUN | Not run because package cannot pass Stage 1 and closes HOLD before carrier deletion. Focused compile/identity gates passed. |

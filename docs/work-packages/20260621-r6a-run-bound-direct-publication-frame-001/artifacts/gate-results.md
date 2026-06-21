# Gate Results

Status: pass.
Evidence mode: Static + Ran.

| Gate | Status | Evidence |
|---|---|---|
| Consumer-Path Closure Rule added | PASS | `AGENTS.md` and `docs/work-packages/AGENTS.md` updated; root file is 122 lines. |
| R6A package scaffold | PASS | `package.md`, active prompt, archived prompt index, and required artifact set created. |
| Pre-implementation data-path proof | PASS | Current failure path recorded in `data-path-proof.md` and `internal-scaffolding-evidence.md`. |
| Publication frame implementation | PASS | `DirectRunPublicationFrame`, `DirectPublicationDayRow`, typed operand groups, and `DirectFrameExecutor::run_publication_capture` added in `openwepp-hillslope-orchestrator`. |
| Direct output projection consumers | PASS | Direct HBP/WAT/PASS/loss/manifest projection helpers added in runner and exercised by focused tests. |
| Anti-alias fixtures | PASS | R6A-scope fixtures prove direct runoff projection does not alias `publication.runoff_m` and WAT/PASS read supplied direct operands. Broader production writer anti-alias remains R6 cutover scope. |
| Independent reconstruction | PASS | R6A-scope direct projection consumer test reconstructs WAT/PASS/loss/manifest expected values from independently supplied frame operands; byte/Arrow/checksum reconstruction remains R6 writer cutover scope. |
| No-compatibility proof | PASS | Source scans over new direct builder/projection ranges found no forbidden compatibility source tokens; runtime tests prove default compatibility has `publication_capture_runs = 0` and opt-in capture has `skeleton_runs = 0`. |
| `cargo fmt --check` | PASS | Ran successfully after implementation. |
| `cargo clippy --workspace --all-targets -- -D warnings` | PASS | Ran successfully after fixing R6A clippy findings. |
| `cargo test --workspace` | PASS | Full workspace test gate passed. |
| `cargo deny check` | PASS | `advisories ok, bans ok, licenses ok, sources ok`. |
| Scoped markdown lint | PASS | `markdown-doc lint --path AGENTS.md --path docs/work-packages/AGENTS.md --path docs/ROADMAP.md --path docs/work-packages/README.md --path docs/work-packages/20260621-r6a-run-bound-direct-publication-frame-001 --format json`: 27 files scanned, 0 errors, 0 warnings. |
| `git diff --check` | PASS | Ran successfully after implementation edits. |

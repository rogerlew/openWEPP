# Gate Results

Status: executed-hold.
Evidence mode: Static + Ran.

| Gate | Status | Evidence |
|---|---|---|
| R5E prerequisite or waiver | PASS | R5E complete at pushed commit `d8f6bbea`; package `20260621-r5e-full-ofe-day-endpoint-readiness-001/` records verdict `COMPLETE-R5E-FULL-OFE-DAY-ENDPOINT-READINESS`. |
| Publication ledger canonical promotion | PASS | `docs/architecture/array-native-runtime-specification.md` section `5.2.1 R6 Canonical Publication Operand Ledger` promotes the PERFDEEP06 ledger into canonical architecture authority. |
| Direct publication frame availability | PASS | R6A completed `DirectRunPublicationFrame` and direct projection consumers; current R6 cutover candidate builds direct publication artifacts. |
| HBP byte identity | FAIL | Cutover candidate exits fail-closed: `HBP byte identity failed: direct=1654 bytes compatibility=1654 bytes`. |
| WAT Arrow/metadata parity | NOT RUN | HBP fail-closed gate stops candidate first. |
| PASS Arrow/metadata parity | NOT RUN | HBP fail-closed gate stops candidate first; current fixture lacks PASS parquet output. |
| loss JSON parity | NOT RUN | HBP fail-closed gate stops candidate first; direct helper now emits schema-shaped JSON. |
| manifest parity | BLOCKED | Production manifest writer still uses compatibility provenance/checksum surfaces; candidate blocks if earlier families pass. |
| Anti-alias fixtures | NOT RUN | Current direct operands are not parity-grade and cannot close anti-alias acceptance. |
| Independent operand reconstruction | NOT RUN | Current direct operands are not parity-grade and cannot close reconstruction acceptance. |
| No-compatibility proof | BLOCKED | Direct projection helpers avoid compatibility reads, but the candidate parity gate and manifest path still read compatibility surfaces; no production no-compat proof exists. |
| Default-disabled H2637 gate | NOT RUN | No default-path behavior changed; full H2637 benchmark deferred because R6 completion gates are blocked. |
| Endpoint/RSS evidence | BLOCKED | Cutover candidate exits at HBP parity before a valid endpoint exists. |
| Focused Rust tests | PASS | `cargo test -p openwepp-runner r6_ -- --nocapture`; `cargo test -p openwepp-runner --test r6_direct_publication_cutover_cli_contract -- --nocapture`; `cargo test -p openwepp-runner r6a_ -- --nocapture`; `cargo test -p openwepp-runner r2a_default_fixture_run_constructs_no_direct_runtime_skeleton -- --nocapture`. |
| Runner package tests | PASS | `cargo test -p openwepp-runner`. |
| Full Rust gates | PASS | `cargo fmt --check`; `cargo clippy --workspace --all-targets -- -D warnings`; `cargo test --workspace`; `cargo deny check`. |
| Scoped markdown lint | PASS | `markdown-doc lint --path docs/ROADMAP.md --path docs/work-packages/README.md --path docs/work-packages/20260621-r6-direct-publication-cutover-001 --format json`. |
| `git diff --check` | PASS | No whitespace errors. |
| Dual review and verification | PASS | Delegated review findings were dispositioned; final review and verification artifacts record the current parity/manifest hold. |

Any `FAIL`, `BLOCKED`, or unjustified `NOT RUN` status blocks R6 completion.
Current final disposition is executed-hold.

# Gate Results

Status: T-B2-REDO executed

Evidence mode: Ran + Static

| Gate | Result | Evidence |
|---|---:|---|
| W-A no production edits | PASS | Only package documentation/artifact files edited. |
| Current watershed CLI behavior recorded | PASS | Ran `openwepp-cli-watershed`; exit `1`, `CLIWAT-E-010`/`IMP-E-004`, zero output files. |
| `jpond=0` classified | PASS | Classified parser defect in `impoundment-no-pond-finding.md` with openWEPP and legacy citations. |
| Routing/output seams mapped | PASS | Recorded CLI, orchestrator, kernel, writer, and contract seams in `watershed-routing-scope.md`. |
| totalwatsed3 input contract documented | PASS | Recorded openWEPP schema and wepppy/audit expected columns in `watershed-routing-scope.md`. |
| Scoped markdown lint | PASS | `markdown-doc lint --path docs/work-packages/20260613-wshed01-watershed-routed-outputs-totalwatsed3-closure-001 --format json`: `25` files scanned, `0` errors, `0` warnings. |
| Whitespace diff check | PASS | `git diff --check`: no findings. |
| Heavy batch/comparator runs | NOT RUN | Not in W-A scope; current blocker occurs before HBP routing/output. |
| Rust fmt/clippy/test/deny closure | NOT RUN | No production Rust edits in W-A. |
| W-B contract amendment | PASS | `openwepp-watershed-runfile-contract.md` pins explicit `jpond=0` no-pond semantics and preserves required `inputs.pw0_imp`. |
| W-B red tests | PASS | Focused zero-impoundment parser tests failed before production edit with old `IMP-E-004` behavior. |
| W-B parser/CLI tests | PASS | Parser contract `18` passed; explicit zero-impoundment CLI regression `1` passed; runtime seed guard `3` passed. |
| W-B arboreal-dendrite seam | PASS | Real CLI proceeds past `CLIWAT-E-010`; next hard stop is `CLIWAT-E-020` / `WKERNEL-WS10-CHANNEL-E-003`; output file count `0`. |
| W-B fmt/clippy | PASS | `cargo fmt --check`; `cargo clippy -p openwepp-input-contract -p openwepp-runner --tests -- -D warnings`. |
| W-B scoped markdown lint | PASS | `markdown-doc lint --path docs/work-packages/20260613-wshed01-watershed-routed-outputs-totalwatsed3-closure-001 --format json`: `25` files scanned, `0` errors, `0` warnings. |
| W-B heavy batch/comparator runs | NOT RUN | W-B is a focused parser/CLI seam; no comparator or totalwatsed3 batch is meaningful before W-C clears channel routing. |
| W-C hard-stop classification | PASS | `CLIWAT-E-020` / `WKERNEL-WS10-CHANNEL-E-003` classified as valid zero-sediment HBP payload rejected by over-strict guard; hidden `nchnum=0` output-disabled state also classified. |
| W-C contract amendment | PASS | `SC-ROUTE-001` version `45` pins zero-sediment contributor semantics and `nchnum=0` output-disabled channel semantics. |
| W-C focused tests | PASS | WS10 W-C tests `2` passed; watershed writer tests `2` passed; watershed CLI output regression `1` passed. |
| W-C real CLI routed output | PASS | Configured and legacy-discovery arboreal-dendrite runs exited `0`, each emitting all `14` watershed parquet files. |
| W-C anti-placeholder output | PASS | `totalwatsed3.parquet` has `2192` rows; WAT fields include nonzero `P`, `RM`, `SoilWaterTotal`; `runvol` agrees with `Q * Area / 1000.0`. |
| W-C fmt/clippy/test/deny | PASS | `cargo fmt --check`; `cargo clippy --workspace --all-targets -- -D warnings`; `cargo test --workspace`; `cargo deny check`. |
| W-C scoped markdown lint | PASS | `markdown-doc lint --path docs/work-packages/20260613-wshed01-watershed-routed-outputs-totalwatsed3-closure-001 --format json`: `26` files scanned, `0` errors, `0` warnings. |
| W-C totalwatsed3 closure audit | NOT RUN | W-D owns the wepppy totalwatsed3 audit and independent water-balance residual closure. |
| W-D focused producer tests | PASS | `cargo test -p openwepp-runner watershed_wat::tests -- --nocapture`: `2` passed; `cargo test -p openwepp-watershed-output writer_preserves_multiple_watershed_daily_rows_and_wat_fields -- --nocapture`: `1` passed. |
| W-D real CLI routed output | PASS | Fresh configured and legacy-discovery arboreal-dendrite runs exited `0` and emitted routed outputs. |
| W-D totalwatsed3 publication defects | PASS | Exact hydrology fields publish as `m^3`; depth aliases remain mm; `latqcc` is outlet-only; profile/interception fields publish; profile audit violations are zero. |
| W-D totalwatsed3 closure audit | FAIL | Configured and legacy-discovery audits both report `closure_reconstructed_with_storage_total_mm=2950.498418` and `closure_reconstructed_with_storage_pct_of_precip=17.772166`. |
| W-D comparator/heavy-runner subagent | NOT RUN | The comparator-suite runner was not used; W-D ran command-level configured/legacy audits directly in this session. |
| W-D final fmt/clippy/test/deny | PASS | `cargo fmt --check`; `cargo clippy --workspace --all-targets -- -D warnings`; `cargo test --workspace`; `cargo deny check`. |
| W-D final diff/doc lint | PASS | `git diff --check`: no findings; `markdown-doc lint --path docs/work-packages/20260613-wshed01-watershed-routed-outputs-totalwatsed3-closure-001 --format json`: `27` files scanned, `0` errors, `0` warnings. |
| T-A no production edits | PASS | Only package documentation/artifact files edited. |
| T-A authoritative semantics read | PASS | Read `/home/workdir/wepppy/wepppy/wepp/interchange/totalwatsed3.py` and `/home/workdir/wepppy/tools/totalwatsed3_daily_closure_audit.py`; recorded cited semantics in `totalwatsed3-cli-scope.md`. |
| T-A schema sampling | PASS | Sampled `/wc1/runs/ar/arboreal-dendrite/wepp/output/interchange/H.pass.parquet`, `H.wat.parquet`, `H.soil.parquet`, and `H.element.parquet` with pyarrow; row counts and key columns recorded. |
| T-A CLI scope artifact | PASS | `totalwatsed3-cli-scope.md` defines inputs, aggregation semantics, openWEPP-native schema, independent closure identity, red tests, and T-B/T-C breakdown. |
| T-A production Rust gates | NOT RUN | T-A is a no-production-code design increment. |
| T-A scoped markdown lint | PASS | `markdown-doc lint --path docs/work-packages/20260613-wshed01-watershed-routed-outputs-totalwatsed3-closure-001 --format json`: `28` files scanned, `0` errors, `0` warnings. |
| T-B red tests | PASS | `cargo test -p openwepp-runner --test totalwatsed3_cli_contract` failed before implementation because `CARGO_BIN_EXE_openwepp-cli-totalwatsed3` was not defined. |
| T-B focused CLI tests | PASS | `cargo test -p openwepp-runner --test totalwatsed3_cli_contract`: `2` passed. |
| T-B unit lineage registry test | PASS | `cargo test --test sim_contract_boundary_unit_registry`: `15` passed; `watershed_totalwatsed3.Runoff` is publication-only PASS-volume lineage. |
| T-B real producer run | PASS | `cargo run -p openwepp-runner --bin openwepp-cli-totalwatsed3 -- --input-dir /wc1/runs/ar/arboreal-dendrite/wepp/output/interchange --output /tmp/openwepp_wshed01_tb/totalwatsed3.parquet`: `CLITW3-I-001 wrote 2192 rows`. |
| T-B parquet schema/readability | PASS | Pyarrow read confirmed `2192` rows, `79` columns, and no nulls in required water-balance columns checked. |
| T-B audit read | PASS | wepppy `totalwatsed3_daily_closure_audit.py` exited `0`, read the file without schema repair, and reported zero profile violations. Closure residual remains a T-C gate. |
| T-B watershed ownership relocation | PASS | `openwepp-cli-watershed` no longer owns WAT/PASS totalwatsed3 aggregation; the dedicated CLI owns native totalwatsed3 production. |
| T-B full fmt/clippy/test/deny | PASS | `cargo fmt --check`; `cargo clippy --workspace --all-targets -- -D warnings`; `cargo test --workspace`; `cargo deny check`. |
| T-B final diff/doc lint | PASS | `git diff --check`: no findings; `markdown-doc lint --path docs/work-packages/20260613-wshed01-watershed-routed-outputs-totalwatsed3-closure-001 --format json`: `28` files scanned, `0` errors, `0` warnings. |
| T-B2 red tests | PASS | Focused PASS runoff-delivery test initially failed because `hillslope_pass` and `append_runoff_delivery_rows_to` did not exist; per-hillslope totalwatsed3 fixture initially failed because the CLI required combined `H.pass.parquet`. |
| T-B2 focused tests | PASS | `cargo test -p openwepp-runner mofe01_tb2_pass_runvol_uses_terminal_outlet_transfer_volume_not_per_ofe_sum -- --nocapture`; `cargo test -p openwepp-runner --test totalwatsed3_cli_contract totalwatsed3_cli_reads_openwepp_per_hillslope_pass_and_wat_surfaces -- --nocapture`; `cargo test --test sim_contract_boundary_unit_registry hphys0278_output_unit_registry_covers_output_schema_unit_metadata -- --nocapture`. |
| T-B2 real native PASS emission | PASS | Release `openwepp-cli-hill` reran arboreal-dendrite p1-p36 with `outputs.pass_parquet`; output counts: `hbp=36`, `wat=36`, `pass_parquet=36`. |
| T-B2 HBP/WAT anchor stability | PASS | SHA-256 comparison of all `H1..H36.hbp` and `H1..H36.wat.parquet` against `/tmp/openwepp_mofe01_mi_final/output`: `anchor_mismatches=0`. |
| T-B2 PASS outlet identity | PASS | DuckDB audit over native PASS/WAT files: `78912` rows, `max_abs_runvol_diff_m3=1.4551915228366852e-11`, `avg_abs_runvol_diff_m3=2.476430939298028e-14`. |
| T-B2 native totalwatsed3 production | PASS | `target/release/openwepp-cli-totalwatsed3 --input-dir /tmp/openwepp_wshed01_tb2/output --output /tmp/openwepp_wshed01_tb2/totalwatsed3.parquet`: `CLITW3-I-001 wrote 2192 rows`. PASS sum and totalwatsed3 sum differ by `1.7881393432617188e-07 m^3` from floating accumulation order. |
| T-B2 full fmt/clippy/test/deny | PASS | `cargo fmt --check`; `cargo clippy --workspace --all-targets -- -D warnings`; `cargo test --workspace`; `cargo deny check`. |
| T-B2 final diff/doc lint | PASS | `git diff --check`: no findings; `markdown-doc lint --path docs/work-packages/20260613-wshed01-watershed-routed-outputs-totalwatsed3-closure-001 --format json`: `28` files scanned, `0` errors, `0` warnings; `cargo fmt --check`: no findings. |
| T-B2-REDO red test | PASS | `cargo test -p openwepp-runner mofe01_tb2_redo_pass_runvol_uses_outlet_ofe_area_not_hillslope_area -- --nocapture` failed against the old T-B2 `QOFE * publication area` formula: expected `0.5 m3`, observed `1.0 m3`. |
| T-B2-REDO focused tests | PASS | `cargo test -p openwepp-runner mofe01_tb2_redo_pass_runvol_uses_published_q_area_not_qofe_area -- --nocapture`; `cargo test -p openwepp-runner --test totalwatsed3_cli_contract totalwatsed3_cli_reads_openwepp_per_hillslope_pass_and_wat_surfaces -- --nocapture`. |
| T-B2-REDO corrected native PASS emission | PASS | Release `openwepp-cli-hill` reran arboreal-dendrite p1-p36 under `/tmp/openwepp_wshed01_tb2_redo_qarea`; output counts: `hbp=36`, `wat=36`, `pass_parquet=36`, `manifests=36`. |
| T-B2-REDO HBP/WAT anchor stability | PASS | SHA-256 comparison of all `H1..H36.hbp` and `H1..H36.wat.parquet` against `/tmp/openwepp_mofe01_mi_final/output`: `anchor_mismatches=0`. |
| T-B2-REDO independent PASS dual audit | PASS | DuckDB audit over `78912` rows: `max_abs_pass_minus_q_area_m3=0.0`; old `QOFE * Area` bug differs by up to `21766.4323911278 m3`; corrected total PASS `sum_runvol=6851275.733726179 m3`. |
| T-B2-REDO annual precipitation bound | PASS | Water-year per-hillslope DuckDB audit: `252` annual hillslope-water-years, `violation_count=0`, `max_runvol_precip_ratio=0.9857497687436844`, `max_excess_m3=-67.62322402014661`. |
| T-B2-REDO native totalwatsed3 production | PASS | `target/release/openwepp-cli-totalwatsed3 --input-dir /tmp/openwepp_wshed01_tb2_redo_qarea/output --output /tmp/openwepp_wshed01_tb2_redo_qarea/totalwatsed3.parquet`: `CLITW3-I-001 wrote 2192 rows`; totalwatsed3/PASS `runvol` diff `9.313225746154785e-10 m3`. |
| T-B2-REDO audit-read residual recording | PASS | wepppy `totalwatsed3_daily_closure_audit.py` read the corrected output and reports `closure_reconstructed_with_storage_total_mm=6948.564523`; this is recorded as T-C hold evidence, not T-B2-REDO closure. |
| T-B2-REDO full fmt/clippy/test/deny | PASS | `cargo fmt --check`; `cargo clippy --workspace --all-targets -- -D warnings`; `cargo test --workspace`; `cargo deny check`. |
| T-B2-REDO final diff/doc lint | PASS | `git diff --check`: no findings; `markdown-doc lint --path docs/work-packages/20260613-wshed01-watershed-routed-outputs-totalwatsed3-closure-001 --format json`: `29` files scanned, `0` errors, `0` warnings. |

Subagent note: comparator/heavy-batch subagent was not used because W-A required
only a single current-behavior CLI run and static source characterization.
For W-B, no comparator subagent was used because the increment required
focused parser/CLI gates only; the package comparator/closure surface remains
blocked behind W-C routing.
For W-C, no comparator subagent was used; command-level comparisons and gates
were run directly in this session because the comparator/heavy-runner subagent
was unavailable. No totalwatsed3 audit is claimed for W-C.
For W-D, no comparator-suite subagent was used. Configured and legacy-discovery
CLI/audit evidence was run directly and recorded in
`totalwatsed3-wd-audit-finding.md`.
For T-A, no comparator-suite subagent was used because the increment is
design-only. Static source reads and a pyarrow schema sample were run directly
in this session.
For T-B, no comparator-suite subagent was used. Focused tests, full Rust
gates, the real arboreal-dendrite producer run, and the wepppy audit read were
run directly in this session as command-level evidence.
For T-B2, no comparator-suite subagent was used. Focused tests, full Rust
gates, the real arboreal-dendrite native-output rerun, anchor hash comparison,
and parquet audits were run directly in this session as command-level evidence.
For T-B2-REDO, no comparator-suite subagent was used. Focused tests, full Rust
gates, the corrected arboreal-dendrite native-output rerun, anchor hash
comparison, DuckDB PASS/bound audits, and wepppy audit read were run directly
in this session as command-level evidence.

# Gate Results

Status: W-D executed-hold

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

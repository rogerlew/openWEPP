# Gate results

Status: `PHASE 4 COMPLETE — SELECTED GATES RECORDED; V61 EVIDENCE LIMITATION`

Evidence mode: `Static + Ran`

The comparator role was attempted first and failed before execution with the
fixed-role service error `You've hit your usage limit for GPT-5.3-Codex-Spark`
(2026-09-04). No comparator result is represented as local evidence. Because
the retained checkout has no Rust/source/authority changes, the parent ran the
bounded applicable checks below; full campaign/population gates were not
applicable to this documentation/raw-only final diff.

| Gate | Command / identity | Result | Evidence |
|---|---|---|---|
| Quick integrity | `git diff --check`; JSON matrix parse; memory JSONL parse (26 rows); `bash -n artifacts/raw/run_memory_observer.sh`; `nix develop -c cargo fmt --all -- --check` from `/workdir/openWEPP` | PASS | `raw/package_quick_checks.log`, SHA-256 `abdfe5a19fa0cabda132fc3abd88d133fa67cd898a9bc3ca9e6479b59fd1a81f` |
| Current scoped build | `timeout 900 nix develop -c cargo check -p openwepp-runner --lib` at HEAD `0d56001ed` | PASS, exit 0 | `raw/current_scoped_cargo_check.log`, SHA-256 `d9b2f57dc7fcfda92a9270bbb0b49a09c0333a9877a5eaa7e5f02633291520d4` |
| Authentic retained release probe | Release one-day test `hillslope::tests::stage3_laned_release_one_ofe_positive_baseline_profile`, one test thread, exact cached binary, exit 0 | PASS; exact closure/counters/output identity | `raw/current_baseline_profile_run1.log` (historical fallback transcript; admitted paired set is in `raw/current_memory_results.jsonl`) |
| Admitted R1 paired observer control | `timeout 600 artifacts/raw/run_memory_observer.sh`, fresh CPU-0 processes, 2 warmups + 12 AB/BA blocks | PASS; exit 0, 24 arms complete, zero gaps, exact protected values | `raw/current_memory_results.jsonl`, samples, manifest and hashes in `raw/README.md` |
| Isolated v31 reconstruction | Ordered patch replay in detached worktree; exact-HEAD and a28 overlays | FAIL/NO EQUIVALENCE; v31 patch replay 153 pass/99 fail; no candidate build | `raw/v31_patch_replay_status.log`, `raw/v31_head_overlay_apply.log`, `raw/v31_a28_overlay_check.log`, `raw/v31_isolated_reconstruction_check.log` |
| Isolated v61 reconstruction | 38 source patches from ordered session inventory replayed at exact HEAD; scoped orchestrator/runner cargo check | FAIL/NO EQUIVALENCE; 22 pass/16 fail; cargo check exit 101 | `raw/v61_patch_replay_status.log`, `raw/v61_isolated_cargo_check.log` |
| Historical v31 binary focused audit | Exact cached binary `f9386eec...fbeeaf`, focused real-runner parity test | PASS, 1/1, exit 0; at least one completed replay probe asserted and forced-complete parity passed | `raw/historical_v31_binary_component_replay_run.log` |
| Historical v31 binary release profile | Same exact cached binary, authentic one-OFE ignored release test | FAIL, exit 101 at `component_replay_audit.rs:131`; reproduces missing `58/14/16/28` aggregate before JSON | `raw/historical_v31_binary_release_profile_run.log` |
| Historical v31 debugger audit-return capture | Same exact unstripped binary; breakpoint at `take_covered_component_dependency_replay_audit` return; no source/binary mutation | PASS diagnostic capture; 2,000 N=2/S=6 rows decoded, all completed/!failed; 200 rows (iteration 0/1 for maps 0..99) are `48/14/10/24`, and the remaining 1,800 are `54/14/16/24` (600 iteration-0/1 rows for the other 300 maps plus 1,200 iteration-2/3/4 rows); zero target rows | `raw/historical_v31_gdb_take_return.log`, `raw/historical_v31_gdb_take_return_summary.txt` |
| Historical v31 lifecycle capture | Same exact binary; breakpoints at audit begin/take/aggregate | PASS diagnostic lifecycle separation; authentic audit is first take and forced-complete oracle is a distinct second take | `raw/historical_v31_gdb_lifecycle_counts.log` |
| v61 candidate-cut reconstruction | Session patch indexes 18–40 before cleanup/reversion patch 41, detached worktree | FAIL/NO EQUIVALENCE; 15/23 apply, scoped cargo check exit 101 with 7 errors | `raw/v61_candidate_cut_replay_status.log`, `raw/v61_candidate_cut_cargo_check.log` |
| v61 manual context reconciliation | Detached candidate-cut tree only; failed contexts repaired against actual post-index-17 text | `cargo check -p openwepp-runner --tests` PASS; documented changed-Rust manifest `19a07121…23421` is reproducible over 13 paths (an earlier undocumented canonical computation recorded `2fc5e8ca…e6eb7f`); neither is comparable to historical `650f6713…57d41` without historical Git-index/per-file manifest custody; not an authenticated candidate treatment | `raw/v61_manual_reconciliation_feasibility.log` |
| v61 manually reconciled real-consumer release probe | Detached candidate-cut tree only; release binary `d71fcef567e02ad1a06d5268bfee20dd84c4e751785f5ec4d4798dbe463ce3bb` | PASS diagnostic feasibility; exit 0, 200 feed-forward calls, exact `48/56/20/32/4` counters and closure, `rss_kib=58,440`, `run_wall_us=4,028,455`; source/index custody is nonhistorical, so no treatment attribution | `raw/v61_manual_reconciled_release_probe.log`, SHA-256 `dbced82c9d15ad63faa652e2bdf609b90a9a7d241b02fea845e5b634fb9955c1` |
| v61 reverse-cleanup reconstruction | Clean exact-HEAD detached clone; reverse successful cleanup payloads 56..41 | FAIL/NO EQUIVALENCE; 12 inversions applied, index 52 formatting-context failure, resulting tree `cargo check -p openwepp-runner --tests` exit 101 with 10 errors; cleanup payloads omit placement context and candidate-side request edits | `raw/v61_reverse_cleanup_reconstruction_verification_b.log`, SHA-256 `f79dcd05571582bec7695020d6f845de6b687c257bfb58b3dd20437a13311a3c` |
| Authority anti-evasion / AUTH11 | Not run | N/A — no authority suite, cohort fixture, or required-case binding was touched | Scope record above |

The predecessor's expected-red revision-31/revision-61 heavy comparator results
remain unchanged and are cited as historical evidence, not rerun or rewritten.

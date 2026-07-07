# Gate Results

Status: PASS. Evidence mode: Static + Ran.

| Gate | Result | Evidence |
|---|---|---|
| Clean starting tree | PASS-WITH-EXPECTED-DIRTY | Execution started from `main` ahead of origin by scaffold commit with package-local untracked artifacts; no branch switch. |
| Markdown/doc lint | PASS | Final-tree rerun: `markdown-doc lint ...`: `20 files validated`, `0 errors`, `0 warnings`. |
| Contract/profile/BEI checks | PASS-DEFERRED | Final-tree rerun: `SC-OFEROUTE-001` 7 BEI rows / 6 science-review-follow-on; `SC-OFEROUTE-002` 4 BEI rows / 4 science-review-follow-on. |
| SC unit compliance | PASS | `check_sc_unit_compliance.sh --path` for `SC-OFEROUTE-001` and `SC-OFEROUTE-002`: no findings. |
| Focused implicit/friction/source-memory tests | PASS | Bare-skin focused: 5 passed. Source-memory/focused Lane-D covered by final `ofe_routing` run. |
| Focused Lane-D / `ofe_routing` tests | PASS | Final-tree rerun: `cargo test -p openwepp-hillslope-orchestrator ofe_routing -- --nocapture`: `95 passed`, `1 ignored`, `247 filtered`, `151.43 s`. |
| Case-4 full-hybrid oracle ladder | PASS | Included in final focused `ofe_routing` run: `case4_hybrid_manning_ladder_meets_iwagaki_oracle ... ok`. |
| H2637 active hybrid timing/profile | PASS | `artifacts/h2637-active-hybrid-time.log` baseline and `artifacts/h2637-active-hybrid-after-effective-time.log` after exact evaluator. |
| Solve-cost counter before/after evidence | PASS | Map evaluations `151435969 -> 0`; user `38.39 s -> 33.37 s`; wall `0:38.41 -> 0:33.43`. |
| Fidelity/timing ratification audit | PASS-NO-PROMOTION | `artifacts/timing-and-fidelity.md` and `artifacts/ratification-audit.md`; active-output numeric dust ratified, selector remains experimental/unpromoted. |
| Protected-output byte identity | NOT APPLICABLE | No default/off selector surface touched; active hybrid outputs are explicitly not byte-identical and are audited instead. |
| Authority anti-evasion guard | PASS | Final-tree rerun: `bash tools/release/check_authority_suite_antievasion.sh`; required-suite guard `cargo nextest run --test auth11_required_suite_obligation_guards_contract`: 2 passed. |
| `git diff --check` | PASS | Final-tree rerun. |
| `cargo fmt --check` | PASS | Final-tree rerun. |
| `cargo clippy --workspace --all-targets -- -D warnings` | PASS | Final-tree rerun. |
| `cargo nextest run --workspace --profile full` | PASS | Final-tree rerun: `1438` tests run, `1438 passed`, `4 skipped`, `584.742 s`. |
| `cargo deny check` | PASS | Final-tree rerun: advisories, bans, licenses, sources OK. |
| `.rs` line-count governance | PASS-WARN | `kinematic_wave.rs` `2125` lines triggers WARN but not 3000-line mandatory split; rationale/follow-on recorded in `implementation.md`. |

Final verification artifacts:

- `artifacts/verification-final-gates.md` records the initial 14-command
  subagent batch.
- `artifacts/verification-final-tree-rerun.md` records the fresh final-tree
  rerun after review-fix tests and final docs edits.

# Gate Results

Status: complete
Evidence mode: ran

Final post-review gate root:

- `/tmp/hphys0291_final_gates_post_review_20260605T023206Z`

Required gates:

| Gate | Result | Log |
| --- | --- | --- |
| `cargo fmt --check` | `PASS rc=0` | `/tmp/hphys0291_final_gates_post_review_20260605T023206Z/cargo_fmt_check.log` |
| `cargo clippy --workspace --all-targets -- -D warnings` | `PASS rc=0` | `/tmp/hphys0291_final_gates_post_review_20260605T023206Z/cargo_clippy_workspace.log` |
| `cargo test --workspace` | `PASS rc=0` | `/tmp/hphys0291_final_gates_post_review_20260605T023206Z/cargo_test_workspace.log` |
| `cargo deny check` | `PASS rc=0` | `/tmp/hphys0291_final_gates_post_review_20260605T023206Z/cargo_deny_check.log` |
| `bash tools/release/check_authority_suite_antievasion.sh` | `PASS rc=0` | `/tmp/hphys0291_final_gates_post_review_20260605T023206Z/authority_suite_antievasion.log` |
| `cargo test --test auth11_required_suite_obligation_guards_contract -- --nocapture` | `PASS rc=0` | `/tmp/hphys0291_final_gates_post_review_20260605T023206Z/auth11_required_suite.log` |

Note:

- Ran: an earlier final gate attempt failed because HPHYS0289 source-level
  contract authority still expected flux-preferred routed melt access. The
  test was corrected to require flux-only routed melt, then all gates were
  rerun from scratch and passed.
- Ran: a post-review gate attempt at
  `/tmp/hphys0291_final_gates_post_review_20260605T022953Z` failed clippy for
  strict float equality in the new executable lifecycle test. The test was
  corrected to use tolerance checks, and the full gate set above was rerun
  from scratch and passed.

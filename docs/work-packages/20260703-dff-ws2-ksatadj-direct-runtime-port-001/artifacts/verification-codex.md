# WS-2 Verification Evidence

Evidence: Ran unless marked Static. Date: 2026-07-03.

## Focused Gates

| Command | Result |
|---|---|
| `cargo test -p openwepp-hillslope-orchestrator direct_ksatadj -- --nocapture` | PASS, 7 tests |
| `cargo test -p openwepp-runner active_ksatadj -- --nocapture` | PASS, 2 tests |
| `cargo test --test dff_ws2_ksatadj_direct_runtime -- --nocapture` | PASS, p313 direct-runtime fixture; manifest counter proved 2191 active `ksatadj` evaluator invocations for `ksatadj = 1` and 0 for the same fixture with only that flag disabled |
| `cargo test -p openwepp-runner compatibility_runtime_deletion_removes_obsolete_transition_modes -- --nocapture` | PASS, 1 test |
| `cargo test -p openwepp-runner r7 -- --nocapture` | PASS, 16 tests |
| Focused snow-density / Paradigm-2 source guards affected by the split | PASS after guard source-path updates |

## Closure Gates

| Command | Result |
|---|---|
| `cargo fmt --check` | PASS |
| `cargo clippy --workspace --all-targets -- -D warnings` | PASS |
| `cargo nextest run --workspace --profile full` | PASS: 1259 passed, 1 skipped, 1 slow; 590.283 s |
| `cargo deny check` | PASS: advisories, bans, licenses, sources ok |
| `git diff --check` | PASS |

## Doc Tooling

Static + Ran: `wctl`, `markdown-extract`, `markdown-edit`, and `uk2us` are
available on this host. `wctl doc-lint` is staged-file oriented in this
environment and reported `files_scanned: 0`, `errors: 0`, `warnings: 0` because
the worktree changes are not staged. Attempts to pass `--path` also reported `0`
files validated; no scoped Markdown lint result is available from this wrapper
for unstaged package files.

## Notes

- The first full nextest attempts failed only because source guards still read
  the pre-refactor monolith path. Those failures were fixed by updating guard
  source targets to the new split helper ownership.
- A first p313 on/off comparison showed the published HBP/loss outputs are
  byte-identical for this fixture when only `ksatadj` is disabled. The
  integration test therefore uses the run-manifest audit counter as the
  end-to-end branch-activation marker and does not claim p313 output magnitude
  sensitivity.
- No science-contract BEI-specific lint was run separately; `SC-SUBHYD-001`
  was amended as the governing science contract and the full workspace nextest
  includes the existing contract/lint integration targets.

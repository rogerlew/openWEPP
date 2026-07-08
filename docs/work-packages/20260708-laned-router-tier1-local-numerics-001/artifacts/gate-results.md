# Gate Results

Status: `EXECUTED-HOLD-APPROXIMATION-ENVELOPE`

Executed gates:

| Gate | Result |
|---|---|
| `git status --short --branch` at start | clean except branch ahead from scaffold commit |
| Contract pre-implementation gate | expected compile failure before implementation |
| `cargo test -p openwepp-hillslope-orchestrator ofe_routing::kinematic_wave --lib` | PASS, `26 passed` |
| `cargo test -p openwepp-hillslope-orchestrator ofe_routing::cascade --lib` | PASS, `6 passed` |
| `cargo test -p openwepp-hillslope-orchestrator ofe_routing::friction --lib` | PASS, `9 passed` |
| `cargo test -p openwepp-hillslope-orchestrator ofe_routing::d10b_reconciliation_tests --lib` | PASS, `11 passed`, `142.01 s` |
| `cargo nextest run --workspace --profile full --test laned_shadow_h2637` | PASS, `8 passed`, `2 skipped`, `26.423 s` |
| ignored active H2637 | PASS, `1 passed`, `9 skipped`, `447.438 s` |
| release runner build | PASS, hash `5b6788c795600d6329a46bb12b52f3c3107938ca29e5e3d0726cbf91075fa01e` |
| H2637 release timing | PASS, median `11.90 s` user, `3.15x` vs D15A active baseline |
| `perf stat -d` | PASS, `11.946992679 s` elapsed, IPC `2.15` |
| H2637 pre-change vs rev-47 delta | PASS, baseline worktree `46532c28`; runvol and peakro bit-identical; active outlet/storage/tail-fold deltas recorded |
| `.venv/bin/python tools/check_sc_binding_exposure.py docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md` | PASS-DEFERRED, `8` rows, `7` science-review follow-on rows |
| `bash tools/release/check_sc_unit_compliance.sh --path docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md` | PASS |
| `markdown-doc lint --path docs/work-packages/20260708-laned-router-tier1-local-numerics-001 --path docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md --path docs/work-packages/README.md --path docs/ROADMAP.md --format json` | PASS, `32` files scanned, `0` errors, `0` warnings |
| `git diff --check` | PASS |
| `cargo fmt --check` | PASS |
| `cargo clippy --workspace --all-targets -- -D warnings` | PASS |
| `cargo nextest run --workspace --profile full` | PASS, `1436 passed`, `3 skipped`, `582.999 s` |
| `cargo deny check` | PASS |

Comparator delegation:

- `comparator_suite_runner` was available and ran the first heavy H2637 gates.
  It found the original `NonFiniteState` blocker at lane 11/day 36; the parent
  fixed the local numerics and reran the H2637 gates to pass.

Review-driven reruns:

- The first full nextest rerun failed only
  `ofe_routing::cascade::tests::case3_vegetated_strip_backs_up_more_water_than_bare`
  because the old `1.005` depth-margin assertion overstated the rev-47
  vegetated-strip signal. The directional assertion was corrected to `1.001`,
  the focused cascade suite passed, and the full profile then passed.
- The ignored H2637 log and release-binary metadata log were regenerated after
  the final production code change; the earlier failed/stale local logs are
  superseded.
- Final verification B initially found this artifact omitted the required
  markdown-doc lint row; the scoped command was already run and is now recorded
  above.

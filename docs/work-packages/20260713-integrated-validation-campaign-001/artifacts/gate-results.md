# Gate Results

Status: `FAIL`

Evidence class: **Ran**

Frozen production source:
`f80a115148e75a08269eb14a8c1b0e7791ca891a`.
Pre-existing dirty files were confined to package evidence; no production,
test, contract, or fixture file was edited during delegated execution.

## Command Ledger

| ID | Command | Tests/result | Exit | Elapsed | Max RSS KB | Log | Status |
| --- | --- | --- | ---: | ---: | ---: | --- | --- |
| 00 | `bash tools/release/check_authority_suite_antievasion.sh` | authority anti-evasion PASS | 0 | `0:00.06` | 12,672 | `logs/00-anti-evasion.log` | PASS |
| 01 | `cargo nextest run --test auth11_required_suite_obligation_guards_contract` | 2/2 | 0 | `0:01.37` | 48,384 | `logs/01-auth11.log` | PASS |
| 02 | `cargo nextest run --test laned_shadow_h2637 --run-ignored ignored-only -E 'test(=h2637_native_active_owner_routes_and_closes)'` | 1/1; nine skipped | 0 | `7:21.69` | 196,060 | `logs/02-h2637-native-active-owner.log` | PASS |
| 03 | `cargo nextest run --test laned_shadow_h2637 h2637_active_fails_closed_without_routing_coefficients` | 1/1 | 0 | `0:01.14` | 48,000 | `logs/03-h2637-active-missing-coefficients.log` | PASS |
| 04 | `cargo nextest run --test laned_shadow_h2637 h2637_default_mixed_routing_coefficients_fails_closed` | 1/1 | 0 | `0:00.80` | 48,384 | `logs/04-h2637-mixed-coefficients.log` | PASS |
| 05 | `cargo nextest run --test laned_shadow_h2637 h2637_default_malformed_routing_coefficients_fails_closed` | 1/1 | 0 | `0:00.59` | 48,000 | `logs/05-h2637-malformed-coefficients.log` | PASS |
| 06 | `cargo nextest run --test erosion_single_ofe_p61_sediment` | 1/1 | 0 | `0:16.56` | 204,372 | `logs/06-erosion-p61.log` | PASS |
| 07 | `cargo nextest run --test erosion_multi_ofe_p102_chain` | 1/1 | 0 | `0:31.60` | 205,904 | `logs/07-erosion-p102.log` | PASS |
| 08 | `cargo nextest run --workspace --profile erosion` | 367/367; 1,580 skipped | 0 | `2:28.08` | 209,544 | `logs/08-erosion-profile.log` | PASS |
| 09 | `cargo nextest run --workspace --profile frost` | 320/320; 1,627 skipped | 0 | `9:17.45` | 198,528 | `logs/09-frost-profile.log` | PASS |
| 10 | `cargo nextest run -p openwepp-runner --test watershed_cli_behavior_contract wshedw7r_p102_sediment_active_fixture_publishes_nonzero_sediment_and_jobs_identity` | 1/1; 28 skipped | 0 | `0:43.76` | 742,388 | `logs/10-w7r-p102-publication.log` | PASS |
| 11 | `cargo nextest run -p openwepp-runner --test mt3_hbp_hourly_consumer_contract` | 7/7 | 0 | `0:07.82` | 458,224 | `logs/11-mt3-hourly-consumer.log` | PASS |
| 12 | `cargo nextest run -p openwepp-runner --test totalwatsed3_cli_contract` | 17/17 | 0 | `0:03.24` | 736,888 | `logs/12-totalwatsed3.log` | PASS |
| 13 | `cargo nextest run -p openwepp-watershed-orchestrator hourly_tests` | 30/30; 99 skipped | 0 | `0:00.69` | 45,312 | `logs/13-watershed-hourly-tests.log` | PASS |
| 14 | `cargo nextest run -p openwepp-runner` | 213/213 | 0 | `1:46.30` | 748,312 | `logs/14-runner-package.log` | PASS |
| 15 | `cargo nextest run -p openwepp-watershed-orchestrator` | 129/129 | 0 | `0:01.14` | 45,312 | `logs/15-watershed-package.log` | PASS |
| 16 | `bash tools/release/run_release_candidate_gates.sh` | broad test lane failed | 101 | `8:50.55` | 382,244 | `logs/16-release-candidate.log` | FAIL |

The `.time` record matching each log contains the full resource and exit data.
The release script ran with neither `--skip-stability` nor
`--skip-authority-required`.

## Release Failure

The default release candidate stopped in broad parallel `cargo test` execution
at `tests/integration/laned_shadow_h2637.rs`. These three tests unexpectedly
received a successful `HillslopeRunReport` instead of the required fail-closed
result:

- `h2637_active_fails_closed_without_routing_coefficients`;
- `h2637_active_and_disable_are_mutually_exclusive`; and
- `h2637_active_and_shadow_are_mutually_exclusive`.

The same frozen-source missing-coefficients test passed alone in command 03;
the other two conflicts are the known shared-environment family exposed by
broad parallel execution. That attribution does not convert a required release
gate to PASS. The script returned 101, its remaining release lanes did not run,
and no rerun-until-green or source fix was attempted.

## Blocked Closure Gates

The stop-on-real-nonzero instruction makes all subsequent Phase 6 commands
`BLOCKED`:

| Gate | Status | Reason |
| --- | --- | --- |
| `cargo fmt --check` | BLOCKED | release candidate exit 101 |
| `cargo clippy --workspace --all-targets -- -D warnings` | BLOCKED | release candidate exit 101 |
| `cargo nextest run --workspace --profile full` | BLOCKED | release candidate exit 101 |
| `cargo deny check` | BLOCKED | release candidate exit 101 |
| scoped `markdown-doc lint` | BLOCKED | release candidate exit 101 |
| `git diff --check` | BLOCKED | release candidate exit 101 |

Disposition: `HOLD-INTEGRATED-VALIDATION`. Restart requires a frozen source at
which the successor's exact pinned-input release lane passes without skip
flags.

## HOLD Evidence Gates

After recording the blocker and successor package, scoped Markdown lint passed
29 files with zero findings (`logs/17-hold-markdown.log`) and
`git diff --check` passed (`logs/18-hold-diff.log`). These validate the HOLD
evidence only; they do not replace the blocked Phase 6 closure gates.

After accepted review corrections and dual verification, the final scoped
`markdown-doc lint` command again passed 29 files with zero errors or warnings,
and `git diff --check` passed. These are terminal HOLD-document checks only;
the Phase 6 closure gates remain blocked.

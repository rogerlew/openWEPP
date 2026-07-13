# Close INTVAL Release Harness Process-Isolation Defect

Status: `QUEUED`

Package ID: `20260713-dc-intval-release-nextest-isolation-001`

Defect ID: `INTVAL-REL-001`

Execution mode: `package-end-to-end`

This DC-ExecPlan follows `docs/codex_exec_plans.md` and
`docs/defect_closure_execplans.md`. Maintain Progress, Surprises & Discoveries,
Decision Log, and Outcomes & Retrospective while executing.

## Purpose / Big Picture

Close the release gate defect that runs explicitly nextest-only H2637 selector
tests under threaded libtest. After correction, the default release candidate
command must preserve process-per-test environment isolation, pass the three
selector fail-closed cases, and continue through required authority/stability
lanes without weakening or skipping any gate.

## Progress

- [x] (2026-07-13 UTC) Reproduce `INTVAL-REL-001` in the frozen integrated-
  validation release run and bind the mechanism to libtest shared environment.
- [ ] Add a source-level regression that rejects release use of threaded
  `cargo test --workspace` for the canonical Rust closure lane.
- [ ] Correct the release script to use the canonical full nextest profile.
- [ ] Run the exact no-skip release candidate command with the pinned stability
  inputs through all lanes.
- [ ] Complete focused/full gates, dual review, dual verification, and terminal
  disposition.

## Surprises & Discoveries

- The H2637 source header already states the tests are nextest-only, but the
  release script still invokes `cargo test --workspace`. Three selector tests
  pass independently and fail together when process environment is shared.

## Decision Log

- Decision: correct the release orchestrator rather than serializing or
  weakening the H2637 tests.
  Rationale: nextest is repository-canonical and supplies the required
  process-per-test isolation; the production fail-closed assertions are valid.
  Date/Author: 2026-07-13 / Codex.

## Outcomes & Retrospective

Queued from integrated-validation HOLD. Record final correction and resume
commit here after execution.

## Correction Authority Envelope

Observed violation: default `bash tools/release/run_release_candidate_gates.sh`
exits 101 because `h2637_active_fails_closed_without_routing_coefficients`,
`h2637_active_and_disable_are_mutually_exclusive`, and
`h2637_active_and_shadow_are_mutually_exclusive` share Lane D selector
environment variables under libtest. The missing-coefficients case passed in a
separate nextest process earlier at the same source; source inspection binds
the same collision mechanism to the two mutual-exclusion failures.

In-scope writes are `tools/release/run_release_candidate_gates.sh`, its README,
a narrow source-level release contract test under `tests/integration/`, and
this package/roadmap/catalog. `tests/integration/laned_shadow_h2637.rs` is
read-only unless review proves its nextest-only contract is internally wrong.
Production crates, physics, contracts, selectors, fixtures, thresholds,
authority/stability lanes, and skip flags are protected.

Allowed correction is replacing the stale workspace libtest closure command
with `cargo nextest run --workspace --profile full`, preserving formatting,
Clippy, deny, release binary build/staging/lint, required authority, and
stability behavior. No serial workaround, ignored test, skip, retry-until-green,
or production selector change may carry closure.

## Conversion Rule And Seven-Gate Bar

Reproduction, mechanism, ownership, safety, testability, and validation are
already bounded. Repository governance makes nextest the canonical full gate,
and the H2637 source declares process isolation. Therefore execution must land
the direct release-script correction and may not HOLD while the in-envelope
edit and validation remain possible.

## Plan And Acceptance

First add a regression that reads the release script and requires the full
nextest command while rejecting `cargo test --workspace` as the Rust closure
lane. Then edit the script and README. Run the three H2637 tests together under
nextest, the regression, and this exact no-skip release command:

    bash tools/release/run_release_candidate_gates.sh \
      --cohort-seeds-csv /workdir/wepp-forest/docs/work-packages/20260503-wb05b-forest-hillslope-closure-sweep/artifacts/audits/_meta/defect_seeds.csv \
      --watchlist-csv /workdir/wepp-forest/docs/ablation/hillslope_watchlist.csv \
      --expect-suite wb05b_1166=1166 \
      --expect-suite release_gate_watchlist=19

The stability inputs are from `/workdir/wepp-forest` commit
`375ccc296ed1ea491f599ff1b1a25b415d494a2a`; their SHA-256 values are
`42b7d827d842ecbe75843175a80ab4f67a097784156658df8fb849161eb98958` and
`42214345a228d27a0536b771dd73068dc897d369f54cb8a197457dea675e26ab`,
respectively. Verify both before the release run and HOLD on mismatch. Finally
run formatting, all-target Clippy, full nextest, deny, Markdown, and diff
checks. Dual reviewers must confirm no release lane was removed or weakened;
dual verifiers must reproduce the source guard and inspect full release logs.

Acceptance is the unchanged release entry point, supplied only the exact pinned
stability inputs and expectations above, exiting zero through workspace,
binary, release-lint, required-authority, and stability lanes, with all three
H2637 selector cases passing in isolated processes. The integrated-validation
campaign must then restart in full at the correction commit.

## Subagent Authorization

Subagent requirement: **REQUIRED**. This package explicitly authorizes and
requires `comparator_suite_runner` for H2637, release, authority, stability,
and full gates, with compact logs/metrics written only under package artifacts.
It authorizes two independent reviewers and two independent verifiers to write
only their named artifacts.

## Security Impact Gate

Preserve argument arrays, fail-fast shell behavior, fixture verification,
authority enforcement, and release lint. No network, credential, path,
dependency, or production-runtime change is authorized.

# Codex Verification

Evidence mode: Static + Ran.

Static:
- Read `package.md`, package artifacts, `SC-OFEROUTE-001`, and the current Rust/test diff.
- Checked selector-test coverage against the operator-required matrix.
- Checked current artifact inventory against the package-required artifact list.

Ran:
- `git diff --check` -> PASS.
- `cargo fmt --check` -> PASS.
- `cargo nextest run --test laned_shadow_h2637` -> PASS: `8` tests run, `8` passed, `2` skipped.
- Earlier in this verification session, the ignored H2637 active-owner vector was run with `cargo nextest run --test laned_shadow_h2637 --run-ignored ignored-only h2637_native_active_owner_routes_and_closes` -> PASS: `1` test run, `1` passed, `8` skipped, wall `598.801s`.

## Findings

### High - Package completion artifacts and closure gates are incomplete

Paths:
- `docs/work-packages/20260708-laned-router-conditional-default-activation-001/package.md:145`
- `docs/work-packages/20260708-laned-router-conditional-default-activation-001/package.md:167`
- `docs/work-packages/20260708-laned-router-conditional-default-activation-001/package.md:174`
- `docs/work-packages/20260708-laned-router-conditional-default-activation-001/artifacts/disposition.md:8`
- `docs/work-packages/20260708-laned-router-conditional-default-activation-001/artifacts/final-disposition.md:3`

The package declares `EXECUTED-COMPLETE-CONDITIONAL-DEFAULT-ACTIVATION`, but `artifacts/gate-results.md` is still absent. I found no package artifact recording the required full closure loop: markdown/doc lint, contract/profile/BEI checks, line-count disposition, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo nextest run --workspace --profile full`, or `cargo deny check`.

Local line count also needs disposition: `crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/00_builders_and_authority.rs` is `2804` lines, which is in the repository WARN range.

### High - Finding disposition contradicts the checked-in review artifact

Paths:
- `docs/work-packages/20260708-laned-router-conditional-default-activation-001/artifacts/review-codex.md:17`
- `docs/work-packages/20260708-laned-router-conditional-default-activation-001/artifacts/review-codex.md:55`
- `docs/work-packages/20260708-laned-router-conditional-default-activation-001/artifacts/disposition.md:20`

`review-codex.md` still records high/medium findings and a `BLOCKED` verdict. Some of that review is now stale because later edits added evidence and fixed parts of the issue, but `disposition.md` says "No open findings" without accepting, rejecting, superseding, or otherwise reconciling the review findings. Package closure should not claim no open findings until the stale findings are explicitly dispositioned.

### Medium - Selector resolver lacks direct unit/contract-derived coverage

Paths:
- `docs/work-packages/20260708-laned-router-conditional-default-activation-001/package.md:157`
- `crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/00_builders_and_authority.rs:18`
- `tests/integration/laned_shadow_h2637.rs:546`

The required selector matrix is covered through the H2637 integration target, and the current focused target passes. However, the package specifically requires unit/contract-derived tests for the selector resolver. I found only fixture-level integration coverage for `DirectLanedActiveDefaultEligibility` and selector precedence. Either add direct resolver tests or record an explicit package disposition explaining why the integration fixture is the accepted substitute.

## Selector Coverage

- All-coeff default active: covered by ignored acceptance test `h2637_native_active_owner_routes_and_closes` at `tests/integration/laned_shadow_h2637.rs:575`; package evidence records matching default-active and explicit-active hashes.
- No-coeff fallback inactive: covered by `h2637_legacy_shadow_fails_closed_without_routing_coefficients` at `tests/integration/laned_shadow_h2637.rs:299`.
- Mixed fail closed: covered by `h2637_default_mixed_routing_coefficients_fails_closed` at `tests/integration/laned_shadow_h2637.rs:546`.
- Explicit active missing-coeff fail closed: covered by `h2637_active_fails_closed_without_routing_coefficients` at `tests/integration/laned_shadow_h2637.rs:429`.
- Explicit disable: covered by the ignored acceptance test at `tests/integration/laned_shadow_h2637.rs:575`.
- Active+shadow conflict: covered by `h2637_active_and_shadow_are_mutually_exclusive` at `tests/integration/laned_shadow_h2637.rs:464`.
- Active+disable conflict: covered by `h2637_active_and_disable_are_mutually_exclusive` at `tests/integration/laned_shadow_h2637.rs:495`.
- Additional malformed coefficient fail-closed coverage exists at `tests/integration/laned_shadow_h2637.rs:562`.

## Non-Blocking Debt / Follow-Ups

- Keep the default-active H2637 vector ignored if runtime cost requires it, but preserve explicit package evidence for the ignored run because the normal focused target skips it.
- The current `consumer-path-proof.md` is mostly static plus manifest counters. That is acceptable as supporting evidence, but full closure still depends on the missing gate-results artifact and full gate loop above.

## Verdict

BLOCKED for package completion. The current selector tests cover the requested cases and the focused integration target passes, but package closure is not acceptable until missing gate evidence is recorded and the checked-in review findings are explicitly dispositioned.

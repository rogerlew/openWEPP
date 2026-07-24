# TESTGATE Science Gate Proportionality

Package ID: `20260724-testgate-science-gate-proportionality-001`

Status: `SUPERSEDED / MERGED-INTO-ROADMAP-ORDER-2`

## Objective

Align executable TESTGATE selection with ADR-0041 and the canonical testing
strategy so ordinary science increments run affected correctness gates and
record quality as `DEFERRED_TO_QUALITY_CI`.

## Rationale

The policy currently escalates an unmapped Rust file in a science-sensitive
package to `CRITICAL`, admits empirical/migration fixtures to the deterministic
full profile, runs workspace doctests more broadly than affected public
surfaces require, and represents required authority as one all-suite command.
Those implementation choices contradict the lifecycle separation already
required by `docs/standards/testing-and-gate-strategy.md`.

## Intended Write Set

- `.config/nextest.toml`
- `Cargo.toml`
- `crates/openwepp-gate-planner/**`
- `gate-policy/v1/**`
- `tests/integration/testgate_align_authority_contract.rs`
- `tests/integration/testgate_ci_executor_contract.rs`
- `tools/release/**`
- `docs/work-packages/20260724-testgate-science-gate-proportionality-001/**`
- `docs/work-packages/README.md`

## Included Scope

- Map ordinary science package paths to bounded or integrated-domain ownership
  while retaining exact critical triggers and unknown-path escalation.
- Keep touched A0/A1/A3, contract, conservation, consumer, serialization, and
  publication obligations non-deferrable.
- Remove empirical/external-observation and migration-development cohorts from
  routine deterministic full regression and expose explicit manual/domain
  profiles for them.
- Select affected doctests at increment scope and retain workspace doctests at
  campaign/release scope.
- Run dependency-policy checks at increment scope only for dependency,
  manifest, lockfile, toolchain, or deny-policy changes.
- Select required authority suites by the affected process binding rather than
  executing unrelated suites.
- Remove the retired combined-quality node/input from TESTGATE.

## Excluded Scope

- Weakening applicable science contracts, hard invariants, authority suites,
  conservation checks, or consumer-path evidence.
- Changing scientific equations, fixtures, expected values, or production
  runtime behavior.
- Weakening campaign/release full-workspace correctness obligations.

## Acceptance

- Representative ordinary science changes plan as `BOUNDED_COMPONENT` or
  `INTEGRATED_DOMAIN` with typed `DEFERRED_TO_QUALITY_CI`.
- Every mapped process change retains its applicable A0/A1/A3 and explicit
  integration/conservation/consumer obligations.
- Known critical triggers remain `CRITICAL`; unknown paths remain fail-closed.
- Routine deterministic full regression excludes only explicitly classified
  manual/domain cohorts, each reachable through a named profile and trigger.
- Increment doctest and dependency-policy selection follows the canonical
  lifecycle standard; campaign/release selection remains complete.
- Authority inventory is exact and process-scoped.
- Planner/source anti-evasion tests, profile contract tests, focused planner
  tests, Clippy, and the exact terminal plan pass before disposition.
- Changed-head TESTGATE qualification is owned by roadmap Order 6.

## Implementation Intent

Risk is `CRITICAL` because this package changes gate selection, filtering, and
authority lanes. Cheap policy/schema/anti-evasion checks run before full
correctness regression. The package must retain negative tests proving that deleted,
unknown, security-boundary, global build, coverage-policy, and test-filter
changes cannot be downgraded.

## ADR-0041 Prospective Disposition

Roadmap Order 2 owns this package's remaining executable scope. No historical
attempt, receipt, or evidence is relabeled. This package must not be dispatched
or independently resumed.

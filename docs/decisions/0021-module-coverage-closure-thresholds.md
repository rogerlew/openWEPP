# ADR-0021: Module coverage closure thresholds are binding policy

**Status:** Accepted
**Date:** 2026-06-14 UTC
**Deciders:** Roger Lew, Codex
**Author of draft:** Claude Code (drafted and ratified at decider Roger Lew's direction, 2026-06-14)
**Builds on:** [ADR-0011](0011-architecture-first-top-down-science-contracts.md)
**Authoring authority:** [docs/standards/module-test-enhancement-authoring-guide.md](../standards/module-test-enhancement-authoring-guide.md), [docs/standards/rust-scientific-coding-standard.md](../standards/rust-scientific-coding-standard.md) §7.5–7.8

## Context

A full-workspace `cargo llvm-cov` run put the engine at 71.9% line / 75.2%
region coverage. The decisive finding was not the aggregate: the erosion-phase
modules (`hydrology_phase_erod13/14/19`) carried fully-specified contract
Test-Vector Obligations yet sat at ~1% line coverage. Obligations written as
prose, with no coverage gate binding them to implemented tests, let entire
guard/reject families go untested while the contract read as complete.

The QA standard now hoists the case-family taxonomy and the obligation-to-test
binding (§7.5–7.8), and the module test-enhancement guide defines a closure
procedure with a coverage threshold. But those documents stated the thresholds
as a "ratifiable default." A default is not a gate: a test-enhancement package
could raise coverage by an arbitrary amount and call it closure, and review had
no objective bar. This ADR makes the thresholds binding so that closure means a
fixed thing and the obligation binding cannot be skipped.

Line coverage alone is the wrong gate here. It rated the `erod*` modules far
above their real state because it does not see that a guard's reject arm never
executed. Region coverage counts each branch — which is exactly where the §7.5
domain-reject / missing-symbol / non-finite / fail-closed obligations (families
D–H) live. The metric must be branch-sensitive or it will certify the precise
failure that prompted this ADR.

## Decision

1. **Adopt the module coverage closure thresholds as binding policy.** Measured
   on the eligible surface (Decision 3):
   - **Science tier** (kernel / contract-bearing / conservation-law modules):
     **≥ 90% region AND ≥ 90% line.**
   - **Glue tier** (parser / orchestration-runner / IO-adapter / output):
     **≥ 85% region AND ≥ 85% line.**
   - **Per-function floor:** no eligible function below **75% region** without a
     documented `// COVERAGE-EXCLUDE` justification.

   Region is the binding metric; line is reported and gated alongside it.

2. **The obligation-to-test binding is a non-waivable gate, independent of the
   percentage.** 100% of a module's applicable contract Test-Vector Obligation
   families (§7.5 A–H) must be implemented and listed in the obligation→test
   map. A percentage pass does not substitute for the obligation binding, and an
   obligation pass does not substitute for the percentage.

3. **Eligible-surface exclusions are a closed list and are reviewed, never
   self-asserted** (guide §3): binary entrypoints, `#[derive]` output,
   observability-only formatting arms, non-default `#[cfg(feature)]` paths, and
   type-impossible arms annotated `// COVERAGE-EXCLUDE`. Shrinking the
   denominator to reach a number is non-conforming.

4. **Tier assignment.** A module is science-tier if it carries kernel math, a
   contract invariant, or a conservation identity; otherwise glue-tier. An
   ambiguous module defaults to **science-tier** (the stricter bar).

5. **Scope, floor, and ratchet.** The thresholds bind module test-enhancement
   packages and any package that adds or materially changes a module's tests.
   They are a floor, not a ceiling, and are **non-regressive**: a package may not
   drop a module already above its tier bar back below it. They are **not** a
   global per-PR line-coverage gate — the engine-wide gate philosophy stays
   pass/fail on contract suites and authority lanes per the correctness-authority
   model.

6. **Tuning authority.** A tier percentage may be changed only by a superseding
   ADR. The obligation binding (Decision 2) is not tunable.

## Consequences

- A module test-enhancement package can no longer close below its tier bar or
  with unbound obligations. "Raised coverage somewhat" is not closure;
  `HOLD` is permitted only as an owned, scoped boundary (guide §6).
- The `erod*` pattern — specified-but-unimplemented obligations — becomes
  non-conforming the moment such a module is taken up by a test-enhancement
  package. The contract bar and the coverage bar now converge.
- This does **not** retroactively fail existing modules or block unrelated PRs.
  It governs test-enhancement packages and test-affecting changes going forward;
  it is not a new CI line-coverage threshold on every commit.
- **Enforcement wiring.** Until the §7.6 machine guard (binding ordinary `SC-*`
  obligations to tests, modeled on `auth11_required_suite_obligation_guards`)
  ships, the thresholds and the obligation binding are enforced as a package
  exit gate (guide §6) and a review-checklist item (standard §7.4#6). The guard,
  once built, is the automated enforcement of Decision 2.
- Consistent with ADR-0011: the obligations are SC-* contract-derived, so this
  ADR adds an enforcement floor without introducing a new correctness authority.

## Citations

- `docs/standards/module-test-enhancement-authoring-guide.md` §2–3 (thresholds,
  exclusions, procedure).
- `docs/standards/rust-scientific-coding-standard.md` §7.5–7.8 (case-family
  taxonomy, obligation-to-test binding, authoring mechanics, non-kernel
  families).
- `docs/specifications/correctness-authority-model.md` (lane/failure-class
  model; "test laws, not parity numbers").
- ADR-0011 (SC-* contract is the correctness authority).
- Coverage assessment, 2026-06-13: workspace 71.9% line / 75.2% region;
  `hydrology_phase_erod13/14/19` ~1% line with fully-specified obligations.

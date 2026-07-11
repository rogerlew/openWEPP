# ADR-0021: Module coverage and complexity-risk closure thresholds are binding policy

**Status:** Accepted
**Date:** 2026-06-14 UTC
**Deciders:** Roger Lew, Codex
**Author of draft:** Claude Code (drafted and ratified at decider Roger Lew's direction, 2026-06-14)
**Amendment:** Per-function CRAP ≤ 30 complexity-risk bound (Decision 6) added 2026-06-14, same day, at decider Roger Lew's direction.
**Amendment:** Eligible-surface classification tightened 2026-07-11 at decider
Roger Lew's direction; thresholds are unchanged.
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

Coverage is also necessary but not sufficient. A `cargo-crap` (CRAP, Change Risk
Anti-Patterns) scan surfaced a cluster of functions at cyclomatic complexity
112–165 (`parse_layout`, `compute_active_frost_coupling`, `run_erod14_wave2`,
`run_lateral_transfer`, `run_runoff_reconciliation`, and others). A
fully-covered function of complexity 130 is still high change-risk: coverage
alone does not bound it. CRAP combines the two — `CC² · (1 − cov)³ + CC` — and
at full coverage collapses to cyclomatic complexity, so it rewards the only
durable fix for those functions: decomposition into smaller, individually
testable units. It fingers exactly the high-risk kernels and is the natural
complement to the coverage floor.

## Decision

1. **Adopt the module coverage closure thresholds as binding policy.** Measured
   on the eligible surface (Decision 3):
   - **Science tier** (kernel / contract-bearing / conservation-law modules):
     **≥ 90% region AND ≥ 90% line.**
   - **Glue tier** (parser / orchestration-runner / IO-adapter / output):
     **≥ 85% region AND ≥ 85% line.**
   - **Per-function floor:** no eligible function below **75% region** without
     an accepted `R-OBSERVABILITY` or `R-INFRASTRUCTURE` disposition under
     Decision 3. `// COVERAGE-EXCLUDE` is reserved for `X-IMPOSSIBLE` arms.

   Region is the binding metric; line is reported and gated alongside it.

2. **The obligation-to-test binding is a non-waivable gate, independent of the
   percentage.** 100% of a module's applicable contract Test-Vector Obligation
   families (§7.5 A–H) must be implemented and listed in the obligation→test
   map. A percentage pass does not substitute for the obligation binding, and an
   obligation pass does not substitute for the percentage.

3. **Eligibility is classified at symbol or arm granularity, defaults to
   eligible, and is reviewed, never self-asserted** (guide §3). A file or module
   is not excluded merely because it is a binary, parser, error module, adapter,
   or formatter host.

   | Class | Treatment | Closed-list meaning |
   | --- | --- | --- |
   | `E-SCIENCE` | aggregate + 75% floor + CRAP | process math, contract invariants, conservation, numerical guards |
   | `E-PRODUCTION` | aggregate + 75% floor + CRAP | accepted-input decisions, state/control flow, ordering, error precedence, serialization, consumer/publication behavior |
   | `R-OBSERVABILITY` | aggregate retained; reviewed per-function floor/CRAP exception allowed | pure `Display`/`Debug` text with no machine-read code, state change, validation, or control effect |
   | `R-INFRASTRUCTURE` | aggregate retained; reviewed 75% floor exception allowed, CRAP remains binding | low-complexity dependency-origin failure arms not deterministically selectable through the public boundary after normal/error mapping is covered |
   | `X-GENERATED` | denominator excluded | compiler/derive/generated code with no hand-authored source branch |
   | `X-NONDEFAULT-CFG` | denominator excluded only from the measured profile | code not compiled in that profile; becomes eligible whenever that feature/configuration is in package scope or a shipping gate |
   | `X-DELEGATING-MAIN` | denominator excluded | literal branch-free `main` that only delegates to a covered/tested runner; argument parsing and command behavior remain eligible |
   | `X-IMPOSSIBLE` | denominator excluded | type-impossible arm proven unreachable by construction and annotated `// COVERAGE-EXCLUDE` |
   | `R-IRREDUCIBLE-CRAP` | aggregate + coverage floor retained; reviewed CRAP exception only | minimum-decomposed domain branching whose further split would obscure the authoritative decision table |

   The following are always eligible when hand-authored: parser grammar and
   cardinality decisions; validation and fail-closed guards; typed error codes
   or messages consumed by machines; state transitions; key/domain/order
   selection; numerical boundary handling; output schema/value mapping; and
   real consumer handoffs. Mixed-purpose functions are wholly eligible; extract
   a pure shell before requesting an exception.

   Every non-eligible or retained-exception row records the exact symbol/arm,
   class, source lines, denominator treatment, semantic-impact analysis,
   exercised public behavior, and two independent reviewer dispositions.
   Wildcard, module-wide, name-pattern-only, inherited, or “hard to test”
   exclusions are non-conforming. An old disposition must be revalidated when
   its source hash, role, complexity, or public behavior changes. Shrinking the
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

6. **Per-function complexity-risk bound (CRAP ≤ 30).** Adopt the CRAP metric
   (`CC² · (1 − cov)³ + CC`) at the conventional threshold **30** as a
   per-function closure condition: every eligible function in a module under a
   test-enhancement package scores CRAP ≤ 30 (`cargo-crap`, LCOV from the same
   llvm-cov run). Because CRAP = CC at full coverage, a function above 30 is
   reduced by **behavior-preserving decomposition, landed test-first** (the
   coverage gate is the safety net) — never by adding tests alone. That
   decomposition is implementation work (Codex; the mechanical-refactor guide),
   sequenced after characterization coverage. Repo-wide adoption uses a
   `cargo-crap` **baseline**: functions outside a module under active enhancement
   are held no-regression and burned down by future packages, so the gate is
   adoptable against the current high-complexity backlog without blocking
   unrelated work.
7. **Tuning authority.** A coverage tier percentage or the CRAP threshold may be
   changed only by a superseding ADR. The obligation binding (Decision 2) is not
   tunable.

## Consequences

- A module test-enhancement package can no longer close below its tier bar or
  with unbound obligations. "Raised coverage somewhat" is not closure;
  `HOLD` is permitted only as an owned, scoped boundary (guide §6).
- The `erod*` pattern — specified-but-unimplemented obligations — becomes
  non-conforming the moment such a module is taken up by a test-enhancement
  package. The contract bar and the coverage bar now converge.
- Closing a module with a CC > 30 function (e.g. `run_erod14_wave2`, CC 131) now
  entails decomposing it, test-first. This deliberately enlarges some
  test-enhancement packages into a cover-then-decompose arc — or splits them
  into a coverage package and a follow-on mechanical-refactor package — rather
  than letting a fully-covered monster function pass as closed.
- This does **not** retroactively fail existing modules or block unrelated PRs.
  It governs test-enhancement packages and test-affecting changes going forward;
  it is not a new CI line-coverage threshold on every commit.
- Raw CRAP rankings are discovery evidence, not the actionable queue. Nightly
  selection classifies every row above 30 before ranking, removes only accepted
  exceptions from actionable excess, publishes both raw and actionable counts,
  and continues down the ranking until the requested number of eligible modules
  is selected. A module with only accepted exception rows is `NO-ACTION`, not a
  forced coverage package.
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
- `cargo-crap` 0.2.2 complexity scan, 2026-06-14: cyclomatic complexity 112–165
  on `parse_layout`, `compute_active_frost_coupling`, `run_erod14_wave2`,
  `build_hillslope_pl_runtime_surfaces_from_management`,
  `ws20_route_case12_segment_family`, `run_lateral_transfer`,
  `execute_hillslope_run`, `run_runoff_reconciliation` — above any CRAP
  threshold at full coverage; reducible only by decomposition.
- `.cargo-crap.toml` pins the ratified threshold (30).

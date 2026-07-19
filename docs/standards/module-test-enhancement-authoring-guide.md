# Module Test-Enhancement Work-Package Authoring Guide

Status: Active
Last updated: 2026-07-11

Scope: how to scope and execute a work package that raises a particular Rust
module to closure-grade test coverage. Companion to
[kernel-work-package-preparation.md](kernel-work-package-preparation.md) (which
covers kernel/science change packages); this document covers the narrower
test-enhancement package shape that takes one already-built module from
measured-undercovered to closure.

## Relation to standards

This guide is the execution procedure and closure threshold. The substance it
enforces is defined in
[rust-scientific-coding-standard.md](rust-scientific-coding-standard.md):

- §7.5 — test case-family taxonomy (A–H).
- §7.6 — obligation-to-test binding.
- §7.7 — test-authoring mechanics (naming, fixtures, property tests, float
  comparison, determinism).
- §7.8 — non-kernel layer case families (parser, scheduler/runner, output).

Do not restate those here; reference them.

## 1) When to use this shape

Use a module test-enhancement package when:

- a module measures below the closure threshold (§2) in a coverage run; or
- a contract's Test-Vector Obligations are unbound (no implemented test, per
  §7.6); or
- a layer's case families (§7.5/§7.8) are demonstrably incomplete.

One package targets **one module** — a crate-internal path coherent enough for a
single agent to take end-to-end without intervention. Do not bundle unrelated
modules; do not fragment a single module across packages. If the module also
needs a physics/contract change, that is a kernel package (different guide), not
a test-enhancement package.

## 2) Closure threshold (Normative)

Coverage closure for the targeted module is met when **all** of the following
hold:

1. **Obligation binding (hard, percentage-independent).** 100% of the module's
   applicable contract Test-Vector Obligation families (§7.5 A–H) and named
   obligations (§7.6) are implemented and listed in the obligation→test map.
2. **Aggregate coverage on the eligible surface (§3):**
   - **Science tier** — kernel, contract-bearing, or conservation-law modules:
     **≥ 90% region AND ≥ 90% line.**
   - **Glue tier** — parser, orchestration/runner, IO-adapter, output modules:
     **≥ 85% region AND ≥ 85% line.**
3. **Per-function coverage floor.** No eligible function below **75% region**
   without an accepted `R-OBSERVABILITY` or `R-INFRASTRUCTURE` disposition from
   §3. A source comment alone does not grant an exception. (Prevents a green
   aggregate from hiding a wholly untested function.)
4. **Per-function complexity-risk bound (CRAP ≤ 30).** Every eligible function in
   the module scores **CRAP ≤ 30** under `cargo-crap` (LCOV from the same
   llvm-cov run; threshold 30). Repository closure uses the adjudicated gate,
   not raw `--fail-above`, because reviewed retained rows remain visible in the
   denominator. CRAP is
   `CC² · (1 − cov)³ + CC`, so at full coverage it collapses to cyclomatic
   complexity: a function above the bound is reduced by **decomposition, not by
   adding tests**. Decomposition is behavior-preserving and lands **test-first**
   — the §2.2–2.3 coverage is the safety net for the split — so a module whose
   offenders need decomposition either bundles a behavior-preserving refactor
   after its tests, or splits closure across a test-enhancement package (coverage
   + obligations) and a follow-on mechanical-refactor package (CRAP), per the
   single-authority sizing in §1.
5. **Eligible surface** = module lines minus the documented exclusions in §3.

**Region coverage is the primary metric, not line.** A module can show 90% line
while a guard's reject arm never executes; region counts each branch/guard arm,
which is exactly where the §7.5 D–H obligations live. Report both; gate on both;
treat region as the binding one.

### Why 90% / 85% — and not 80% or 100%

- This is conservation-law code whose guard and reject arms **are** the contract
  (§7.5 D–H). An 80% bar would bless leaving entire guard families untested —
  the exact failure mode already observed, where erosion-phase obligations were
  fully specified yet the modules sat at ~1% line coverage. 90% region forces
  the enumerated arms to be exercised.
- 100% is rejected: it forces test theater on derive/`Display`/unreachable arms.
  The exclusion mechanism (§3) handles those honestly instead of inflating the
  denominator.
- Glue tier gets 85% because parser/IO layers carry more legitimately defensive
  and integration-only paths; the obligation binding (§2.1) still applies in
  full to any contract they implement.

These thresholds are **binding policy**, ratified by
[ADR-0021](../decisions/0021-module-coverage-closure-thresholds.md). A tier
percentage may be changed only by a superseding ADR; the obligation binding
(§2.1) is not tunable.

## 3) Eligibility classification

Classify each hand-authored source symbol or arm before subtracting anything
from a denominator or actionable CRAP ranking. Eligibility defaults to
`E-PRODUCTION`; classification is granted by review, not asserted by the
implementer. A module path, filename, trait, or naming pattern is never enough.

| Class | Aggregate denominator | 75% function floor | CRAP ≤30 | Required proof |
| --- | --- | --- | --- | --- |
| `E-SCIENCE` | included | required | required | science/contract/closure obligation map |
| `E-PRODUCTION` | included | required | required | public behavior and applicable A–H map |
| `R-OBSERVABILITY` | included | reviewed exception | reviewed exception | pure human-facing formatting; no machine consumer, branching side effect, state, validation, or control use |
| `R-INFRASTRUCTURE` | included | reviewed exception | required | public success/error mapping covered; only nondeterministic dependency-origin failure arms remain; low complexity |
| `R-LOW-COMPLEXITY-PRODUCTION` | included | reviewed exception | required | preclassified CLI/parser/validation/error/glue; CRAP ≤30; named A–H/error-priority and subprocess-consumer evidence; exact debt row; dual review |
| `X-GENERATED` | excluded | not applicable | not applicable | no hand-authored executable branch |
| `X-NONDEFAULT-CFG` | excluded from this profile | not applicable in this profile | not applicable in this profile | exact cfg absent from measured profile; feature becomes eligible when scoped/shipped |
| `X-DELEGATING-MAIN` | excluded | not applicable | not applicable | literal branch-free delegation; runner/CLI process behavior covered elsewhere |
| `X-IMPOSSIBLE` | excluded | not applicable | not applicable | construction proof, exact arm, reviewer signature, `// COVERAGE-EXCLUDE` annotation |
| `R-IRREDUCIBLE-CRAP` | included | required | reviewed exception | minimum-decomposition proof and authoritative decision-table rationale |

Always classify these as eligible when hand-authored:

- parser grammar, record cardinality, alias priority, and normalization;
- validation, domain rejection, null/non-finite handling, and fail-closed paths;
- typed error codes/messages read by machines and error-precedence selection;
- state transitions, key construction, ordering, deduplication, and domain
  selection;
- numerical boundary handling, accumulation, normalization, and units;
- schema/value serialization, publication, and real-consumer handoffs.

Eligibility does not forbid the narrow
`R-LOW-COMPLEXITY-PRODUCTION` floor disposition. It may be applied only to an
individually reviewed low-complexity `E-PRODUCTION` function after its raw row
remains in the aggregate denominator and debt ledger. It never applies to
science or conservation math, numerical guards, schema/value/publication
arithmetic, or a mixed function whose uncovered branch can alter accepted-input
state or output without named direct evidence. Record exact region counts,
CC/CRAP, uncovered branches, error-priority tests, subprocess consumer IDs,
same-source non-regression, and two reviewer dispositions.

If one function mixes an observability/delegation shell with eligible behavior,
the whole function is eligible. Extract the pure shell behavior-preserving
before requesting a narrower exception.

Every `R-*` or `X-*` entry in `coverage-closure.md` must record:

1. stable classification ID and exact symbol/arm plus source lines;
2. source SHA-256 and measured profile;
3. aggregate, function-floor, and CRAP treatment;
4. why the code cannot affect science, accepted input, state/control, machine
   errors, serialization, publication, or consumer behavior;
5. tests exercising the reachable public behavior;
6. independent Review A and Review B dispositions.

No wildcard or module-wide exclusions are allowed. “Hard to test,” “owned by a
dependency,” low current coverage, and prior acceptance are not sufficient.
Revalidate an old disposition whenever the source hash, semantic role,
complexity, or public behavior changes.

## 4) Procedure

1. **Authorize and scope.** One module; confirm tier (science vs glue); name the
   package `YYYYMMDD-mte<NN>-<module-slug>-001` under `docs/work-packages/` and
   register it in `docs/work-packages/README.md`. Scaffold per
   [kernel-work-package-preparation.md](kernel-work-package-preparation.md) §2
   (truthfulness labels, dual review/verification where the package requires).
2. **Baseline measurement.** Pin the before-numbers as evidence:
   - `cargo llvm-cov --workspace --ignore-run-fail --json --output-path <artifacts>/coverage_before.json`
   - Use the **one-shot** form above. The standalone `cargo llvm-cov report`
     subcommand cannot reconstruct the object-file list and reports all zeros;
     do not use it for these packages. `--ignore-run-fail` tolerates known
     preexisting red tests so they do not abort the coverage merge — attribute
     any such failure in evidence (it must not be one this package introduced).
   - Filter the JSON to the module's files; record per-file region/line/function
     and the eligible-surface denominator after §3 exclusions.
   - Emit LCOV for CRAP from the same run:
     `cargo llvm-cov --workspace --ignore-run-fail --lcov --output-path <artifacts>/lcov.info`,
     then `cargo crap --workspace --lcov <artifacts>/lcov.info --threshold 30 --format markdown`
     → `<artifacts>/crap_before.md`. Record every eligible function in the module
     with CRAP > 30. Refresh the repo CRAP baseline so functions outside this
     module are held no-regression, not re-litigated here.
3. **Gap classification.** For each uncovered region, assign exactly one bucket:
   missing case family (→ author per §7.5/§7.8), unbound obligation (→ §7.6),
   eligible production gap, retained reviewed exception, or denominator
   exclusion (→ §3 with exact classification evidence).
4. **Author tests.** Codex authors. One row/test per case family; property tests
   for range-invariants (§7.7). Assert typed status/error codes and the
   conservation identity — not parity numbers. If an eligible function exceeds
   CRAP 30, land its characterization tests **first** here, then decompose it
   behavior-preserving (same package or a follow-on mechanical-refactor package,
   §2.4); never attempt to satisfy CRAP by coverage alone (impossible above
   CC 30).
5. **Build the obligation→test map.** Table: family/obligation → test fn(s) →
   status. 100% of applicable families bound.
6. **Re-measure.** Emit `coverage_after.json`; confirm §2 thresholds on the
   eligible affected surface and no per-function floor breach. Run the
   terminal-plan CRAP gate; confirm every affected eligible function is at most
   30 or has an exact current adjudication. Critical, campaign, and release
   boundaries also require an empty workspace actionable set.
7. **Gate loop.** Execute the exact terminal plan under
   `testing-and-gate-strategy.md`, including formatting, affected
   warnings-denied Clippy and tests, doctest/inventory checks, and the obligation
   guard
   (`auth11_required_suite_obligation_guards_contract`) where the module carries
   external-authority suite bindings. Critical, campaign, release, and explicit
   rollback boundaries retain the conservative full workspace, cargo-deny, and
   global-CRAP loop.
8. **Evidence and disposition.** Disposition states before→after coverage, the
   eligible-surface definition, the exclusion list, the obligation→test map, and
   the threshold pass.

## 5) Required evidence artifacts

- `coverage_before.json` / `coverage_after.json` — raw llvm-cov export.
- `lcov.info` — LCOV used for CRAP.
- `crap_before.md` / `crap_after.md` — `cargo-crap` report (markdown) with the
  module's eligible functions and their CRAP scores before/after.
- `coverage-closure.md` — raw and eligible-adjusted before/after tables,
  classification ledger with exact symbols/lines/hashes and reviewer
  dispositions, tier + thresholds, pass/fail.
- `obligation-to-test-map.md` — family/obligation → test fn → status; 100%
  bound.
- `gate-logs/` — per the gate loop (§4.7).
- Standard package scaffolding per the kernel preparation guide.

## 6) Exit criteria

- Closure threshold (§2) met on the eligible surface, **or** an explicit, owned,
  scoped `HOLD` naming the residual, the owner/follow-on package, and why
  closure is not yet possible. An ownerless or unscoped `HOLD` is a governance
  failure.
- Obligation binding (§2.1) at 100%.
- All exclusions justified and reviewed.
- Gate loop green, or any preexisting failure attributed and shown not to be
  introduced by this package.

## 7) Anti-patterns

- **Coverage theater.** Asserting trivially-true facts (a constant, a getter) to
  lift the percentage while the law goes untested — reviewer rejects.
- **Substring weakening.** Broadening a failing assertion to a vague substring to
  make it pass (a `tests/AGENTS.md` pitfall); move the assertion to the canonical
  authority instead.
- **Exclusion abuse.** Marking a guard, parser, error-precedence, state,
  serialization, or publication arm excluded to dodge its test. An annotation
  is valid only for `X-IMPOSSIBLE`; it does not exempt ordinary defensive code.
- **Denominator gaming.** Counting an excluded binary `main` as covered, or
  shrinking the eligible surface to hit the number rather than testing the code.
- **CRAP-by-coverage.** Trying to push a high-CC function under CRAP 30 with
  tests alone — mathematically impossible (CRAP ≥ CC at full coverage) and a
  coverage-theater magnet. Reduce complexity by decomposition.
- **Suppression abuse.** `--allow`-ing a function that merely grew, instead of
  decomposing it. Suppressions are for irreducible domain branching only.

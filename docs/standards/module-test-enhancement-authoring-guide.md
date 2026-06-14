# Module Test-Enhancement Work-Package Authoring Guide

Status: Active

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
3. **Per-function floor.** No eligible function below **75% region** without a
   written `// COVERAGE-EXCLUDE` justification. (Prevents a green aggregate from
   hiding a wholly untested function.)
4. **Eligible surface** = module lines minus the documented exclusions in §3.

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

## 3) Documented exclusions

Lines removed from the eligible denominator. Each exclusion is justified in the
package evidence and reviewed — exclusions are granted, not self-asserted.

1. Binary entrypoints (`src/bin/*.rs` `main`, argument-parse shells) — exercised
   by CLI/process integration tests, not unit coverage.
2. `#[derive(...)]`-generated code.
3. Observability-only `Display`/`Debug`/formatting arms with no branching logic.
4. `#[cfg(feature = ...)]` paths not built in the default test profile (note
   them; cover under the feature if/when it ships in the default gate).
5. Arms annotated `// COVERAGE-EXCLUDE: unreachable by construction — <reason>`.
   Must be rare and reviewer-signed. A type-impossible `else`/`unreachable!`
   qualifies; "we did not get to it" does not.

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
     pre-existing red tests so they do not abort the coverage merge — attribute
     any such failure in evidence (it must not be one this package introduced).
   - Filter the JSON to the module's files; record per-file region/line/function
     and the eligible-surface denominator after §3 exclusions.
3. **Gap classification.** For each uncovered region, assign exactly one bucket:
   missing case family (→ author per §7.5/§7.8), unbound obligation (→ §7.6),
   or legitimate exclusion (→ §3 with justification).
4. **Author tests.** Codex authors. One row/test per case family; property tests
   for range-invariants (§7.7). Assert typed status/error codes and the
   conservation identity — not parity numbers.
5. **Build the obligation→test map.** Table: family/obligation → test fn(s) →
   status. 100% of applicable families bound.
6. **Re-measure.** Emit `coverage_after.json`; confirm §2 thresholds on the
   eligible surface and no per-function floor breach.
7. **Gate loop.** `cargo fmt --check`;
   `cargo clippy --workspace --all-targets -- -D warnings`;
   `cargo test --workspace`; `cargo deny check`; plus the obligation guard
   (`auth11_required_suite_obligation_guards_contract`) where the module carries
   external-authority suite bindings.
8. **Evidence and disposition.** Disposition states before→after coverage, the
   eligible-surface definition, the exclusion list, the obligation→test map, and
   the threshold pass.

## 5) Required evidence artifacts

- `coverage_before.json` / `coverage_after.json` — raw llvm-cov export.
- `coverage-closure.md` — per-file before/after table, eligible-surface
  denominator, exclusion list with justifications, tier + thresholds, pass/fail.
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
- Gate loop green, or any pre-existing failure attributed and shown not to be
  introduced by this package.

## 7) Anti-patterns

- **Coverage theater.** Asserting trivially-true facts (a constant, a getter) to
  lift the percentage while the law goes untested — reviewer rejects.
- **Substring weakening.** Broadening a failing assertion to a vague substring to
  make it pass (a `tests/AGENTS.md` pitfall); move the assertion to the canonical
  authority instead.
- **Exclusion abuse.** Marking a guard arm `COVERAGE-EXCLUDE` to dodge writing
  its reject test. Exclusions are for type-impossible arms, not unwritten cases.
- **Denominator gaming.** Counting an excluded binary `main` as covered, or
  shrinking the eligible surface to hit the number rather than testing the code.

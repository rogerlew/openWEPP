# Order 3 Pre-Implementation Intent Plan

Evidence class: Static.

Base head: `f0da06bbd6b395d9c8340c550ad0453ad8a39e15`.

Risk: `CRITICAL`.

## Identity Contract

One admitted quality attempt binds the source head and manifest, Git index,
Cargo/Rust/LLVM/CRAP/Nextest versions, feature selection, coverage mode,
Nextest configuration, ordered profiles, instrumented target identity, runner,
workflow, run, and attempt. A mismatch is an integrity failure; there is no
full-only fallback.

The payload excludes `quality_evidence_id`, an envelope digest, and every
digest of itself. Canonical JSON is UTF-8, sorted keys, compact separators, no
NaN/Infinity, followed by one newline only when stored as a file. The ID is
SHA-256 of the canonical payload bytes without that storage newline.

## Inventory And Execution Contract

The collector independently enumerates:

1. selected nonignored `full` tests;
2. selected nonignored `science-manual` tests; and
3. all canonical nonignored workspace tests with the profile default filter
   explicitly ignored.

It requires disjoint selected profile sets and exact equality between their
union and the canonical set. It then executes `full` followed by
`science-manual` in one instrumented target and requires each compact JUnit set
to equal its admitted inventory with no failures, errors, skips, or duplicates.

## Coverage And CRAP Contract

Per-profile `.profraw` inputs are isolated and content-indexed locally. The
collector derives full-only and science-only LCOV for contribution proof, then
stages both exact input sets for deterministic merged LCOV. Only merged LCOV is
passed to `cargo crap` and the canonical adjudication registry/evaluator.

The historical run `30113946779` 18-row snowbench ledger is immutable input.
Every exact row is joined to current merged and science-only CRAP symbol
metrics. A row with no science-only execution contribution remains explicit
observational debt; a row cannot remain actionable merely because the old
full-only LCOV was used.

## Publication Contract

Only these files may appear in the published directory:

- `quality-envelope.json`
- `quality-payload.json`
- `run-status.json`
- `inventory-full.json`
- `inventory-science-manual.json`
- `inventory-workspace.json`
- `junit-full.xml`
- `junit-science-manual.xml`
- `adjudicated-crap-report.json`
- `adjudicated-crap-report.md`
- `coverage-summary.json`

Their total indexed size must not exceed 100 MiB. Raw LCOV, profraw, build,
reconstruction, logs, and cache content remain beneath the local-only attempt
root and are rejected from publication even when compressed.

## Selected Gates

- focused integration and evaluator tests;
- Python compile and negative CLI probes;
- profile/source guards;
- Rustfmt and warnings-denied Clippy for the integration contract;
- package/catalog documentation lint;
- exact write-set, diff, prompt, and line-count reconciliation;
- quality-specific pre-heavy admission;
- delegated sequential instrumented collection;
- two independent reviews, finding disposition, and two independent terminal
  verifications.

The instrumented `full` profile is the selected full-workspace correctness
regression. Metric debt is observational; every identity, inventory, execution,
merge, source-freeze, compactness, or verification defect blocks closure.

## Attempt-5 Follow-On Intent

Evidence class: Ran / Static.

Attempt 5 admitted clean committed head `a5722028`, then exactly three
gate-planner tests rejected the execution clone because its sole Git-status
entry was the observatory-created, separately identity-bound `.venv` symlink.
A local clone does not inherit the source checkout's `.git/info/exclude`.

The correction is limited to replacing the execution clone's local Git exclude
metadata with exact bytes `/.venv\n` immediately after creating the required
symlink. Broad inherited/local rules are not preserved. The symlink target
remains explicitly hashed by
`working_tree_identity`; the exact local-exclude bytes are also identity-bound
so later ignore-policy drift fails. Tracked bytes, index metadata, and every
other nonignored untracked file remain identity-bound and fail-closed.

Selected correction gates:

- behavioral self-test proving `.venv` is Git-clean while symlink-target drift
  changes working-tree identity, other untracked drift stays visible, broad
  rules are removed, and intermediate metadata symlinks fail;
- focused quality-observatory integration/source-contract tests;
- Python compile/self-test, Rustfmt, warnings-denied contract Clippy, docs and
  diff checks;
- two independent revised-candidate measurement/security reviews;
- clean committed exact-checkout gate-planner cases and fresh delegated
  one-process observatory transition;
- two independent terminal verifications of any publication.

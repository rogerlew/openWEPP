# Adjudicated CRAP Closure Gate

Status: `COMPLETE`

Date: `2026-07-13`

Execution mode: `package-end-to-end`

## Objective

Turn the completed CQR pre-integration campaign's empty actionable CRAP set
into a binding, executable closure ratchet. Every production Rust function with
CRAP strictly greater than 30 must either be removed from the terminal census
or match an exact, current, independently adjudicated exception. Implementation
packages must additionally identify touched production files from their frozen
base and must never introduce a new actionable workspace row.

## Authority And Rationale

ADR-0021 already ratifies CRAP at most 30 and the symbol-level eligibility
taxonomy. The CQR pre-integration campaign removed all actionable rows and
retained exactly two source-hash-bound `R-OBSERVABILITY` formatter rows. A raw
`cargo crap --fail-above` gate would reject that adjudicated terminal state;
therefore this package implements the campaign's raw-versus-actionable model
without weakening the threshold or creating a second classification system.

User direction on 2026-07-13 explicitly authorizes this implementation and
requires dual agent review with finding disposition.

## Included Scope

- A fail-closed checker for `cargo-crap` JSON using the exact CQR production
  filter and deduplication tuple.
- A machine-readable registry containing only the two existing CQR
  adjudications and their original evidence.
- Touched-file discovery against a package base ref plus a workspace-wide
  actionable-row ratchet.
- A driver that can generate fresh LCOV/CRAP evidence or assess a supplied CRAP
  JSON artifact.
- Focused tests for pass, new-row failure, stale-hash failure, malformed input,
  filtering, deduplication, and touched-file reporting.
- Binding governance, release-gate integration, CI dependencies, and operator
  documentation.
- Package evidence, dual independent review/verification, finding disposition,
  and final closure disposition.

## Excluded Scope

- Changing the CRAP threshold, formula, coverage profile, or ADR-0021
  eligibility taxonomy.
- Creating new exceptions or re-adjudicating the two accepted formatter rows.
- Refactoring production Rust or changing scientific behavior.
- Treating CRAP as model-validation or physical-adequacy evidence.
- Modifying the preexisting uncommitted V&V strategy documentation changes.

## Intended Write Set

- `AGENTS.md`
- `.cargo-crap.toml`
- `Cargo.toml`
- `.github/workflows/release-gates.yml`
- `tools/release/check_adjudicated_crap.py`
- `tools/release/run_adjudicated_crap_gate.sh`
- `tools/release/adjudicated_crap_exceptions.json`
- `tools/release/run_release_candidate_gates.sh`
- `tools/release/README.md`
- `tests/python/test_adjudicated_crap_gate.py`
- `docs/decisions/0021-module-coverage-closure-thresholds.md`
- `docs/dev-guide/07-contributing.md`
- `docs/standards/code-quality-refactor-authoring-guide.md`
- `docs/standards/mechanical-refactor-authoring-guide.md`
- `docs/standards/module-test-enhancement-authoring-guide.md`
- `docs/standards/rust-scientific-coding-standard.md`
- `docs/work-packages/AGENTS.md`
- `docs/work-packages/README.md`
- `docs/work-packages/templates/cqr-nightly-package.md`
- `docs/work-packages/templates/cqr-nightly-kickoff-prompt.md`
- `docs/work-packages/20260713-adjudicated-crap-closure-gate-001/**`

## Subagent Authorization And Requirement

Subagent authorization: this package explicitly authorizes spawning/delegating
to one `comparator_suite_runner` subagent for heavy CRAP and closure execution,
and two independent reviewer/verifier subagents for implementation,
anti-evasion, governance, and evidence review. Expected outputs are compact
command results or package-local review artifacts. The runner has read-only
source access and bounded write access to `artifacts/heavy-run.md`. Reviewer A
may write only `artifacts/review-a.md`; Reviewer B may write only
`artifacts/review-b.md`.

Subagent requirement: REQUIRED. The parent must delegate the heavy live CRAP
measurement and full closure commands when the runner is available. The two
reviewers must work independently from the terminal implementation and must not
read each other's review before submitting their own.

## Phase Plan

1. Freeze the authority map and scaffold the package.
2. Implement the registry, checker, driver, focused tests, and release wiring.
3. Amend the existing CRAP governance so the executable rule and package
   closure language are consistent.
4. Run focused synthetic tests and reproduce the completed campaign's exact
   two-raw/zero-actionable result from its retained CRAP JSON.
5. Delegate current-source heavy measurement and repository closure gates.
6. Dispatch two independent reviews, fix accepted findings, rerun affected
   gates, and disposition every finding.
7. Record final truthfulness, line-count, security, and closure disposition.

## Exit Criteria

| ID | Criterion |
| --- | --- |
| `ACRAP-001` | The checker applies the exact CQR production filter, strict `> 30` threshold, and exact deduplication tuple. |
| `ACRAP-002` | The retained campaign JSON closes with two raw, two adjudicated, and zero actionable rows. |
| `ACRAP-003` | Synthetic new, stale, malformed, wildcard, or under-evidenced adjudications fail closed. |
| `ACRAP-004` | A package base ref produces an explicit touched production-file set, while any unmatched workspace row remains blocking even outside that set. |
| `ACRAP-005` | The adjudication registry contains only the two accepted CQR rows and binds file hash, symbol, classification, CC, and original dual-review evidence. |
| `ACRAP-006` | Release automation invokes the gate by default and archives machine and human reports. |
| `ACRAP-007` | ADR-0021, root/package closure guidance, standards, and operator documentation describe one consistent adjudicated gate. |
| `ACRAP-008` | Focused Python tests pass; shell syntax, YAML, JSON, Markdown, and diff checks pass. |
| `ACRAP-009` | Required Rust closure commands pass on terminal source, or the package records a legitimate external blocker and remains `HOLD`. |
| `ACRAP-010` | Two independent reviews/verifications are complete and every finding is dispositioned; accepted findings are fixed and reverified. |
| `ACRAP-011` | No new `.rs` file is created and all touched `.rs` files satisfy line-count governance. |
| `ACRAP-012` | Security review confirms no secret, unsafe execution, untrusted path traversal, or fail-open registry behavior. |

Statuses for every criterion are `PASS`, `FAIL`, `BLOCKED`, or `NOT RUN`; any
state other than `PASS` blocks complete disposition.

## Security Impact

The checker consumes repository-local JSON and Git metadata, hashes files, and
writes reports only to operator-selected paths. It must reject paths escaping
the repository, wildcard adjudications, malformed schemas, missing evidence,
duplicate adjudications, and stale source hashes. It must not execute content
from the CRAP report or adjudication registry.

## Progress

- [x] (2026-07-13) User authorized implementation and dual agent review.
- [x] (2026-07-13) Applicable instructions and CQR terminal evidence mapped.
- [x] (2026-07-13) Implement executable gate and 17 focused tests.
- [x] (2026-07-13) Integrate binding governance and release automation.
- [x] (2026-07-13) Run focused and initial terminal heavy verification.
- [x] (2026-07-13) Complete dual independent initial review; both reviewers
  accepted the growth extraction and placed the governance gate on HOLD.
- [x] (2026-07-13) Accept and implement every review finding.
- [x] (2026-07-13) Complete the first post-review hardened-driver run and
  Reviewer A verification.
- [x] (2026-07-13) Accept and remediate Reviewer B's two residual verification
  findings: pre-acquisition stale output and omitted Rust toolchain identity.
- [x] (2026-07-14) Regenerate fresh evidence at `2/2/0`; both independent
  reviewers return `PASS` and lift their HOLDs.

## Surprises And Discoveries

- The CQR source hashes are whole-file SHA-256 values, so either retained
  exception automatically becomes stale after any edit to its host file.
- The raw `cargo-crap` workspace report contains tests, package artifacts, and
  duplicate compilation rows; the historical production filter is therefore a
  correctness requirement, not presentation cleanup.
- A long-running measurement can overlap another agent's edit. Closure now
  compares before/after/final manifests over production sources, all Rust/Cargo
  measurement inputs, the gate code, the registry, HEAD, and the Git index.
- Retained artifacts are useful for adjudication reproduction but are never
  current closure evidence; their status is explicitly `ASSESSMENT-*`.

## Decision Log

- Decision: enforce an empty adjudicated actionable workspace set in addition
  to touched-file reporting.
  Rationale: changes to tests can increase CRAP in source files whose Rust text
  was not touched; a touched-source-only rule would miss that regression.
  Date/Author: 2026-07-13 / Codex, following user direction.
- Decision: bootstrap only the two terminal CQR adjudications.
  Rationale: the package implements prior reviewed authority and is not a venue
  for inventing new exceptions.
  Date/Author: 2026-07-13 / Codex.

## Outcomes And Retrospective

The package closes `PASS`. Its fresh, closure-eligible census assesses `8,330`
production entries across the exact `17/17` crate census, retains the two
historical adjudications, and reports zero actionable rows. Manifest-v2
before/after/final snapshots are byte-identical over `216` production sources
and `419` measurement inputs, including the Rust toolchain selector. Both
reviewers lifted their initial HOLDs after every accepted finding and both
residual verification gaps were fixed and independently reverified. The gate
also exposed and drove closure of one real growth-state CRAP regression rather
than merely documenting the ratchet.

## Recovery And Idempotence

The checker and report generation are read-only with respect to source. Reports
may be regenerated into a clean output directory. If coverage collection is
interrupted, rerun the driver; no partial report is accepted as a pass.

## Change Note

Initial package scaffold authored on 2026-07-13 from explicit user direction.
Closed `COMPLETE` on 2026-07-14 after the final fresh seal and dual independent
verification.

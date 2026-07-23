# TESTGATE Sequential Package Authority Recovery

Package: `20260722-testgate-sequential-package-authority-recovery-001`
Status: `ACTIVE`
Defect: `RTR-044`
Cause: `GATE-PACKAGE-SEQUENTIAL-AUTHORITY-NOT-COMPOSABLE`

This ExecPlan is a living document maintained under
`docs/codex_exec_plans.md` and `docs/defect_closure_execplans.md`.

## Purpose / Big Picture

Close RTR-044 so TESTGATE can prove a base-to-head change assembled from
multiple prospectively authorized packages without retroactively widening any
package. After this correction, both intent and pre-HEAVY admission consume one
canonical, commit-ordered authority-chain artifact. A planned aggregate package
and a later prospective prerequisite package can therefore authorize their own
successive corrections while zero-authority, ambiguous-authority, retroactive,
and malformed chains fail closed.

## Progress

- [x] (2026-07-22) Retained the failed cheap audit and opened RTR-044 durably.
- [x] (2026-07-22) Scaffolded this prospective correction package before code.
- [x] (2026-07-22) Accepted both architecture-review HOLDs and prospectively
  expanded the artifact-binding write set before planner edits.
- [x] (2026-07-22) Added focused tests for single, zero, ambiguous, retroactive,
  scaffold, and prerequisite authority sequences.
- [x] (2026-07-22) Implemented one canonical sequential authority reconstruction consumed by
  intent and pre-HEAVY admission.
- [x] (2026-07-22) Ran focused non-HEAVY validation and reconciled exact diff/line counts.
- [x] (2026-07-22) Obtained dual independent implementation-review PASS
  dispositions with no actionable findings.
- [ ] Commit the correction, durably close RTR-044 at that exact commit, and
  obtain dual terminal verification.
- [ ] Rebuild the release planner and delegate exactly one changed-head
  `INCREMENT` qualification attempt.

## Correction Authority Envelope

- Observed violation: at exact HEAD `59557953`, the canonical singular
  `validate-package` audit rejected the CQR B02 aggregate because a later,
  independently prospective RTR-043 prerequisite changed paths outside the
  aggregate write set. Audit ID:
  `8def403640c88bff45f2b33ab8ef70b9f50ebd6cdfd92ab84d7f60871a78631e`.
- In-scope mechanism: package authority discovery, reconstruction,
  serialization, CLI exposure, intent admission, and pre-HEAVY package
  admission for commit ranges.
- Allowed edits: add a versioned canonical authority-chain artifact; validate
  each commit against package authority present before its non-scaffold
  changes; allow a package scaffold only to establish authority for later
  commits; make Python intent call the canonical planner implementation; make
  Rust pre-HEAVY consume or reconstruct the identical chain; add fail-closed
  regression and source-contract coverage.
- Explicit anchor: the operator-supplied intent package must exist and be
  active at the base. Only that anchor and valid packages scaffolded inside the
  audited range may authorize implementation or package-tree paths; unrelated
  historical packages are never candidates. A root `*-execplan.md` is distinct
  planning state that may cover only its exact path while its bound prior
  lifecycle remains prospective.
- Lifecycle and prompt binding: Rust parses exactly one sanctioned write-set
  heading and one exact status field. Only enumerated prospective lifecycle
  states authorize; unknown, misspelled, terminal, and blocking states do not.
  A terminal package may only perform one byte-preserving active-to-archived
  prompt move. The chain binds the anchor's one active Markdown prompt, its
  blob digest, and exact live directory membership.
- History posture: each first-parent transition is audited with renames
  disabled. A merge is one atomic transition from its first parent; authority
  arriving from another parent is treated as a new scaffold and cannot
  authorize sibling merge contents. Dirty mode fails closed until a separately
  specified synthetic-step contract exists.
- Acceptance: the B02 aggregate plus RTR-043 sequence is READY without
  widening either historical package; a single package remains READY; no
  authority, multiple matching authorities, authority introduced in the same
  correction commit, malformed scaffold, and unmet prerequisite sequences are
  INVALID with typed reasons; intent and pre-HEAVY bind the same artifact ID.
- Protected boundaries: no historical package write-set edits, no retroactive
  authority, no lowering of exact path reconciliation, no HEAVY execution by
  the parent, no branch switch, push, deploy, or manual TESTGATE dispatch.

## Conversion Rule

The reproduced mechanism is owned, safe, testable, and measurable inside this
envelope. The package must land the canonical correction and may not close as
diagnostic-only HOLD while that route remains available.

## Intended Write Set

- `crates/openwepp-gate-planner/src/package_validation.rs`
- `crates/openwepp-gate-planner/src/pre_heavy.rs`
- `crates/openwepp-gate-planner/src/pre_heavy_coverage_tests.rs`
- `crates/openwepp-gate-planner/src/main.rs`
- `crates/openwepp-gate-planner/src/planner.rs`
- `crates/openwepp-gate-planner/src/planner_coverage_tests.rs`
- `crates/openwepp-gate-planner/src/executor.rs`
- `crates/openwepp-gate-planner/src/verifier.rs`
- `gate-policy/v1/schemas/**`
- `gate-policy/v1/impact-map.json`
- `tools/local_ci/testgate.py`
- `tools/local_ci/testgate_qualification.py`
- `tests/python/test_testgate.py`
- `tests/integration/testgate_ci_executor_contract.rs`
- `tests/integration/testgate_assure_campaign_currency_contract.rs`
- `docs/standards/testing-and-gate-strategy.md`
- `docs/work-packages/README.md`
- `docs/work-packages/20260720-testgate-recovery-trust-001/**`
- `docs/work-packages/20260722-cqr-nightly-b02-aggregate-001/**`
- `docs/work-packages/20260722-testgate-sequential-package-authority-recovery-001/**`

No other tracked path is writable without a prospective reviewed amendment.

## Required Reading

- `AGENTS.md`
- `crates/AGENTS.md`
- `tests/AGENTS.md`
- `docs/work-packages/AGENTS.md`
- `docs/standards/AGENTS.md`
- `docs/codex_exec_plans.md`
- `docs/defect_closure_execplans.md`
- `docs/standards/testing-and-gate-strategy.md`
- `docs/work-packages/20260720-testgate-recovery-trust-001/package.md`
- this package and the in-scope implementation/test surfaces

## Subagent Authorization

Subagent authorization: this package explicitly authorizes subagent
spawning/delegation to two independent read-only implementation reviewers, two
read-only terminal verifiers, and one comparator runner for the final exact-head
qualification. Expected outputs are package-local review/verification artifacts
and retained external comparator evidence. Write access is read-only except for
the comparator's ignored artifact root.

## Plan of Work

First preserve this scaffold in its own commit. Add regression fixtures that
describe commit history rather than merely a final tree. Implement sequential
reconstruction once in the Rust planner and expose it through a CLI command
whose JSON output is the canonical chain artifact. Replace Python's independent
single-package selection with that command, and make pre-HEAVY use the same Rust
reconstruction and artifact identity. Retain singular package validation for
callers that explicitly request it.

The artifact binds the explicit anchor, ordered commit/parent/tree identities,
no-rename path deltas, parent package blob digests and normalized write sets,
scaffold child digests, per-path allocations, final net paths, terminal prompt
owner/digest, and its derived chain ID. Python retains the exact Rust-emitted
artifact bytes and never reparses Markdown or rematches paths. The terminal plan
binds the chain ID; pre-HEAVY independently reconstructs live state and rejects
any artifact or plan mismatch.

Then run formatting, schema validation, focused Rust and Python tests, package
audit tests, and source-contract checks. Obtain dual independent review, fix and
disposition all findings, commit the reviewed correction, and append the
durable RTR-044 CLOSED record bound to that exact commit. Dual terminal
verification follows without HEAVY.

Finally rebuild and hash `target/release/openwepp-gate-plan`, generate the exact
intent/terminal plan and READY pre-HEAVY audit, and delegate exactly one changed
HEAD `INCREMENT` qualification attempt. The parent never executes HEAVY.

## Validation and Acceptance

Focused validation must prove successful single and sequential authorities and
fail-closed zero, ambiguous, retroactive, scaffold, and prerequisite cases.
The real CQR B02 history from aggregate scaffold `ddd0e4aa` through the current
head must produce READY and list both prospective authority packages in commit
order. Python intent and Rust pre-HEAVY must report the same chain artifact ID.
All selected non-HEAVY terminal checks must pass before review.

## Idempotence and Recovery

All focused validations are read-only apart from disposable outputs under
`/tmp`. Preserve the RTR-044 reproducer and all earlier attempt roots. Do not
rerun an unchanged expensive gate. If a focused check exposes another tooling
defect, retain its evidence, open it durably, correct it prospectively, and only
then resume qualification on a changed head.

## Surprises & Discoveries

- Observation: singular package admission is correct for one package but cannot
  represent multiple separately prospective authorities across a commit range.
  Evidence: audit ID `8def4036...631e` at HEAD `59557953`.
- Observation: the canonical recovery range needs path-level composition when
  separate prospective packages own disjoint paths in one later commit.
  Evidence: the committed recovery history reconstructed `READY` only after
  allocation selected the newest unambiguous owner for each exact path.
- Observation: executor fixture binding initially grew `executor.rs` to 3,008
  lines. A behavior-neutral documentation cleanup and removal of an obsolete
  disabled pre-HEAVY test restored the executor to 2,999 lines before review.

## Decision Log

- Decision: reconstruct authority in commit order and reject same-commit
  retroactive authority.
  Rationale: prospective authority must exist before the correction it covers;
  final-tree unioning would erase that temporal guarantee.
  Date/Author: 2026-07-22 / Codex.
- Decision: produce one canonical Rust-owned chain artifact and make Python
  intent consume it.
  Rationale: duplicate reconstruction algorithms would create a new trust gap
  between intent and pre-HEAVY admission.
  Date/Author: 2026-07-22 / Codex.
- Decision: require an explicit base anchor and restrict authority candidates to
  that anchor plus packages validly scaffolded inside the audited range.
  Rationale: scanning every historical package would let stale broad write sets
  silently capture current intent.
  Date/Author: 2026-07-22 / Codex after dual architecture review.
- Decision: accept exactly one `Declared Write Set` or `Intended Write Set`
  heading, reject mixed/duplicate headings, and keep all matching in Rust.
  Rationale: both headings are present in the prospective B02 history; Python
  parsing would create divergent wildcard and lifecycle semantics.
  Date/Author: 2026-07-22 / Codex after dual architecture review.
- Decision: reject dirty authority chains and audit merge commits atomically
  from their first parent.
  Rationale: this closes current TOCTOU ambiguity without silently skipping
  merged content or inventing an under-specified synthetic worktree identity.
  Date/Author: 2026-07-22 / Codex after dual architecture review.
- Decision: represent top-level CQR ExecPlans as digest- and lifecycle-bound,
  exact-path-only planning state rather than package authority.
  Rationale: historical recovery planning was prospectively scaffolded outside
  a package directory, but it must never authorize source, sibling, or package
  changes. Terminal planning state cannot self-authorize later edits; only a
  strictly newer prospective package may supersede its exact path.
  Date/Author: 2026-07-22 / Codex after implementation review.
- Decision: permit terminal package state only to archive an unchanged active
  prompt under the same filename, with exactly the deletion/addition pair.
  Rationale: prompt archival finalizes lifecycle after status completion but is
  not continuing package authority. Terminal package state shadows older broad
  authorities inside its own tree, while a strictly newer prospective package
  may explicitly supersede it.
  Date/Author: 2026-07-22 / Codex after implementation review.

## Outcomes & Retrospective

Pending implementation, review, durable closure, and changed-head
qualification.

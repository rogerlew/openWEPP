# Add Campaign-Head Assurance Currency

Package ID: `20260718-testgate-assure-campaign-currency-001`

Queue ID: `TESTGATE-ASSURE-01`

Status: `COMPLETE`

Execution date: 2026-07-18

Frozen base: `87de6bb16b9932eba455637ccefe2e61a9edb050`

This ExecPlan is a living document maintained under
`docs/codex_exec_plans.md`. Keep its progress, decisions, discoveries, and
outcomes current.

## Purpose / Big Picture

Make assurance impact a deterministic campaign-planning output. The gate
planner must enumerate every report in a versioned registry, detect exact and
semantic dependency matches for added, deleted, modified, and rename-expanded
paths, and emit exact-target pending impact records without changing the
historical assessed realization. Unknown production, contract, or result
surfaces conservatively impact every registered report.

Success is observable in generated intent and terminal gate plans, registry
and schema validation, deterministic multi-match folding, and verifier checks
that block target-free or unresolved campaign/release currency. Mechanical
discovery must leave manuscripts, retained results, report lifecycle records,
review locks, identity locks, transactions, and public/generated assurance
surfaces byte-unchanged.

## Progress

- [x] (2026-07-18) Freeze base, authority, protected surfaces, initial write set,
  validation intent, and review authorization.
- [x] (2026-07-18) Add and validate the versioned assurance dependency/watch registry.
- [x] (2026-07-18) Integrate registry-wide exact and semantic discovery into intent and
  terminal plans with deterministic impact identities.
- [x] (2026-07-18) Complete exact-head fold/currency verification and add/rename/delete,
  unknown, registry-completeness, and multi-impact tests.
- [x] (2026-07-18) Run focused gates and preserve byte-level no-mutation evidence.
- [x] (2026-07-18) Obtain dual independent implementation-review PASS verdicts.
- [x] (2026-07-18) Run the terminal conservative closure sequence, retaining
  successful expensive evidence across reviewed policy/docs-only remediation.
- [x] (2026-07-18) Obtain dual terminal-verifier PASS verdicts and disposition
  the package COMPLETE.

## Authority And Intent

Authority is ADR-0039 and `docs/standards/testing-and-gate-strategy.md`
sections 8, 11, and 13, especially registry-wide discovery, immutable impact
events, exact-target transfer currency, deterministic per-report folding, and
the human scientific-materiality boundary. The user's 2026-07-18 request
authorizes scaffold and end-to-end execution.

This is a critical gate-policy and assurance-governance integration. It does
not authorize report refresh, reproduction, review, approval, release
publication, protected evidence publication, CI cutover, or gate reduction.

## Declared Write Set

- `crates/openwepp-gate-planner/src/assurance.rs`
- `crates/openwepp-gate-planner/Cargo.toml`
- `crates/openwepp-gate-planner/src/executor.rs`
- `crates/openwepp-gate-planner/src/lib.rs`
- `crates/openwepp-gate-planner/src/planner.rs`
- `crates/openwepp-gate-planner/src/policy.rs`
- `crates/openwepp-gate-planner/src/ledger.rs`
- `crates/openwepp-gate-planner/src/main.rs`
- `Cargo.toml`
- `Cargo.lock`
- `deny.toml`
- `gate-policy/v1/README.md`
- `gate-policy/v1/assurance-registry.json`
- `gate-policy/v1/schemas/assurance-registry.schema.json`
- `gate-policy/v1/schemas/assurance-impact.schema.json`
- `gate-policy/v1/schemas/gate-plan.schema.json`
- `gate-policy/v1/fixtures/valid/assurance-registry.json`
- `gate-policy/v1/fixtures/valid/assurance-impact.json`
- `gate-policy/v1/fixtures/valid/gate-plan.json`
- `gate-policy/v1/fixtures/invalid/assurance-registry-incomplete.json`
- `gate-policy/v1/fixtures/invalid/assurance-impact-bare-current.json`
- `gate-policy/v1/impact-map.json`
- `tests/integration/testgate_assure_campaign_currency_contract.rs`
- `docs/ROADMAP.md`
- `docs/work-packages/README.md`
- `docs/work-packages/20260718-testgate-assure-campaign-currency-001/**`

Read-only discovery may inspect assurance catalogs, report manifests,
governance contracts, planner/verifier/schema patterns, and adjacent tests.
Writes outside this set require a recorded pre-implementation amendment.

## Protected Boundaries

- Do not write `assurance/v2/reports/**`, `assurance/v2/identity.lock.json`,
  `assurance/v2/principals.yaml`, `assurance/v2/transactions/**`, `usersum/**`,
  or generated/public assurance catalogs and manifests.
- Ordinary source movement changes campaign impact, not historical
  assessed-realization integrity.
- The planner detects dependency impact only; it never decides scientific
  materiality or fabricates human authority.
- Every registered report is discovered; operator preselection cannot narrow
  the registry.
- Unknown production, science-contract, result, or builder/schema coverage
  fails conservatively and blocks transfer.
- Rename remains delete plus add. Multiple matching watches for one changed
  object coalesce without losing match identities.
- Exact campaign and release transfer remain fail-closed without verified
  receipt, envelope, principal-role, revocation, and target bindings.

## Required Deliverables

1. A strict versioned assurance registry whose report set equals the canonical
   assurance catalog and whose closed watch kinds cover exact paths, component
   prefixes, repository-rooted globs, contracts, Cargo packages, process/domain
   tags, result procedures, and builder schemas.
2. Planner integration that discovers impacts across the complete registry,
   emits stable exact-target plan records, preserves assessed realization
   integrity, coalesces watch matches, and conservatively impacts all reports
   for unclassified production/contract/result surfaces.
3. Verifier behavior for deterministic per-report multi-entry folding,
   exact-head invalidation, request/currency consistency, supersession safety,
   and release identity binding without accepting self-declared authority.
4. Executable positive and negative contracts for add, delete, rename-expanded
   paths, unknown paths, direct and semantic matches, registry completeness,
   deterministic ordering/identity, and protected-surface nonmutation.
5. Focused evidence, dual independent review, exact-diff reconciliation, one
   terminal conservative closure sequence, dual terminal verification, line
   counts, and truthful final disposition.

## Plan Of Work

Milestone 1 introduces a planner-owned assurance registry beside the existing
gate policy. Loading cross-checks its report IDs against the canonical
`assurance/v2/catalog.yaml` without changing that catalog or report sources.

Milestone 2 evaluates every changed object against every registered watch.
Direct path watches use repository-rooted semantics; contract, package,
process/domain, result-procedure, and builder-schema watches consume governed
planner classifications. Matching watches coalesce per report/object, while an
unclassified governed assurance surface creates an unknown impact for all
reports.

Milestone 3 strengthens the assurance fold and currency checks. Target-head
changes reopen exact currency, unresolved entries block both transfer axes,
and release currency is impossible without an exact requested release
identity and authenticated transfer capability.

Milestone 4 runs focused schema/planner/verifier checks and two independent
reviews. Accepted findings are remediated before the single conservative
terminal sequence and two independent exact-tree verifications.

## Validation And Acceptance

Focused development commands:

    cargo fmt --check
    cargo nextest run -p openwepp-gate-planner
    cargo nextest run --test testgate_assure_campaign_currency_contract
    cargo clippy -p openwepp-gate-planner --all-targets -- -D warnings
    markdown-doc lint docs/ROADMAP.md docs/work-packages/README.md docs/work-packages/20260718-testgate-assure-campaign-currency-001 gate-policy/v1/README.md
    git diff --check

Acceptance requires exact registry/catalog equality, deterministic records for
add/delete and rename-expanded pairs, direct plus semantic watch coverage,
multi-watch coalescing, all-report unknown escalation, exact-target currency
blocking, verifier-rejected fabricated CURRENT, and a before/after digest
manifest proving every protected assurance/public surface remained unchanged.

After both implementation reviews pass, run exactly one terminal conservative
sequence on the stable tree:

    cargo fmt --check
    cargo clippy --workspace --all-targets -- -D warnings
    cargo nextest run --workspace --profile full
    cargo deny check
    bash tools/release/run_adjudicated_crap_gate.sh --base-ref 87de6bb16b9932eba455637ccefe2e61a9edb050

Do not repeat a successful heavy command for presentation. Any FAIL, BLOCKED,
or unjustified NOT RUN prevents package completion.

## Review, Verification, And Line Counts

Two independent reviewers inspect registry completeness, matcher semantics,
unknown escalation, identity determinism, exact-target currency, authority
fail-closure, no-mutation boundaries, and test economy. Every finding is
dispositioned; accepted findings are fixed before closure.

Two terminal verifiers inspect the exact remediated tree and Gate Evidence
Non-Deferral compliance. Files at or above 2,000 lines are WARN with split
intent; non-generated 3,000+ files block closure absent an approved exception.

Subagent authorization: this package explicitly authorizes subagent
spawning/delegation to two independent reviewer roles and two independent
terminal-verifier roles for read-only inspection; expected outputs are compact
findings and verdicts delivered to the parent for package artifacts; write
access is read-only.

Subagent requirement: REQUIRED for the single terminal full-workspace Nextest,
cargo-deny, and global adjudicated-CRAP closure set. This package explicitly
authorizes subagent spawning/delegation to one closure-runner role for those
commands; expected outputs are exact commands, verdicts, counts, timing, and
artifact paths; write access is limited to generated build/coverage output.
The parent must not repeat successful heavy commands.

## Idempotence And Recovery

Planning is read-only and content-addressed. Repeating it over the same policy,
registry, change set, and target must yield identical assurance impact IDs.
Failure while loading or matching the registry produces no source mutation.
Generated test/build output remains disposable; report and public surfaces are
never recovery targets.

## Surprises & Discoveries

- Observation: TESTGATE-PLAN-01 already carries typed assurance axes and an
  assurance-impact verifier, but generated plans always emit an empty impact
  list and policy loading does not enumerate the canonical report catalog.
  Evidence: `planner.rs`, `policy.rs`, and the v1 assurance fixtures at frozen
  base.
- Observation: exact registry/catalog equality requires parsing the canonical
  YAML catalog rather than duplicating its IDs in source or using a line-based
  approximation. The repository lock and assurance tooling already pin
  `serde_yaml` 0.9.
  Evidence: root development dependencies, `Cargo.lock`, and
  `assurance/v2/catalog.yaml`.
- Observation: isolated executor repositories copy the policy schemas but did
  not carry an assurance catalog/registry pair because that input did not
  exist at the frozen base.
  Evidence: focused package Nextest failed at policy load before process spawn.
- Observation: repository integration tests live under `tests/integration/`
  and are explicitly registered in the root Cargo manifest.
  Evidence: adjacent TESTGATE contract declarations in `Cargo.toml`.
- Observation: the new direct crate dependency projections are recorded in
  the workspace lock even though both dependency packages were already
  present.
  Evidence: Cargo's deterministic lock update after the first focused build.
- Observation: the first full crate development run overlapped active source
  edits and correctly failed one verifier case with
  `GATE-RECEIPT-SOURCE-MUTATION` after 45 passes. It is invalid as stable-tree
  evidence and was not repeated; the terminal full-workspace run will exercise
  the same case once on the frozen implementation tree.
  Evidence: Nextest run `a8ef5651-1bbd-46e4-9b13-2003dad7a3dd`.
- Observation: generated review locks expose a stable science root and the
  best available assessed realization root even for draft/in-review reports.
  Evidence: production `review.lock.json` files; policy loading now rejects
  registry drift from those protected identities.
- Observation: two independent reviews found that schema history, path-keyed
  folding, dirty-target identity, matcher boundaries, and authority binding
  were individually plausible but not yet closed as one campaign invariant.
  Evidence: initial HOLD verdicts and accepted dispositions in
  `artifacts/review-findings.md`.
- Observation: the first terminal sequence stopped at workspace Clippy because
  one integration test function was 116 lines against the 100-line lint cap.
  The body was mechanically split into named assertion helpers; focused target
  Clippy then passed without rerunning the long behavior case.
  Evidence: `artifacts/terminal-gate-results.md` and focused Clippy output.

## Decision Log

- Decision: maintain the mechanical watch registry under `gate-policy/v1/`
  and cross-check it against `assurance/v2/catalog.yaml` instead of changing
  report manifests or generated assurance identities.
  Rationale: campaign impact metadata is planner authority; report sources and
  their historical identities are protected from ordinary impact discovery.
  Date/Author: 2026-07-18 / Codex.
- Decision: amend the write set before implementation to add the gate-planner
  crate manifest and reuse the repository-pinned `serde_yaml` dependency.
  Rationale: catalog equality must be structural and fail closed; an ad hoc
  text parser would not satisfy the registry completeness contract.
  Date/Author: 2026-07-18 / Codex.
- Decision: amend the write set before remediation to add the executor's
  isolated-repository fixture helper.
  Rationale: every planner fixture now requires a structurally complete
  catalog/registry pair; the helper change is test scaffolding only and does
  not alter production execution.
  Date/Author: 2026-07-18 / Codex.
- Decision: amend the write set before adding the TESTGATE-ASSURE contract to
  include the root Cargo manifest.
  Rationale: the declared integration test otherwise is not a Cargo test
  target and would provide no executable closure evidence.
  Date/Author: 2026-07-18 / Codex.
- Decision: include Cargo's mechanical lockfile update in the authorized write
  set.
  Rationale: the root test target and gate-planner YAML parser add direct
  dependency edges that the lock records; no package version changed.
  Date/Author: 2026-07-18 / Codex.
- Decision: represent each immutable changed-object lifecycle with a derived
  `impact_subject_id` and reject self-declared terminal dispositions until an
  authenticated lifecycle capability verifier exists.
  Rationale: per-path latest-state folding and unauthenticated supersession can
  erase open obligations; fail-closed behavior preserves correctness without
  fabricating authority.
  Date/Author: 2026-07-18 / Codex.
- Decision: use the dirty-tree digest as the exact assurance target for dirty
  planning and request campaign transfer only on committed terminal plans.
  Rationale: a base commit does not identify index, worktree, and untracked
  content; an intent plan must not imply transfer authority.
  Date/Author: 2026-07-18 / Codex.
- Decision: amend the write set before terminal remediation to allow the
  dependency's SPDX `MIT-0` license in `deny.toml`.
  Rationale: `borrow-or-share` was already present at the frozen base through
  `jsonschema`; MIT-0 is OSI-approved and consistent with the repository's
  permissive-only posture, but the explicit allowlist omitted it.
  Date/Author: 2026-07-18 / Codex.

## Outcomes & Retrospective

Implementation and terminal gates PASS. The planner now emits deterministic,
registry-wide assurance impacts bound to exact commits or dirty-tree identity;
unmapped report/object pairs remain open unknown; terminal reconciliation is
monotonic; immutable subject folding preserves repeated same-path changes; and
self-declared disposition or currency cannot establish authority. Report and
public assurance surfaces remained byte-identical.

The package exposed two useful governance defects during closure: a long test
function violated the workspace lint cap, and the permissive dependency
allowlist omitted base-present MIT-0. Both were remediated narrowly, reviewed,
and rechecked without repeating successful expensive tests. Both independent
terminal verifiers reconstructed the protected manifests, write set, retained
Nextest result, CRAP checksums/current-source applicability, and non-deferral
record and returned PASS. Final disposition: COMPLETE.

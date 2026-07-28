# Gate Planner Governance Authority Alignment

Package ID:
`20260727-gate-planner-governance-authority-alignment-001`

Queue ID: `GATE-LINT-GOV-01`

Status: `ACTIVE`

Authorization: the user's 2026-07-27 direction to scaffold and execute
Governance authority alignment, roadmap Order 1 under ADR-0043.

This ExecPlan is maintained under `docs/codex_exec_plans.md`.

## Purpose

Make the operative repository guidance match ADR-0043. After this package,
agents select and execute canonical validation requirements directly; the
frozen gate planner and TESTGATE cannot authorize, block, or close prospective
work. Underlying correctness, science, assurance, quality, security, review,
and protected-data obligations remain binding from their own authorities.

## Progress

- [x] 2026-07-27 20:30 PDT — Scaffold Order 1 from the reviewed amendment
      specification.
- [ ] Apply operative agent, work-package, testing, local-command, prompt, and
      template amendments.
- [ ] Reconcile source-coupled guard tests and impact-policy authority rows.
- [ ] Pin historical generation-17 policy identity independently of the live
      standard.
- [ ] Apply frozen/superseded package status overlays.
- [ ] Run direct focused validation and exact-diff reconciliation.
- [ ] Complete dual independent review, dual verification, and finding
      disposition.
- [ ] Archive the prompt, close catalogs, and commit the package.

## Included Scope

- Apply the exact Order-1 amendment clauses accepted in
  `20260727-gate-planner-advisory-linter-roadmap-001`.
- Remove prospective planner/TESTGATE admission, receipt, pre-heavy, CI-lane,
  repair-prerequisite, and lifecycle authority from operative guidance.
- Preserve direct canonical requirements, conservative unknown-impact handling,
  exact-diff reconciliation, content/input-bound evidence reuse, assurance
  approval/transfer, ADR-0041 quality posture, and campaign/release correctness.
- Delete the legacy executor source-literal integration guard and registration.
- Migrate the schema/alignment guard to ADR-0043/direct-governance assertions.
- Remove planner-policy critical rows while preserving independent
  external-authority anti-evasion rows.
- Record immutable historical generation-17 strategy identity.
- Reconcile the five package-local status surfaces named by Order 0.

## Excluded Scope

- No gate-planner Rust/Python executable, schema implementation, workflow, or
  receipt/ledger deletion.
- No advisory-linter implementation.
- No TESTGATE, planner transition, trusted runner, CI dispatch, or attestation.
- No CAL-04B execution, calibration design/result, publication, Harvard access,
  or protected-data state.
- No science-contract, kernel, model, or runtime behavior change.
- No Order-2 or later package scaffold.

## Declared Write Set

- `AGENTS.md`
- `Cargo.toml`
- `docs/ROADMAP.md`
- `docs/standards/testing-and-gate-strategy.md`
- `docs/standards/local-ci-gate-selection.md`
- `docs/standards/prompt-wording-guidance.md`
- `docs/work-packages/AGENTS.md`
- `docs/work-packages/README.md`
- `docs/work-packages/gate-planner-advisory-linter-roadmap.md`
- `docs/work-packages/templates/cqr-nightly-package.md`
- `docs/work-packages/templates/cqr-nightly-kickoff-prompt.md`
- `tools/local_ci/README.md`
- `tests/integration/testgate_ci_executor_contract.rs`
- `tests/integration/testgate_align_authority_contract.rs`
- `gate-policy/v1/impact-map.json`
- `gate-policy/history/adr0039-generation17.json`
- `docs/work-packages/20260727-gate-planner-external-dag-transaction-adapter-001/package.md`
- `docs/work-packages/20260727-gate-planner-external-dag-closeout-correction-001/package.md`
- `docs/work-packages/20260727-gate-planner-auth11-terminal-node-selection-001/package.md`
- `docs/work-packages/20260727-gate-planner-auth11-fixed-inventory-test-provider-001/package.md`
- `docs/work-packages/20260727-testgate-first-attempt-ledger-bootstrap-001/package.md`
- this package subtree

The untracked
`docs/audits/20260727_gate_planner_demotion_readiness_audit.md` is read-only
supplemental input and remains outside this package's write set and commits.

## Landing Rule

ADR-0043 governs this package from authorization. Execute direct manual
requirements selected from the exact diff. Do not request or require a final
planner plan, pre-heavy audit, receipt, TESTGATE transition, trusted runner
admission, or planner-certified closeout. Existing impact-map rows that classify
planner/policy changes as critical are migration inputs, not authority to invoke
the frozen system.

This rule does not waive direct focused governance tests, anti-evasion guards,
documentation checks, review, verification, exact-diff reconciliation, or any
independently applicable correctness/security requirement.

## Phase Plan

### Phase A — Governance Text

Apply the exact heading- and phrase-anchored replacements in
`artifacts/policy-conflict-matrix.md` from Order 0. Keep root guidance concise.
Replace the canonical testing strategy's planner/executor/receipt machinery
with direct agent selection, execution, evidence, campaign deferral, and
advisory analysis.

### Phase B — Enforcement And History

Remove the obsolete executor-literal guard. Migrate the alignment guard to
assert ADR-0043, the direct landing rule, preserved A0/A1/A3 and quality
boundaries, historical identity, and absence of prospective TESTGATE authority.
Remove only the three planner/policy authority rows from the impact map. Create
a literal historical registry for SHA-256
`74203b294dcea4c7f3ecb5fe4110a425d938d2ec75bde60cfc646a54fea3f5e9`,
Git blob `ab8fe3e4db61df6691a96a11fa2034b90036bfb2`, and source commit
`57f5f6f1f1649022d47124de856108c6a11cc483`. Historical verifiers must use the
old object, never the changed live path.

### Phase C — Frozen Status Overlay

Keep completed packages complete. Mark every incomplete named planner
prerequisite `FROZEN / SUPERSEDED BY ADR-0043`, retain its historical progress
and outcome, and state that explicit user authorization is required to resume.
Align the catalog.

### Phase D — Direct Validation And Closure

Run focused governance tests, authority anti-evasion, JSON/documentation
validation, diff hygiene, exact write-set reconciliation, dual review, dual
verification, and line-count governance. Archive this package prompt and close
the roadmap/catalog. Do not run planner/TESTGATE or a full workspace merely
because the old policy called this surface critical.

## Direct Validation

```text
cargo nextest run --test testgate_align_authority_contract
bash tools/release/check_authority_suite_antievasion.sh
cargo nextest run --test auth11_required_suite_obligation_guards_contract
markdown-doc lint --path AGENTS.md
markdown-doc lint --path docs/work-packages/AGENTS.md
markdown-doc lint --path docs/standards
markdown-doc lint --path docs/work-packages/20260727-gate-planner-governance-authority-alignment-001
git diff --check
```

Also parse every changed JSON file, prove the historical Git blob exists and
hashes to the recorded SHA-256, confirm the removed test registration is absent,
confirm no incomplete named planner package remains `ACTIVE`, and reconcile the
exact package-owned diff.

## Acceptance

- No operative guidance assigns prospective permission, execution, lifecycle,
  evidence, receipt, ledger, repair-prerequisite, runner, or CI authority to
  TESTGATE or the planner.
- The advisory linter remains read-only, non-authoritative, nonblocking, and
  absent from CI.
- Direct correctness/science/security/package obligations remain explicit and
  cannot be waived by agent discretion or linter failure.
- The migrated guard and anti-evasion checks pass.
- Historical generation-17 verification is independent of changed live bytes.
- No stale nearest-package `ACTIVE` state can resume frozen planner work.
- Dual reviews, finding disposition, dual verification, line-count governance,
  documentation checks, and exact-diff reconciliation pass.

## Subagent Authorization

Subagent authorization: this package explicitly authorizes subagent
spawning/delegation to two independent read-only reviewers and two independent
read-only verifiers for governance alignment, preserved correctness/security/
science boundaries, source-coupled guard adequacy, historical identity, frozen
status overlays, exact-diff reconciliation, and closure evidence. Expected
outputs are compact findings or verification results with exact paths. Write
access is read-only. No heavy runner or comparator is selected.

## Surprises And Discoveries

- None yet.

## Decision Log

- 2026-07-27: ADR-0043's direct manual landing rule controls this package; the
  demoted planner cannot authorize its own demotion.
- 2026-07-27: Focused governance/anti-evasion checks are selected. No production
  or kernel path changes, and no full-workspace run is selected.

## Outcomes And Retrospective

Populate after implementation, validation, review, and verification.

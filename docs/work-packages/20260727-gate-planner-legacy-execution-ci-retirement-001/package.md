# Legacy Gate-Planner Execution And CI Retirement

Package ID:
`20260727-gate-planner-legacy-execution-ci-retirement-001`

Queue ID: `GATE-LINT-ADV-02`

Status: `ACTIVE`

Authorization: the user's 2026-07-27 instruction to scaffold and execute
advisory-linter roadmap Order 4.

Base commit: `c5dc88fc063927f3bbb3941cab07fbdf77758aa9`

Execution mode: `package-end-to-end`.

This ExecPlan is maintained under `docs/codex_exec_plans.md`.

## Objective

Delete the retired gate-planner execution/control plane and TESTGATE CI after
an exact live-consumer inventory, migrate the remaining direct authority and
optional quality-observatory consumers, and preserve historical policy identity
through its existing immutable Git-object registry.

## Implementation Intent

`control-plane retirement / behavior-preserving direct-authority migration`.
No kernel, model, science, calibration design, Harvard content, or prospective
validation authority changes.

## Included Scope

- Exact consumer inventory and row-by-row disposition of the Order-0 migration
  map.
- Delete the legacy `openwepp-gate-planner` crate, controller, qualification,
  resolver, tests, schemas, fixtures, and TESTGATE workflow after consumer
  migration.
- Remove the crate from workspace membership and dependencies.
- Migrate direct science-contract admission inputs out of `gate-policy/v1`.
- Preserve immutable generation-17 historical identity and its direct
  Git-object verification guard.
- Migrate optional quality observation from TESTGATE qualification/priority to
  explicit exact-source identity with no linter or planner dependency.
- Retire or rename forest1 TESTGATE-specific operational naming that remains a
  live quality-observatory dependency.
- Migrate surviving direct governance and anti-evasion tests.
- Update operative documentation, catalogs, and roadmap state.

## Excluded Scope

- No Order 5 scaffold or linter utility qualification.
- No linter feature change.
- No CAL execution, model command, Harvard access, freeze, holdout, or
  calibration result.
- No rewrite or deletion of historical receipts, ledgers, CAL attempts, logs,
  or immutable generation-17 policy bytes.
- No weakening of science-contract admission, assurance, protected-data,
  release, or direct correctness requirements.
- No CI role for the advisory linter.

## Declared Write Set

- `Cargo.toml`
- `Cargo.lock`
- `.config/nextest.toml`
- `crates/openwepp-gate-planner/**`
- `gate-policy/v1/**`
- `gate-policy/history/adr0039-generation17.json`
- `.github/workflows/testgate-shadow.yml`
- `.github/workflows/testgate-conservative.yml`
- `.github/workflows/conservative-correctness.yml`
- `.github/workflows/quality-observatory.yml`
- `tools/local_ci/README.md`
- `tools/local_ci/testgate.py`
- `tools/local_ci/testgate_qualification.py`
- `tools/local_ci/resolve_testgate_comparison_base.py`
- `tools/local_ci/resolve_testgate_intent_package.py`
- `tools/local_ci/quality_observatory_workflow.py`
- `tools/local_ci/quality_observatory.py`
- `tools/local_ci/run_quality_observatory_child.sh`
- `tools/ci/omarchy-runner/**`
- `tools/release/check_science_contract_admission.sh`
- `tools/release/README.md`
- `tools/release/authority-policy/**`
- `tests/python/test_testgate.py`
- `tests/python/test_testgate_qualification.py`
- `tests/python/test_resolve_testgate_comparison_base.py`
- `tests/python/test_resolve_testgate_intent_package.py`
- `tests/python/test_quality_observatory_workflow.py`
- `tests/integration/testgate_align_authority_contract.rs`
- `tests/integration/testgate_assure_campaign_currency_contract.rs`
- `tests/integration/advisory_linter_authority_contract.rs`
- `tests/integration/quality_observatory_workflow_contract.rs`
- `docs/ROADMAP.md`
- `docs/work-packages/README.md`
- `docs/work-packages/gate-planner-advisory-linter-roadmap.md`
- `docs/decisions/0043-gate-planner-is-a-non-authoritative-advisory-linter.md`
- this package subtree

The user-owned untracked
`docs/audits/20260727_gate_planner_demotion_readiness_audit.md` remains
unchanged, unstaged, and excluded.

## Phase Plan

1. Scaffold and commit this autonomous package.
2. Record exact live-consumer counts for every migration-map row.
3. Migrate direct authority, quality-observatory, runner, and historical
   verification consumers.
4. Delete zero-consumer planner, TESTGATE, schema, fixture, and CI surfaces.
5. Run focused direct guards, quality workflow tests, Python tests, workspace
   metadata/build checks, anti-evasion gates, documentation checks, and exact
   consumer scans.
6. Run campaign-strength full-workspace correctness through the required
   comparator-suite runner.
7. Complete dual independent review, finding disposition, dual independent
   verification, exact-diff reconciliation, prompt archival, and closure.

## Validation

Direct focused requirements:

```text
.venv/bin/python tools/local_ci/quality_observatory_workflow.py self-test
cargo nextest run --test advisory_linter_authority_contract
cargo nextest run --test quality_observatory_workflow_contract
bash tools/release/check_science_contract_admission.sh \
  --base-ref HEAD^ --head-ref HEAD
bash tools/release/check_authority_suite_antievasion.sh
cargo nextest run --test auth11_required_suite_obligation_guards_contract
cargo metadata --no-deps --format-version 1
markdown-doc lint --path \
  docs/work-packages/20260727-gate-planner-legacy-execution-ci-retirement-001
git diff --check
```

Terminal closure additionally requires direct full-workspace Nextest and strict
Clippy through `comparator_suite_runner`, exact zero-consumer scans, schema/JSON
parsing, workflow syntax/static contract checks, production line-count
governance, and exact write-set reconciliation.

## Acceptance

- Every migration-map row has a current exact consumer count and disposition.
- No live source, test, workflow, controller, or documentation instruction
  invokes gate-planner or TESTGATE prospectively.
- The legacy crate, controller, TESTGATE workflow, obsolete schemas/fixtures,
  and their registrations are absent.
- The optional quality observer admits exact source identity directly and has
  no TESTGATE, planner, linter, receipt-authority, or lifecycle dependency.
- Direct science-contract admission and external-authority anti-evasion guards
  retain their inputs and pass.
- Historical generation-17 identity resolves the exact frozen Git object and
  digest without a planner executable or mutable live-policy dependency.
- Direct canonical commands and full-workspace correctness pass.
- No linter CI integration or lifecycle status is introduced.
- Dual review, finding disposition, dual verification, line-count governance,
  exact-diff reconciliation, and truthful prompt/catalog closure pass.

## Security Impact

Security-sensitive deletion is fail-closed. Any nonzero or unknown live
consumer count makes that row ineligible for deletion. External-authority
anti-evasion and Harvard custody owners remain intact. Historical evidence is
never rewritten.

## Subagent Authorization And Requirement

Subagent authorization: this package explicitly authorizes subagent
spawning/delegation to one `comparator_suite_runner` for the required
full-workspace Nextest/Clippy closure runs and to two independent read-only
reviewers plus two independent read-only verifiers for consumer inventory,
migration correctness, security boundaries, direct command survival, and exact
closure. Expected outputs are compact command results, artifact paths, and
findings. Write access is read-only.

Subagent requirement: REQUIRED. The parent must delegate the broad
full-workspace closure runs to `comparator_suite_runner` and must not run those
heavy suites locally unless the subagent is unavailable with recorded evidence.

## Progress

- [x] 2026-07-27 — Scaffolded from accepted roadmap Order 4.
- [x] 2026-07-27 — Exact consumer inventory completed.
- [x] 2026-07-27 — Direct owners migrated and zero-consumer surfaces deleted.
- [x] 2026-07-27 — Focused validation passed.
- [ ] Full-workspace closure passed.
- [ ] Dual review and finding disposition passed.
- [ ] Dual verification and exact-diff closure passed.
- [ ] Prompt archived, catalogs closed, and completion committed.

## Decision Log

- 2026-07-27: Historical policy identity remains in
  `gate-policy/history/adr0039-generation17.json`; the legacy verifier CLI does
  not survive without a named live consumer.
- 2026-07-27: A nonzero/unknown consumer count prevents deletion of that row but
  does not create a linter or modeling hold.
- 2026-07-27: A focused Nextest run exposed retired package selectors in
  `.config/nextest.toml`; the package write set was amended and those selectors
  were removed before validation continued.
- 2026-07-27: Exact-diff reconciliation found a transcribed base-SHA suffix in
  the scaffold. The declared base was corrected to the resolvable scaffold
  parent `c5dc88fc063927f3bbb3941cab07fbdf77758aa9`.

## Outcomes

Populate after implementation and verification.

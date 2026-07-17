# Implementation Handoff

Evidence class: `Static`

ADR-0039 and the testing/gate standard settle policy. This handoff scopes, but
does not authorize or pre-complete, the implementation follow-up.

## Required Alignment Surfaces

### Governance and instructions

- `AGENTS.md`
- `docs/work-packages/AGENTS.md`
- `tests/AGENTS.md`
- `docs/codex_exec_plans.md`
- `docs/standards/prompt-wording-guidance.md`
- `docs/standards/kernel-work-package-preparation.md`
- `docs/standards/mechanical-refactor-authoring-guide.md`
- `docs/standards/module-test-enhancement-authoring-guide.md`
- `docs/standards/code-quality-refactor-authoring-guide.md`
- `docs/standards/local-ci-gate-selection.md`
- `docs/standards/rust-scientific-coding-standard.md`
- `docs/decisions/0021-module-coverage-closure-thresholds.md`
- `docs/specifications/correctness-authority-model.md`
- work-package templates and active prospective prompts that duplicate terminal
  full-gate wording.

These surfaces should point to one gate-lifecycle authority. Specialized test,
coverage, conservation, review, and correctness obligations remain in their
own documents.

### Gate planning and receipts

Implement a repository-owned planner with stable human and JSON output. The
follow-up must decide the binary/package name, but the accepted interface must
support the equivalent of:

```text
gate plan --boundary increment --base <ref> --head <ref>
gate run --plan <plan.json>
gate verify-receipt <receipt.json>
gate status --campaign <id>
gate certify --campaign <id> --head <ref>
```

Implementation must include:

- Cargo metadata and reverse-dependency expansion;
- canonical Git raw change sets and normalized Cargo graphs across the supported
  target/feature/dependency-kind matrix;
- a versioned non-Cargo impact map;
- pre-implementation intent planning and exact-diff terminal reconciliation;
- deterministic risk-reason rules;
- a typed per-gate DAG with stable IDs, prerequisites, executor/adapter kind,
  argument arrays, environment allowlist, expected inventory, acceptance rule,
  timeout/retry/failure policy, artifact contract, blocking boundary, and reuse
  class;
- zero-test and planned/executed-inventory checks;
- distinct `plan_id`, pre-execution `execution_key`, and post-execution
  `receipt_id`;
- canonical transitive-input manifests for layered
  execution/authority/documentation/assurance roots;
- immutable receipts and receipt reuse verification;
- unsigned receipt plus attestation-envelope schemas and verifier;
- closed trust/reuse classes, hermetic executor confinement, protected-CI
  attestations, issuer/principal authority and revocation;
- campaign lifecycle, admission, amendment, head-chaining, bootstrap, backstop,
  concurrent compare-and-swap reduction, protected evidence branch/tag
  certification, and
  gate-ledger schemas;
- explicit critical escalation and no generic downgrade command; and
- fixtures that replay representative completed packages and campaigns.

### Test and quality execution

- `.config/nextest.toml`: align profiles with affected, checkpoint, full, domain,
  assurance, and release roles without embedding policy solely in comments.
- `tools/local_ci/`: make timing and receipt capture executor-owned.
- `tools/release/run_adjudicated_crap_gate.sh` and checker: add affected-surface
  increment mode while preserving current full global mode for critical,
  campaign, and release closure.
- `tools/release/run_release_candidate_gates.sh`: consume a current campaign
  certificate, run release-only gates, and avoid duplicate execution where a
  verified combined Nextest/coverage path is accepted.
- `.github/workflows/release-gates.yml`: split presubmit, post-submit/backstop,
  campaign, and explicit release triggers.

The package must benchmark and verify—not assume—that an instrumented Nextest
run can replace the current separate full and coverage executions. Test
inventory, doctest treatment, coverage, CRAP, failure semantics, and runtime
must be compared.

### Assurance

Extend `openwepp-assurance plan` or an adjacent planner surface to represent:

- exact path and versioned semantic-watch dependencies;
- separate assessed-realization integrity, campaign-impact disposition,
  campaign-head transfer, and release-transfer axes;
- deterministic impact-entry identity, coalescing, ownership, and transition;
  and
- conservative unknown/add/rename/delete handling that catches newly introduced
  relevant files;
- exact target-bound axes and deterministic multi-impact folding; and
- registry-wide campaign discovery plus release public-inventory equality.

Ordinary campaign impact detection must remain analysis-free and non-mutating.
It may not rebuild reports, rewrite prose, rebind hashes, change lifecycle,
invalidate historical scientific results, or publish internal state.

## Suggested Package Decomposition

1. **TESTGATE-ALIGN-01 — Governance alignment and schemas.** Reconcile
   instructions/ADR-0021, define the impact-map, plan, receipt, campaign, and
   assurance-impact schemas, and add source-level consistency guards.
2. **TESTGATE-PLAN-01 — Mechanical planner and receipt verifier.** Implement
   changed-path, Cargo, explicit-edge, risk, plan, execution-inventory, and
   receipt behavior with retained-campaign fixtures.
3. **TESTGATE-CI-01 — Executor, coverage/CRAP, and CI lanes.** Add affected
   coverage, prototype combined full coverage, split CI/release orchestration,
   and record timing improvements without weakening gates.
4. **TESTGATE-ASSURE-01 — Campaign-head assurance currency.** Extend the
   assurance dependency planner and integrate it with campaign status.

Packages may be combined when their write sets and gates remain coherent, but
governance/schema authority must land before enforcement code.

## Complete Transition Inventory

The implementation planning pass must inventory and either edit or explicitly
disposition at least these concrete surfaces:

- assurance authorities:
  `docs/governance/scientific-assurance-v2-architecture.md`,
  `docs/governance/scientific-assurance-v2-source-build-contract.md`, and
  `docs/governance/scientific-assurance-dossier-lifecycle.md`;
- assurance data contracts: `assurance/v2/catalog.yaml`,
  `assurance/v2/schemas/`, every registered `assurance/v2/reports/*/report.yaml`,
  `assurance/v2/identity.lock.json`, review locks, and transaction receipts;
- assurance planner/identity/assembly/publication code under
  `crates/openwepp-assurance/src/v2/` and the corresponding
  `tests/integration/assurance_v2_*_contract.rs` suites;
- release transition/export/materialization:
  `tools/release/check_assurance_release_transition.sh`,
  `tools/release/check_assurance_dossier_exports.sh`, and
  `tools/release/materialize_assurance_v2_release.sh`;
- public/generated assurance catalogs and manifests under `assurance/`,
  `assurance/generated/`, and `usersum/assurance/`;
- gate runners and status contexts under `tools/release/`, `tools/local_ci/`,
  `.config/nextest.toml`, and `.github/workflows/release-gates.yml`; and
- GitHub branch/tag rulesets for `openwepp-evidence/**`, the dedicated evidence-
  publisher GitHub App and revocation record, atomic-push capability evidence,
  and provider-side captured configuration;
- open work packages, campaign declarations, retained gate evidence, and any
  automation that currently assumes one package equals one full qualification.

## Staged Adoption And Rollback

Adoption must proceed in this order:

1. land governance plus versioned schemas and source-level consistency guards;
2. run the new planner in shadow mode beside current commands without reducing
   current gates;
3. replay retained completed campaigns and record selector/receipt differences;
4. run nonblocking presubmit, post-submit, and periodic observation lanes;
5. disposition every selection miss, false narrowing, inventory mismatch, and
   root-currency discrepancy;
6. enable blocking increment selection only after acceptance thresholds are met;
7. cut campaign/release consumers over to verified certificates; and
8. retain a documented rollback that restores the previous conservative full
   runner without accepting new-format receipts as legacy passes.

Cutover must implement the standard's fixed scorecard and stable status
contexts, including external repository ruleset/branch-protection changes and
their captured before/after evidence. The planner context is never a substitute
for the aggregate execution context.

Campaigns active at schema introduction are imported through the bootstrap rule
with prior artifacts marked `LEGACY_UNVERIFIED` unless they can be independently
reconstructed and content-verified. No migration script may backdate intent,
deferral, or transfer currency.

## Acceptance Scenarios

The follow-up is not complete until fixtures prove at least:

1. an isolated unconsumed process crate selects its own component/contract
   tests and targeted CRAP, not unrelated snow and runner suites, only after the
   isolated-workspace-member proof succeeds;
2. activating that crate in production escalates to domain consumer gates;
3. changing a shared calendar or restart structure escalates to critical full;
4. deleting or filtering a test triggers global coverage/CRAP;
5. an unknown production path fails into critical escalation;
6. a documentation-only review edit reuses current executable evidence;
7. a contract edit invalidates the applicable authority root;
8. a campaign cannot certify with deferred obligations;
9. an unchanged exact campaign receipt is safely reusable at its permitted
   boundary;
10. a snow/canopy semantic watch marks campaign-head impact without changing
    the historical report or public surface; and
11. an explicit release runs release-only obligations against an exact current
    campaign certification;
12. changed constants, shared types/traits, macros/build inputs, and feature
    changes cannot produce an empty affected coverage surface;
13. affected/full doctest and placeholder/stub scans produce inventory-bound
    reusable receipts at their assigned boundaries;
14. an applicable A3 suite is selected and cannot be deferred for an affected
    process family;
15. a terminal-plan discovery remains pending or uses a governed campaign
    amendment without retroactive deferral;
16. the default backstop becomes overdue and blocks further increment closure;
17. exact campaign global CRAP evidence is reused at release when all bound
    inputs match, and invalidated when one changes; and
18. assurance add/rename/delete/unknown semantic cases conservatively block
    transfer without changing historical realization integrity;
19. additive/bounded test changes retain affected CRAP, while proven coverage
    loss or unknown contribution triggers global measurement;
20. the affected CRAP run includes every known covering test for its functions;
21. positive, missing, ambiguous, provisional, and stale A0 admission cases
    fail or pass exactly as governed;
22. A2/A4/A5/A6 divergence separates valid execution from investigation and
    prospective blocking promotion;
23. an omitted public assurance report blocks release, while complete historical
    exclusion leaves no public release object;
24. protected evidence branch/tag publication survives crash/retry, rejects partial or
    forged evidence, and verifies from a fresh clone without changing the
    subject source commit;
25. concurrent disjoint/overlapping increments replan against the exact current
    head and stale compare-and-swap loses safely;
26. local, wrong-repository/ref, replayed, revoked-issuer, and trust-escalated
    receipts cannot satisfy campaign/release;
27. ignored files, `$HOME`, `PATH`, network, mutable tools, and clock/random
    inputs invalidate hermetic reuse unless confined and bound;
28. matrix/shard IDs, DAG cycles, duplicate artifacts, predicate injection,
    prerequisite failure, and flaky retries reduce deterministically;
29. target/feature/build/proc-macro/dev dependency and staged/worktree cases
    produce stable Git/Cargo impact sets;
30. multiple assurance impacts fold independent of order and later changes
    reset exact-head currency; and
31. missing/canceled dynamic jobs, planner-pass/executor-fail, status migration,
    and rollback fail closed under the stable aggregate contexts; and
32. a bootstrapped `LEGACY_UNVERIFIED` obligation reaches certification only by
    adopted replan/rerun or an atomic named replacement, never direct promotion;
33. the complete A0–A6 outcome matrix covers `CONFORMS`, `DIVERGES`,
    `INCONCLUSIVE`, `NOT_EVALUATED`, execution failure, unpromoted A4/A5, and
    promoted A4/A5 predicates, including A1/A3 blocking cases;
34. `STALE -> PENDING -> FAIL/BLOCKED/PASS`, obligation supersession, and
    bootstrap replacement fold exactly and idempotently;
35. assurance refresh completion that omits one impact ID remains pending, and
    dangling, cross-report, cross-target, open-replacement, withdrawal, and
    valid atomic supersession cases fold correctly;
36. changing executor, arguments, prerequisite node IDs, acceptance predicate,
    retry policy, matrix/shard coordinate, or artifact namespace changes
    `node_id`, while runtime output does not;
37. Git rename produces delete plus add, and non-UTF-8 path, intent-to-add,
    unmerged index, unsupported sparse/submodule, ignored ambient-input, and
    index/worktree disagreement cases reject or plan exactly as governed; and
38. release rejects otherwise-current `NON_REUSABLE`, `SAME_EXECUTION`,
    unauthenticated receipt, wrong-subject envelope, and nonaccepted trust-class
    evidence.

## Protected Boundaries

- Do not reduce ADR-0021 thresholds or loosen exception evidence.
- Do not make selection agent-dependent.
- Do not allow generic skip, bless, or downgrade operations.
- Do not conflate a focused pass with campaign or release certification.
- Do not expose internal assurance currency as a public merit grade.
- Do not treat an implementation package as complete until its own selected
  increment gates pass, even when campaign-wide gates are deferred.

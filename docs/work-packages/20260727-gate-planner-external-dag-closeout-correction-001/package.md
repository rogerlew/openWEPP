# Gate Planner External DAG Closeout Correction

Package ID: `20260727-gate-planner-external-dag-closeout-correction-001`

Queue ID: `GATE-EXTERNAL-DAG-DC-01`

Status: `FROZEN / SUPERSEDED BY ADR-0043`

ADR-0043 freezes this incomplete planner prerequisite as historical evidence.
Resuming it requires explicit user authorization; no prospective work may infer
authority from its former `ACTIVE` status.

Authorization: the user's 2026-07-27 direction to complete CAL-04B, rework the
work-package scaffold, use delegated review, and disposition findings.

This defect-closure ExecPlan is maintained under
`docs/defect_closure_execplans.md`.

## Objective

Prospectively correct the authenticated external-DAG adapter at clean anchor
`76fc06a65a5a5f4305ff5130913e969387fc0aa3`, prove the canonical package chain,
and complete the prerequisite's focused and campaign-strength closure without
executing CAL-04B population or opening Harvard data.

## Defects

The superseded adapter package cannot authorize heavy closure because its
write set was widened after its original scaffold. Independent re-review also
found eight executable defects:

- Generation B consumes verifier capabilities before LIGHT and attempts to
  consume them again during READY audit construction;
- audit construction reconstructs the same external source inventory twice;
- audit construction admits the same durable ledger twice;
- audit/preflight evaluation precedes HEAVY STARTED, leaving early failures
  outside the required balanced lifecycle;
- publication recovery uses pathname-based restore/delete operations rather
  than descriptor-relative no-follow custody;
- verifier attestations have no enforced dispatch/transaction freshness;
- exact CSV reconstruction does not reject unknown or malformed headers; and
- external error classification depends on string matching and collapses
  distinct receipt, custody, ledger, and identity failures.

The retained reproducer is the independent review of clean commit
`76fc06a65a5a5f4305ff5130913e969387fc0aa3`.

## Included Scope

- one compatible verifier-capability verification/consumption transition;
- one independent audit inventory reconstruction;
- one canonical ledger admission and one verification of its resulting proof;
- STARTED-before-audit ordering with exactly one terminal record on every
  representable failure;
- descriptor-relative no-follow publication recovery with root-swap races;
- verifier-attestation freshness, exact CSV-header rejection, and typed
  external errors;
- adversarial and real consumer-path regression tests for those corrections;
- canonical package-chain validation from this prospective scaffold;
- focused gates, canonical pre-heavy audit, one fresh exact-head full-workspace
  run, authority-suite anti-evasion gates, dual review, dual verification, and
  prerequisite closeout;
- truthful HOLD/INVALID disposition of the superseded adapter package; and
- CAL-04B prerequisite handoff after this package closes.

## Excluded Scope

- calibration science, simulation physics, observations, objectives, domains,
  acceptance rules, or reconstruction algorithms;
- CAL-04B population, publication, freeze, Harvard opening, or holdout;
- history rewriting, changing the superseded package base, blessing existing
  implementation bytes, or weakening package-chain validation;
- broad planner refactoring unrelated to the named defects.

## Declared Write Set

- `crates/openwepp-gate-planner/src/external_dag.rs`
- `crates/openwepp-gate-planner/src/external_dag/audit.rs`
- `crates/openwepp-gate-planner/src/external_dag/custody.rs`
- `crates/openwepp-gate-planner/src/external_dag/tests.rs`
- `crates/openwepp-gate-planner/src/external_outputs.rs`
- `crates/openwepp-gate-planner/src/pre_heavy.rs`
- `crates/openwepp-gate-planner/src/publication.rs`
- `gate-policy/v1/schemas/external-dag-plan.schema.json`
- `gate-policy/v1/schemas/external-verifier-attestation.schema.json`
- `gate-policy/v1/schemas/publication-receipt.schema.json`
- `gate-policy/v1/schemas/holdout-opening-token-receipt.schema.json`
- `docs/work-packages/20260727-canopy-cal-04b-calibration-readiness-and-ensemble-execution-001/artifacts/external-dag-transaction-plan.json`
- `docs/work-packages/20260727-canopy-cal-04b-calibration-readiness-and-ensemble-execution-001/tools/execute-prefix.py`
- `docs/work-packages/20260727-canopy-cal-04b-calibration-readiness-and-ensemble-execution-001/tools/custody.py`
- `docs/work-packages/20260727-canopy-cal-04b-calibration-readiness-and-ensemble-execution-001/tools/freeze-verify.py`
- `docs/work-packages/20260727-canopy-cal-04b-calibration-readiness-and-ensemble-execution-001/tools/publish-results.py`
- `docs/work-packages/20260727-canopy-cal-04b-calibration-readiness-and-ensemble-execution-001/tools/test_execute_prefix.py`
- `docs/work-packages/20260727-canopy-cal-04b-calibration-readiness-and-ensemble-execution-001/tools/test_freeze_custody.py`
- `docs/work-packages/20260727-canopy-cal-04b-calibration-readiness-and-ensemble-execution-001/tools/test_publish_results.py`
- `docs/work-packages/20260727-gate-planner-external-dag-transaction-adapter-001/package.md`
- `docs/work-packages/20260727-gate-planner-external-dag-transaction-adapter-001/prompts/active/execute.md`
- `docs/work-packages/20260727-gate-planner-external-dag-transaction-adapter-001/prompts/archived/**`
- `docs/work-packages/20260727-gate-planner-external-dag-transaction-adapter-001/artifacts/implementation-gates.md`
- `docs/work-packages/20260727-gate-planner-external-dag-transaction-adapter-001/artifacts/implementation-review-findings.md`
- `docs/work-packages/20260727-gate-planner-external-dag-transaction-adapter-001/artifacts/final-disposition.md`
- `docs/work-packages/20260727-canopy-cal-04b-calibration-readiness-and-ensemble-execution-001/package.md`
- `docs/work-packages/20260727-canopy-cal-04b-calibration-readiness-and-ensemble-execution-001/artifacts/gate-evidence.md`
- `docs/work-packages/README.md`
- `docs/planning/canopy-phenology-assurance-roadmap.md`
- `docs/work-packages/20260727-gate-planner-external-dag-closeout-correction-001/**`

No other path is writable. Any newly discovered correction outside this set
requires a new prospective package; this package's write set must not widen.

## Security And Science Invariants

- The Rust planner remains the sole READY authority.
- Pre-LIGHT custody admission verifies capabilities without mutation. The
  READY audit owns the sole rename from `capabilities/` to
  `consumed-capabilities/`, returns an immutable consumed-root proof, and HEAVY
  verifies that proof without re-consumption.
- If LIGHT fails before audit consumption, capabilities remain reusable. After
  audit consumption, the attempt is non-retryable with those capabilities;
  restart requires distinct newly dispatched capabilities and retains the
  prior attempt's terminal disposition.
- The audit independently reconstructs source inventory exactly once.
- Durable ledger admission is evaluated exactly once and the returned proof is
  verified without re-admission.
- Every HEAVY STARTED record remains balanced by one typed terminal record.
- Harvard remains sealed; no token or Harvard content access is permitted.
- CAL command order, inputs, environment, outputs, custody semantics, and all
  scientific behavior remain frozen.
- The documented 4,320,812,180-byte production reconstruct trace constraint is
  accepted as a bounded focused-test limitation; actual authenticated CAL
  execution remains mandatory downstream consumer proof.

## Execution Plan

1. Commit this scaffold without production edits and obtain two independent
   read-only scaffold reviews.
2. Before Rust edits, update the catalog and roadmap so this successor is the
   only active adapter authority; retain the invalid predecessor bytes as
   evidence and record its non-executable disposition here.
3. Add named red adversarial fixtures that reproduce every defect and retain
   counter, ledger, capability-tree, and race outputs.
4. Implement the smallest typed correction in the declared files.
5. Run focused Nextest, warnings-denied Clippy, CAL Python integration tests,
   formatting, schema, documentation, diff, and line-count checks.
6. Obtain dual independent implementation review and close every finding.
7. Commit the complete intended closure state and run canonical
   `validate-package-chain` from this scaffold to that exact head.
8. Construct canonical READY and delegate the selected full-workspace and
   anti-evasion heavy gates once to the comparator runner.
9. Reconcile the exact terminal diff, obtain dual terminal verification,
   disposition the superseded package, archive prompts, and close.

## Prospective Gates

- `cargo nextest run -p openwepp-gate-planner`;
- `cargo clippy -p openwepp-gate-planner --all-targets --all-features -- -D warnings`;
- CAL-04B Python `test_*.py` discovery;
- formatting, JSON/schema, documentation, diff, exact-write-set, prompt-state,
  and line-count checks;
- canonical package-chain `PASS` from this scaffold commit;
- canonical pre-heavy report `READY`;
- fresh exact-head `cargo nextest run --workspace --profile full`;
- `bash tools/release/check_authority_suite_antievasion.sh`;
- `cargo nextest run --test auth11_required_suite_obligation_guards_contract`;
- two independent implementation reviews and two terminal verifications.

## Exit Criteria

- All eight executable defects have direct adversarial and consumer-path proof.
- The package chain and pre-heavy audit pass at the exact admitted commit.
- Every selected focused and heavy gate passes with retained receipts.
- The superseded package is truthfully closed as `HOLD/INVALID` and points to
  this completed successor.
- CAL-04B remains on tooling hold until this package closes, then receives a
  clean authenticated restart handoff.
- All findings are closed, dual verification passes, and active prompts are
  archived.

## Subagent Authorization

Subagent authorization: this package explicitly authorizes subagent
spawning/delegation to bounded implementation workers, two independent
read-only scaffold/implementation reviewers, two independent read-only
terminal verifiers, and the `comparator_suite_runner` for selected heavy gate
execution. Expected outputs are code/evidence within the declared write set
and compact read-only findings, receipts, artifact paths, and verdicts.
Workers are not alone in the codebase, own only their assigned files, and must
not revert concurrent edits.

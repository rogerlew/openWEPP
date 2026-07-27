# Gate Planner External DAG Closeout Correction

Package ID: `20260727-gate-planner-external-dag-closeout-correction-001`

Queue ID: `GATE-EXTERNAL-DAG-DC-01`

Status: `ACTIVE`

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
found three executable defects:

- Generation B consumes verifier capabilities before LIGHT and attempts to
  consume them again during READY audit construction;
- audit construction reconstructs the same external source inventory twice;
- audit construction admits the same durable ledger twice.

The retained reproducer is the independent review of clean commit
`76fc06a65a5a5f4305ff5130913e969387fc0aa3`.

## Included Scope

- one compatible verifier-capability verification/consumption transition;
- one independent audit inventory reconstruction;
- one canonical ledger admission and one verification of its resulting proof;
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
- Capabilities are verified without mutation until the one canonical
  consumption point; every required capability is consumed exactly once.
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
2. Add red adversarial fixtures that reproduce double capability consumption,
   duplicate inventory reconstruction, and duplicate ledger admission.
3. Implement the smallest typed correction in the declared Rust files.
4. Run focused Nextest, warnings-denied Clippy, CAL Python integration tests,
   formatting, schema, documentation, diff, and line-count checks.
5. Obtain dual independent implementation review and close every finding.
6. Commit the complete intended closure state and run canonical
   `validate-package-chain` from this scaffold to that exact head.
7. Construct canonical READY and delegate the selected full-workspace and
   anti-evasion heavy gates once to the comparator runner.
8. Reconcile the exact terminal diff, obtain dual terminal verification,
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

- All three executable defects have direct adversarial and consumer-path proof.
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

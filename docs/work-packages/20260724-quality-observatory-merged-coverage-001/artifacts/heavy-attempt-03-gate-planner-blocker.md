# Heavy Attempt 03: Gate-Planner Predecessor Blocker

Evidence class: Ran / Static.

Attempt:
`/home/workdir/openWEPP-quality-history/20260724-order3-local-attempt3-LuBliP`.

Admission:
`254fa4113e3213ac5695aaaa3b9fbf558c957a71f0f87811bf8e426e9784b162`.

Exact log:
`/home/workdir/openWEPP-quality-history/20260724-order3-local-attempt3-LuBliP/local/nextest-full.log`.

## Result

| Gate | Disposition | Evidence |
| --- | --- | --- |
| Pre-heavy admission | `PASS` | `full=2279`, `science-manual=36`, `workspace=2315`; exact partition admitted. |
| Instrumented `full` | `FAIL` | `2,279` run; `2,272` passed, including 8 slow; 7 failed; 31 skipped; `1,466.867s`. |
| Instrumented `science-manual` | `NOT RUN` | Full is a required predecessor and failed. |
| LCOV derivation and merge | `NOT RUN` | Both execution profiles must pass first. |
| Observational adjudicated CRAP | `NOT RUN` | Merged LCOV does not exist. |
| Compact publication | `NOT RUN` | No valid payload exists; published directory is empty. |
| Dual terminal verification | `NOT RUN` | There is no published evidence to verify. |

No `PermissionDenied` failure recurred. Tooling defect 02 is corrected.

## Exact Failing Tests

1. `executor::coverage_tests::ready_audited_heavy_preserves_import_and_final_receipt_bindings`
   — fixture still supplies retired `adjudicated-crap-v1`; current schema
   rejects it with `GATE-SCHEMA-REJECTED`.
2. `executor::tests::terminal_plan_detects_out_of_manifest_source_mutation_and_verifies_invalid_receipt`
   — expected `.github/probe.yml` was not created after the plan became invalid.
3. `planner::tests::terminal_reconciliation_reports_added_paths_and_escalation`
   — expected semantic reconciliation but observed the fixture's `Cargo.lock`
   removal/exact-checkout path first.
4. `pre_heavy::coverage_tests::exact_planner_output_reconstructs_through_the_public_audit_path`
   — fixture failed `GATE-COMMITTED-CHECKOUT-NOT-EXACT`.
5. `verifier::tests::coverage_tests::ready_audit_verification_preserves_order_and_exact_verdict`
   — fixture still supplies the retired quality gate and fails
   `GATE-SCHEMA-REJECTED`.
6. `verifier::tests::receipt_verification_reconstructs_identity_dag_inventory_and_artifacts`
   — fixture failed `GATE-COMMITTED-CHECKOUT-NOT-EXACT`.
7. `verifier::tests::verifier_accepts_truthful_fail_and_blocked_receipts`
   — fixture failed `GATE-COMMITTED-CHECKOUT-NOT-EXACT`.

## Ownership And Hold-Lift

Static: the Order-3 diff does not touch `gate-policy/**` or
`crates/openwepp-gate-planner/**`. The failing schema and test surfaces last
changed in predecessor commit
`8ce1ab22c1ecebff8a8696b21559ba781d6af59b`.

Those paths are outside this package's declared write set. A prerequisite
gate-planner tooling package must:

1. align executor/planner/pre-heavy/verifier fixtures with the Order-2
   `DEFERRED_TO_QUALITY_CI` schema and exact committed-checkout contract;
2. run all seven exact failing tests warnings-denied;
3. run the gate-planner crate regression required by its intent;
4. return this package to `ACTIVE` through an authorized fresh admission.

No further Order-3 heavy attempt is authorized until that prerequisite closes.

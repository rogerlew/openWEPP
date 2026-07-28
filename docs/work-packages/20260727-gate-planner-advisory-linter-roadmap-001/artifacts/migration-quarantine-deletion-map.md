# Migration, Quarantine, And Deletion Map

Classifications:

- `RETAIN_ADVISORY`: rewrite or extract only the read-only analytical idea.
- `MIGRATE`: bind the live responsibility to a direct owner before removal.
- `FREEZE_HISTORICAL_VERIFY`: read-only support for named old evidence only.
- `DELETE`: remove after consumer count reaches zero.

| Current surface | Target | Consumer/replacement owner | Deletion condition |
| --- | --- | --- | --- |
| `planner.rs`, `policy.rs` | `RETAIN_ADVISORY` | Order-3 `tools/validation/workplan-lint` | Copy only cited deterministic analysis; the old modules remain in the legacy crate until Order 4 |
| `repository.rs`, `documentation.rs`, `package_validation.rs` | `RETAIN_ADVISORY` | Order-3 `tools/validation/workplan-lint` | Reimplement only frozen allowlisted reads and declaration findings; no lifecycle verdicts |
| `nextest_inventory.rs` | `RETAIN_ADVISORY` | Order-3 `workplan-lint` suggested-command analysis | Retain inventory mapping only; no Nextest launch or admission semantics |
| `canonical.rs`, `error.rs`, `artifact_contract.rs` | `RETAIN_ADVISORY` | Order-3 canonical JSON/digest, typed error, and read-only artifact helpers | Copy only helpers exercised by the thin slice; no legacy-crate dependency |
| Legacy crate `main.rs`, `lib.rs`, and `Cargo.toml` | `DELETE` | Order-3 neutral command has a separate manifest/entrypoint | Every migrated or historical submodule reaches its deletion condition |
| `*_tests.rs`, inline legacy tests, and `external_dag/tests.rs` | `DELETE` | Order-3 focused advisory tests and any temporary historical-verifier tests | Delete with the exact legacy capability they characterize |
| `executor.rs`, `executor_source.rs`, `execution_context.rs`, `execution_nextest.rs`, `execution_temp.rs` | `MIGRATE` | Agents using direct commands; CAL direct executor from Order 2 | `.github/workflows/testgate-shadow.yml`, `tools/local_ci/testgate.py`, and CAL adapter consumers are removed |
| `pre_heavy.rs` | `DELETE` | None after Order-1 governance replacement | Operative pre-heavy requirements and controller calls are removed |
| `resume.rs` | `DELETE` | None; an interrupted direct command follows its owning package's explicit restart rule | `executor.rs` and controller resume imports are removed |
| `ledger.rs` | `FREEZE_HISTORICAL_VERIFY` | `openwepp-gate-plan verify-ledger` for retained campaign ledgers | No retained audit invokes the command and history has a documented direct read path |
| `verifier.rs` | `FREEZE_HISTORICAL_VERIFY` | `.github/workflows/testgate-shadow.yml` receipt-envelope verification and retained `verify-receipt*` audits | Workflow is retired and no named retained audit invokes the commands |
| `assurance.rs` | `MIGRATE` | `assurance/v2/AGENTS.md`, report leads, assurance steward, release owner | Direct validity, impact, approval, publication, campaign-transfer, and release-transfer disposition is documented and tested before planner state is deleted |
| `checkpoint_mirror.rs` | `FREEZE_HISTORICAL_VERIFY` | `.github/workflows/testgate-shadow.yml` recovery/history steps until that workflow is retired | Named workflow consumer is removed and retained history has a documented direct read path |
| `external_dag.rs`, `external_dag/**`, `external_outputs.rs` | `MIGRATE` | CAL-04B `execute-prefix.py` and `verify-external-transaction` move to the Order-2 direct executor | CAL direct path proves primary failure retention and no planner dependency |
| `publication.rs` | `MIGRATE` | CAL-04B `publish-results.py` moves only required custody/result durability to the Order-2 owner | Harvard and result durability replacement proofs pass |
| `tools/local_ci/testgate.py` | `DELETE` | Manual route plus direct documented commands | Operative guidance and workflows no longer invoke it |
| `tools/local_ci/testgate_qualification.py` | `DELETE` | Order-5 advisory utility protocol | Historical TESTGATE qualification remains documented and no workflow invokes it |
| `tests/integration/testgate_ci_executor_contract.rs` and its `Cargo.toml` registration | `DELETE` | None; it asserts legacy execution/pre-heavy source literals | Order 1 removes the guard and registration with the executor-authority clauses it protects |
| `tests/integration/testgate_align_authority_contract.rs` and its `Cargo.toml` registration | `MIGRATE` | Order-1 direct-governance guards and Order-3 advisory schema tests | Source-literal/blocking-schema assertions are removed; preserved anti-evasion assertions have direct owners |
| `gate-policy/v1/impact-map.json`, `gate-definitions.json`, `execution-matrix.json` | `MIGRATE` | Order-3 compact advisory mapping; `tools/release/check_science_contract_admission.sh` retains its direct anti-evasion inputs until separately amended | Direct anti-evasion consumer and thin slice use replacement sources; legacy planner consumers are gone |
| `gate-policy/v1/assurance-registry.json` | `MIGRATE` | Direct assurance dependency registry under `assurance/v2` governance | Assurance tooling no longer imports planner lifecycle state |
| `gate-policy/v1/schemas/external-*.json`, `holdout-opening-token-receipt.schema.json`, `publication-receipt.schema.json` | `MIGRATE` | CAL Order-2 direct executor/custody owner | Replacement schemas prove the Harvard boundary and old CAL consumers are gone |
| All other `gate-policy/v1/schemas/*.json` | `FREEZE_HISTORICAL_VERIFY` | Legacy `openwepp-gate-plan` verifier/ledger/package audit commands | No named retained audit or workflow consumes them |
| `gate-policy/v1/fixtures/valid/**`, `fixtures/invalid/**`, `fixtures/replay/**` | `FREEZE_HISTORICAL_VERIFY` | Legacy crate characterization and historical receipt/selection verification tests | All corresponding historical verifiers are retired |
| `gate-policy/v1/README.md` | `DELETE` | Replacement advisory-tool and history documentation | Every retained policy/schema has new documentation |
| `.github/workflows/testgate-shadow.yml` | `DELETE` | None; linter has no CI role | Historical run retention has a documented direct read path |
| `.github/workflows/quality-observatory.yml` TESTGATE identity check | `MIGRATE` | Optional operator-directed quality workflow | Workflow accepts its own explicit trusted source identity without TESTGATE or linter status |
| `.github/workflows/conservative-correctness.yml`, `release-gates.yml` direct commands | `MIGRATE` | Canonical campaign/release guidance and separately authorized workflows | Requirements and commands are explicit without planner authority |
| Existing receipts and ledgers under `/testgate-history` and repository artifacts | `FREEZE_HISTORICAL_VERIFY` | Maintainer audit through frozen `verify-receipt*`/`verify-ledger` until those consumers are retired | Never rewrite; verification is read-only and non-prospective |
| Existing CAL attempts/logs under `/home/workdir/cal04b-objects` and package artifacts | `FREEZE_HISTORICAL_VERIFY` | CAL-04B incident reconstruction and scientific audit | CAL package closes and retention owner confirms no verifier dependency |
| Frozen planner package-local `Status:` fields | `MIGRATE` | Order-1 status overlay in the exact package files below | Every incomplete file says `FROZEN / SUPERSEDED BY ADR-0043`; completed packages retain `COMPLETE`; catalogs agree |

The globs above are disjoint within `gate-policy/v1`: the named migrated schema
patterns take precedence, then “all other schemas”; fixtures and README are
separate. Order 4 must record an exact `rg`-based live-consumer count for every
row. A nonzero or unknown count makes that row ineligible for deletion; it does
not create a linter or modeling hold.

## Frozen Package Status Overlay

Order 1 reconciles these exact nearest-package authority surfaces:

- `20260727-gate-planner-external-dag-transaction-adapter-001/package.md`;
- `20260727-gate-planner-external-dag-closeout-correction-001/package.md`;
- `20260727-gate-planner-auth11-terminal-node-selection-001/package.md`;
- `20260727-gate-planner-auth11-fixed-inventory-test-provider-001/package.md`;
  and
- `20260727-testgate-first-attempt-ledger-bootstrap-001/package.md`.

Completed packages keep `COMPLETE`. Every incomplete planner prerequisite gets
`FROZEN / SUPERSEDED BY ADR-0043`, retains its original result and progress
history, and states that it cannot be resumed without explicit user
authorization. Catalogs receive the same status. This is an overlay, not a
rewrite of historical findings or evidence.

## Historical Policy Identity

Historical generation-17 verification binds the original testing strategy:

- SHA-256
  `74203b294dcea4c7f3ecb5fe4110a425d938d2ec75bde60cfc646a54fea3f5e9`;
- Git blob `ab8fe3e4db61df6691a96a11fa2034b90036bfb2`; and
- the source commit recorded by the historical receipt.

Order 1 records these literals in the frozen historical-consumer registry
before changing the live strategy. Historical verification resolves that exact
Git object and digest; it never re-derives old policy identity from the
post-Order-1 live path. Old receipts and policy bytes are not rewritten.

## Harvard Replacement Boundary

`publication.rs` and the external transaction adapter are not deletable until
Order 2 names and proves a smaller protected-data owner with all of these
properties:

1. a checksum-bound nonempty calibration freeze;
2. two independent read-only verifier PASS records;
3. a durable `OPENED_ONCE` transition completed before the first Harvard
   content read;
4. freeze, executable, command, input, digest, and lock checks;
5. no rerun after a crash following the open transition;
6. Harvard inputs mounted or opened read-only; and
7. the holdout process has no calibration-output write capability or path
   anywhere, including parent, child, cleanup, recovery, and publication paths.

Until that proof passes, Harvard remains sealed and the old adapter remains
read-only historical/migration reference. Failure of the adapter is not a
reason to route CAL through the future linter.

## Implementation Choice

Order 3 should create a clean neutral thin slice and selectively copy small
static-analysis concepts. It must not link the current crate as a library:
doing so would retain execution, state, publication, and custody dependencies
inside the advisory trust boundary. Order 4 deletes the old crate after
migration and historical-consumer review.

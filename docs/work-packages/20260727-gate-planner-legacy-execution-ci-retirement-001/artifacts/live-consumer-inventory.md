# Live Consumer Inventory

Evidence class: Ran + Static.

This matrix is keyed one-to-one to the 31 rows in Order 0's
`migration-quarantine-deletion-map.md`. A consumer is a unique file that
imports, invokes, reads, or operationally instructs use of the legacy surface.
Historical package/audit prose, generated code visualization, build output,
and negative-absence assertions are not consumers.

## Row-By-Row Disposition

| # | Order-0 surface | Legacy live consumers | Replacement owner count and paths | Disposition |
| ---: | --- | ---: | --- | --- |
| 1 | `planner.rs`, `policy.rs` | 0 (`C01`) | 2: `tools/validation/workplan_lint.py`, `tools/validation/test_workplan_lint.py` | Only neutral read-only analysis was independently implemented; legacy files deleted. |
| 2 | `repository.rs`, `documentation.rs`, `package_validation.rs` | 0 (`C02`) | 2: advisory source and test above | Frozen reads/declaration findings independently implemented; legacy files deleted. |
| 3 | `nextest_inventory.rs` | 0 (`C03`) | 2: advisory source and test above | Suggested-command mapping is in the neutral linter; inventory/execution module deleted. |
| 4 | `canonical.rs`, `error.rs`, `artifact_contract.rs` | 0 (`C04`) | 2: advisory source and test above | Only independently exercised JSON/digest/error helpers survive; legacy files deleted. |
| 5 | Legacy `main.rs`, `lib.rs`, `Cargo.toml` | 0 actionable; 1 negative test (`C05`) | 2: `tools/validation/workplan-lint`, `tools/validation/workplan_lint.py` | Legacy crate and workspace registration deleted. |
| 6 | Legacy/inline/external-DAG tests | 0 (`C06`) | 1: `tools/validation/test_workplan_lint.py` for retained advisory behavior | Deleted with the capabilities characterized. |
| 7 | Executor/context/Nextest/temp modules | 0 (`C07`) | 2: root `AGENTS.md`; CAL `tools/execute-prefix.py` | Agents run canonical commands directly; CAL uses its direct executor. |
| 8 | `pre_heavy.rs` | 0 actionable; 1 historical catalog mention (`C08`) | 2: root `AGENTS.md`, `docs/standards/testing-and-gate-strategy.md` | Deleted; direct validation requirements remain. |
| 9 | `resume.rs` | 0 (`C09`) | 1: `docs/work-packages/AGENTS.md` | Deleted; package-owned restart rules govern interruption. |
| 10 | `ledger.rs` / `verify-ledger` | 0 (`C10`) | 0 | Executable deleted because no named retained audit invokes it; raw historical bytes remain untouched. |
| 11 | `verifier.rs` / `verify-receipt*` | 0 (`C11`) | 0 | Executable deleted because no named retained audit invokes it; no replacement verifier was invented. |
| 12 | `assurance.rs` | 0 (`C12`) | 2: `assurance/v2/README.md`, `crates/openwepp-assurance/src/v2.rs` | Deleted after direct assurance roles/governance survived independently. |
| 13 | `checkpoint_mirror.rs` | 0 (`C13`) | 0 | Deleted; no recovery workflow consumer remains. `tools/local_ci/README.md` documents retirement, not a replacement mirror. |
| 14 | `external_dag*`, `external_outputs.rs` | 0 (`C14`) | 2: CAL `execute-prefix.py`, `validate_executor.py` | Deleted after Order-2 direct primary-failure proof. |
| 15 | `publication.rs` | 0 (`C15`) | 2: CAL `publish-results.py`, `test_publish_results.py` | Deleted after bounded direct publication/custody proof. |
| 16 | `tools/local_ci/testgate.py` | 0 (`C16`) | 2: root `AGENTS.md`, canonical testing strategy | Controller and tests deleted; manual direct route is operative. |
| 17 | Qualification and resolver scripts | 0 (`C17`) | 0 prospective replacement; Order 5 remains unscaffolded | Deleted; no workflow invokes qualification and no linter qualification CI exists. |
| 18 | `testgate_ci_executor_contract.rs` | 0 actionable; 1 negative-absence assertion (`C18`) | 0 | Deleted with the executor/pre-heavy clauses it characterized. |
| 19 | `testgate_align_authority_contract.rs` | 0 (`C19`) | 1: `advisory_linter_authority_contract.rs` | Renamed/migrated to direct governance, schema, history, and frozen-status guards. |
| 20 | Impact map, definitions, execution matrix | 0 old-path consumers (`C20`) | 2: direct admission script and authority contract | Four direct JSON/schema inputs survive under `tools/release/authority-policy`; definitions were reduced to five live A1 records. Nine unreachable AUTH11 rows and their dead definition were deleted; anti-evasion remains independently direct. |
| 21 | Assurance registry | 0 (`C21`) | 2: `assurance/v2/README.md`, `crates/openwepp-assurance/src/v2.rs` | Deleted; direct assurance ownership does not import planner state. |
| 22 | External/holdout/publication schemas | 0 (`C22`) | 3: CAL `holdout.py`, `validate.py`, `validate_scaffold.py` | Legacy schemas deleted after direct protected-data guards survived. |
| 23 | All other v1 schemas | 0 (`C23`) | 1: direct authority contract | Obsolete schemas deleted; two named direct schemas survive elsewhere and validate their two data inputs. |
| 24 | All v1 fixtures | 0 (`C24`) | 0 | Deleted with historical-verifier characterization; omitted Python-ledger fixture also deleted. |
| 25 | v1 README | 0 (`C25`) | 2: direct authority README and `tools/local_ci/README.md` | Deleted after replacement documentation landed. |
| 26 | `testgate-shadow.yml` | 0 actionable; 1 negative test (`C26`) | 0 | Deleted; historical read path documented directly. |
| 27 | Quality workflow identity check | 0 in quality workflow/controller/runner (`C27`) | 3: quality workflow, controller, runner | Migrated to exact source identity and quality-only occupancy/storage. |
| 28 | Conservative/release direct commands | 0 planner coupling; 1 negative-only test file with 3 assertions (`C28`) | 2: `conservative-correctness.yml`, `release-gates.yml` | Canonical commands remain direct; advisory linter has no CI role. |
| 29 | `/testgate-history` receipts/ledgers | 0 live operational references (`C29`) | 0 operational replacements | External historical bytes were not rewritten; `tools/local_ci/README.md` documents retirement, and the runner now uses `/quality-history`. |
| 30 | CAL attempts/logs | 0 actionable; 2 CAL negative tests (`C30`) | 2: CAL `retain.py`, package artifacts | Preserved under CAL's direct incident/scientific-audit owner; no planner verifier dependency. |
| 31 | Frozen planner package statuses | 0 resumable incomplete packages (`C31`) | 5 exact package status files: four frozen, one complete | Order-1 overlay remains exact; history was not rewritten. |

## Reproducible Scan

Rows `C01` through `C29` used `rg -l` over this operational scope:

```text
Cargo.toml Cargo.lock .config .github tools tests assurance AGENTS.md
docs/standards docs/decisions docs/ROADMAP.md docs/work-packages/README.md
```

The repeated command was:

```sh
rg -l --hidden --glob '!target/**' --glob '!code-viz/**' \
  '<row-regexp>' <operational-scope>
```

The exact row regexps were:

```text
C01 openwepp_gate_planner::(planner|policy)|openwepp-gate-planner/src/(planner|policy)
C02 openwepp_gate_planner::(repository|documentation|package_validation)
C03 nextest_inventory\.rs|openwepp_gate_planner::nextest_inventory
C04 openwepp_gate_planner::(canonical|error|artifact_contract)
C05 openwepp-gate-planner|openwepp_gate_planner
C06 executor_coverage_tests|external_dag/tests|openwepp-gate-planner/src/.+tests
C07 openwepp_gate_planner::(executor|executor_source|execution_context|execution_nextest|execution_temp)
C08 openwepp_gate_planner::pre_heavy|pre_heavy\.rs
C09 openwepp_gate_planner::resume|resume\.rs
C10 openwepp-gate-plan.+verify-ledger|verify-ledger
C11 openwepp-gate-plan.+verify-receipt|verify-receipt
C12 openwepp_gate_planner::assurance|openwepp-gate-plan.+assurance
C13 openwepp_gate_planner::checkpoint_mirror|checkpoint_mirror\.rs
C14 openwepp_gate_planner::external_(dag|outputs)|verify-external-transaction
C15 openwepp_gate_planner::publication|openwepp-gate-plan.+publish
C16 tools/local_ci/testgate\.py
C17 testgate_qualification|resolve_testgate_(comparison_base|intent_package)
C18 testgate_ci_executor_contract
C19 testgate_align_authority_contract
C20 gate-policy/v1/(impact-map|gate-definitions|execution-matrix)\.json
C21 gate-policy/v1/assurance-registry\.json
C22 gate-policy/v1/schemas/(external-|holdout-opening-token-receipt|publication-receipt)
C23 gate-policy/v1/schemas/
C24 gate-policy/v1/fixtures/
C25 gate-policy/v1/README\.md
C26 testgate-shadow\.yml
C27 TESTGATE|testgate
C28 openwepp-gate-plan|openwepp-gate-planner
C29 /testgate-history
```

`C27` was restricted to `.github/workflows/quality-observatory.yml`,
`tools/local_ci/quality_observatory*.py`, and `tools/ci/omarchy-runner`.
`C28` was restricted to the conservative/release workflows and their contract
test. `C30` used the same planner-name regexp over the CAL tools; both matches
are explicit negative-absence assertions. `C31` used `rg -n '^Status:'` over
the five exact Order-1 overlay packages and observed four
`FROZEN / SUPERSEDED BY ADR-0043` values and one `COMPLETE`.

Raw nonzero literal results were manually classified and are stated in the
matrix: `C05`, `C18`, `C26`, `C28`, and `C30` are negative assertions;
`C08` is historical catalog prose. They are not imports, invocations, reads,
or operative instructions. Every other legacy probe returned zero actionable
files.

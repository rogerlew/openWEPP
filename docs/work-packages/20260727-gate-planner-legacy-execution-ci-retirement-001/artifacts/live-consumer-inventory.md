# Live Consumer Inventory

Evidence class: Ran + Static.

Counts exclude historical work-package/audit prose, generated code
visualizations, build output, and tests whose only use of a retired token is a
negative-absence assertion.

| Migration-map surface | Pre-migration live owner | Terminal live consumers | Disposition |
| --- | --- | ---: | --- |
| Static planner/policy/repository/package modules | Legacy crate and controller | 0 | Deleted; neutral Order-3 linter is independent. |
| Legacy crate manifest, CLI, library, and tests | Workspace, TESTGATE workflow, legacy assurance test | 0 | Deleted with workspace dependency and Nextest profiles. |
| Executor/context/Nextest/temp modules | TESTGATE controller/workflow | 0 | Deleted; agents run canonical commands directly. |
| Pre-heavy/resume/ledger/verifier/checkpoint modules | TESTGATE controller/workflow and legacy tests | 0 | Deleted; no named live historical verifier consumer remains. |
| Assurance planner integration | Legacy campaign-currency test | 0 | Deleted; direct assurance governance/crate remains. |
| External DAG/output/publication modules | CAL adapter before Order 2 | 0 | Deleted after Order-2 direct executor/publication/custody proof. |
| `tools/local_ci/testgate.py` | TESTGATE workflow and Python tests | 0 | Deleted with tests. |
| TESTGATE qualification/resolvers | Legacy controller/tests | 0 | Deleted with tests. |
| `testgate-shadow.yml` | Planner/controller and quality qualification | 0 | Deleted after quality migration. |
| Conservative correctness workflow | Direct hosted rollback commands | 1 direct workflow | Renamed `conservative-correctness.yml`; no planner dependency. |
| Quality-observatory TESTGATE qualification/priority | Optional quality workflow/controller | 0 | Migrated to exact source identity and quality-only occupancy/lease. |
| Forest1 TESTGATE history naming | Optional quality workflow/runner | 0 | Migrated to preserved `openwepp-quality-history` at `/quality-history`. |
| Impact map and authority definitions | Science-contract admission guard | 1 direct script | Moved to `tools/release/authority-policy`; definitions reduced to six directly consumed A1/anti-evasion records. |
| Execution matrix and assurance registry | Legacy planner/test | 0 | Deleted; direct assurance and validation owners remain. |
| External/publication/custody schemas | Legacy crate after Order 2 | 0 | Deleted; CAL package-local direct schemas/guards remain. |
| Other v1 schemas and all fixtures | Legacy crate characterization | 0 | Deleted. |
| Historical generation-17 identity | Direct governance guard | 1 read-only test | Retained unchanged in `gate-policy/history`; exact Git blob and digest verification passes without planner. |
| Historical receipts/ledgers/CAL attempts | External historical evidence owners | Not rewritten | Outside repository deletion scope; bytes and meaning preserved. |
| Frozen planner package status overlay | Historical package/catalog records | Historical prose only | Retained unchanged. |

## Terminal Searches

The exact searches over `Cargo.toml`, `Cargo.lock`, `.config`, `.github`,
`tools`, `tests`, and `assurance` find no live planner executable, crate,
TESTGATE workflow/controller, resolver, qualification, or `gate-policy/v1`
consumer. Remaining source matches are negative-absence assertions in
`quality_observatory_workflow_contract.rs` and
`test_workplan_lint.py`.

Direct policy consumers are exactly:

- `tools/release/check_science_contract_admission.sh`; and
- `tests/integration/advisory_linter_authority_contract.rs`.

Historical registry consumers are the direct authority contract plus
documentation that names the immutable read path.

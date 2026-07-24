# Long-running TESTGATE fixture assessment

Evidence class: Static

Source: `target/cqr37-testgate-relocated-audit-r2/nextest/affected/junit.xml`
from the affected-quality run completed 2026-07-24.

## Finding

Seven tests consumed approximately 43.5 of the affected run's 47.9 minutes.
Five verifier tests independently constructed the same normalized receipt.
Because nextest executes each test in a separate process, the process-local
`OnceLock` did not share that fixture. Every construction ran live
`cargo nextest list` inventory discovery for the selected workspace, packages,
and test targets before synthesizing JUnit evidence.

This is fixture construction cost, not verifier algorithm cost.

## Classification

| Fixture family | Routine disposition | Rationale |
| --- | --- | --- |
| Verifier receipt, failure, inventory, envelope, and attestation semantics | Keep in affected/full with deterministic inventory and JUnit fixtures | These are production contract and coverage obligations; they do not require a live repository inventory to test their semantics. |
| Exact planner output reconstructed through the public audit path | Development-only live fixture | This deliberately repeats complete repository discovery and reconstruction. Run it when planner inventory enumeration, repository reconstruction, gate definitions, or audit bindings change. Exact-head TESTGATE execution independently exercises the same live production path. |
| Ready-audit end-to-end verification | Development-only live fixture; retain its fast local ordering/guard assertions in routine tests | Its admitted fixture archives and reconstructs an isolated repository. The routine verifier suite separately protects receipt identity, audit admission, execution-context ordering, and downstream delegation. |
| Executor source-mutation, fail/blocked, and pass lifecycle fixtures | Keep in affected/full | They directly protect execution and recovery failure semantics. Their combined cost is bounded relative to the redundant inventory fixtures. |
| Terminal reconciliation and ordinary planner semantic fixtures | Keep in affected/full pending fixture-root optimization | They protect downgrade, removed-gate, changed-path, and escalation behavior. They use fixed inventories; their cost is not grounds for reclassification without isolating the remaining repository/tool-record work. |

## Development-only trigger

The live reconstruction profile is required during development when a change
touches:

- `NextestInventory` or inventory-source selection;
- gate definitions, package/test selectors, or inventory hashing;
- repository reconstruction or exact-plan audit logic;
- JUnit normalization identity;
- executor/verifier reconstruction-root bindings.
- the READY-audit end-to-end reconstruction boundary.

It is not a routine work-package completion gate for unrelated science work.

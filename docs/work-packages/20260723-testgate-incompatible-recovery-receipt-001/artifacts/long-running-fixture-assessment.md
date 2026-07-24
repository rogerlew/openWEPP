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
| Local verifier receipt, failure, inventory, envelope, and attestation guards | Keep in affected/full | These production contract checks operate on bounded values and artifacts. |
| Exact receipt reconstruction variants for truthful non-pass, normalized JUnit, identity/DAG, and envelope subject bundles | Development-only live fixtures | Each variant invokes exact repository-plan reconstruction. Their local guards remain routine; their live repository binding is one development cohort. |
| Exact planner output reconstructed through the public audit path | Development-only live fixture | This deliberately repeats complete repository discovery and reconstruction. Run it when planner inventory enumeration, repository reconstruction, gate definitions, or audit bindings change. Exact-head TESTGATE execution independently exercises the same live production path. |
| Ready-audit end-to-end verification | Development-only live fixture; retain its fast local ordering/guard assertions in routine tests | Its admitted fixture archives and reconstructs an isolated repository. The routine verifier suite separately protects receipt identity, audit admission, execution-context ordering, and downstream delegation. |
| Local executor state-machine, source-mutation attribution, attempt, and receipt guards | Keep in affected/full | These directly protect execution and recovery semantics without compiling reconstructed repository workspaces. |
| Full executor pass, fail/blocked, source-mutation, public-stage, and READY-audited lifecycle fixtures | Development-only live fixtures | They compile and execute reconstructed repository workspaces. The exact-head TESTGATE is the production lifecycle proof; the explicit development cohort retains these variant checks when executor or reconstruction behavior changes. |
| Ordinary planner semantic fixtures | Keep in affected/full | They use bounded values and fixed inventories. |
| Complete terminal reconciliation fixture | Development-only live fixture | It repeatedly constructs complete repository plans to cover downgrade, removed-gate, changed-path, and escalation combinations. Local reconciliation guards remain routine. |

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

The fixtures use `cfg_attr(not(coverage), ignore = ...)`: normal affected/full
regression skips them, while fresh LLVM coverage/CRAP still executes them.
Consequently a science package does not pay this reconstruction cost unless its
mechanical affected-package closure includes `openwepp-gate-planner`; campaign
and release global quality retain complete branch evidence.

## Measured result

- Ran: routine `openwepp-gate-planner` affected profile: 172 passed, 14
  skipped, 60.432 seconds (61.06 seconds including command overhead).
- Ran: focused routine verifier/reconciliation surface: 9 passed in 0.013
  seconds (3.16 seconds including compilation/command overhead).
- Ran: workflow profile contract: passed.
- Ran: crate Clippy with warnings denied: passed.
- Ran: fresh affected CRAP without conditional coverage execution: failed with
  28 actionable rows, proving that unconditional ignore was unacceptable.
- Ran: fresh affected CRAP with `cfg(coverage)` re-enabling the cohort:
  intentionally stopped after approximately 18 minutes while still performing
  live inventory enumeration. This is partial diagnostic evidence, not a CRAP
  pass and not closure evidence.

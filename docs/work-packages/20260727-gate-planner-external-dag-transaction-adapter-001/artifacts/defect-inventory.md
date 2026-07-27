# Defect Inventory

Status: `FROZEN FOR SCAFFOLD REVIEW`

Evidence class: `Static`

| ID | Defect | Reproducer | Impact | Correction boundary |
|---|---|---|---|---|
| GED-001 | CAL-04B coordinator has no canonical READY barrier | `tools/execute-prefix.py` loops through `hubbard_producer` | population can start without authenticated admission | gate-planner external DAG transaction |
| GED-002 | Package observer emits noncanonical identity-incomplete receipts | `tools/observe.py` schema 01 | repository, runner, audit, prerequisite, and toolchain drift are not fully bound | canonical receipt and verifier |
| GED-003 | Standalone audit transport would be forgeable | §8.5.1 rejects self-hashed READY | Python-only correction cannot establish authority | same-process Rust transition |
| GED-004 | CAL-04B commands mutate the checkout | reconstruction and summary outputs target package artifacts | canonical executor correctly reports source mutation | confined external outputs plus post-transaction import |
| GED-005 | Harvard holdout crosses independent verifier process boundaries | freeze and two verifier receipts precede `holdout` | first audit cannot authorize later standalone HEAVY | second authenticated transaction |
| GED-006 | External scientific outputs lack canonical exhaustive preservation | current gate receipt artifact model covers planner process outputs | scientific bytes could escape receipt closure | versioned output-manifest contract |

The prepare-only CAL-04B attempt is `NON_REUSABLE` because its schema predates
the correction. It will be archived byte-preservingly; no receipt is imported
or rewritten.

# TESTGATE Defect Inventory

Status: scaffold baseline; execution must confirm source locations and add exact
failing/passing evidence.

| ID | Observed failure | Owning correction | Required proof |
| --- | --- | --- | --- |
| `TGCA-001` | Heavy gates can start without one final closure-readiness decision. | Planner audit artifact plus executor/workflow block. | Each cheap defect fixture stops before a heavy process starts. |
| `TGCA-002` | A new package is absent from its historical base and heading spelling can reject otherwise intended authority. | Scaffold-only schema validation, explicit scaffold-commit admission, and terminal reconciliation. | Uncommitted scaffold reports commit required without authorizing execution; committed package passes; stale, widened, malformed, or undeclared authority fails. |
| `TGCA-003` | Path omissions, Markdown/schema errors, missing artifacts, prompt state, and line counts can surface after heavy work. | Cheap prerequisite aggregation. | Every fixture fails pre-heavy with a typed reason. |
| `TGCA-004` | Planning, execution, and verification may create competing inventories or argument order. | Canonical admitted inventory and DAG plus independent verifier enumeration/comparison. | Admitted-inventory mutation or mismatch is rejected; independent enumeration must match; executor uses exact node IDs and vectors. |
| `TGCA-005` | Toolchain, binary, feature, fixture, environment, or runner mismatch can surface only after launch. | Complete identity preflight. | Each identity mutation blocks before heavy launch. |
| `TGCA-006` | Shared output paths and mutable caches can collide or poison later attempts. | Immutable attempt allocation, disjoint namespaces, content-addressed safe caches, mutation guards. | Collision, alias, cache-key omission, and source/index mutation fixtures fail closed. |
| `TGCA-007` | Full regression can be executed again solely to collect coverage. | Combined instrumented full node after parity proof. | Exact inventory/result parity and LCOV/CRAP compatibility; duplicate scheduling rejected. |
| `TGCA-008` | Unrelated documentation can invalidate executable evidence. | Separate root manifests and verifier reuse reasons. | Docs-only change preserves executable receipt currency but invalidates docs lint. |
| `TGCA-009` | Attempt history under ephemeral directories is insufficient for exact time/cost audit. | Hash-chained local ledger plus digest-bound trusted-workflow upload, indexing, retention, and re-ingestion. | Failed and successful pre/post-receipt attempts survive process and runner/job reset and render an exact timeline. |
| `TGCA-010` | Repeated infrastructure workarounds can trigger repeated expensive retries without correcting tooling. | Typed defect records and recurrence stop rule. | First policy retry is retained; the same cause blocks a second heavy retry. |
| `TGCA-011` | A late-node failure can cause a new attempt to rerun the entire successful prefix. | Verified cross-attempt import of current, target-reusable per-node receipts and context-aware resume. | Recovery reuses each eligible PASS by receipt ID; rejected receipts record exact §10.4 trust/reuse/context reasons. |

No row may close on prose alone. Each correction needs a failing-before fixture,
a passing implementation test, and real helper/workflow-path acceptance.

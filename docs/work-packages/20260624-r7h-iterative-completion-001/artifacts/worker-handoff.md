# Worker Handoff

Status: closed `OPT-IN` by operator decision on 2026-06-24.

Do not resume `HOLD-R7H-TYPED-FROST-FREEZE-PARITY`. The remaining
direct-vs-compatibility frost divergence is now a contract-tracked delta under
reopened `GAP-SNOWFREEZE-002`, not an R7H blocker to grind.

Retained state:

- Direct default-candidate timing: `61.40 s`, within the `<=91.2 s` R7H budget.
- Explicit direct timing: `64.19 s`, within budget.
- Direct manifests report `compatibility_edge_invocations=0`.
- Direct default and explicit direct outputs match each other.
- Default compatibility and explicit rollback outputs match each other.
- Compatibility, rollback, and shadow paths remain intact.
- Direct remains opt-in; default activation is not approved.

Next work:

- Author a frost-depth fidelity Defect-Closure ExecPlan before code.
- Validate heat-flow/frost-depth behavior against historic frost-depth
  observations via site hillslope models, using ADR-0017 external-authority
  discipline and `SC-SNOWFREEZE-001` `INV-SNOWFREEZE-047`.
- Use pilot fixtures under `tests/fixtures/snowfreeze_observed/`.
- Treat tolerances from the `GAP-SNOWFREEZE-002` Observation Validation
  Addendum as provisional until hydrology-reviewer ratification.

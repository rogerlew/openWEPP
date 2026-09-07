# Codex Execution Plans

"The plan and its explicit, versioned dependencies contain everything required to execute the task. The plan states the objective, current state, authority, write set, steps, acceptance criteria, and recovery/handoff information; it does not duplicate referenced procedures or historical narratives."

## Plan contract
State objective/observable outcome, authorization/protected boundaries, current
state, exact write set, steps/commands, acceptance and recovery. Maintain
Progress, Surprises & Discoveries, Decision Log, Outcomes & Retrospective as
concise sections or explicit handoff links. Bind required dependencies to exact
paths, relevant sections and revision/content hashes; freeze new documents
before review. Check bindings on continuation. Changed authority/criteria need
explicit impact review; missing references block dependent action. Never follow
acceptance drift silently. Versioned dependencies supply context without
teaching repository basics or duplicating procedures/historical narratives.

The common execution/closure contract is docs/work-packages/AGENTS.md.
Authors follow docs/work-packages/role-authoring.md. Preserve autonomous steps,
observable tests and recovery. Scaffold before substantive edits; freeze
acceptance. Direct validation follows docs/standards/testing-and-gate-strategy.md.
DC additionally binds docs/defect_closure_execplans.md; triggered specialized
procedures route through the common guide. Historical artifacts retain original
meaning; this guidance and active templates are prospective.

## Comparator authority
ADR-0017 makes comparator agreement a flag rather than a target for all
comparator/ledger work packages. ExecPlans that classify legacy-comparator
residuals must require like-for-like unit and lineage-stage proof before any
`OPENWEPP-DEFECTIVE` verdict, must include `HARNESS-SURFACE-MISMATCH` as a peer
verdict for unit or surface-pairing defects, must prohibit waiving independent
correctness authority for openWEPP-defect labels, and must keep `HOLD` findings
owned by a named follow-on gate rather than unscoped.


## Compact skeleton
Objective/authorization; current state; versioned dependencies; intended paths;
steps/validation commands; frozen acceptance; Progress; Surprises & Discoveries;
Decision Log; Outcomes & Retrospective; recovery/worker-handoff link.
The common guide owns independent reviews, dispositions, verifications,
line-count and non-deferral obligations.

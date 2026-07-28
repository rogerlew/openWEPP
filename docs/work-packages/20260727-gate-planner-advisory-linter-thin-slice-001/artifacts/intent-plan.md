# Intent Plan

Evidence class: `Static`

Implementation intent: `agent-tooling / read-only advisory analysis`.

The thin slice reads bounded repository/package state and emits advice. It
never executes advice, owns lifecycle state, or becomes a prerequisite. The
manual route remains complete and independent.

Terminal diff expectations:

- new neutral Python tool, tests, fixtures, and operator documentation;
- Order 3 package and catalog/roadmap status;
- no legacy planner, workflow, policy, CAL, science, or protected-data edit.

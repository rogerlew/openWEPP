# ASSURE-04B Review A

Evidence classes: Static and Ran

Review mode: independent read-only internal coding-agent review; not scientific
peer review.

Ran: format, focused all-target clippy, assurance crate tests, three assurance
integration suites, and diff checks passed on the review freeze. The reviewer
recorded no tree edits.

## Findings

### ASSURE04B-A01 — High

`resolve_node_state` returned intrinsic stale before inspecting prerequisite
states. A stale consumer could mask a blocked prerequisite, contrary to the
planner contract.

Required remedy: precedence must be intrinsic blocked, blocked prerequisite,
intrinsic stale, changed prerequisite, then current; add unit and real-report
regression coverage.

### ASSURE04B-A02 — High

The inherited confinement reader checked path metadata and later reopened the
path with `fs::read`. A concurrent component/final replacement could substitute
a symlink between check and read.

Required remedy: descriptor-relative no-follow opens; validate and read the
same descriptor; add bounded substitution/race coverage.

### ASSURE04B-A03 — Low

Focused test counts and line-count artifacts predated the final test-only review
amendment. The quick workspace result also required renewal after remediation.

Required remedy: refresh all count-bearing evidence before disposition/heavy
closure.

Review recommendation: HOLD until accepted findings are fixed and verified,
then run mandatory heavy/CRAP and terminal verification gates.

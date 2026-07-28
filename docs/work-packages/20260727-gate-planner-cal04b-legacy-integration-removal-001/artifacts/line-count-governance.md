# Line-count Governance

Evidence class: `Static`

Order 2 changes no Rust source file. Therefore the work-package thresholds of a
warning at 2,000 lines and mandatory refactor for a nonexempt file at 3,000
lines are not triggered.

The Python changes deliberately delete the planner coordinator, capability and
attestation protocol, and external publication wrapper. The terminal diff has
more deletions than insertions in the CAL integration surface; no new
infrastructure subsystem was introduced.

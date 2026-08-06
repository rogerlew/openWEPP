# Modularization Plan

Evidence class: `Static`.

Baseline `runoff_reconciliation.rs`: `3,177` lines.

Move `resolve_stage3_liquid_routing` and its thermal/layer helpers into
`runoff_reconciliation/stage3_solver.rs`. Move carrier and evaluation operators
into `runoff_reconciliation/stage3_solver/evaluation.rs`. Retain the public snow
partition entry point and unrelated runoff/frost/density helpers in the original
file. The original caller reaches one `pub(super)` solver seam; all other moved
items remain private to the extracted module tree.

No opportunistic cleanup is allowed. Extraction and evaluation changes are
separate commits and parity checks.

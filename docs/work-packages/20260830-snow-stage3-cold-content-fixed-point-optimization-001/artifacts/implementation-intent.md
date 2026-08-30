# Implementation intent

Status: `EXECUTED`

Evidence mode: `Static`

Critical kernel-numerics increment. Diagnose first; if correction is selected,
use contract-first sequencing. No tolerance, constitutive physics, persistence,
receipt, restart, output, event, or topology change is intended. Exact terminal
diff controls final validation selection.

Terminal reconciliation confirms the implementation stayed inside this
intent. The only production behavior change is unpublished fixed-point iterate
selection; the terminal-endpoint method was split mechanically for line-count
governance.

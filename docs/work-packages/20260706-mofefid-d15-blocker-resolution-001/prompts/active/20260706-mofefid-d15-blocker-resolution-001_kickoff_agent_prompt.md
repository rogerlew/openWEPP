# Kickoff Agent Prompt

Execute package
`docs/work-packages/20260706-mofefid-d15-blocker-resolution-001/`.

Subagent authorization: this package explicitly authorizes
spawning/delegating to `comparator_suite_runner`, `explorer`,
`rust_code_reviewer`, and `rust_qa_reviewer` subagents for timing/comparator
runs, source/authority audit, implementation review, verification, and heavy
gate execution. Expected outputs are compact findings, timing metrics, gate
metrics, log paths, and package-local review or verification artifact text.
Write access is read-only unless a subagent is explicitly assigned a bounded
implementation fix in the intended write set.

First close the terminal-bin/day-boundary `NegativeOutletBin` blocker from the
D15 rerun. Only attempt active owner wiring after the D10B-corrected H2637
routed timing path completes and the activation readiness audit is green.

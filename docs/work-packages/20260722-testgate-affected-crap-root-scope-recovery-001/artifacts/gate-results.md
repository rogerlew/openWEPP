# Gate Results

Status: review corrections pass focused gates; repeat dual review pending.

Ran: Python checker suite passes 29/29. It covers valid production scope,
unknown scope, measurement-only, nested, test-only, and out-of-tree target
rejection (including a test target placed under `src/`), exact preflight JSON
binding/tamper rejection, and a real driver probe proving root `openwepp` fails
before `llvm-cov.log` exists.

Ran: actual repository preflight rejects `openwepp` with exit 2 and explicit
`requires global quality`; no coverage process starts.

Ran: Bash syntax passes. Formatting passes. The executor/policy integration
targets pass 19/19 in 37.739 seconds, and their target Clippy command passes.
Python byte compilation and diff hygiene pass.

Ran: package audit is `READY` with zero unauthorized paths; audit ID
`2f552572ba88a19143528484fa80eb5d2dbd15d2fc6f11dd0112be72e0949914`.

Ran: direct evaluation of the retained root-package CRAP JSON against the full
production dependency set reports 1,533 actionable rows. This static retained
evaluation proves dependency expansion cannot provide valid affected closure;
no coverage traversal was rerun.

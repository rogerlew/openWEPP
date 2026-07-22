# Gate Results

Status: focused checker/driver correction gates passing; dual review pending.

Ran: Python checker suite passes 23/23. It covers valid production scope,
unknown scope, measurement-only rejection, exact preflight JSON, and a real
driver probe proving root `openwepp` fails before `llvm-cov.log` exists.

Ran: actual repository preflight rejects `openwepp` with exit 2 and explicit
`requires global quality`; no coverage process starts.

Ran: Bash syntax passes. Formatting passes. The executor/policy integration
targets pass 19/19 in 37.141 seconds, and their target Clippy command passes.
Python byte compilation and diff hygiene pass.

Ran: direct evaluation of the retained root-package CRAP JSON against the full
production dependency set reports 1,533 actionable rows. This static retained
evaluation proves dependency expansion cannot provide valid affected closure;
no coverage traversal was rerun.

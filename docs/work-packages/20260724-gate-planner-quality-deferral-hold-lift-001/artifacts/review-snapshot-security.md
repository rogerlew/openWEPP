# Snapshot Isolation Security Review

Evidence class: Static / Ran.

Result: `PASS`.

No findings.

`git clone --local --no-hardlinks` isolates the checkout and Git objects. The
private clone binds only the canonical repository-local `.venv`, overwrites
its private exclude with exact bytes `/.venv\n`, and requires complete
porcelain status to be empty before committed observation. Other untracked
paths remain visible.

Production clean-checkout and audit guards are unchanged. Existing negative
coverage still rejects a dirty execution checkout, and the exact public-audit
reconstruction consumer remains real. The clone and artifact roots retain
precise RAII or explicit cleanup.

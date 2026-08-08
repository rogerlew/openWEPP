# Default-Off Boundary Evidence

Status: `queued`

Evidence mode: `not-run`

Prove that only tests or explicit package-local diagnostics consume the new
crate, no existing production manifest or call site depends on it, no runtime
selector/default changed, no public output was added, and hydrologic state is
read-only at the API boundary.

# Invalidated Pre-Terminal Evidence

The first complete 16-cell run used release binary
`9a60ecf599ac49ef1c20fd4877882587e8d781b0ec90637acaa7e14a1f40f547`.
It passed all diagnostic closures and established the same scientific result,
but the subsequent Clippy-driven extraction of the trace suffix helper changed
the exact release binary. These artifacts are retained as invalidated
provenance and do not satisfy terminal current-head evidence. The terminal run
must use the rebuilt binary and supersedes this directory.

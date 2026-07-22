# Gate Results

Status: focused correction gates passing; dual review pending.

Ran: `cargo fmt --all -- --check` passes.

Ran: exact combined Nextest selection passes 2/2 in 40.581 seconds:

- `workspace_and_bound_context_plan_build_preserves_exact_graph_selection`;
- `local_verifier_guards_cover_retry_prerequisite_audit_and_binding_edges`.

Ran: `cargo clippy -p openwepp-gate-planner --all-targets -- -D warnings`
passes.

Ran: the exact failed gate command,
`cargo clippy --workspace --all-targets -- -D warnings`, passes in 13.58
seconds. No doctest, full Nextest, CRAP, or other HEAVY node was run locally.

Ran: canonical package admission from scaffold `3e9a1427` is `READY` with zero
unauthorized paths and audit ID `24743747...23ff1` at correction review.

Static: dual independent implementation reviews pass at exact correction commit
`8b26689c`. RTR-033 is closed by durable ledger entry `1a40c57e...64990`.

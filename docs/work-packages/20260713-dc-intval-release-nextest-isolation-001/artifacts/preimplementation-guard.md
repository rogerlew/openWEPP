# Preimplementation Guard

Status: `STATIC-RED-GATE-WITH-REPORTED-RUN`

Evidence class: **Static**. The interactive run output was observed but not
archived and does not carry terminal closure.

Command:

```text
cargo nextest run --test auth04_release_gate_authority_stack_contract -E 'test(=intval_rel001_release_workspace_gate_uses_nextest_process_isolation)'
```

Static comparison against start commit `1a6a0349` proves that the guard requires
the full-profile nextest command while the pre-fix release script contains only
`cargo test --workspace`. The interactive nextest run reported the expected
failure, but its raw output was not archived.

The archived exact release log is terminal run evidence: after the one-line
correction its full profile passed all 1,945 selected tests, including this
source guard.

# Plan Reconstruction Failure

Evidence class: `Ran` and `Static`

Committed source range:
`ec5d725fa89bebae516a7ccc69d3a199a8c91d11..9f482f70d5112bc6f99b3f48820312bc7f15329b`.

The local helper generated intent plan `96a35c5f...` and terminal plan
`3883dba1...`: CRITICAL, 12 nodes, and 2,184 unique inventory items. Independent
`openwepp-gate-plan reconcile` rejected it with
`GATE-TERMINAL-RECONSTRUCTION`.

A second terminal-plan invocation used the same base, head, authorized paths,
boundary, campaign, predecessor intent ID, binary, repository, and policy. Its
canonical JSON differs from the rejected plan only in:

- `execution_context.environment_manifest_sha256`;
- derived `execution_key`; and
- derived `plan_id`.

Every source, changed object, policy/configuration/tool/fixture identity, node,
argument, prerequisite, and inventory field is identical. Static inspection
locates the mechanism in `environment_record`: it hashes every process variable.
The validated gate registry permits only `PATH`, `CARGO_HOME`, `RUSTUP_HOME`,
and `RUSTUP_TOOLCHAIN`; invoker bookkeeping such as `_` is undeclared but changes
across the Python helper and direct CLI.

Disposition: both plans are non-executable. The package is prospectively amended
before planner edits to close `TESTGATE-ENV-PROJECTION-DETERMINISM-01` by binding
the union of policy-declared allowlists and retaining every other execution-
context identity.

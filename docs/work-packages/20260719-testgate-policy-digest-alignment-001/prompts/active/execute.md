# Execute TESTGATE Policy Digest Alignment

Scope: local repository defect closure. Edit only the package write set; no
network, workflow, runner, production Rust, test, schema, or science mutation.

Execution mode: package-end-to-end.

Required reading:

- Core: `AGENTS.md`, `docs/work-packages/AGENTS.md`,
  `docs/defect_closure_execplans.md`,
  `docs/standards/testing-and-gate-strategy.md`, this package, and
  `artifacts/required-reading-map.md`.
- Conditional: `tests/AGENTS.md` before interpreting focused tests.
- On-demand: `gate-policy/v1/impact-map.json`, planner policy loader, and the
  three TESTGATE integration contracts.

Required-reading budget: 147,125 local bytes, `OK` (`<=400000`); map:
`artifacts/required-reading-map.md`.

Task: close `TESTGATE-POLICY-DIGEST-01` end-to-end. Reconstruct the exact
strategy digest, change only the stale `policy_sha256` field, run only the
declared focused gates, complete review and terminal disposition, and archive
this prompt.

Constraints: do not broaden or weaken policy; do not edit tests to obtain PASS;
do not run a workspace/full suite, coverage, CRAP, Clippy, deny, workflow, or
runner action. Rerun only invalidated focused gates.

Subagent requirement: two independent read-only reviewer/verifier roles. This
prompt explicitly authorizes subagent spawning/delegation by the parent for the
review scope and outputs defined in `package.md`.

Autonomy: execute through disposition unless a declared hard boundary is
proven.

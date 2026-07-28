# Execute Legacy Planner And CI Retirement

Scope: local repository control-plane retirement; flat-file reads/edits only;
no external connectivity or external-system action is required.

Execution mode: package-end-to-end.

Execute every phase in `package.md` sequentially through disposition.

Required reading:

- Core and conditional files in `artifacts/required-reading-map.md`.
- On-demand direct consumers for each migration row before editing it.

Required-reading budget: 116,035 bytes, `OK`; map:
`artifacts/required-reading-map.md`.

Retire only zero-consumer legacy planner/TESTGATE surfaces. Preserve direct
authority, immutable historical identity, optional quality observation,
anti-evasion, Harvard custody, and direct canonical commands. Do not scaffold
Order 5 or run CAL/model/Harvard work.

Subagent requirement: REQUIRED. Spawn `comparator_suite_runner` for broad
full-workspace Nextest/Clippy closure; do not run those heavy suites on the
parent model unless unavailable with recorded evidence. This prompt explicitly
authorizes subagent spawning/delegation to that runner and two independent
read-only reviewers plus two independent read-only verifiers for the scopes in
`package.md`; outputs are compact results and findings; write access is
read-only.

Autonomy: execute end-to-end without requesting user direction unless a
declared hard boundary is proven.

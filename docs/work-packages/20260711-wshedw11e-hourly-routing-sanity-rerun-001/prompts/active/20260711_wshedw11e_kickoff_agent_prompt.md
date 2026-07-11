# WSHED-W11E Kickoff Prompt

Scope: local repository routing-characterization task; flat-file reads/edits
and local executable tests only; no external connectivity.

Execution mode: package-end-to-end. Execute every phase in `package.md`
sequentially through disposition.

Required reading: Core is root `AGENTS.md`, `docs/codex_exec_plans.md`,
`docs/work-packages/AGENTS.md`, the catalog, this package, and W11C/W11D final
dispositions. Conditional is crate, science-contract, prompt, and local-CI
governance when triggered. On-demand is the three W11D-amended contracts.
Record exact bytes and triggers in `artifacts/required-reading-map.md`.

Files: only the bounded documentation write set in `package.md`.

Task: rerun the current seven-test real watershed-CLI sanity suite in debug and
against an exactly rebuilt release binary, compare with W11C historical
findings, classify current behavior, and complete every required gate.

Constraints: do not change production code, contracts, tests, fixtures, or
canonical guards. Treat typed E003 as success only for declared inadmissible MC
grids and prove admitted static/dynamic MC routes execute separately.

Conservation/output acceptance: use external HBP inputs and public terminal
outputs; do not treat the writer balance as independent proof; bind the result
to W11D's independent reconstruction evidence.

Subagent requirement: REQUIRED. Spawn `comparator_suite_runner` for release,
clippy, erosion, full, and deny. This prompt explicitly authorizes subagent
spawning/delegation to that runner and two bounded reviewer/verifier agents;
outputs are compact metrics and named package artifacts; write access is
restricted by `package.md`.

Autonomy: execute end-to-end without requesting user direction unless a hard
external blocker is proven.

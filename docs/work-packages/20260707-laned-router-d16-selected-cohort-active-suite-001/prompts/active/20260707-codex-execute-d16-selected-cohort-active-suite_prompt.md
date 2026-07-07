# Codex Execution Prompt: D16 Selected-Cohort Active Suite

Scope: local repository science-contract/kernel evidence task; flat-file
reads/edits only; no external network actions required.

Execution mode: package-end-to-end (default).

Phase plan: execute all phases in `package.md` sequentially through
disposition.

Required reading:

Core:
- `AGENTS.md`
- `docs/work-packages/AGENTS.md`
- `docs/standards/AGENTS.md`
- `docs/standards/prompt-wording-guidance.md`
- `docs/specifications/science-contracts/AGENTS.md`
- `docs/contracts/openwepp-management-lanuse-authority-contract.md`
- `docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md`
- `docs/specifications/science-contracts/contracts/SC-OFEROUTE-002.md`
- `tools/owcmp/AGENTS.md`
- `tools/owcmp/specification.md`
- `docs/work-packages/20260707-laned-router-d16-selected-cohort-active-suite-001/package.md`

Conditional:
- `/home/workdir/wepppy/AGENTS.md`
- `/home/workdir/wepppy/wepppy/nodb/mods/disturbed/route_coefficients.py`
- `/home/workdir/wepppy/wepppy/nodb/mods/disturbed/disturbed.py`
- `/home/workdir/wepppy/wepppy/wepp/management/managements.py`
- `tests/fixtures/AGENTS.md`

On-demand:
- `tools/owcmp/suites/*.json`
- Prior D16 packages named by `package.md`

Required-reading budget: local package pre-read is WARN; map:
`artifacts/required-reading-map.md`.

Files:
- `docs/work-packages/20260707-laned-router-d16-selected-cohort-active-suite-001/**`
- `docs/work-packages/README.md`

Task: execute the package objective end-to-end for the declared scope.

Constraints: canonical `SC-*` authority; typed guards; no silent defaults; no
surrogate route coefficients; no legacy-field bridge; no mutation of
`/wc1/runs/*`; generated external-member active inputs must be package-local
and sourced from Disturbed class bindings plus WEPPpy Disturbed native
management production.

Real consumer proof: active plain and explicit hybrid must run through
`openwepp-cli-hill` with `OPENWEPP_LANED_ACTIVE=1`, not through a shadow,
wrapper, or parser-only harness.

Subagent requirement: REQUIRED for review/verification if subagent tooling is
available. This prompt explicitly authorizes subagent spawning/delegation to
comparator/timing, science-authority review, package QA, and verification
subagents for selected-cohort run verification, delta review, gate review, and
package disposition review. Outputs: compact findings plus artifact/log paths.
Write access: read-only unless a bounded package-artifact fix is explicitly
assigned.

Autonomy: execute package phases end-to-end and update required artifacts
without requesting additional user direction unless hard-blocked.

Outputs: update package artifacts/disposition for all completed phases.

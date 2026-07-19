# TESTGATE-PLAN-01 Kickoff

Scope: local repository gate-policy engineering; flat-file reads/edits and
read-only local Git/Cargo inspection only; no external connectivity or external
system mutation.

Execution mode: package-end-to-end.

Phase plan: execute every phase in `package.md` sequentially through final
disposition.

Required reading:

- Core: `AGENTS.md`, `crates/AGENTS.md`, `tests/AGENTS.md`,
  `docs/work-packages/AGENTS.md`, `docs/standards/AGENTS.md`, `package.md`,
  `docs/standards/testing-and-gate-strategy.md`, and
  `gate-policy/v1/README.md`.
- Conditional: `docs/standards/prompt-wording-guidance.md` for package/prompt
  edits; the six v1 schemas for their implementing module; retained package
  artifacts for replay construction.
- On-demand: Cargo manifests, tests, and Git history needed to implement and
  prove an exact behavior.

Required-reading budget: 154,685 local bytes, `OK`; map:
`artifacts/required-reading-map.md`.

Files: only the declared write set in `package.md`.

Task: implement and execute TESTGATE-PLAN-01 end-to-end in shadow mode.

Constraints: deterministic fail-closed planning; typed errors and argument
vectors; canonical identities and transitive roots; exact receipt and reuse
verification; no executor, CI cutover, gate reduction, evidence publication,
campaign certification, or assurance mutation.

Subagent requirement: REQUIRED only for the single terminal heavy closure set.
This prompt explicitly authorizes subagent spawning/delegation to the
closure-runner, two independent reviewer, and two independent terminal-verifier
roles described in `package.md`; outputs are compact command evidence and
findings; write access is read-only except generated build/coverage output for
the closure runner.

Autonomy: execute package phases end-to-end without requesting additional user
direction unless hard-blocked. Do not rerun successful broad gates merely to
refresh evidence.

Outputs: maintain package artifacts and disposition truthfully with `Static:`
and `Ran:` evidence labels.

# Execute the Stage-3 cold-content fixed-point optimization

Execution mode: package-end-to-end review closure

Autonomy: execute every remaining package phase through truthful disposition
without asking for intervention unless a genuine authority, safety,
external-state, or operator-decision blocker is reached.

Subagent authorization: this prompt explicitly authorizes subagent
spawning/delegation to two independent review subagents and two independent
verification subagents for the terminal package diff, science/numerics
contract compliance, test adequacy, gate-evidence audit, and final disposition.
Expected outputs are compact findings or verification results in the assigned
package artifacts. Write access is limited to each assigned artifact;
production, contract, test, and all other package files are read-only. Standing
user/session authorization was supplied on 2026-08-30.

## Required reading

Core: `/workdir/openWEPP/AGENTS.md`, `docs/codex_exec_plans.md`,
`docs/work-packages/AGENTS.md`, `docs/work-packages/README.md`, and this
package's `package.md`.

Conditional before kernel/contract review:
`docs/standards/kernel-work-package-preparation.md`,
`docs/standards/testing-and-gate-strategy.md`,
`docs/specifications/science-contract-authoring-procedure.md`,
`docs/specifications/science-contracts/kernel-process-contract-profile.md`,
`docs/specifications/science-contracts/index.md`, `crates/AGENTS.md`, and
`tests/AGENTS.md`.

On demand: `SC-SNOWENERGY-001.md`, predecessor optimization evidence, and the
covered fixed-point source/tests.

## Closure requirements

Review the `792af753e..HEAD` implementation and package evidence. Preserve the
exact 60-second floor, authentic final replay, exact event/discrete predicates,
96-iteration fail-closed ceiling, unchanged physical tolerances, and strict
mass/energy/receipt closure. Production must not persist diagnostics. Record
and disposition every finding, run dual independent verification, reconcile
the exact terminal diff, and close only if all current-scope gates pass.

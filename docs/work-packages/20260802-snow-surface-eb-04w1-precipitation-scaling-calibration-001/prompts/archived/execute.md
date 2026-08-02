# Execute SNOW-SURFACE-EB-04W1

Lifecycle: archived after execution and review disposition.

Scope: local empirical precipitation-scaling calibration study; package-local
fixture copies and artifacts only; no external connectivity.

Execution mode: package-end-to-end. Execute every phase in `package.md`
sequentially through disposition.

## Required Reading

Core:

- `/home/workdir/openWEPP/AGENTS.md`
- `/home/workdir/openWEPP/docs/codex_exec_plans.md`
- `/home/workdir/openWEPP/docs/work-packages/AGENTS.md`
- `/home/workdir/openWEPP/docs/work-packages/README.md`
- package-local `package.md`

Conditional:

- `docs/decisions/0042-science-implementation-and-calibration-readiness.md`
- `docs/standards/testing-and-gate-strategy.md`

On demand:

- EB-04W package, freeze, results, execution tool, and figure sidecars;
- EB-04R real-run harness;
- `SC-SNOWFREEZE-001` only for interpreting unchanged diagnostic fields.

Required-reading budget: see `artifacts/required-reading-map.md`.

Task: execute the exact four-lane, eight-multiplier grid; verify that only
daily precipitation changes; reproduce the `1.0` baseline; apply the frozen
joint magnitude/chronology decision rule; and publish accessible figures with
Markdown sidecars.

Constraints: no production physics, contract, fixture, observation, selector,
default, or public-schema changes. Search values are
`ASSUMED_FOR_EXECUTION`; calibration observations are not independent
validation. Do not change the grid, objective, metrics, or roles after results.

Conservation/output acceptance: retain operand lineage, independently verify
source-to-scaled precipitation, reject protected-token changes, reconstruct
the EB-04W ledgers, and do not close on self-consistency alone.

Subagent requirement: this prompt explicitly authorizes subagent
spawning/delegation to two independent reviewers and two terminal verifiers;
outputs are compact named package artifacts and write access is limited to
those files. No heavy comparator or workspace suite is selected prospectively.

Autonomy: execute end to end without requesting additional direction unless a
hard blocker prevents an acceptance criterion from being evidenced.

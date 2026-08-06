# Execute Stage 3 Terminal Meltout And Land-Surface Handoff

Scope: local repository science-contract/kernel work; flat-file reads/edits
only; no external connectivity or external-system actions are required.

Execution mode: package-end-to-end.

Phase plan: execute every phase in `package.md` sequentially through truthful
disposition unless a declared hard authority boundary is proven.

## Required Reading

Core:

- `/home/workdir/openWEPP/AGENTS.md`
- `/home/workdir/openWEPP/docs/codex_exec_plans.md`
- `/home/workdir/openWEPP/docs/work-packages/AGENTS.md`
- `/home/workdir/openWEPP/docs/work-packages/README.md`
- `/home/workdir/openWEPP/docs/work-packages/20260805-snow-stage3-terminal-meltout-soil-handoff-001/package.md`

Conditional for contract/kernel edits:

- `docs/specifications/science-contracts/AGENTS.md`
- `docs/specifications/science-contract-authoring-procedure.md`
- `docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `docs/specifications/science-contracts/index.md`
- `docs/standards/testing-and-gate-strategy.md`

On-demand for touched mechanisms:

- `SC-SNOWENERGY-001`, `SC-SNOWFREEZE-001`, `SC-WATBAL-001`,
  `SC-SOIL-001`, `SC-RUNOFFPART-001`, and `SC-EVAP-001`
- predecessor package and Snowbird seasonal evaluation
- named libsnobal and pinned WEPP `tmpadj`/frost/water/runoff/evaporation
  baseline sources in `package.md`

Required-reading budget: `508833` local bytes, `WARN`; see
`artifacts/required-reading-map.md` and remeasure before edits.

Task: execute the package objective end-to-end for the declared write set.
Phase 1 is a binding go/no-go gate. If complete receiving-surface and coupled
state authority cannot be admitted, stop before production edits and close a
truthful authority `HOLD` or prospectively scaffold the bounded successor.

Constraints: contract-first sequence is contracts, contract-derived tests,
pre-implementation gate, then production code. Preserve typed guards and exact
units. Do not silently default or canonicalize invalid states.

No surrogate physics: production code must implement reviewed contract-backed
or baseline-authoritative physics. A residual bucket, epsilon heat capacity,
temperature clamp, direct dump of snow excess into soil, fixed `1 minute`
claim of stability, or other proxy is forbidden. Define an implicit or
error-controlled terminal integrator, state/error tolerances, event
bracketing, convergence failure, and flux reevaluation for warming and cooling.

Real consumer proof: the direct runner must consume the admitted event-local
or persistent shadow path. A seasonal claim requires coupled persistence for
all affected surface-cover, surface-liquid, soil thermal/water, and frost
state. Wrappers, skeletons, daily reinitialization, CoE-coupled receiving state,
or internal-only counters cannot carry that claim.

Conservation/output acceptance: record operand lineage, separate plausible
aliases, reject wrong formulas, reconstruct produced ledgers independently,
run real closure/magnitude audits, and align diagnostic metadata. Localize the
earliest combined melt/sublimation exhaustion event within contract tolerance,
flush terminal retained liquid once, and route surface liquid through
infiltration before ponding/overflow and residual runoff. One-sided bounds and
self-consistency are supporting evidence only.

Subagent requirement: REQUIRED: spawn `comparator_suite_runner` for all heavy
batch/full-workspace runs. This prompt explicitly authorizes subagent
spawning/delegation to the reviewers, verifiers, and comparator runner declared
in `package.md`; outputs are compact findings/metrics and log paths; write
access is read-only.

Autonomy: execute all phases and update artifacts without requesting further
direction unless a hard authority boundary is proven.

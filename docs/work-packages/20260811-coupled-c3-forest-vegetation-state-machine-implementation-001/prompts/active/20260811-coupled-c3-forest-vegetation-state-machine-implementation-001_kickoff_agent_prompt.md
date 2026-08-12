# Kickoff: Implement OPENWEPP_C3_WOODY_V1

Scope: local openWEPP repository kernel implementation; flat-file and command
work in `/home/workdir/openWEPP` only; no external connectivity, deployment,
publication, remote branch, push, or external message.

Execution mode: package-end-to-end.

Phase plan: execute every milestone in `package.md` sequentially through
terminal disposition. Do not stop at a partial scientific endpoint.

## Required Reading

### Core

- `/home/workdir/openWEPP/AGENTS.md`
- `/home/workdir/openWEPP/docs/codex_exec_plans.md`
- `/home/workdir/openWEPP/docs/work-packages/AGENTS.md`
- `/home/workdir/openWEPP/docs/work-packages/README.md`
- `/home/workdir/openWEPP/docs/standards/kernel-work-package-preparation.md`
- `/home/workdir/openWEPP/docs/standards/testing-and-gate-strategy.md`
- `/home/workdir/openWEPP/docs/work-packages/20260811-coupled-c3-forest-vegetation-state-machine-implementation-001/package.md`
- `/home/workdir/openWEPP/docs/specifications/science-contracts/AGENTS.md`
- `/home/workdir/openWEPP/crates/AGENTS.md`
- `/home/workdir/openWEPP/tests/AGENTS.md`

### Conditional

- `docs/specifications/science-contract-authoring-procedure.md`,
  `docs/specifications/science-contracts/kernel-process-contract-profile.md`,
  and `docs/specifications/science-contracts/index.md` only if execution finds a
  contract-authority defect; stop before production implementation of that
  defect because contract edits are not authorized here.
- Nearest nested `AGENTS.md` for every production/test path before editing.

### On Demand

- `SC-VEGETATION-001`, `SC-BIOGEOCHEM-001`, `SC-LANDSURFACEENERGY-001`,
  `SC-EVAP-001`, `SC-WATBAL-001`, `SC-PLANT-001`, `SC-RESIDUE-001`, and
  `SC-SNOWFREEZE-001` when their operands/ownership are touched.
- The authority predecessor package's definition JSON, equation/parameter/
  ownership/numerical ledgers, oracle, vectors, and final evidence.
- Existing crate READMEs, source, tests, and Cargo manifests implicated by the
  frozen write set.
- RHESSys/GIS2RHESSys source only for the offline migration adapter and only as
  implementation/format provenance, never new scientific authority.

Required-reading budget: `528317` exact local Core bytes, `WARN`; the catalog
is the dominant required onboarding surface and cannot move on-demand because
it governs live package/campaign relationships. Per-path bytes and triggers are
recorded in `artifacts/required-reading-map.md`.

## Task

Implement the entire digest-bound `OPENWEPP_C3_WOODY_V1` state machine through
the package milestones. Update every required artifact as work proceeds. The
canonical contracts and model definition are read-only implementation
authority. No production edit begins until Milestone 0 and the pre-
implementation gate pass.

Constraints: strict caller configuration and complete state; no hidden default;
typed guards; no canonicalize-and-proceed; no fallback solver, conductance/wind
floor, scalar stress endpoint, agricultural PMET donation, direct soil/BGC
mutation, mixed-profile average, or partial commit.

No surrogate physics: production code must implement the exact admitted
equations and numerical algorithms. Surrogate, provisional, proxy, heuristic,
or empirical stand-in physics is forbidden. Missing/contradictory authority is
a named hold-for-authority boundary, not permission to guess.

Conservation/output acceptance: freeze operand lineage and area/time bases;
separate plausible aliases with poison fixtures; reject known wrong formulas;
independently reconstruct water, energy, C, N, and dry material; prove real
closure and byte-identical rollback. Self-consistency and one-sided bounds are
supporting evidence only.

Real consumer boundary: this package may execute the full public implementation
through a default-off diagnostic harness. It must prove production runtime
selection and legacy PMET/GSI-final-canopy paths are unchanged. It does not
claim production consumer cutover; that remains a separately authorized later
package.

Subagent requirement: REQUIRED. This prompt explicitly authorizes subagent
spawning/delegation, subject to standing user/session authorization, to one Rust
correctness reviewer, one independent science/closure reviewer, one
`comparator_suite_runner` for heavy authority/full-workspace/benchmark gates,
and two independent terminal verifiers. Outputs are their bounded package
review, verification, and gate-log artifacts. The parent must not run heavy
batch commands when the comparator runner is available.

Autonomy: execute end-to-end without requesting additional user direction
unless a hard authority, write-set, tool-policy, or external dependency blocker
remains after safe in-scope routes are exhausted. A milestone is complete only
when its own required gates have direct current evidence. Do not relabel missing
current evidence as future scope.

Outputs: complete production implementation, tests/fixtures/benchmarks,
artifacts, reviews, verifications, archived kickoff, and truthful terminal
disposition. No activation, calibration, deployment, publication, or push.

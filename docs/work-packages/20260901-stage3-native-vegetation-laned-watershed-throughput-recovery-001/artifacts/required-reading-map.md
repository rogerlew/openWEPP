# Required-reading map

Status: `SCAFFOLDED — RECOMPUTE AT EXECUTION INTAKE`

Evidence mode: `Static`

## Applicable instruction chain

Ran during scaffolding:

`tools/agents/find-agents --for docs/ROADMAP.md docs/work-packages/README.md docs/work-packages/20260901-stage3-native-vegetation-laned-watershed-throughput-recovery-001/package.md docs/work-packages/20260901-stage3-native-vegetation-laned-watershed-throughput-recovery-001/prompts/active/kickoff.md docs/work-packages/20260901-stage3-native-vegetation-laned-watershed-throughput-recovery-001/artifacts/required-reading-map.md`

Applicable instructions for the scaffold are root `AGENTS.md` and
`docs/work-packages/AGENTS.md` for package-tree/catalog files; root
`AGENTS.md` alone applies to `docs/ROADMAP.md`.

## Core execution reading

| Path | Purpose |
| --- | --- |
| `AGENTS.md` | Repository invariants, routing, science authority, and validation posture. |
| `docs/codex_exec_plans.md` | Living ExecPlan requirements. |
| `docs/defect_closure_execplans.md` | Autonomous defect closure and legitimate HOLD boundaries. |
| `docs/work-packages/AGENTS.md` | Package lifecycle, artifacts, review, verification, and gate rules. |
| `package.md` | Authorized objective, architecture rule, scope, phases, and exit criteria. |
| predecessor `package.md` | Suspended source/workspace context; not execution authority. |
| predecessor `r151-disposition.md` | Canonical unresolved runtime evidence and owner stop boundary. |
| `docs/standards/testing-and-gate-strategy.md` | Pre-implementation intent, exact-diff selection, and terminal gates. |
| `docs/standards/kernel-work-package-preparation.md` | Kernel package preparation and conservation evidence. |
| `docs/standards/numerical-solver-architecture.md` | Binding single-authority solver and anti-accretion requirements. |
| `docs/decisions/0044-prohibit-accretive-production-solver-dispatch.md` | Accepted architecture decision and current Stage 3 quarantine. |

## Conditional execution reading

| Trigger | Paths |
| --- | --- |
| Any Rust edit | `crates/AGENTS.md`, `tests/AGENTS.md`, and nearest nested `AGENTS.md`. |
| Kernel/branch/tolerance edit | science-contract `AGENTS.md`, contract authoring procedure, kernel process profile, unit governance, correctness authority model, and every affected `SC-*`. |
| Snow/vegetation/LSE work | `SC-SNOWENERGY-001`, `SC-VEGETATION-001`, `SC-LANDSURFACEENERGY-001`, and linked external-authority obligations. |
| Lane D/MOFE work | `SC-OFEROUTE-001`, `SC-RUNOFFPART-001`, `SC-WATBAL-001`, Lane D ADRs, and management/LANUSE authority. |
| Frozen litter work | `20260830-frozen-forest-litter-phase-authority-001/package.md`, its admitted authority artifacts, and `references/vendorable/gmd-10-1621-2017-isba-meb-litter.pdf`. |
| Benchmark/gate execution | `docs/standards/local-ci-gate-selection.md` and local CI timing documentation. |
| Legacy process provenance | Exact pinned baseline files at commit `dac3c950d8b16cc73774bf5ce2e7e11f80baac70`. |

## On-demand routing

Phase 0 must enumerate source files reached by the real runner and add their
nearest instructions, module READMEs, contracts, and predecessor artifacts
before edits. Do not bulk-read every v33--v57 artifact; select the files needed
to classify each reachable production seam and its deletion/migration status.

## Byte-budget disposition

Scaffold-time core total: `261999` local bytes, `OK` under the canonical
`<=400000` threshold. This count includes the eleven core rows above plus this
map. Recompute after the package and inherited worktree
are frozen at execution intake. Heavy contract and historical solver materials
should remain conditional or on-demand unless the selected write set makes them
mandatory.

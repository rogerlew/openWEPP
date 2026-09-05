# Required-reading map

Status: `EXECUTION INTAKE RECOMPUTED`

Evidence mode: `Static + Ran`

## Applicable instruction chain

Ran during execution intake for every exact selected contract/source/test path:

`tools/agents/find-agents --for docs/ROADMAP.md docs/work-packages/README.md docs/work-packages/20260901-stage3-native-vegetation-laned-watershed-throughput-recovery-001/package.md docs/work-packages/20260901-stage3-native-vegetation-laned-watershed-throughput-recovery-001/prompts/active/kickoff.md docs/work-packages/20260901-stage3-native-vegetation-laned-watershed-throughput-recovery-001/artifacts/required-reading-map.md`

Applicable instructions are root `AGENTS.md`; `docs/work-packages/AGENTS.md`
for package artifacts; `crates/AGENTS.md` for Rust; `tests/AGENTS.md` for
integration tests; and `docs/specifications/science-contracts/AGENTS.md` for
the canonical contract. There is no nearer nested crate instruction file for
the selected orchestrator or runner paths.

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

Execution-intake required total: `769514` local bytes, `WARN` above 400,000
and below the 800,000 `REQUIRES-JUSTIFICATION` threshold. The increase is
necessary because the exact write set makes the complete 433,972-byte
`SC-SNOWENERGY-001` contract, contract procedure/profile, unit/correctness
governance, crate/test instructions, and local-CI standard mandatory. Other
large SC contracts remain on demand and are loaded only if the exact diff
touches their authority.

Phase-0 discovery also selected `SC-VEGETATION-001`,
`SC-LANDSURFACEENERGY-001`, `SC-COUPLEDTIME-001`, `SC-OFEROUTE-001`,
`SC-RUNOFFPART-001`, `SC-WATBAL-001`, and `SC-SYSTEM-001` as protected
on-demand authority. They are not silently converted into a broad write set.

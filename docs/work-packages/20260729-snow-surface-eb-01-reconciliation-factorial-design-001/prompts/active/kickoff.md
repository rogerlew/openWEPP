# Kickoff: SNOW-SURFACE-EB-01

Scope: local repository snow-science reconciliation and experimental-design
work; flat-file reads/edits only; no external system mutation is required.

Execution mode: package-end-to-end.

Phase plan: execute all phases in `package.md` sequentially through
disposition.

## Required Reading

Core:

- `/home/workdir/openWEPP/AGENTS.md`
- `/home/workdir/openWEPP/docs/codex_exec_plans.md`
- `/home/workdir/openWEPP/docs/work-packages/AGENTS.md`
- `/home/workdir/openWEPP/docs/work-packages/README.md`
- `/home/workdir/openWEPP/docs/work-packages/20260729-snow-surface-eb-01-reconciliation-factorial-design-001/package.md`

Conditional:

- `/home/workdir/openWEPP/docs/standards/testing-and-gate-strategy.md` before
  declaring validation intent.
- `/home/workdir/openWEPP/docs/specifications/science-contracts/AGENTS.md`,
  `docs/specifications/science-contract-authoring-procedure.md`,
  `docs/specifications/science-contracts/kernel-process-contract-profile.md`,
  and `docs/specifications/science-contracts/index.md` if execution discovers
  that a canonical contract or kernel decision must change. Stop and amend the
  package before such an edit.
- applicable nested `AGENTS.md` files discovered with
  `tools/agents/find-agents --for` before any write.

On-demand:

- `docs/planning/snow-surface-energy-balance-roadmap.md`
- `docs/planning/snow-frost-fidelity-strategy.md`
- `docs/planning/paradigm2-multilayer-snow-specification.md`
- `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
- the Stage 0, Stage 3, Stage A/B, canopy-stratum, and cross-SNOTEL packages
  named in `artifacts/required-reading-map.md`
- relevant production source and fixture files named during reconciliation

Required-reading budget: `425660` local bytes, `WARN`; map:
`artifacts/required-reading-map.md`. The large catalog is retained in Core
because it supplies current package status; the much larger snow contract and
strategy remain On-demand.

## Task

Execute the package objective end-to-end:

1. freeze the exact base, intent, dependencies, and observation-role procedure;
2. reconcile current authority, source implementation, selectors, consumers,
   mass/energy ledgers, and prior candidate dispositions;
3. freeze observation correspondence and roles;
4. pre-register comparable baseline (`B`), longwave-only (`L`),
   sublimation-only (`S`), and combined (`LS`) cells;
5. define marginal, combined, and interaction estimands;
6. define independent mass/energy reconstruction and rejected formulas;
7. decide successor admission and stop-loss;
8. produce accessible figures with Markdown sidecars; and
9. complete direct validation, reviews, finding disposition, terminal
   verification, and final disposition.

Files: edit only the declared write set in `package.md`. Production source,
canonical contracts, fixtures, assurance sources, and public outputs are
read-only.

Constraints: contract-first sequencing; canonical `SC-*` authority; typed
units and signs; no silent defaults; no canonicalize-and-proceed behavior; no
site tuning; no validation leakage; no forcing rescaling.

No surrogate physics: production code must implement actual contract-backed or
baseline-authoritative physics. Surrogate, provisional, proxy, empirical
stand-in, and heuristic process formulations are forbidden. EB-01 makes no
production physics edit.

Additive/interaction requirement: do not assume that longwave and sublimation
effects add. Freeze and later require `B/L/S/LS`, the two marginal effects, the
combined effect, and
`Y(LS) - Y(L) - Y(S) + Y(B)`. Require one physical ledger in which sublimated
mass and latent energy are each debited exactly once.

Conservation/output acceptance: record operand lineage; separate plausible
aliases; reject known wrong formulas; require independent reconstruction plus
real mass/energy closure and magnitude audits; align units and metadata.
Self-consistency and one-sided bounds are supporting evidence only.

Validation: declare pre-execution intent and reconcile the exact terminal diff
under `docs/standards/testing-and-gate-strategy.md`. Run applicable direct
commands; no validation-planning executable has authority.

Subagent authorization: this prompt explicitly authorizes subagent
spawning/delegation to two read-only scientific/implementation reviewers and
two read-only terminal verifiers. Outputs are the four named package artifacts;
write access is bounded to those exact files.

Subagent requirement: none for ordinary analysis. If a heavy batch,
comparator, or campaign-strength full-workspace gate is selected, REQUIRED:
spawn `comparator_suite_runner` for that execution; do not run it on the parent
model unless unavailable, in which case record command-level evidence.

Autonomy: execute every package phase and update the living package and
artifacts without requesting user direction unless a declared hard blocker is
reached.

Outputs: populate all required artifacts, disposition every review finding,
and close only when every current-scope exit criterion has direct evidence.

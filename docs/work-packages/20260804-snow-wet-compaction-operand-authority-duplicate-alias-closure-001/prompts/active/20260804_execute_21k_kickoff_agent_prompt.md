# Execute 21K End To End

Scope: local repository science-contract/kernel defect-closure task; flat-file
reads/edits and local command execution only; no external connectivity is
required.

Execution mode: package-end-to-end (default).

Phase plan: execute every phase in `package.md` sequentially through terminal
disposition. Close defect `SNOW-WETCOMPACT-DUP-001` end-to-end.

## Required Reading

Core:

- `/home/workdir/openWEPP/AGENTS.md`
- `/home/workdir/openWEPP/docs/codex_exec_plans.md`
- `/home/workdir/openWEPP/docs/work-packages/AGENTS.md`
- `/home/workdir/openWEPP/docs/work-packages/README.md`
- `/home/workdir/openWEPP/docs/work-packages/20260804-snow-wet-compaction-operand-authority-duplicate-alias-closure-001/package.md`

Conditional, all triggered for this package:

- `/home/workdir/openWEPP/docs/defect_closure_execplans.md`
- `/home/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
- `/home/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `/home/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/home/workdir/openWEPP/docs/standards/testing-and-gate-strategy.md`
- `/home/workdir/openWEPP/docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`
- applicable instruction files listed by
  `/home/workdir/openWEPP/tools/agents/find-agents --for <write paths>`

On-demand, load only for the touched mechanism:

- `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
- `references/copyrighted/noaa_6392_DS1.md`, density and melt-metamorphism
  sections
- current caller/density/mass-transition sources named in `package.md`
- pinned baseline snow/melt/rain source under
  `/workdir/wepp-forest_260430_baseline`
- predecessor audit/adjudication artifacts named in
  `artifacts/required-reading-map.md`

Required-reading budget: `548607` local bytes, `WARN`; map:
`artifacts/required-reading-map.md`.

## Task

1. Freeze the exact current operand lineage and the primary-source chronology.
2. Materialize and verify the development-only Snowbird precipitation-scaled
   CLI without modifying canonical `p8.cli`.
3. Amend the canonical contract before production code.
4. Add a contract-derived anti-alias regression and record its pre-fix failure.
5. Implement the minimum authority-backed exact-one correction in the real
   producer-to-density-consumer path.
6. Prove independent operand, mass, density, layer, and Stage-3 closure.
7. Run the canonical four-site lane and the Snowbird scaled lane separately;
   never use the scaled lane to prove physics.
8. Complete dual independent review, finding disposition, dual terminal
   verification, exact-diff reconciliation, line-count governance, security,
   roadmap/catalog update, worker handoff, and disposition.

Constraints: contract-first sequencing; canonical `SC-*` authority; pinned
baseline provenance; typed guards; no silent defaults; no unbounded clamping;
no canonicalize-and-proceed for domain violations.

DC closure: do not stop at HOLD while source reading, implementation,
contract/test work, or validation remains possible inside the declared
envelope. Do not relay an intermediate diagnostic step into another package.

HOLD legitimacy audit: any HOLD must name the boundary, cite evidence, list the
in-envelope correction route considered, and explain why that route cannot
close this defect now.

No surrogate physics: production code must implement actual contract-backed
physics. Surrogate, provisional, proxy, empirical stand-in, and heuristic
process formulas are forbidden.

Real consumer proof: show the real direct density consumer reads the corrected
exact-one operand and prove the old aliased sum, wrappers, adapters, skeletons,
shadow paths, and compatibility detours do not carry the closure claim.

Conservation/output acceptance: record operand lineage, separate all plausible
aliases numerically, reject known wrong formulas, reconstruct from independent
hourly outputs, run real closure and magnitude audits, and align metadata and
diagnostics. One-sided bounds and exact producer self-consistency are supporting
evidence only.

Subagent requirement: REQUIRED. This prompt explicitly authorizes subagent
spawning/delegation to bounded read-only authority investigators, two
independent reviewers, two independent terminal verifiers, and
`comparator_suite_runner`. Outputs: compact findings or named package artifacts
and retained log paths. Investigator access is read-only; reviewers/verifiers
may write only their named artifacts; comparator write access is limited to
logs and generated `target/` evidence. Spawn `comparator_suite_runner` for all
heavy batch/closure/comparator/cohort runs; do not run those on the parent model
unless the role is unavailable and the failure is recorded. Use
`rust_code_reviewer` and `rust_qa_reviewer` for the Rust review gates.

Autonomy: execute end-to-end and keep all living artifacts current without
requesting user direction unless genuinely hard-blocked.

Outputs: completed package artifacts, authority-backed correction or legitimate
HOLD, verified derived fixture, exact command evidence, and updated roadmap and
catalog.

# Execute INTVAL-20260713 End To End

Scope: local openWEPP repository validation work; flat-file reads and bounded
package/test artifact edits only; no external connectivity is required.

Execution mode: `package-end-to-end`.

Phase plan: execute every phase in `package.md` sequentially through exact
`PASS-INTEGRATED-VALIDATION` or `HOLD-INTEGRATED-VALIDATION` disposition.

Required reading:

- Core: `/home/workdir/openWEPP/AGENTS.md`, `docs/codex_exec_plans.md`,
  `docs/work-packages/AGENTS.md`, this package, and the required-reading map.
- Conditional: crate/test guidance before touching those trees; science-
  contract governance and the exact contract for kernel/authority analysis;
  defect-closure governance when a semantic defect or gap is reproduced.
- On-demand: scenario-owning packages, fixture provenance, mechanism contracts,
  and pinned baseline sources only for the active lane.

Required-reading budget: current mandatory local total 89,108 bytes including
this package and map; `OK` under 400,000 bytes. Recompute at intake. Map:
`artifacts/required-reading-map.md`.

Task: execute the campaign from one clean frozen commit. Populate all artifacts
with **Static** or **Ran** evidence, exercise real production consumers,
independently reconstruct conservation/publication operands, and preserve
fixture/output hashes.

Constraints: validation-only fixed-source campaign; no production or contract
edits here. Canonical contracts control correctness. The pinned baseline is
provenance and comparator evidence, not the target. Preserve typed fail-closed
behavior; do not default, clamp, edit fixture results, or introduce surrogate,
provisional, proxy, empirical, or heuristic physics.

Defect transition: diagnose a reproduced semantic defect or authority gap to a
named boundary, scaffold a separate DC-ExecPlan whose first action is `close
defect <ID>`, and close this campaign HOLD with a full restart condition. Never
mix pre-fix and post-fix evidence.

Real consumer proof: producer-only, skeleton, shadow, internal-frame, or
counter evidence cannot close a public-path claim. Name producer, typed state,
runner handoff, downstream consumer, output, and negative old-path proof.

Conservation/output acceptance: record operand lineage and units, separate
aliases, reject wrong formulas, reconstruct from outputs, run closure/magnitude
audits, and align metadata/schema. One-sided bounds and self-consistency are
supporting evidence only.

Subagent requirement: **REQUIRED**. Spawn `comparator_suite_runner` for all
release, stability, authority, domain, full-workspace, comparator, and
serial/parallel runs; record command-level unavailability before local
substitution. This prompt explicitly authorizes subagent spawning/delegation to
that runner, two reviewers, two verifiers, and a read-only fixture/operand
inventory agent. Outputs are compact metrics, hashes, findings, and log paths;
writes are limited to assigned package artifacts.

Autonomy: execute end to end without user intervention unless hard-blocked.

Outputs: maintain the plan and artifacts, disposition every finding, update
roadmap/catalog state, commit terminal evidence, and leave a clean worktree.

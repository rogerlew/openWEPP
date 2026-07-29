# CANOPY-LITTER-SOURCE-AUTHORITY-01 Kickoff

Scope: local openWEPP science-contract and kernel engineering; flat-file
reads/edits only; no external mutations.

Execution mode: `package-end-to-end`.

Autonomy: execute every phase in `package.md` sequentially through
disposition; request operator help only for a specifically identified
unavailable primary article or an out-of-envelope authority choice.

Required reading:

- Core: `/workdir/openWEPP/AGENTS.md`,
  `docs/codex_exec_plans.md`, `docs/work-packages/AGENTS.md`,
  `docs/work-packages/README.md`, and package-local `package.md`.
- Conditional: `docs/specifications/science-contract-authoring-procedure.md`,
  `docs/specifications/science-contracts/kernel-process-contract-profile.md`,
  and `docs/specifications/science-contracts/index.md`.
- On-demand: `SC-PLANT-001`, `SC-RESIDUE-001`, ADR-0042, CAL-05, pinned
  baseline residue paths, and candidate primary literature.

Required-reading budget: `621442 bytes`, `WARN`; map:
`artifacts/required-reading-map.md`.

Task: close recurring needle and fine-woody source authority end-to-end within
the declared envelope.

Constraints: contract first; contract-derived tests second;
pre-implementation contract gate third; production edits fourth. Use typed
guards, no silent defaults, no missing-source zero, and no
canonicalize-and-proceed behavior.

No surrogate physics: production code must implement actual contract-backed
or baseline-authoritative physics. A prescribed measured boundary flux is
valid only when labeled as an external source, not as a predictive canopy law.

Conservation/output acceptance: record operand lineage, separate plausible
aliases in tests, reject known wrong formulas, independently reconstruct mass,
prove the real residue/depth/frost/erosion consumers, and align research-output
metadata.

Subagent requirement: this prompt explicitly authorizes subagent
spawning/delegation to two independent prospective science reviewers, two
terminal science reviewers, two terminal verifiers, and a
`comparator_suite_runner` if heavy gates are selected. Outputs are compact
package-local review/verification artifacts and logs; write access is bounded
to assigned artifacts.

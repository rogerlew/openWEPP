Scope: local repository science-contract/kernel migration task; flat-file
reads/edits only; no external connectivity.

Execution mode: package-end-to-end (default).

Phase plan: execute all phases in
`docs/work-packages/20260708-laned-router-active-router-timestep-policy-adjudication-001/package.md`
sequentially through disposition.

Required reading:

Core:
- `AGENTS.md`
- `docs/work-packages/AGENTS.md`
- `docs/specifications/science-contracts/AGENTS.md`
- `docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md`
- `docs/standards/AGENTS.md`
- `docs/standards/prompt-wording-guidance.md`
- `crates/AGENTS.md`
- `tests/AGENTS.md`
- package-local `package.md`
- package-local `artifacts/required-reading-map.md`
- previous day-792 final disposition, mechanism attribution, and worker handoff

Conditional:
- contract authoring/profile docs if `SC-OFEROUTE-001` changes
- prior Tier-2 packages only when reconstructing adjudication surfaces

Required-reading budget: OK; map:
`artifacts/required-reading-map.md`.

Task: adjudicate the active-router timestep policy end-to-end for the declared
scope. Add a diagnostic-only max-`dt` selector only if needed and only with
fail-closed active trace gating. Run the controlled `mn_corn_h4` day-792
ladder, analyze same-`dx` timestep refinement and same-`max_dt` spatial
refinement, and disposition the contract implications before handoff.

Constraints: contract-first sequencing; canonical `SC-OFEROUTE-001` authority;
typed guards; no silent defaults; no production mesh default flip; no
routed-shape threshold widening; no surrogate physics or heuristic damping.

Subagent requirement: REQUIRED for review and verification; this prompt
explicitly authorizes subagent spawning/delegation to review, verification,
and comparator/timing roles for package evidence review and gate verification;
outputs: compact package-local findings and verification notes; write access:
read-only unless explicitly assigned a bounded package-artifact write set.

Autonomy: execute package phases end-to-end and update required artifacts
without requesting additional user direction unless hard-blocked.

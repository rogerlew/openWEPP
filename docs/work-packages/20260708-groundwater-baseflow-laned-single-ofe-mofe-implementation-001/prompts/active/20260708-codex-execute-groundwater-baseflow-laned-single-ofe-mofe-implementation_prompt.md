# Codex Execution Prompt

Scope: local repository science-contract/kernel migration task; flat-file
reads/edits only; no external connectivity.

Execution mode: package-end-to-end (default).

Phase plan: execute all phases in
`docs/work-packages/20260708-groundwater-baseflow-laned-single-ofe-mofe-implementation-001/package.md`
sequentially through disposition.

Required reading (read before edits):

Core:

- `AGENTS.md`
- `docs/work-packages/AGENTS.md`
- `docs/standards/AGENTS.md`
- `docs/standards/prompt-wording-guidance.md`
- `docs/specifications/science-contracts/AGENTS.md`
- `crates/AGENTS.md`
- `tests/AGENTS.md`
- `docs/ROADMAP.md` `## Watershed Runtime Performance Queue`
- `docs/specifications/science-contracts/contracts/SC-GWBASEFLOW-001.md`
- `docs/specifications/science-contracts/contracts/SC-INFILE-GWCOEFF-001.md`
- `docs/work-packages/20260708-groundwater-baseflow-srivastava-authority-001/package.md`
- `docs/work-packages/20260708-groundwater-baseflow-srivastava-authority-001/artifacts/worker-handoff.md`
- `docs/work-packages/20260708-openwepp-management-yaml-canonical-authorization-001/package.md`
- `docs/work-packages/20260708-openwepp-management-yaml-canonical-authorization-001/artifacts/worker-handoff.md`
- `docs/work-packages/20260708-landuse-migration-cli-spec-implementation-001/package.md`
- `docs/work-packages/20260708-landuse-migration-cli-spec-implementation-001/artifacts/worker-handoff.md`
- `docs/work-packages/20260708-groundwater-baseflow-laned-single-ofe-mofe-implementation-001/artifacts/required-reading-map.md`

Conditional:

- `docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md` before
  active Lane D ledger or `INV-OFEROUTE-012` edits.
- `docs/specifications/science-contracts/contracts/SC-SUBHYD-001.md` before
  touching `latqcc` lineage.
- `docs/specifications/science-contracts/contracts/SC-INFILE-MANAGEMENT-YAML-001.md`
  before YAML input-surface or native-authority changes.
- `docs/contracts/openwepp-management-lanuse-authority-contract.md` before
  changing Lane D default/native eligibility.
- `docs/specifications/science-contract-authoring-procedure.md` and
  `docs/specifications/science-contracts/kernel-process-contract-profile.md`
  before amending `SC-*` contracts.

On-demand: use the source files listed in `artifacts/required-reading-map.md`
for touched mechanisms only.

Required-reading budget: approximately `135000` bytes Core, `OK`; map:
`artifacts/required-reading-map.md`.

Files: package files plus the write set declared in `package.md`.

Task: implement `SC-GWBASEFLOW-001` groundwater/baseflow for the Lane D
canonical path, including Lane D single-OFE (`lane_count = 1`) and Lane D MOFE
(`lane_count > 1`) execution.

Constraints: contract-first sequencing; canonical `SC-*` authority; pinned
baseline provenance where implementation details need source confirmation;
typed guards; no silent defaults; no canonicalize-and-proceed for domain
violations; no non-Lane-D production expansion; no route-coefficient projection
or sidecar authority.

No surrogate physics: production code must implement actual contract-backed or
baseline-authoritative groundwater/baseflow physics. Surrogate, provisional,
proxy, empirical stand-in, or heuristic stand-ins are forbidden. Missing
authority is a hold-for-authority boundary; known in-scope physics is an
implementation obligation.

Real consumer proof: prove generated `gwbfv`/`gwdsv` reach the real HBP/pass or
watershed consumer before claiming export/publication closure. Producer-only,
parser-only, skeleton, shadow, wrapper, adapter, or inventory proof cannot
close the package.

Conservation/output acceptance: record operand lineage; separate plausible
aliases in fixtures; reject known wrong formulas (`latqcc`, `cbase`, current
soil deep percolation, active surface sources); run independent reconstruction
plus real closure/magnitude audit; align metadata/schema. One-sided bounds and
self-consistency are supporting evidence only.

Subagent requirement: REQUIRED for heavy batch/closure/comparator runs
(`cargo nextest run --workspace --profile full`, clippy, deny, comparator
batches). Do NOT run them on the parent model unless the subagent is
unavailable, in which case record command-level evidence. Standing user
authorization for openWEPP subagent delegation is expected in the session. This
prompt explicitly authorizes subagent spawning/delegation to review,
verification, and gate-runner subagents for contract review, implementation
review, consumer-path verification, focused test/comparator execution, and
full closure gates; outputs: package-local review/verification artifacts and
compact gate summaries; write access: read-only unless explicitly widened.

Autonomy: execute package phases end-to-end and update required artifacts
without requesting additional user direction unless hard-blocked.

Outputs: update package artifacts/disposition for all completed phases.

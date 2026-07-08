# Execute Watershed HBP Hourly Water/Sediment Consumption

Scope: local repository science-contract/kernel migration task; flat-file
reads/edits only; no external connectivity.

Execution mode: package-end-to-end (default).

Phase plan: execute all phases in
`docs/work-packages/20260708-laned-router-watershed-hbp-hourly-water-sediment-consumption-001/package.md`
sequentially through disposition, but do not implement until
`20260708-laned-router-tier1-local-numerics-001` has completed and its final
disposition has been read.

Required reading:

Core:

- `AGENTS.md`
- `docs/work-packages/AGENTS.md`
- `docs/specifications/science-contracts/AGENTS.md`
- `docs/standards/AGENTS.md`
- `docs/standards/prompt-wording-guidance.md`
- `docs/work-packages/20260708-laned-router-tier1-local-numerics-001/package.md`
- `docs/work-packages/20260708-laned-router-watershed-hbp-hourly-water-sediment-consumption-001/artifacts/required-reading-map.md`
- `docs/work-packages/20260708-laned-router-watershed-hbp-hourly-water-sediment-consumption-001/package.md`

Conditional:

- Tier 1 `artifacts/final-disposition.md` and `artifacts/worker-handoff.md`
  after Tier 1 completes.
- Contract-authoring procedure/profile docs before any `SC-*` edit.
- `SC-INFILE-HBP-001`, `SC-ROUTE-001`, `SC-OFEROUTE-001`, `SC-SED-001`, and
  `SC-RUNOFFPART-001` by touched mechanism as defined in the required-reading
  map.
- `crates/AGENTS.md` and `tests/AGENTS.md` before Rust/test edits.

Required-reading budget: OK for core pre-edit set; map:
`artifacts/required-reading-map.md`.

Files:

- `docs/work-packages/20260708-laned-router-watershed-hbp-hourly-water-sediment-consumption-001/**`
- `docs/work-packages/README.md`
- `docs/ROADMAP.md`
- `docs/specifications/science-contracts/contracts/SC-INFILE-HBP-001.md`
- `docs/specifications/science-contracts/contracts/SC-ROUTE-001.md`
- `docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md`
- `docs/specifications/science-contracts/contracts/SC-SED-001.md`
- `crates/openwepp-runner/src/hillslope/04_direct_publication.rs`
- `crates/openwepp-runner/src/watershed_supervisor.rs`
- `crates/openwepp-watershed-orchestrator/src/lib_mod/network_frame.rs`
- `crates/openwepp-watershed-orchestrator/src/lib_mod/kernel/direct.rs`
- `crates/openwepp-watershed-orchestrator/src/lib_mod/kernel/types.rs`
- `crates/openwepp-watershed-orchestrator/src/lib.rs`
- watershed integration tests as needed.

Task: execute the package objective end-to-end for the declared scope.

Constraints: contract-first sequencing; canonical `SC-*` authority; typed
guards; no silent defaults for missing active hourly authority; no surrogate
sediment physics; no mesh-policy/default-activation/Tier-1 optimization edits;
no compatibility wrapper, skeleton, shadow, producer-only, parser-only, or
inventory-only evidence carrying the closure claim.

Real consumer proof: prove the active routed hourly runoff and hourly sediment
pair is produced into HBP, parsed by the watershed supervisor, carried in
`HillslopeContribution`, and consumed by the production watershed/channel
kernel. Prove equal daily totals with different hourly distributions change a
named watershed/channel consumer surface. Prove the old daily scalar fallback
does not carry the active-routed closure claim.

Conservation/output acceptance: record operand lineage; reject daily scalar and
synthetic-fill aliases; independently reconstruct hourly runoff and sediment
sums; record water/sediment closure; do not close on one-sided bounds or exact
self-consistency alone.

Subagent requirement: REQUIRED for heavy watershed fixture/comparator/full
closure runs when available. This prompt explicitly authorizes subagent
spawning/delegation to review, verification, comparator, watershed fixture, and
release-gate subagents for package-local review, verification, consumer-path
proof, watershed fixture execution, and closure gates; outputs: compact metrics
plus package artifact paths; write access: bounded to package artifacts unless
explicitly assigned implementation fixes.

Autonomy: execute package phases end-to-end and update required artifacts
without requesting additional user direction unless hard-blocked.

Outputs: update package artifacts/disposition for all completed phases.

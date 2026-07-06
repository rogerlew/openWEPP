# D13 Kickoff Prompt

Scope: local repository science-contract/kernel migration task; flat-file
reads/edits only; no external connectivity.

Execution mode: package-end-to-end (default).

Phase plan: execute all phases in
`docs/work-packages/20260705-mofefid-d13-routed-hydrograph-erosion-shape-001/package.md`
sequentially through disposition.

Required reading (read before edits):

Core:

- `AGENTS.md`
- `docs/work-packages/AGENTS.md`
- `docs/specifications/science-contracts/AGENTS.md`
- `docs/standards/prompt-wording-guidance.md`
- `docs/work-packages/20260705-mofefid-d13-routed-hydrograph-erosion-shape-001/package.md`
- `docs/work-packages/20260705-mofefid-d13-routed-hydrograph-erosion-shape-001/artifacts/required-reading-map.md`
- `docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md`
- `docs/specifications/science-contracts/contracts/SC-SED-001.md`
- `docs/decisions/0036-hydrograph-resolved-sediment-transport-and-routing.md`
- `docs/planning/mofe-fidelity-campaign-strategy.md` §6.1
- `docs/work-packages/20260705-mofefid-d12-melt-limb-hourly-shape-001/artifacts/final-disposition.md`
- `docs/work-packages/20260705-mofefid-d12-melt-limb-hourly-shape-001/artifacts/worker-handoff.md`
- `crates/openwepp-hillslope-orchestrator/src/direct_runtime/03_executor.rs`
- `crates/openwepp-hillslope-orchestrator/src/direct_runtime/runoff.rs`
- `crates/openwepp-runner/src/hillslope/04_direct_publication.rs`
- `crates/openwepp-runner/src/hillslope/laned_shadow.rs`

Conditional:

- Erosion runtime internals, HBP/interchange contracts, and H2637
  fixture/test surfaces listed in `artifacts/required-reading-map.md` when
  touched.

On-demand:

- Local CI gate selection, unit governance, contract authoring procedure, and
  D10/D14 package artifacts only when the touched mechanism requires them.

Required-reading budget: 485,790 bytes, WARN (>400,000 and <800,000);
map: `artifacts/required-reading-map.md`.

Files: declared write set in `package.md`.

Task: execute D13 end-to-end. Close the ADR-0036 active-routed-water
erosion-shape blocker by making/proving the Wave-1 hourly erosion substrate
consume the routed hydrograph when Lane D routing owns water, rather than
DC01 source-shape weights. Start with contract-first authority; do not
implement runtime changes before the pre-implementation contract gate is
recorded.

Constraints: contract-first sequencing; canonical `SC-*` authority; typed
guards; no silent defaults; no canonicalize-and-proceed for invalid routed
hydrograph, hourly water, or hourly sediment state; no surrogate/provisional/
proxy/heuristic routing or erosion physics; default/off runtime behavior must
stay byte-flat.

Real consumer proof: prove producer source, frame state, runner handoff,
erosion hourly substrate, HBP EVENT surfaces, and negative proof that old DC01
weights are not carrying the active-mode closure claim.

Conservation/output acceptance: record operand lineage; separate plausible
aliases; reject wrong formulas; run independent reconstruction plus H2637 or
equivalent closure audit; align contract text with metadata/manifest claims;
do not close on self-consistency alone.

Protected boundaries: no production/default activation, no D10 shock-numerics
acceptance, no D11 friction-source changes, no D12 melt-limb source-shape
changes, no D14 profiling/optimization, no D15/D16 policy, no watershed/
channel routing changes.

Subagent requirement: REQUIRED. Spawn `comparator_suite_runner` for all heavy
batch/closure/comparator runs (full workspace nextest, H2637 fixture batches,
and equivalent closure gates) when available; do NOT run them on the parent
model unless the subagent is unavailable, in which case record command-level
evidence. This prompt explicitly authorizes subagent spawning/delegation to
`rust_code_reviewer`, `rust_qa_reviewer`, `explorer`, and
`comparator_suite_runner` for read-only source audit, review, verification,
fixture inspection, H2637/Lane D evidence, and heavy gate execution. Outputs:
compact findings, metrics, log paths, and package-local artifact text. Write
access: read-only unless the operator assigns a bounded write set.

Autonomy: execute package phases end-to-end and update required artifacts
without requesting additional user direction unless hard-blocked.

Outputs: update package artifacts/disposition for all completed phases.

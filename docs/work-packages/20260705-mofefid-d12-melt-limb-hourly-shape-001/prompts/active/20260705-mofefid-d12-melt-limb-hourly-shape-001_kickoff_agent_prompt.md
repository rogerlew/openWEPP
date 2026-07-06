# D12 Kickoff Prompt

Scope: local repository science-contract/kernel migration task; flat-file
reads/edits only; no external connectivity.

Execution mode: package-end-to-end (default).

Phase plan: execute all phases in
`docs/work-packages/20260705-mofefid-d12-melt-limb-hourly-shape-001/package.md`
sequentially through disposition.

Required reading (read before edits):

Core:

- `AGENTS.md`
- `docs/work-packages/AGENTS.md`
- `docs/specifications/science-contracts/AGENTS.md`
- `docs/standards/prompt-wording-guidance.md`
- `docs/work-packages/20260705-mofefid-d12-melt-limb-hourly-shape-001/package.md`
- `docs/work-packages/20260705-mofefid-d12-melt-limb-hourly-shape-001/artifacts/required-reading-map.md`
- `docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md`
- `docs/planning/mofe-fidelity-campaign-strategy.md` §6.1
- `docs/work-packages/20260705-mofefid-laned-activation-increment-001/package.md`
- `docs/work-packages/20260706-mofefid-d11-gap007-dynamic-friction-closure-001/artifacts/worker-handoff.md`
- `crates/openwepp-hillslope-orchestrator/src/direct_runtime/runoff.rs`
- `crates/openwepp-hillslope-orchestrator/src/direct_runtime/01_publication.rs`
- `crates/openwepp-runner/src/hillslope/laned_shadow.rs`

Conditional:

- Snow/liquid state and direct publication files listed in
  `artifacts/required-reading-map.md` when touched.
- Companion `SC-*` contracts only if source ownership or unit governance
  changes.
- H2637 fixture/test surfaces when reproducing `days_uniform_shape`.

On-demand:

- ADR-0036, local CI gate selection, unit governance, contract authoring
  procedure, and pinned-baseline snow/runoff source files only when the touched
  mechanism requires them.

Required-reading budget: 323,774 bytes, OK (<400,000-byte WARN threshold);
map: `artifacts/required-reading-map.md`.

Files: declared write set in `package.md`.

Task: execute D12 end-to-end. Close the melt-limb hourly source-shape gap by
adding/proving a source-authorized snowmelt/routed-liquid hourly source limb
with exact daily-sum closure, or close in legitimate HOLD with a source
authority boundary. Start with contract-first authority; do not implement
runtime changes before the pre-implementation contract gate is recorded.

Constraints: contract-first sequencing; canonical `SC-*` authority; typed
guards; no silent defaults; no canonicalize-and-proceed for invalid melt,
runoff, or hourly-shape state; no surrogate/provisional/proxy/heuristic
snowmelt timing or routing physics; daily runoff volume authority remains
unchanged unless a contract amendment explicitly says otherwise.

Real consumer proof: prove the real shared source-shape path and Lane D shadow
consumer read the D12 source-authorized limb. Uniform fallback cannot carry the
closure claim for source-authorized melt/routed-liquid days.

Conservation/output acceptance: record operand lineage; separate plausible
aliases; reject wrong formulas; run independent reconstruction plus H2637
closure/shape audit; align contract text with metadata/manifest claims; do not
close on self-consistency alone.

Protected boundaries: no production/default activation, no D10 shock-numerics
acceptance, no D11 friction-source changes beyond preserving rev 21, no D13
erosion hourly-shape switch, no D14 profiling/optimization, no D15/D16 policy.

Subagent requirement: REQUIRED. Spawn `comparator_suite_runner` for all heavy
batch/closure/comparator runs (full workspace nextest, H2637 fixture batches,
and equivalent closure gates) when available; do NOT run them on the parent
model unless the subagent is unavailable, in which case record command-level
evidence. Standing user authorization is present for this package: the
2026-07-06 operator request explicitly authorizes subagent
spawning/delegation for D12. This prompt explicitly authorizes subagent
spawning/delegation to `rust_code_reviewer`, `rust_qa_reviewer`, `explorer`,
and `comparator_suite_runner` for read-only source audit, review,
verification, fixture inspection, H2637/Lane D shadow evidence, and heavy gate
execution. Outputs: compact findings, metrics, log paths, and package-local
artifact text. Write access: read-only unless the operator assigns a bounded
write set.

Autonomy: execute package phases end-to-end and update required artifacts
without requesting additional user direction unless hard-blocked.

Outputs: update package artifacts/disposition for all completed phases.

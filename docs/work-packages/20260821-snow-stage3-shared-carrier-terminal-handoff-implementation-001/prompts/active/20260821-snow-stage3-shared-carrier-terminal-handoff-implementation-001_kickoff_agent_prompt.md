# Execute the Child 2C shared-carrier terminal handoff implementation

Scope: local openWEPP repository engineering; flat-file reads/edits and local
build/test execution only; no external connectivity, deployment, release, or
production activation.

Execution mode: package-end-to-end (default).

Autonomy: execute the package phases end-to-end, update the living plan and
named artifacts as evidence advances, and stop only at an explicit in-scope
HOLD or the stated terminal boundary.

Phase plan: execute all phases in `package.md` sequentially through review,
verification, disposition, exact-diff reconciliation, and a truthful terminal
release boundary. Do not request user intervention unless an external hard
blocker prevents an in-scope action.

## Required reading

Core (read before edits):

- `AGENTS.md`
- `docs/work-packages/AGENTS.md`
- `docs/specifications/science-contracts/AGENTS.md`
- `docs/work-packages/20260821-snow-stage3-shared-carrier-terminal-handoff-implementation-001/package.md`
- `docs/standards/testing-and-gate-strategy.md`
- `docs/standards/kernel-work-package-preparation.md`
- `docs/standards/prompt-wording-guidance.md`
- `docs/specifications/science-contracts/index.md`
- `docs/specifications/science-contracts/contracts/SC-COUPLEDTIME-001.md`
- `docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001.md`
- `docs/specifications/science-contracts/contracts/SC-SNOWENERGY-001.md`
- `docs/specifications/science-contracts/contracts/SC-VEGETATION-001.md`
- `docs/specifications/science-contracts/contracts/SC-VEGETATIONTRANSACTION-001.md`
- `docs/work-packages/20260821-snow-stage3-shared-carrier-authority-closure-001/artifacts/final-disposition.md`
- `docs/work-packages/20260821-snow-stage3-shared-carrier-authority-closure-001/artifacts/worker-handoff.md`
- `docs/work-packages/20260819-snow-stage3-terminal-meltout-lse-handoff-implementation-001/package.md`

Conditional: read the actual scheduler, owner, restart, LSE, snow, liquid,
hydrology, soil-thermal, BGC, and publication modules identified by intake;
read the historical Child 1 artifacts only for consumed evidence and negative
boundary proof.

On-demand: pinned legacy baseline files only for exact provenance mapping;
existing comparator and observed-data materials only when a declared evidence
obligation requires them. They are not correctness targets.

Required-reading budget: the current Core set is `677448` local bytes,
calculated with `wc -c`; this is `WARN` (`>400000` and `<=800000`) under
`docs/standards/kernel-work-package-preparation.md`. Record the per-file map
and command in `artifacts/required-reading-map.md` before edits. No file is in
the `REQUIRES-JUSTIFICATION` range; defer conditional and on-demand material
until intake identifies the touched mechanisms.

## Task

Consume the released Child 2C contracts on current `main` and implement the
shared carrier and terminal snow-to-real-owner handoff in one complete-owner
parent transaction. Freeze the actual scheduler consumer and exact write set
before production edits. Prove the shared canopy-air node, sealed projected
wind, reciprocal longwave, support-domain event coalescing, zero-duration
liquid custody, post-event real-owner continuation, restart/replay protection,
rollback, publication ordering, and independent ledgers.

The historical Child 1 checkpoint `83cf6eb8e` is evidence only. Do not reset,
branch, rewrite, or relabel that package.

## Constraints

- Contract-first sequencing and pre-implementation gate before production edits.
- Production physics must be contract-backed or pinned-baseline authoritative;
  surrogate, provisional, proxy, and heuristic stand-ins are forbidden.
- Never use raw 10 m wind as subcanopy wind or a fixed forest attenuation
  multiplier. Never create separate canopy-air nodes or duplicate fluxes.
- Never execute below-domain LSE, drop a remainder, scale a longer result, or
  integrate a rate during the zero-duration event transition.
- Preserve V10, Restart V1/V2/V3, Child 2B receipts, defaults, and CoE
  production ownership.
- Prove the real downstream consumer; reference, skeleton, adapter, wrapper,
  and shadow paths cannot carry the closure claim.
- For mass/energy/liquid outputs, record operand lineage, separate plausible
  aliases, reconstruct independently, and run a real closure/magnitude audit.
- Required current-scope gates may not be silently deferred. A missing gate is
  `HOLD` with a named boundary and legitimacy audit.

Conservation/output acceptance: record operand lineage; add anti-tautology
tests; name rejected formulas and aliases; independently reconstruct snow,
liquid, vapor, energy, longwave, event-time, and published outputs; run a real
closure/magnitude audit; and align metadata/schema evidence. Exact producer
self-consistency is supporting evidence only.

## Subagent requirement

Subagent authorization: this prompt explicitly authorizes subagent
spawning/delegation to two implementation/code-review agents and two
verification agents for the declared carrier, handoff, consumer-path,
rollback, and conservation scope. Their outputs are compact findings,
command/count summaries, and named package artifacts; implementation write
access is limited to the frozen source/test set, while reviewers/verifiers are
read-only except named artifacts. Require the `comparator_suite_runner` role
for any selected critical full-workspace or heavy comparator batch; do not run
that batch on the parent model unless the role is unavailable and the package
records command-level justification first.

## Outputs

Update `artifacts/required-reading-map.md`, `implementation-intent.md`,
`consumer-path-proof.md`, `operand-lineage.md`, all gate/review/verification
artifacts, the final disposition, and the worker handoff as evidence advances.
Keep `package.md` Progress, Surprises & Discoveries, Decision Log, and
Outcomes & Retrospective current. Close only as the default-off actual-consumer
boundary described in `package.md`; production activation and CoE retirement
remain unauthorized.

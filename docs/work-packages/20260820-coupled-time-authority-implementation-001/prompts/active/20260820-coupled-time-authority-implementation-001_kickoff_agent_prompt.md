# Implement coupled time authority V1 end to end

Scope: local contract-first Rust architecture and integration; no external
connectivity, messaging, deployment, release, or production cutover.

Execution mode: package-end-to-end (default).

Phase plan: execute every phase in `package.md` sequentially through truthful
disposition. Do not stop after contract prose or clock types.

Required reading before edits:

Core: `/workdir/openWEPP/AGENTS.md`, `docs/codex_exec_plans.md`,
`docs/work-packages/AGENTS.md`, `docs/work-packages/README.md`,
`docs/standards/testing-and-gate-strategy.md`, and this package's `package.md`.

Conditional: before contract/kernel edits read
`docs/specifications/science-contracts/AGENTS.md`,
`docs/specifications/science-contract-authoring-procedure.md`,
`docs/specifications/science-contracts/kernel-process-contract-profile.md`,
`docs/specifications/science-contracts/index.md`, `crates/AGENTS.md`, and
`tests/AGENTS.md`. Read unit, restart, serialization, digest, or prompt
standards when those surfaces are selected.

On-demand: the Stage 3 campaign and terminal-handoff HOLD; touched sections of
`SC-SNOWENERGY-001`, `SC-LANDSURFACEENERGY-001`, `SC-VEGETATION-001`,
`SC-VEGETATIONTRANSACTION-001`, restart and forcing/simulation identity
contracts; and source/test modules identified by Phase 0 inventory.

Required-reading budget: remeasure local Core bytes at intake using
`artifacts/required-reading-map.md`. Apply `OK <=400000`, `WARN >400000`, and
`REQUIRES-JUSTIFICATION >800000`; explain every mandatory heavy read above the
last threshold and keep mechanism-specific material on demand.

Task: admit `SC-COUPLEDTIME-001` and implement the exact-integer, single-clock,
attempt/accept/reject, adaptive restart, atomic multi-owner, parent-buffered
publication subsystem plus its orchestrator reference consumer. Freeze an
exact inventory and implementation intent first. Refresh current main rather
than trusting the older Richards inventory.

Contract-first sequence: canonical contract, contract-derived tests,
independent vectors/reference calculator, passing pre-implementation gate, then
production Rust. No production edit may precede that gate.

Implementation completeness: a type-only crate, vegetation-local support
override, independently advanceable owner clock, mock-only unit test without an
orchestrator consumer, or publication before parent commit is a HOLD. Exercise
at least three owners, segmentation, adaptive slabs, retry rollback, event
boundary, restart, scheduled-once behavior, reduction, and atomic finalization.

Protected boundaries: preserve V10 bytes and behavior; do not implement V11,
snow-carrier equations, Richards/Lane D/soil--plant physics, the terminal snow
receiver, selectors/defaults, CoE retirement, or publication/cutover.

No surrogate physics: this package implements time/transaction semantics only.
Do not add proxy, heuristic, fitted, or provisional process physics to obtain a
consumer test. Use deterministic mock owners for the reference consumer.

Typed safety: use checked integer arithmetic, bounded versioned wire formats,
typed fail-closed guards, deterministic error precedence, and no silent
defaults, unbounded clamps, floating support identity, or canonicalize-and-
proceed behavior.

Real consumer proof: prove the hillslope orchestrator seam consumes the shared
clock and cannot accept mismatched owner support, advance on rejection, or
publish staged results. Include a negative old-path/bypass audit. This does not
claim a physical production adopter.

Validation: select exact current commands under the testing strategy; record
truthful `Static:` and `Ran:` evidence, exact source identity, commands, counts,
hashes, failures/retries, exact-diff reconciliation, manifest policy, line
counts, and placeholder/bypass audits. Required 2A gates cannot be deferred to
2B/2C/Richards/campaign closure.

Subagent authorization: this prompt explicitly authorizes subagent
spawning/delegation to time/numerics authority, Rust/API,
serialization/restart reviewers, a bounded heavy-gate runner, and two terminal
verifiers. Outputs are compact findings, command/count/hash summaries, and
artifact paths. Reviewers/verifiers are read-only; the runner writes only
ignored logs and bounded package artifacts.

Autonomy: execute the full package and maintain every artifact through final
disposition without requesting direction unless hard-blocked. Disposition all
review findings as accepted, rejected, deferred, or follow-up; a required
current-scope finding or gate cannot remain deferred at completion.

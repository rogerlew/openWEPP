# Implement coupled time authority V1 end to end

Required hardening anchor commit:
`46b0c679ae98cedd1c259295ab02202d24846706`

The clean launch HEAD may be this anchor or its single documentation-only
kickoff-repin child. Record the exact launch HEAD in the intake artifact.

Before edits:

```bash
cd /workdir/openWEPP
test "$(git merge-base --is-ancestor \
  46b0c679ae98cedd1c259295ab02202d24846706 HEAD; echo $?)" = "0"
test "$(git branch --show-current)" = "main"
git status --short --branch
git diff --check
```

Do not pull, reset, rebase, merge over, switch branches, amend the hardening
commit, create a PR, or push. Local commits are permitted at the authority
checkpoint and terminal package boundary.

Scope: local contract-first Rust architecture and integration; no external
connectivity, messaging, deployment, release, or production cutover.

Execution mode: package-end-to-end (default).

Files: edits are limited to the exact intended write set in `package.md`:
`SC-COUPLEDTIME-001` and its registry row; the selected coupled-time crate;
the bounded hillslope-orchestrator reference-consumer seam; additive new
persisted-restart types only when inventory proves ownership;
`tests/integration/coupled_time_*`; root Cargo manifests/lock; this package;
and truthful campaign roadmap/catalog lifecycle updates. Amend the prospective
write set and gates before any broader edit. Vegetation, snow, Lane D,
Richards, soil-thermal, BGC, existing DirectV10 restart wire, selectors, and
defaults are protected.

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

Required-reading budget: `501127` local Core bytes at hardened
scaffold time, `WARN`. Remeasure at intake in
`artifacts/required-reading-map.md`. Thresholds are `OK <=400000`,
`WARN >400000`, and `REQUIRES-JUSTIFICATION >800000`; keep mechanism-specific
material on demand.

Task: admit `SC-COUPLEDTIME-001` and implement the exact-integer, single-clock,
attempt/accept/reject, adaptive restart, atomic multi-owner, parent-buffered
publication subsystem plus its orchestrator reference consumer. Freeze an
exact inventory and implementation intent first. Refresh current main rather
than trusting the older Richards inventory.

Contract-first sequence: canonical contract, contract-derived tests,
independent vectors/reference model, passing pre-implementation gate, then the
Phase-2A authority release cycle: two independent contract reviews, complete
finding disposition/correction, invalidated-gate reruns, two independent
contract verifications, contract promotion/indexing after PASS, and an exact
local authority-checkpoint commit. No production Rust may be edited before that
checkpoint. Package-level A/B/C reviews remain final implementation reviews and
cannot substitute for the contract cycle.

Authority completeness: freeze time origin/wire/range; parent, transaction,
segment, slab, attempt, event, calendar/day and forcing identities; integer-to-
`f64` conversion and proposal quantization/ties; one parent increment;
diagnostic-only attempts; zero-duration custody/state transitions; complete
owner set versus segment participant set; deterministic constraint arbitration;
adopter-owned controller policy/digest; and restart/publication chronology.

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

Conservation/output acceptance: record accepted-slab operand lineage, support,
units and source identity; separate accepted values from rejected attempts,
volume/nominal-duration aliases and precommit buffers; independently
reconstruct parent reductions and publication order; prove no staged output is
externally visible before parent commit. Self-consistency alone is not
acceptance.

Validation: select exact current commands under the testing strategy; record
truthful `Static:` and `Ran:` evidence, exact source identity, commands, counts,
hashes, failures/retries, exact-diff reconciliation, manifest policy, line
counts, and placeholder/bypass audits. Required 2A gates cannot be deferred to
2B/2C/Richards/campaign closure.

Subagent authorization: this prompt explicitly authorizes subagent
spawning/delegation to two preimplementation contract reviewers and verifiers;
three final time/numerics, Rust/API/atomicity, and serialization/restart
reviewers; `comparator_suite_runner`; and two terminal
verifiers. Outputs are compact findings, command/count/hash summaries, and
artifact paths. Reviewers/verifiers are read-only; the runner writes only
ignored logs and bounded package artifacts.

Subagent requirement: REQUIRED. Spawn `comparator_suite_runner` for full
workspace, broad Clippy, cargo-deny, comparator/property population, and other
heavy closure runs. Do not run those batches on the parent model when the
runner is available. If unavailable, retain command-level evidence of the
failure before executing locally.

HOLD legitimacy audit: a HOLD is valid only for an exact authority,
dependency-cycle, wire-compatibility or owner-atomicity contradiction after all
safe in-scope contract, implementation and validation routes are exhausted.
Implementation volume, failing tests, refactoring, controller design, schema
size or heavy-gate cost are not HOLD reasons.

Autonomy: execute the full package and maintain every artifact through final
disposition without requesting direction unless hard-blocked. Disposition all
review findings as accepted, rejected, deferred, or follow-up; a required
current-scope finding or gate cannot remain deferred at completion.

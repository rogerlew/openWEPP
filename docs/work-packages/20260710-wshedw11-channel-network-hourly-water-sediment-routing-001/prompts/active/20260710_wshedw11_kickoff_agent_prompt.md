# WSHED-W11 Kickoff Prompt

This is local repository engineering work in `/workdir/openWEPP`, limited to
flat-file reads and edits in the worktree. No external systems or network
actions are required.

Execution mode: `package-end-to-end`.

Autonomy: execute every phase in `package.md` sequentially through final
disposition. Do not ask for a next-step prompt unless a documented hard blocker
meets a declared hold boundary.

Subagent authorization: this prompt explicitly authorizes subagent
spawning/delegation to source-lineage, Rust review, verification, and
comparator-suite-runner roles for the scopes and artifacts declared in
`package.md`; write access is read-only except for explicitly assigned package
review/verification artifacts.

Subagent requirement: use a comparator-suite-runner subagent for full-workspace
clippy, full nextest, deny, release/comparator, and other heavy closure runs.
Record any actual tool-policy unavailability before a permitted local fallback.

No surrogate physics: implement only canonical contract-backed or pinned
baseline-authoritative channel water/sediment physics. Do not introduce uniform
bin projection, hourly particle-class allocation, independent hourly event
solves, proxy transport, or other heuristic stand-ins without explicit
canonical authority.

Real consumer proof: the production watershed CLI must read HBP hourly water
and sediment, route it through at least two channel nodes, and make the
downstream node consume upstream typed hourly channel output. Wrappers,
adapters, leaf-only tests, scalar summaries, shadow paths, and compatibility
paths cannot carry the closure claim.

Conservation/output acceptance: complete operand lineage before production
edits; add anti-tautology fixtures and explicit rejected formulas; independently
reconstruct water and particle-class sediment closure from produced state;
perform real closure/magnitude audits; and align metadata/schema descriptions.
One-sided bounds and exact self-consistency are supporting evidence only.

## Required Reading

### Core

- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/AGENTS.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/AGENTS.md`
- `/workdir/openWEPP/docs/standards/AGENTS.md`
- `/workdir/openWEPP/docs/work-packages/20260710-wshedw11-channel-network-hourly-water-sediment-routing-001/package.md`

### Conditional

- `docs/specifications/science-contract-authoring-procedure.md`
- `docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `docs/specifications/science-contracts/index.md`
- `docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`
- `crates/AGENTS.md` and `tests/AGENTS.md` before Rust/test edits

### On-Demand

- `docs/specifications/science-contracts/contracts/SC-ROUTE-001.md`
- `docs/specifications/science-contracts/contracts/SC-SED-001.md`
- `docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `docs/specifications/science-contracts/contracts/SC-INFILE-HBP-001.md`
- `docs/decisions/0036-hydrograph-resolved-sediment-transport-and-routing.md`
- completed M-T3 package and artifacts
- pinned baseline files named in `package.md`

Required-reading budget: `352024` local bytes, disposition `OK`; see
`artifacts/required-reading-map.md`.

## Execute

1. Revalidate the scaffold assessment and update the required-reading/source
   maps.
2. Establish baseline and canonical authority before tests or production code.
3. Add contract-derived failing tests and pass the pre-implementation contract
   gate.
4. Implement typed stateful channel-network hourly water and sediment routing
   in the declared write set.
5. Produce real two-channel CLI consumer and independent conservation evidence.
6. Run review, verification, line-count, comparator, and full closure gates.
7. Update every package artifact truthfully and set final disposition only when
   every current-scope gate has direct evidence.

If an exit criterion cannot be proven in this package, continue implementation
or record `EXECUTED-HOLD-*` with a valid hold-legitimacy audit. Do not relabel a
missing current gate as later work.

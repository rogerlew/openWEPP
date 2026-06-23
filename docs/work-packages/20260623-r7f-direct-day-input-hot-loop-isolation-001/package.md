# R7F Direct Day-Input Hot-Loop Isolation

Status: complete.

Package type: Defect-Closure ExecPlan.

Defect ID:

- `HOLD-R7F-DIRECT-DAY-INPUT-BUILDER-COMPATIBILITY-SURFACE-HOT-EDGE`

This package follows `docs/codex_exec_plans.md`,
`docs/defect_closure_execplans.md`, `docs/work-packages/AGENTS.md`,
`docs/specifications/science-contracts/AGENTS.md`, and
`docs/architecture/array-native-runtime-specification.md`.

Subagent authorization: this package explicitly authorizes
spawning/delegating to read-only reviewer and verifier subagents for R7F
hot-loop compatibility isolation review, source-scan review, counter-evidence
review, and line-count governance review. Expected outputs are compact Markdown
findings summarized into `artifacts/review-disposition.md` and
`artifacts/verification.md`; subagents may not edit files.

## Purpose

R7F closes the remaining hot compatibility edge found by
`20260623-r7e-r7h-direct-runtime-completion-001`: production direct execution
still builds every day/OFE input through `DirectPublicationDayInputBuilder`,
which clones/merges `HillslopeWritebackSurface` values and reads
`BoundarySymbol`/`BoundaryValue` symbol maps inside the production direct
day/OFE loop.

The target state is production direct execution whose scheduler bypass is not
misleading: the direct executor must consume typed direct state/day inputs in
the hot loop, and `/direct_runtime_counters/compatibility_edge_invocations`
must be zero because no compatibility edge was invoked in that loop.

## Progress

- [x] Scaffold package, required reading, blocker ledger, compatibility
  inventory, typed-projection plan, consumer-path proof shell, verification
  log, line-count governance, review disposition, and worker handoff.
- [x] Replace the production direct interleaved day-input callback so it does
  not invoke `DirectPublicationDayInputBuilder` inside the production direct
  day/OFE loop.
- [x] Ensure production direct day-input construction does not construct,
  clone, or merge `HillslopeWritebackSurface`, `BoundarySymbol`,
  `BoundaryValue`, symbol registries, indexed surfaces, dense refreshes, dirty
  flushes, or compatibility wrappers in the hot day/OFE loop.
- [x] Keep shadow/cutover/diagnostic compatibility paths available only through
  explicit non-production modes.
- [x] Update focused R7 tests so production direct manifests assert
  `compatibility_edge_invocations = 0` for the right reason.
- [x] Add or update static source scans proving production direct no longer
  calls the compatibility-shaped day-input builder.
- [x] Run focused R7/R6 validation, formatting, diff checks, scoped Markdown
  lint, and any additional gates required by the package disposition.
- [x] Complete dual review, explicit finding disposition, line-count
  governance, worker handoff, and final disposition.

## Correction Authority Envelope

Observed failure:

- `R7F-HOT-COMPATIBILITY-RUNTIME-NOT-ISOLATED`: production direct mode bypasses
  compatibility scheduler/kernel entrypoints, but still uses a
  compatibility-shaped day-input builder in
  `crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers.rs`
  for each production day/OFE. The builder holds `HillslopeWritebackSurface`
  seed/context surfaces, merges climate/lane state into surfaces, reads symbol
  maps, and was counted by R7E/R7H as a compatibility edge.

In-scope mechanisms:

- Production direct day-input construction in
  `crates/openwepp-runner/src/hillslope/05_runner_execution_and_outputs.rs`.
- Direct publication day-input helpers in
  `crates/openwepp-runner/src/hillslope/direct_publication/**`.
- Narrow typed climate/day-input accessors in
  `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/**` when needed
  to consume existing climate-runtime authority without surface projection.
- Direct runtime executor APIs in
  `crates/openwepp-hillslope-orchestrator/src/direct_runtime/**` when needed
  for typed production day-input consumption.
- Focused runner/orchestrator tests proving production direct no-compatibility
  counters and source-scan guards.
- Package-local docs and execution log/catalog updates.

Allowed edit classes:

- Move production direct hot-loop data flow from compatibility surfaces to
  typed direct inputs/state.
- Add narrow typed data accessors that expose already-authoritative parsed or
  adapter-computed values without changing process physics.
- Split helper modules when line-count governance requires it.
- Add fail-closed guards when a production direct typed input is missing or
  unsupported by current canonical authority.
- Update tests and source scans to assert the new no-hot-edge boundary.

Protected boundaries:

- Do not change hydrology, erosion, snow, frost, ET, or routing physics merely
  to satisfy output identity or counters.
- Do not replace missing process authority with provisional formulas.
- Do not hide `DirectPublicationDayInputBuilder`, `HillslopeWritebackSurface`,
  `BoundarySymbol`, `BoundaryValue`, symbol maps, or scheduler surfaces behind
  a differently named production direct hot-loop wrapper.
- Do not delete explicit compatibility, replay, shadow, or diagnostic modes.
- Do not claim R7G performance or R7H release readiness from this package.

## Terminal-State Rule

This package has two honest terminal states:

1. `COMPLETE-R7F-DIRECT-DAY-INPUT-HOT-LOOP-ISOLATION`: production direct
   hot-loop day-input construction is typed and no longer invokes the
   compatibility-shaped builder or compatibility surfaces; source scans and
   focused fixtures prove direct `compatibility_edge_invocations = 0`; R6
   cutover/shadow paths remain intact; all review findings are dispositioned.
2. `HOLD-R7F-<SPECIFIC-BOUNDARY>`: a blocker is proven outside this package's
   envelope with direct evidence and review. The worker handoff must start
   with `close defect <ID>` and name the first implementation action.

Invalid terminal reasons:

- "the builder was renamed";
- "the counter was suppressed";
- "a wrapper hides the compatibility surface";
- "source scan was narrowed to miss the call";
- "R7 tests pass but production direct still uses the builder";
- "static setup still needs work" without proving the hot loop is isolated;
- "another blocker appeared" while that blocker is in-envelope.

## Acceptance Gates

- Static source scan: production direct execution no longer references
  `DirectPublicationDayInputBuilder` in the direct day/OFE loop.
- Static source scan: the production direct day/OFE loop does not construct,
  clone, or merge `HillslopeWritebackSurface`, `BoundarySymbol`,
  `BoundaryValue`, symbol registries, indexed surfaces, dense refreshes, dirty
  flushes, or compatibility wrappers for day-input construction.
- Runtime counter gate: production direct manifests report
  `/direct_runtime_counters/compatibility_edge_invocations = 0`.
- Consumer-path proof: HBP/WAT/PASS/loss/manifest publication for production
  direct still reads `DirectRunPublicationFrame` emitted by the direct
  executor, not compatibility `wb13_rows`.
- Regression gate: focused R7 tests pass.
- Preservation gate: focused R6 direct-publication cutover/shadow tests pass.
- Formatting/doc gate: `cargo fmt --check`, `git diff --check`, and scoped
  Markdown lint pass.
- Closure gate: if this package claims complete implementation, run the Rust
  closure loop required by `docs/work-packages/AGENTS.md` or explicitly record
  a legitimate hold before that point.

## Execution Protocol

1. Read required docs and current R7E/R7H handoff.
2. Inventory every production direct hot-loop compatibility dependency.
3. Implement the smallest typed replacement that removes the production-loop
   edge without changing process physics.
4. Run the focused R7 gate.
5. If the gate fails and the failure is in-envelope, fix it and rerun. Do not
   stop after diagnostic evidence or a renamed blocker.
6. When focused R7 is green, rerun focused R6 and static source-scan evidence.
7. Update artifacts as evidence is produced, complete review/disposition, and
   close complete or hold truthfully.

## Deliverables

- Production direct code path changes.
- Focused tests/source scans for no-hot-compatibility production direct mode.
- Package artifacts:
  - `artifacts/required-reading.md`
  - `artifacts/blocker-ledger.md`
  - `artifacts/compatibility-edge-inventory.md`
  - `artifacts/typed-projection-plan.md`
  - `artifacts/consumer-path.md`
  - `artifacts/verification.md`
  - `artifacts/line-count.md`
  - `artifacts/review-disposition.md`
  - `artifacts/worker-handoff.md`
- Catalog updates in `docs/work-packages/README.md`.

## Security Impact

No secret handling or external I/O changes are intended. The package changes
runtime selection and production execution internals only. It must preserve
fail-closed behavior for missing typed process authority and must not weaken
serialization, checksum, manifest, or sidecar validation.

## Outcomes & Retrospective

Final disposition:
`COMPLETE-R7F-DIRECT-DAY-INPUT-HOT-LOOP-ISOLATION`.

Implementation:

- Added typed climate-day forcing accessors to
  `HillslopeClimateRuntimeRequest`.
- Added `DirectProductionDayInputBuilder`, which parses setup-time lane
  authority from seeded day-zero surfaces but builds each production direct
  day/OFE input from typed climate forcing and committed `DirectRunFrame` /
  `DirectLaneFrame` state.
- Switched `execute_hillslope_direct_production_days` to the typed production
  builder.
- Preserved the compatibility-shaped `DirectPublicationDayInputBuilder` for
  explicit shadow/cutover/diagnostic paths.
- Tightened focused tests so explicit production direct and default-activated
  production direct require zero compatibility-edge invocations, and added a
  source scan proving the typed builder hot-loop body has no runtime-surface
  reads.

Validation:

- Ran: `cargo test -p openwepp-runner r7 -- --nocapture`.
- Ran: `cargo test -p openwepp-runner r6 -- --nocapture`.
- Ran: `cargo fmt --check`.
- Ran: `cargo clippy --workspace --all-targets -- -D warnings`.
- Ran: `cargo test --workspace`.
- Ran: `cargo deny check`.
- Ran: `cargo clippy -p openwepp-runner --all-targets -- -D warnings`.
- Ran: `git diff --check`.
- Ran: `markdown-doc lint --path docs/work-packages/20260623-r7f-direct-day-input-hot-loop-isolation-001 --path docs/work-packages/README.md --format json`.

Residual risk:

- Static process-control authority is still extracted from setup-time seeded
  surfaces. That is intentionally outside the production direct hot loop and is
  future static-authority migration scope, not an R7F hold.
- Active material snow/frost carry remains fail-closed for this typed
  production path until a later package migrates that authority surface-free.

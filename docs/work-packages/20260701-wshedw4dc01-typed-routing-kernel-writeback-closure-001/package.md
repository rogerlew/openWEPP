# WSHED-W4DC01 Typed Routing Kernel Writeback Closure

Status: `EXECUTED-COMPLETE`

Date opened: `2026-07-01`

Package type: Defect-Closure ExecPlan; WSHED-W4 hold-lift implementation
package.

## Objective

Close defect `WSHED-W4-HOLD-001` end to end: the real public
`openwepp-cli-watershed` path still routes by projecting
`WatershedNetworkFrame` into `WatershedWritebackSurface` through
`compatibility_writeback_surface()` and then calling
`execute_watershed_dispatch_with_kernel`. Replace that compatibility edge with
a `WatershedNetworkFrame`-native dispatch path that reads typed channel,
impoundment, topology, and hillslope contribution records and writes typed
`RoutedChannelState`, `RoutedImpoundmentState`, and `WatershedPublicationFrame`
values directly.

## Observed Defect

Defect ID: `WSHED-W4-HOLD-001`.

Observable failure: WSHED-W4 could not close complete because production routing
still depends on symbol-map writeback state:

- public CLI calls `network_frame.compatibility_writeback_surface()`;
- public CLI then calls `execute_watershed_dispatch_with_kernel`;
- orchestrator dispatch still applies `KernelWritebackPayload` into
  `WatershedWritebackSurface`;
- WS10/WS11/WS12/WS18/WS20 helpers still read routing inputs from
  `BoundarySymbol`/`BoundaryValue` maps;
- typed publication currently harvests the compatibility report and therefore
  inherits compatibility zero-default behavior for missing routed operands.

Evidence source:
`docs/work-packages/20260701-wshedw4-typed-watershed-network-frame-001/artifacts/disposition.md`.

## Rationale

W4 landed the typed frame/publication handoff but truthfully held because the
kernel dispatch consumer still reads and writes the old runtime surface. This
package is the hold-lift. It is intentionally not another inventory or
diagnostic package: the first actionable item is to close the named typed
routing kernel/writeback blocker inside the declared authority envelope.

W5 old-runtime deletion is blocked until this package removes production
routing dependence on `WatershedWritebackSurface`. W6 publication/scaling work
is blocked until typed routing and publication provenance are no longer
compatibility-harvested.

## Correction Authority Envelope

### Defect IDs and Observed Violations

- `WSHED-W4-HOLD-001`: production routing uses
  `compatibility_writeback_surface()` and
  `execute_watershed_dispatch_with_kernel`, so W4's required typed
  routing/publication consumer-path gate is not satisfied.

### In-Scope Contracts and Authority

- `docs/architecture/watershed-runtime-architecture-specification.md`
- `docs/decisions/0032-watershed-runtime-ratification.md`
- `docs/specifications/science-contracts/contracts/SC-ROUTE-001.md`
- `docs/specifications/science-contracts/contracts/SC-IMPOUND-001.md`
- `docs/specifications/science-contracts/contracts/SC-RUNOFFPART-001.md`
- `docs/specifications/science-contracts/contracts/SC-SED-001.md`
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- Watershed infile contracts when typed frame builders or parser bindings are
  changed.

If execution changes routing physics, impoundment physics, sediment physics,
publication semantics, unit lineage, domain guards, or output meaning, amend
canonical `SC-*` authority before production code.

### In-Scope Write Set

- `crates/openwepp-watershed-orchestrator/src/lib_mod/network_frame.rs`
- `crates/openwepp-watershed-orchestrator/src/lib_mod/dispatch.rs`
- `crates/openwepp-watershed-orchestrator/src/lib_mod/kernel/**`
- `crates/openwepp-watershed-orchestrator/src/lib_mod/types.rs`
- `crates/openwepp-watershed-orchestrator/src/lib_mod/mod.rs`
- `crates/openwepp-watershed-orchestrator/src/lib.rs`
- `crates/openwepp-runner/src/bin/openwepp-cli-watershed.rs`
- `crates/openwepp-runner/tests/watershed_cli_behavior_contract.rs`
- `tests/integration/**` for typed routing/source-guard/protected-output tests
- `docs/specifications/science-contracts/**` only if contract amendments are
  required by discovered semantic changes.
- `docs/work-packages/20260701-wshedw4dc01-typed-routing-kernel-writeback-closure-001/**`
- `docs/ROADMAP.md`
- `docs/work-packages/README.md`

Any production edit outside this write set requires package amendment before
implementation.

### Allowed Production-Edit Classes

- Add typed routing request/response records derived from
  `WatershedNetworkFrame`.
- Add a frame-native dispatch entrypoint, for example
  `execute_watershed_dispatch_with_frame`, that never materializes
  `WatershedWritebackSurface` for production routing.
- Refactor WS10/WS11/WS12/WS18/WS20 routing helpers to read typed inputs and
  return typed routed state.
- Centralize or preserve current fail-closed runtime-input guards in typed
  frame builders or typed dispatch validation.
- Replace compatibility-harvested publication operands with typed routed-state
  operands.
- Add source guards and protected output tests for typed routing/publication
  consumption.

### Protected Boundaries

- Do not change routing, impoundment, erosion, sediment, runoff-partition, or
  water-balance physics for performance.
- Do not loosen fail-closed domain guards or silently clamp invalid inputs.
- Do not normalize fixture data unless package authority is amended and
  checksum provenance is recorded.
- Do not change public output schemas unless canonical contract authority and
  migration evidence require it.
- Do not delete old runtime files wholesale; W5 owns final deletion after this
  hold is lifted.
- Do not close on adapter-only, producer-only, shadow-only, source-counter-only,
  or test-only evidence.

### Validation Surfaces

- Public `openwepp-cli-watershed` generated-mode runs.
- `watershed_cli_behavior_contract` W2/W3/W4 public behavior tests.
- New typed-routing source guards.
- Committed `tests/fixtures/watershed/carnivorous-adobo/` fixture for protected
  output identity or contract-governed deltas.
- Focused orchestrator typed-routing tests.
- Independent output reconstruction and conservation/magnitude audit for typed
  publication operands.

### Acceptance Criteria

- Public CLI routes without `compatibility_writeback_surface`.
- Production routing loops no longer read or write `WatershedWritebackSurface`,
  `BoundarySymbol`, or `BoundaryValue`.
- `execute_watershed_dispatch_with_kernel` is not used by the production public
  CLI claim; any remaining use is path-scoped as replay, comparator,
  diagnostic, or obsolete-test code.
- Typed frame builders and typed dispatch enforce the same fail-closed domain
  guards currently protected by the compatibility projection, or those guards
  are centralized and covered by typed tests.
- Typed publication does not silently default missing routed operands to zero
  unless canonical output authority explicitly permits that behavior.
- Protected outputs are identity-equivalent on committed fixtures, or deltas are
  contract-governed with accepted review disposition.
- Independent reconstruction and conservation/magnitude audit are recorded.

## Conversion Rule

If this package establishes a reproducible root cause inside the declared
envelope, and the expected behavior is supported by canonical `SC-*` authority,
pinned-baseline provenance, or a contract-authorized physical invariant, it
must proceed through contract amendment when needed, contract-derived tests, a
pre-implementation gate, production correction, validation, and dual-review
disposition. It may not close as `HOLD` merely because further investigation is
possible.

## Seven-Gate Bar

The package may close complete only when all seven gates pass:

1. Reproduction: `WSHED-W4-HOLD-001` is reproduced or statically confirmed on
   the current tree.
2. Mechanism: the remaining old-surface dependency is reduced to named typed
   dispatch/writeback mechanisms.
3. Ownership: the mechanism lies inside the declared write set and contract
   authority.
4. Authority: expected typed behavior is supported by architecture, canonical
   `SC-*` authority, pinned baseline provenance, or a contract-authorized
   physical invariant.
5. Safety: the fix does not loosen guards, silently clamp, invent physics, or
   canonicalize a domain violation away.
6. Testability: contract-derived or source/protected-output tests fail on the
   old path and pass on the typed path.
7. Validation: the acceptance targets are measurable before and after the
   correction.

## Legitimate Hold Boundaries

This package may close `EXECUTED-HOLD` only if one of these boundaries is proven:

- The mechanism requires a different process-family contract authority not
  declared in this envelope.
- Governing canonical authority is missing or contradictory and must be decided
  before production code.
- Existing behavior is proven correct because the input is invalid upstream and
  the fail-closed guard is the right outcome.
- Required evidence cannot be generated in this environment after equivalent
  focused evidence has been attempted or delegated.
- A required production edit is outside the declared write set and cannot be
  safely added by package amendment.

The package must not hold on "inspect the next function", "trace the next
variable", or "root cause is in-envelope but implementation is deferred".

## Included Scope

- Replace production routing/writeback with typed frame-native dispatch.
- Remove production public CLI dependence on `compatibility_writeback_surface`.
- Remove production routing-loop dependence on `WatershedWritebackSurface`,
  `BoundarySymbol`, `BoundaryValue`, and `KernelWritebackPayload`.
- Replace compatibility-harvested publication operands with typed routed-state
  operands and fail-closed missing-operand handling.
- Preserve W2/W3 supervisor, worker-pool, pass-inventory, and fail-closed
  behavior.
- Add source guards and protected output identity or contract-governed delta
  evidence.
- Record conservation/output reconstruction and magnitude audit.

## Excluded Scope

- No W5 full old-runtime deletion after production cutover, except targeted
  code moves needed to remove production dependence.
- No W6 large fixture adoption.
- No worker-pool scheduling changes unless required to preserve W3 behavior.
- No physics changes for performance.
- No output schema redesign.

## Phase Plan

1. Reproduce and classify:
   - confirm current public CLI still uses `compatibility_writeback_surface`;
   - inventory production old-surface read/write sites remaining after W4;
   - update the seven-gate artifact.
2. Authority and contract gate:
   - read relevant `SC-*` contracts for touched routing/publication surfaces;
   - amend contracts and add contract-derived tests if semantics, guards, or
     unit lineage change;
   - record pre-implementation contract gate evidence.
3. Design typed dispatch:
   - define typed routing request/response records;
   - map WS10/WS11/WS12/WS18/WS20 inputs and outputs from current symbol fields
     to typed fields;
   - choose a migration path that keeps compatibility projections out of the
     production public CLI claim.
4. Implement correction:
   - add frame-native dispatch;
   - migrate production routing helpers to typed inputs/outputs;
   - update public CLI to call the typed dispatch path;
   - make typed publication fail closed on missing required routed operands.
5. Tests and evidence:
   - add source guards for public CLI and orchestrator production routing;
   - protect W2/W3 behavior and W4 typed publication behavior;
   - prove protected output identity or contract-governed deltas on committed
     fixtures;
   - run independent reconstruction and conservation/magnitude audit.
6. Review and closure:
   - run focused gates and final closure gates;
   - complete dual review, science/QA verification, line-count governance,
     consumer-path proof, and final disposition;
   - update W4/W5/W6 roadmap state.

## Exit Criteria

- `WSHED-W4-HOLD-001` is closed complete or the package holds only at a
  legitimate declared boundary.
- Public CLI contains no production call to `compatibility_writeback_surface`.
- Public CLI routes through a typed dispatch function consuming
  `WatershedNetworkFrame`.
- Production orchestrator routing loops do not read/write
  `WatershedWritebackSurface`, `BoundarySymbol`, `BoundaryValue`, or
  `KernelWritebackPayload`.
- Remaining old-surface uses are path-scoped and source-guarded as compatibility
  or obsolete-test surfaces.
- Typed publication fails closed on missing required routed operands.
- Protected outputs remain identity-equivalent or deltas are contract-governed.
- Conservation reconstruction and magnitude audit pass.
- Full Rust closure gates pass, or a legitimate hold boundary is documented.

## Required Final Gates

For complete closure, run and record:

1. `cargo fmt --check`
2. `cargo clippy --workspace --all-targets -- -D warnings`
3. `cargo nextest run --workspace --profile full`
4. `cargo deny check`
5. focused W4DC typed routing/source-guard/protected-output/conservation gates.

If a complete gate cannot run, the package must close `EXECUTED-HOLD` with the
exact blocker unless a canonical decision explicitly authorizes a narrower
closure.

## Security and Safety

Do not introduce shell interpolation, network dependencies, credential handling,
silent dependency fallbacks, or broad path discovery. Preserve typed
fail-closed behavior for invalid plans, child failures, pass inventory failures,
missing routed operands, and domain violations. Do not canonicalize-and-proceed
unless a canonical `SC-*` contract explicitly authorizes bounded normalization
with units, thresholds, provenance, tests, and evidence.

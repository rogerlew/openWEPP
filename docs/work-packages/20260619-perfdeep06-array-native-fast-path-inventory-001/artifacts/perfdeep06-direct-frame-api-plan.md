# PERFDEEP06 Direct-Frame API Plan

Status: complete 2026-06-19.
Evidence class: Static.

## API Direction

PERFDEEP07 should add a bounded direct-frame hydrology path that bypasses
`HillslopeKernelRequest` and `KernelWritebackPayload` for the migrated phases.
The path remains opt-in and shadow-compared until identity and endpoint gates
pass.

Before adding more opt-in plumbing, PERFDEEP07 must close the current
default-disabled regression. PERFDEEP05 final-code default-disabled H2637
measured `701.95 s` versus the `669.97 s` reference, and PERFDEEP03
default-disabled had already measured in the `697-708 s` band. Direct-frame API
work must therefore introduce an explicit runtime path split: when all
PERFDEEP opt-ins are off, the scheduler must avoid dense-first request/view
construction, dense slot resolution, indexed shadow setup, and any direct-frame
shadow machinery.

Disabled-path timing protocol for PERFDEEP07:

- run at least three clean H2637 no-UI default-disabled endpoint measurements
  with all PERFDEEP environment opt-ins unset/off;
- record min/median/max seconds and RSS, plus the exact command/environment;
- run the candidate and same-machine control in the same harness/session when
  feasible. If the historical `669.97 s` reference cannot be rerun on the same
  host, PERFDEEP07 must still report a same-machine pre-cleanup control and
  cannot pass the disabled-path gate above `676.67 s` without hard attribution
  to an external environment change;
- PASS requires median `<= 676.67 s`, which is the `669.97 s` reference plus a
  predeclared 1% noise allowance;
- any remaining construction/resolution of dense/indexed/direct-frame
  compatibility plumbing on the disabled path is a static FAIL even if timing is
  within the noise allowance.

Proposed first-cut types:

- `HillslopeDayContext`: immutable lane/day keys, OFE id, contributor count,
  area ratio, geometry, calendar projection, and scheduler message context.
- `HillslopeDayForcing<'a>`: borrowed climate/hyetograph/winter hourly inputs.
  This type owns no per-phase allocation.
- `HillslopeDayFrame`: lane-owned mutable state with named typed scalars,
  fixed MOFE `[f64; 24]` arrays, soil/frost SoA columns, snow runtime state,
  growth scalars, erosion diagnostics, and a publication projection slot.
- `HydrologyFrameView<'a>`: borrow-split view passed to one phase. It can expose
  only the fields that phase owns/consumes, or start as a whole-frame `&mut`
  while PERFDEEP07 proves identity.
- `FrameGuardError`: typed fail-closed guard error carrying phase, field,
  unit, value, bound, and compatibility subject name for diagnostic parity.
- `HillslopeDayPublicationProjection`: typed WB13/PASS/HBP row source built at
  the I/O edge from frame fields.

Example target shape, not production code:

    fn run_percolation_frame(
        frame: &mut HillslopeDayFrame,
        context: &HillslopeDayContext,
        forcing: &HillslopeDayForcing<'_>,
    ) -> Result<(), FrameGuardError>;

    fn run_hydrology_frame_chain(
        frame: &mut HillslopeDayFrame,
        context: &HillslopeDayContext,
        forcing: &HillslopeDayForcing<'_>,
    ) -> Result<(), FrameGuardError>;

## Mixed-Mode Boundary

PERFDEEP07 should port one complete hydrology daily OFE chain over direct frame
APIs and keep the old logical path as a shadow oracle. Logical surfaces may
exist only at these boundaries:

- initial seed from current runtime surface into typed frame;
- shadow comparison against current logical phase outputs;
- non-migrated phase boundary if PERFDEEP07 intentionally stops short of all 14
  phases;
- output/replay/diagnostic adapters outside the migrated normal success path.
- default-disabled execution must bypass all of the above unless an explicit
  opt-in or shadow validation mode is active.

Inside the migrated success path, the direct-frame chain must not construct
`HillslopeKernelRequest`, `KernelWritebackPayload`, `BoundarySymbol`, or
`BoundaryValue`.

## Guard and Validity Policy

Frame fields that are always required should be plain typed fields. Optional
semantics should use explicit validity bitsets or separate optional projection
fields, not `Option<BoundaryValue>` arrays. Guard errors should preserve the
current message-id class and boundary subject compatibility until a contract
ratifies field-oriented diagnostics.

## Shadow Identity

For each OFE-day in a focused fixture and in H2637, PERFDEEP07 should execute
the logical path and direct-frame path, then compare:

- all migrated frame-owned scalar outputs by `f64::to_bits()`;
- all fixed arrays element-by-element;
- WB13 publication projection rows;
- final HBP/WAT/PASS outputs.

## Kill Criteria

- Any unresolved bit identity drift on a migrated field: `HOLD`, not complete.
- Default-disabled H2637 median remains above `676.67 s` after
  removing/bypassing dense-first plumbing without hard same-machine attribution:
  `HOLD` for attribution or `NO-GO` if the package cannot remove the tax.
- Direct-frame hydrology endpoint fails to move materially versus the
  final-code default-disabled path after map/writeback absence is proven:
  `NO-GO` and re-profile before expansion.
- Any required gate depending on a later package: current package is `HOLD`;
  do not relabel it future scope.

## Gate

PASS. PERFDEEP07 can implement a bounded direct-frame hydrology fast path from
this API plan without adding new physics or output authority.

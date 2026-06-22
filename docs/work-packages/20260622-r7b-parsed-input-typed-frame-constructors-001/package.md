# R7B Parsed-Input Typed Frame Constructors

Status: complete.

Package type: Array-native runtime implementation work package.

Objective: introduce production-grade typed constructor APIs for
`DirectRunFrame`, `DirectLaneFrame`, and `DirectDayFrame` so future production
direct mode can be built from parsed run, lane, climate, management, snow,
frost, PMET, and sidecar inputs without accepting hot compatibility storage as
normal direct-frame authority.

Subagent authorization: this package explicitly authorizes
spawning/delegating to read-only reviewer and verifier subagents for
constructor-surface review, static no-compatibility storage review, test/gate
audit, and architecture-catalog consistency review. Expected outputs are
compact Markdown findings summarized into `artifacts/review-disposition.md`;
subagents may not edit files.

## Rationale

R7A reconciled the architecture and made R7B the first implementation step for
future production direct mode. Current direct-runtime construction is still
centered on `DirectRunFrame::skeleton` and `DirectDayFrame::seed`, with runner
adapters seeding direct publication inputs from compatibility execution state.
R7B must add a typed parsed-input constructor boundary that rejects forbidden
hot compatibility storage at the API and type level before R7C can route the
executor through production direct mode.

## Scope

In scope:

- Add or harden typed direct constructor input structs for run, lane, and day
  construction.
- Add constructor APIs that accept parsed/direct seed data rather than
  `HillslopeWritebackSurface`, `BoundarySymbol`, `BoundaryValue`,
  `SymbolRegistry`, indexed surfaces, `HillslopeKernelRequest`,
  `KernelWritebackPayload`, or WB13 rows.
- Move first-class sidecar and parsed-input seed fields into typed direct
  constructor inputs where they are currently needed by R4/R5 spans.
- Add typed finite/domain/unit validation before direct executor entry.
- Record layout/type-size evidence for direct frame state and major
  constructor input families.
- Add roundtrip fixtures for single-OFE, multi-OFE, snow/frost, PMET,
  breakpoint climate, management, and sidecar absence/default constructor
  cases at the typed constructor boundary.
- Preserve the default compatibility path and direct publication cutover
  behavior unless tests prove an intended constructor-only change.

Out of scope:

- Default direct runtime activation.
- Replacing compatibility climate-day execution.
- R7C production executor routing.
- R7D publication producer authority cutover.
- Output schema or public file format changes.
- Science-contract changes or process-physics formula changes.
- Broad runner refactors unrelated to constructor authority.

## Write Set

- `crates/openwepp-hillslope-orchestrator/src/direct_runtime.rs`
- `crates/openwepp-hillslope-orchestrator/src/direct_runtime/**`
- `crates/openwepp-hillslope-orchestrator/src/lib.rs`
- `crates/openwepp-hillslope-orchestrator/src/tests/tests_mod/**`
- `docs/architecture/array-native-runtime-specification.md`
- `docs/work-packages/README.md`
- `docs/work-packages/20260622-r7b-parsed-input-typed-frame-constructors-001/**`

## Phase Plan

1. Scaffold this package, prompt, evidence placeholders, and catalog pointer.
2. Inventory existing direct-frame constructor surfaces and forbidden
   compatibility storage types.
3. Implement typed constructor input structs and constructor APIs for
   `DirectRunFrame`, `DirectLaneFrame`, and `DirectDayFrame`.
4. Bind constructor validation for finite/domain/unit-sensitive fields before
   executor entry.
5. Add constructor fixtures covering required R7B cases and static
   no-compatibility scans.
6. Record type-size/layout evidence, line-count governance, reviews, finding
   disposition, verification, and worker handoff.
7. Run focused Rust tests, static scans, scoped Markdown lint, and broader gates
   feasible within this package.

## Acceptance Criteria

- `DirectRunFrame`, `DirectLaneFrame`, and `DirectDayFrame` have typed
  constructor APIs that do not accept forbidden compatibility storage types.
- Constructor roundtrip fixtures cover single-OFE, multi-OFE, snow/frost, PMET,
  breakpoint climate, management, and sidecar absence/default cases.
- Static scans prove direct constructor input/storage declarations contain no
  forbidden compatibility storage types.
- Constructor output supplies the inputs currently read by R4/R5 direct phase
  spans.
- Typed validation rejects non-finite, negative where non-negative is required,
  invalid lane topology, out-of-range day/lane indices, and invalid area/ratio
  data before executor entry.
- Default compatibility path remains identity-clean and zero-cost-disabled at
  the source level; R7B does not add default-path constructor invocation.
- Focused tests, relevant integration tests, Rust formatting/checks, scoped
  docs lint, and `git diff --check` pass or any skipped broader gate is
  recorded with a truthful blocker.

## Security / Safety Impact

This package adds constructor validation and explicit typed direct input
boundaries. It must not weaken typed errors, fail-closed behavior, sidecar
validation, output parity, or anti-evasion posture. No secrets or generated
binary fixtures are in scope.

## Progress

- [x] Scaffold package.
- [x] Inventory constructor and compatibility storage surfaces.
- [x] Implement typed constructor inputs and APIs.
- [x] Add fixtures and static scans.
- [x] Run validation gates and update evidence.

## Outcomes

Final disposition: `COMPLETE-R7B-PARSED-INPUT-TYPED-FRAME-CONSTRUCTORS`.

R7B added:

- `DirectRunConstructorInputs`, `DirectLaneConstructorInputs`, and
  `DirectDayConstructorInputs`.
- `DirectRunFrame::from_constructor_inputs(...)` and
  `DirectDayFrame::from_constructor_inputs(...)`.
- Pre-executor validation for identity, topology, lane geometry, water state,
  transfer buffers, publication fields, subsurface layers, PMET inputs,
  forcing, day-input handoffs, frost carry projection shape, and typed
  non-finite/domain failure cases.
- Lane-owned day constructor seed storage consumed by `seed_day_frame(...)`
  when present, while preserving existing skeleton/default behavior.
- R7B fixtures for single-OFE defaults, multi-OFE parsed daily inputs,
  snow/frost, PMET, breakpoint climate, management/residue inputs, sidecar
  absence/defaults, invalid domains, static constructor no-compatibility scans,
  and type-size/layout guardrails.

R7B did not activate production direct mode, route default runner execution
through direct constructors, replace direct publication producer authority, or
change output schemas.

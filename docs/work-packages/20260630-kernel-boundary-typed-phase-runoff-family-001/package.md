# Kernel-Boundary Typed Phase Runoff Family

Status: EXECUTED-HOLD-FAMILY-BOUNDARY-EMBEDS-SNOW-FROST-INDEXED-OUTPUTS

Package id: `20260630-kernel-boundary-typed-phase-runoff-family-001`

## Objective

Introduce a typed kernel-boundary seam and prove it on one hydrology family:
runoff, infiltration, and reconciliation, including this family's
kernel-data-dependent diagnostics. The seam must use authority-bearing typed
context and typed result/mutation APIs, not wrappers around
`HillslopeKernelRequest`, `KernelWritebackPayload`, or symbol surfaces.

## Authority

- [Array-native runtime specification](../../architecture/array-native-runtime-specification.md)
  §8.2 step 2 requires typed phase context/result/mutation APIs and step 3
  requires typed diagnostic/event payloads.
- `20260630-kernel-boundary-survivor-inventory-001/` classifies
  `hydrology_phase_runoff_reconciliation.rs`, runoff/infiltration helpers, and
  HPHYS/reconciliation diagnostics as the relevant KB/TRACE surface.
- `20260630-kernel-boundary-typed-diagnostic-events-001/` carries forward typed
  direct-runtime trace payloads for existing R7H direct-state traces.
- [ADR-0025](../../decisions/0025-array-native-hillslope-day-frame.md) remains
  the single-authority typed runtime direction.

## Scope

In scope:

- Read the runoff/infiltration/reconciliation hydrology phase and diagnostics.
- Determine whether a one-family typed phase boundary can be cut safely without
  wrapping symbol surfaces.
- Report the exact carrier-reference surface and the implementation dependency
  that blocks this package.

Out of scope:

- No physics change.
- No output schema change.
- No scheduler/carrier deletion.
- No migration of other hydrology families.

## Execution

The package inspected:

- `hydrology_phase_runoff_reconciliation.rs`
- `support_helpers_mod/runoff_reconciliation.rs`
- `support_helpers_mod/infiltration_reconciliation.rs`
- `hydrology_phase_storage_reconciliation.rs`
- HPHYS/reconciliation diagnostics in `crates/openwepp-runner/src/hillslope/scheduler_trace/`
- the direct R4/R7H runoff and storage typed implementations

## Finding

The intended "one family" is not a narrow request/writeback boundary. In the
legacy scheduler phase, runoff reconciliation also embeds:

- snow partition state and 24-hour snow publication writes;
- frost coupling state, fine-layer state, hourly frost diagnostics, and
  freeze/thaw water terms;
- irrigation event state;
- MOFE hourly saturation and lateral carry arrays;
- regular `KernelWritebackPayload` writes and the indexed writeback fast path;
- HPHYS and per-OFE WB13 diagnostics derived from the same symbol surface.

The production direct runtime already has typed R4/R7H state for this family,
and the carried-forward `diagnostic_events.rs` work typed several direct trace
payloads. The remaining KB/TRACE surface is the compiled scheduler-era family.
Typing only the final result struct while still deriving it from
`HillslopeKernelRequest` or translating it immediately back into
`KernelWritebackPayload` would preserve the symbol surface as authority. That
would violate the array-native specification's false-wrapper rule and the
package objective.

## Gates

| Gate | Status | Evidence |
| --- | --- | --- |
| Typed context/result API introduced for this family | BLOCKED | [Implementation blocker](artifacts/implementation-blocker.md) |
| Family KB+TRACE carrier refs drop | FAIL | [Progress scan](artifacts/progress-scan.md) |
| Protected output identity | NOT RUN | No cutover was made |
| Family diagnostic identity | NOT RUN | No family diagnostic consumer was repointed |
| Focused compile of carried-forward changes | PASS | [Gate evidence](artifacts/gates.md) |
| Full workspace gates | NOT RUN | Held before a valid migration boundary |

## Disposition

Result:
`EXECUTED-HOLD-FAMILY-BOUNDARY-EMBEDS-SNOW-FROST-INDEXED-OUTPUTS`.

The correct next package should define the typed output/mutation vocabulary for
the embedded snow, frost, irrigation, MOFE, and indexed-writeback outputs first,
then cut `hydrology_phase_runoff_reconciliation.rs` over to:

1. extract symbols only at a compatibility adapter edge into owned typed input
   structs;
2. run an authority-bearing typed runoff-reconciliation core;
3. return an owned typed mutation/result object;
4. let only the deprecated compatibility adapter translate that typed result
   back to symbol writeback while scheduler support still exists.

That is the first implementation cut that can reduce carrier refs without
turning the typed boundary into a wrapper.

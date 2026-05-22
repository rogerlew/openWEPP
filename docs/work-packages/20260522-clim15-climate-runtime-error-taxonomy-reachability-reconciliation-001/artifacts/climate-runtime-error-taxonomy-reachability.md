# Climate Runtime Error Taxonomy Reachability (CLIM15)

Evidence mode: `Static`
Status: `implemented`

Static:
- `CLIM-RUNTIME-E-010` was unreachable because strict monotonic-time guard (`E-009`) always rejects `delta_time_s <= 0` first.
- The unreachable `E-010` variant/branch has been retired from shared and watershed taxonomy surfaces.
- Reachable breakpoint-path taxonomy retained and validated by guard behavior:
  - `E-006` negative `drain`
  - `E-009` non-monotone breakpoint time
  - `E-011` cardinality policy/count-range paths

## Taxonomy Reconciliation Actions
1. Shared adapter (`openwepp-climate-runtime-adapter`)
- removed unreachable `PositiveBreakpointDrainWithNonPositiveDeltaTime` variant.
- removed dead `delta_time_s <= 0` guard branch.

2. Watershed contextual runtime seam
- removed contextual mirror variant and mapping arm for retired `E-010` branch.

3. Hillslope taxonomy alignment
- point-count overflow surface errors now map to
`BreakpointCardinalityPolicyExceeded` (policy semantics).

## Reachability Policy
Only guard-emitted paths are accepted as taxonomy closure evidence.

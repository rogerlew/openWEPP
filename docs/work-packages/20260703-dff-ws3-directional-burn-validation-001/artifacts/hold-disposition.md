# Hold Disposition

Evidence class: Static + Ran.

Status: `EXECUTED-HOLD-DFF-WS3-SEDIMENT-PRODUCTION`

## Findings

### High - Sediment ordering cannot close in WS-3 yet

Static: `crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/00_builders_and_authority.rs`
currently sets `wave1_enabled = false` and publishes
`DirectErod13Inputs::zero()` into the direct erosion authority. That prevents
real Wave-1 detachment inputs from reaching the production sediment path.

Static: `docs/specifications/science-contracts/contracts/SC-SED-001.md` requires
real Wave-1/Wave-2 operands for direct erosion/sediment publication. A
runfile-only Wave-2 switch without Wave-1 production operands would not satisfy
the contract-backed consumer-path rule.

Ran: diagnostic p4 execution with Wave-2 enabled completed and the manifest
reported Wave-2 active, but `H4.pass.parquet` still had zero `tdet`, `tdep`,
and `sedcon_*` over all 2,192 rows. This confirms the problem is not merely a
missing matrix assertion.

Disposition: accepted. WS-3 is held and the proper correction is queued in
`../../20260703-dff-ws3a-wave1-wave2-sediment-production-001/package.md`.

### Medium - Runoff and peakflow validation can proceed without native forest lanuse

Static: WS-2 burn effect enters through disturbed soil `ksatadj`, not the WS-1
native forest management lanuse branch. The imported matrix uses the existing
cropland-shaped management files plus disturbed `.sol` policies, matching the
handoff's allowed pre-WS-1 validation path.

Ran: representative p1/p4 direct-runtime execution shows high-burn runoff and
peakflow exceed the matched unburned cell.

Disposition: accepted. The fixture and representative test remain in this
package. Native forest lanuse is not required for the current runoff/peak
characterization.

### Low - Legacy peakflow artifact is an adjudication flag, not a target

Static: `HANDOFF.md` and ADR posture treat the legacy `380150 m3/s` high-burn
peakflow as physically impossible forensic evidence.

Ran: the representative openWEPP p4 run publishes max `peakro` near
`8.24e-6 m3/s`, so the artifact is not reproduced by the WS-2 `ksatadj` direct
runtime path.

Disposition: accepted. A formal magnitude-envelope invariant can be added later
if needed, but WS-3 does not target the legacy peak.

## Hold Legitimacy Audit

Boundary: production Wave-1/Wave-2 sediment operands are missing from the real
direct runtime consumer path.

In-envelope route considered: add a runtime selector that activates Wave-2 for
the WS-3 run. That route is rejected because it does not supply real Wave-1
inputs and still publishes zero sediment in the downstream HBP parquet output.

Required follow-on: implement proper contract-backed Wave-1 and Wave-2
production in a dedicated package, prove nonzero sediment publication on real
cells where physics produces sediment, then return to WS-3 sediment ordering.

# Intake Assessment

Status: `EXECUTED-HOLD-MISSING-CHANNEL-HOURLY-SEDIMENT-SEQUENCING-AUTHORITY`

Evidence mode: `Static` plus targeted `Ran` source/repository commands; runtime
tests: `not-run` because the pre-implementation contract gate blocked edits.

## Proven Current Path

| Stage | Current surface | Static assessment |
|---|---|---|
| HBP parser | `HbpLatestEventPayload.hourly_runoff_volume_m3`, `hourly_sediment_mass_kg` | Paired 24-slot minor-1 arrays are parsed and integral-validated. |
| Supervisor/frame | `HillslopeContribution` | Both arrays reach the typed watershed frame. |
| Leaf-channel water inlet | `assemble_direct_incoming_peak_partition` | Sums hourly volumes, then reduces them to maximum hour-mean discharge, total volume, and active span. |
| Leaf-channel sediment inlet | `hourly_sediment_inlet_kg`, `read_direct_hillslope_sediment_payload` | Sums hourly mass; the channel solve receives total mass and a single rate duration based on sediment active span. |
| Channel output | `RoutedChannelState` | Scalar output only; no routed hourly water or sediment series. |
| Dependency intake | `direct_hourly_resolved_runon` | Hourly hillslope plus any dependency node fails closed with `hillslope_hourly_with_dependency_without_channel_hourly`. |
| Production proof | `mt3_hbp_hourly_consumer_contract.rs` | One HBP hillslope and one channel; no channel-to-channel hourly propagation. |

## Exact Missing Capabilities

1. A contract-defined channel time grid and conservative HBP-bin projection.
2. Stateful routed channel hydrograph output rather than peak/volume/span only.
3. Time-resolved particle-class sediment ingress and routed egress.
4. Typed hourly state on `RoutedChannelState` and dependency superposition.
5. Multi-channel production CLI proof and network conservation reconstruction.
6. Explicit support/failure policy for `ipeak` branches and impoundments.

## Authority Disposition

ADR-0036 D2 explicitly authorizes the first-cut reconstruction
`M[h,k] = S_h * frcflw[k]` using event-level class fractions uniformly across
hours. `SC-SED-001#GAP-SED-008` requires that result to remain labeled as the
day-level blend, not true enriched per-hour composition. HBP minor-1 is therefore
sufficient for this limited W11 ingress rule and no schema extension is needed.

The blocking gap is downstream of HBP. Baseline `chnrt.for` consumes event
per-class masses and runs once after water routing, using constant class flux
over scalar `rundur`. It does not define per-interval WS18-WS26 execution,
bed/profile carry between intervals, or routed hourly class egress. ADR-0036 D3
requires routing the pair but does not specify those process-state transitions.
Inventing independent hourly channel-sediment solves would violate the package's
no-surrogate-physics rule.

## Rejected Closure Claims

- `max(V_h)/3600`, `sum(V_h)`, and active span are not a routed hydrograph.
- `sum(S_h)` and sediment active span are not time-resolved sediment routing.
- A leaf-channel sensitivity test is not channel-network dependency proof.
- Repeating the event solver independently for 24 bins is not stateful channel
  routing.
- Reconstructing upstream hourly output from scalar channel results is not typed
  consumer-path closure.

# ADR-0033: OFE-by-OFE Overland-Flow Routing (Papanicolaou 2018)

Status: **Proposed** (2026-07-02, MOFEFID Lane D / D1)
Deciders: maintainers (operator-directed; ratification pending)
Supersedes/relates: extends the runoff-partition surface (SC-RUNOFFPART-001),
consumes the inter-OFE transfer seam (INV-RUNOFFPART-029), builds on the
DC01 runon re-infiltration semantics (INV-RUNOFFPART-031).

## Context

openWEPP inherited legacy WEPP's **equivalent-plane / equilibrium-storage**
overland-flow representation (Wu, Yevjevich & Woolhiser 1978, R-73): the
hillslope's OFEs are aggregated into one plane with a single space/time-
invariant roughness. Papanicolaou et al. (2018, R-63) — a WEPP-lineage paper
(Flanagan, Frankenberger co-authors) — removes that limitation: route the
event hydrograph **OFE-by-OFE** (each OFE's outlet hydrograph is the next
OFE's upstream boundary condition), compute resistance per-OFE per-timestep
from an additive friction menu (skin/form/wave/vegetation, eqs. 2-6), and
solve the 1-D kinematic wave with a TVD-MacCormack shock-capturing scheme
(eqs. 8-14). Reported deviations from the invariant-resistance assumption
reach 65% in peak runoff.

openWEPP is the right host: lanes **are** OFEs and
`TransferOutput -> TransferInput` (INV-RUNOFFPART-029) is already the
inter-OFE seam. The enhancement replaces the *content* of the transfer (an
hourly hydrograph instead of a daily aggregate) without inventing new
topology. MOFEFID-A01's F-A2 finding and DC01 established that per-OFE
runon **re-infiltrates** downslope (Papanicolaou assumption 2 — per-OFE
Green-Ampt of the routed excess); Lane D makes that hourly-faithful.

## Decision (proposed)

1. Adopt OFE-by-OFE kinematic-wave routing with space/time-variant
   resistance as an **opt-in** hillslope subsystem behind a policy flag.
   The default hillslope runtime stays byte-flat; **default activation is a
   separate later gate** with its own no-regression + endpoint-timing +
   magnitude re-adjudication (operator decision 2026-07-02).
2. Author **SC-OFEROUTE-001** as the governing contract (hillslope overland
   routing; distinct from the watershed-channel SC-ROUTE-001).
3. Structure: friction-factor kernels (D3, pure functions — landed) ->
   single-OFE KWE/TVD solver -> OFE-by-OFE cascade handoff over the
   INV-RUNOFFPART-029 seam, each stage shadow-first with per-increment
   conservation stops.

## Consequences

- Compute-bearing (hourly per-OFE PDE). The opt-in path carries its own
  event-day budget; the default path is protected.
- Frozen-library posture: eq. 2-3 constants + eq. 4 bounds cited secondary
  via R-63; eq. 4/5/6 primaries (R-77/72/78) in hand; unit conventions
  confirmed by the D-val fixtures (Ef 0.91/0.75/0.87/0.88).
- Ratification pending: the friction kernels (D3) are shadow-first and
  reversible; ratifying this ADR authorizes the solver + cascade stages.

## Open questions (for ratification)

- Exact kinematic-viscosity / rainfall-intensity unit convention (pinned by
  D-val Case 1/2 reproduction).
- CFL sub-timestep policy within the hillslope day loop.
- How the routed hourly hydrograph reconciles with the existing WB16 peak
  operands and DC01's daily runon supply (the two must not double-count).

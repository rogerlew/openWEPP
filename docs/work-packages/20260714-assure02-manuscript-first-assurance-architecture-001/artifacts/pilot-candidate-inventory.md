# Pilot Candidate Inventory

Status: complete

Scope: bounded screen of mature, production-relevant non-snow kernels with
retained evidence already discoverable in the current repository. This is not
an exhaustive ranking of every openWEPP process.

## Selection Criteria

The pilot should have:

- a concise scientific question recognizable to domain readers;
- a defensible referent independent of the implementation under test;
- retained or reproducible quantitative evidence;
- a real production producer and downstream consumer;
- low enough process and forcing confounding to expose report architecture;
- useful limitations that can be stated without collapsing the report into a
  gap register; and
- no need to resolve new process physics or publish a premature empirical
  performance claim.

## Candidate Screen

| Candidate | Referent and evidence | Strengths | Confounding or gap | Decision |
| --- | --- | --- | --- | --- |
| Linear groundwater-reservoir recurrence | Srivastava formulation and pinned legacy recurrence; two-day analytical vector; fail-closed guards; HBP/watershed consumer tests; 731-day H2637 recurrence reconstruction | Small equation set, daily units, production relevance, analytical and run-level evidence, clear distinction between code verification and field corroboration | The retained H2637 study is not an empirical baseflow validation; current prototype cannot transfer Priest River performance to openWEPP | **Select** for a formulation/code/integration verification prototype |
| Hillslope erosion mass continuity | `SC-SED-001`; p61/p102 mass reconstructions; production publication consumers | Scientifically important, strong mass evidence, observable quantities | Detachment, deposition, routing, particle classes, runoff forcing, and publication semantics make the scientific argument too broad for an architecture pilot | Defer to portfolio work after the architecture is proven |
| OFE kinematic-wave routing | `SC-OFEROUTE-001`; analytical/synthetic cases; H2637 active routing closure | Strong numerical and conservation evidence; production routing consumer | Mesh, timestep, friction, topology, source-shape, and active-mode coupling remain materially entangled; prior sensitivity findings would dominate the pilot | Reject as low-confounding pilot |
| Watershed channel routing | `SC-ROUTE-001`; same-grid spike/spread and network closure evidence | Real network consumer and clear timing quantities | Multiple routing methods, storage, topology, sediment, baseflow, and timestep policy require a larger methods paper | Defer |
| Soil-water or evapotranspiration daily balance | Process contracts and daily ledger evidence across integrated cases | Familiar hydrologic quantities and conservation framing | A credible assessment needs coupled soil, plant, atmosphere, percolation, lateral-flow, and forcing interpretation; no single bounded retained study was identified | Reject for this pilot; candidate for later integrated report |

## Result

The groundwater recurrence is selected because it lets the prototype show the
full v2 communication pattern—scientific rationale, equations, implementation
method, quantitative results, production consumer, limitations, and
reproduction—without pretending that an integration test answers a watershed-
scale empirical question. The selected claim is narrower than “groundwater is
validated” and more useful than “the test suite passed.”

Snow/frost was not screened. The roadmap explicitly reserves it for the later
flagship synthesis after this smaller architecture pilot.

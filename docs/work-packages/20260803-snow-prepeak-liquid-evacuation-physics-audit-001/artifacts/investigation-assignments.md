# Investigation Assignments

Status: `complete`

Evidence mode: `Static + Ran: six compact read-only reports integrated`

| Role / agent | Primary ownership | Required compact output |
|---|---|---|
| Investigator 1 / `/root/coe_physics` | CoE physics, dimensions, handbook, and legacy provenance | Equation/unit audit with signatures and falsifiers |
| Investigator 2 / `/root/stage3_thermal` | Stage-3 thermal state and liquid routing | Enthalpy/cold-content/refreeze/routing audit |
| Investigator 3 / `/root/runtime_ordering` | Runtime order and state lineage | Exact call/state/publication map and ordering risks |
| Investigator 4 / `/root/surface_energy` | Surface-energy and forcing interactions | Shortwave/cloud/albedo/longwave/T/RH/wind/rain audit |
| Investigator 5 / `/root/density_capacity` | Density, layers, and liquid capacity | Pack-structure and evacuation interaction audit |
| Investigator 6 / `/root/trace_forensics` | Four-site event trace forensics | Cross-year magnitude, counter-signature, and falsification report |

All roles are read-only. The orchestrator records agent identity, task text,
evidence returned, conflicts, and integration disposition during execution.

Dispatched at `2026-08-04T00:25Z`. Each task explicitly prohibited filesystem
writes and required exact `file:line` support, dimensions, magnitude, competing
explanations, confidence, and an explicit falsifier.

All six agents returned without writing files. The orchestrator preserved the
principal disagreement rather than voting: capacity is a small instantaneous
store, while the legacy-routing probe shows the whole capacity-versus-density-
gate state transition is an order-one causal bound. The integrated disposition
therefore distinguishes liquid generation, instantaneous storage, and export
policy.

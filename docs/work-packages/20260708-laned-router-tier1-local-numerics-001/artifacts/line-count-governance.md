# Line-Count Governance

Status: `EXECUTED`

Measured after implementation and review fixes:

```
2429 crates/openwepp-hillslope-orchestrator/src/ofe_routing/kinematic_wave.rs
 627 crates/openwepp-hillslope-orchestrator/src/ofe_routing/cascade.rs
 675 docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md
 338 docs/work-packages/20260708-laned-router-tier1-local-numerics-001/package.md
```

`kinematic_wave.rs` remains in the WARN band. This package touched the file
because the hot local numerics and adjacent private tests live there; extracting
the solver is broader than Tier 1 and would be a behavior-risking refactor. No
file crosses the 3,000-line refactor-required threshold.

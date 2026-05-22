# Plant/Landuse/Growth/Decomposition Boundary Decision Record (PL01)

Status: `complete`
Evidence mode: `Static + Ran`

Static:
- Baseline PL surfaces are cross-cutting runtime state, not parser-local configuration.
- openWEPP architecture requires typed runtime boundary ownership and symbol continuity before kernel implementation closure.

Ran:
- Completed PL01 baseline+openWEPP evidence audit and architecture fit analysis.

## Decision

- outcome: `BOUNDARY_EXTEND_SERIES_REQUIRED`
- allowed values:
  - `BOUNDARY_PL01_DIRECT_IMPLEMENT`
  - `BOUNDARY_EXTEND_SERIES_REQUIRED`

## Rationale

1. Baseline management, growth, and decomposition semantics are state-transition-coupled across `tilage`, `contin`, `watbal`, `ptgra/ptgrp`, `grow`, `decomp`, and `resup`; direct single-package implementation would mix ownership boundaries.
2. openWEPP currently has typed management parsing but lacks management-to-runtime PL state adaptation, so parser closure does not equal runtime closure.
3. Canonical alias continuity for PL symbols is missing in `openwepp-sim-contract`; implementing kernels first would violate symbol continuity policy.
4. Correctness-over-completion requires staged closure: runtime boundary definition first, then kernelization and comparator review.

## Consequences

- PL01 closes as discovery+decision complete.
- Follow-on series is required before any claim of PL runtime semantic closure.
- Kernel implementation should remain blocked until boundary and alias prerequisites land.

## Evidence Links

- `/workdir/wepp-forest_260430_baseline/src/tilage.for:228`
- `/workdir/wepp-forest_260430_baseline/src/contin.for:811`
- `/workdir/wepp-forest_260430_baseline/src/watbal.for:881`
- `/workdir/wepp-forest_260430_baseline/src/grow.for:696`
- `/workdir/wepp-forest_260430_baseline/src/decomp.for:579`
- `/workdir/wepp-forest_260430_baseline/src/resup.for:207`
- `/home/workdir/openWEPP/crates/openwepp-input-contract/src/parsers/management.rs:446`
- `/home/workdir/openWEPP/crates/openwepp-hillslope-orchestrator/src/runtime_inputs.rs:514`
- `/home/workdir/openWEPP/crates/openwepp-hillslope-orchestrator/src/runtime_inputs.rs:1076`
- `/home/workdir/openWEPP/crates/openwepp-sim-contract/src/symbols.rs:255`
- `/home/workdir/openWEPP/docs/architecture/simulation-subsystem-kernel-architecture.md:57`
- `/home/workdir/openWEPP/docs/architecture/simulation-subsystem-kernel-architecture.md:62`

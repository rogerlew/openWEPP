# MOFE Readiness Work-Package Queue

Status: complete
Evidence mode: static
Date: 2026-05-25

## Static
Dependency-ordered follow-on queue for production MOFE readiness closure.

## Queue
1. `MOFE02` - Cross-file OFE Parity Hard Gate
- Objective: enforce `slope.ofe_count == management.topology_count == soil.ntemp`
  in hillslope execution intake before runtime surface merge.
- Scope:
  - wire soil parser `expected_topology_count` in runner,
  - add explicit parity validator for slope/management/soil,
  - add typed runner error family for parity mismatch.
- Entry criteria:
  - MOFE01 report accepted.
- Exit criteria:
  - mismatch inputs fail deterministically with typed error,
  - contract-derived tests cover all pairwise and triad mismatch cases,
  - no silent fallback/auto-clamp behavior.

2. `MOFE03` - Wave-2 Routing Activation and Input Synthesis
- Objective: make EROD14 Wave-2 MOFE routing executable from production input
  surfaces (not only manually seeded tests).
- Scope:
  - derive/seed required `erod14_*` symbols from parsed/runtime state,
  - define activation policy for `erod14_wave2_enabled`,
  - add integration tests from runfile inputs through scheduler execution.
- Depends on: `MOFE02`.
- Exit criteria:
  - MOFE-enabled runs execute Wave-2 path without manual symbol injection,
  - guard-family behavior remains typed and explicit on domain violations.

3. `MOFE04` - MOFE Output/Publications Closure
- Objective: close primary-OFE publication assumptions where MOFE semantics
  require OFE-aware output behavior or explicit aggregation policy.
- Scope:
  - WB13/WAT publication policy review for MOFE contexts,
  - implement OFE-aware publication or canonicalized aggregation contract,
  - add tests proving stable output semantics for multi-OFE runs.
- Depends on: `MOFE03`.
- Exit criteria:
  - output semantics are explicit, tested, and contract-aligned for MOFE runs.

4. `MOFE05` - Watershed Contributor MOFE Metadata and Intake Validation
- Objective: extend watershed contributor intake validation so downstream routing
  can enforce expected contributor MOFE metadata consistency where required.
- Scope:
  - define contributor MOFE metadata contract surfaces,
  - enforce typed intake validation at watershed boundary,
  - add behavior tests for malformed contributor metadata.
- Depends on: `MOFE03` (and optionally `MOFE04` if publication surfaces are used).
- Exit criteria:
  - malformed contributor MOFE metadata hard-fails with typed guard codes.

## Recommended execution order
`MOFE02 -> MOFE03 -> MOFE04 -> MOFE05`

## Ran
- not run

# PL15R Semantic-Parity Direction Assessment

Status: `complete`
Evidence mode: `Static + Ran`
Assessment: `UNRESOLVED / HOLD (provenance-invalid Tier-A lane)`

Static:
- Semantic parity (not bitwise parity) remains project target.
- Tier-A single-OFE daily water-balance surfaces remain high-confidence signals
  only when candidate provenance is direct openWEPP runtime execution.

Ran:
- PL14R schema-aligned strict replay artifacts show strict-pass signatures.
- PL14R provenance and schema-aligned retest artifacts show candidate lane
  substitution from legacy `/tmp/pl08_tiera_cmp_20260522/candidate` outputs.

## Direction Verdict

`UNRESOLVED / HOLD`

Why:

1. Required Tier-A surfaces are not backed by direct openWEPP candidate lane
   emission for the accepted strict-pass classification.
2. Strict-pass signals are legacy-vs-legacy after schema-upcast alignment,
   which is non-authoritative for openWEPP hold-lift claims.
3. Physics parity packages for ET/percolation/lateral/drainage forward-solver
   lane remain explicit prerequisites for valid Tier-A parity closure.

## Scope Note

This assessment rejects PL08 hold-lift on provenance/physics-authority grounds.
Future lift eligibility is gated on the parity-recovery queue in
`pl08-hold-lift-work-package-queue.md` (`CLI10`, `WB17..WB20`, `PL14S`, `PL15S`).

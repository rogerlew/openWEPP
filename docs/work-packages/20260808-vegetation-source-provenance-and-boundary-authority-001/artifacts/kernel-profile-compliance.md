# Kernel-Profile Compliance

Status: PASS; dual independent review complete.

Evidence mode: Static + Ran on 2026-08-08.

- `SC-VEGETATION-001` contains all 18 required kernel-process contract
  sections and a strict Binding Exposure row consolidated to named invariants.
- The authority is boundary-only: it defines topology, ownership, state
  surfaces, Stage A/B/C ordering, exact transfer identities, typed failures,
  guard mapping, and test vectors while admitting no constitutive physiology,
  coefficient, default, tolerance, calibration, or runtime selector.
- Configuration, parameter sets, initial state, and evolving state are
  distinct versioned objects. Exact non-overlapping topology tiles define
  same-rank closure and cross-rank overlap without an independence heuristic.
- Vegetation owns potential demand and finalization; hydrology alone owns
  admissible layer withdrawal and soil mutation; candidate states commit or
  roll back atomically.
- Shared water, radiation, latent energy, dry matter, carbon, and nitrogen
  transfers have exact-one custody and independent two-owner reconstruction.
- Every unavailable constitutive family is named in
  `GAP-VEGETATION-001..010` with `AUTHORITY_MISSING`,
  `IMPLEMENTATION_MISSING`, `NOT_CALIBRATION_READY`, or
  `DIRECT_TRANSLATION_PROHIBITED` posture as applicable.

Ran:

- strict Binding Exposure: PASS, 1 row fully consolidated;
- science-contract unit compliance: PASS with no findings for
  `SC-VEGETATION-001`, `SC-PLANT-001`, `SC-EVAP-001`, `SC-RESIDUE-001`, and
  `SC-LANDSURFACEENERGY-001`; and
- focused contract test after review remediation: PASS, 8/8.

`SC-WATBAL-001` initially reproduced a baseline `SCUNIT-E-009` finding for the
existing registry symbol `Interception`. The touched contract now explicitly
declares that already-published alias and all six touched contracts pass unit
compliance with no findings.

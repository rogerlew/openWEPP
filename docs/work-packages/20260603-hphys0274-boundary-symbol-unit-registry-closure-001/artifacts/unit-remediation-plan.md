# Unit Remediation Plan

Status: completed
Evidence mode: static

Static: Continuation work remains aligned with the HPHYS0273 queue and the
HOLD gaps recorded in the promoted registry spec.

Ran: not-run.

## Follow-Up Packages

- HPHYS0275: expand and apply typed dimensional `BoundaryValue` variants for
  registered high-risk runtime surfaces.
- HPHYS0276: implement named, directional, provenance-backed conversion helpers
  and guard raw dimensional conversion literals.
- HPHYS0277: add high hourly radiation physical flux guard enforcement.
- HPHYS0278: align output unit metadata with registry authority instead of
  hard-coded writer metadata.
- HPHYS0279: lint `SC-*` unit sections and alias unit checks against registry
  requirements.

## Recommended Next Focus

Execute HPHYS0275 next. The registry now identifies where typed boundary values
are still required, and HPHYS0275 is the direct path from registry authority to
runtime fail-closed unit behavior.

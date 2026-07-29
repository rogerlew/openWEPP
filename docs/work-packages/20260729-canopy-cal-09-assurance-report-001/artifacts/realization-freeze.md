# Realization Freeze

Status: `FROZEN`

- Assessed openWEPP source: commit
  `c42dde3136bbbf2b9c8a62ffe96ca6d28d77e615`.
- Report identity: `native-forest-canopy-phenology-evaluation`, version
  `1.0.0`, production-domain `DRAFT`.
- V2 report/schema contract: version 4; strict result schema: version 1.
- Science authority: `SC-PLANT-001`, `SC-RESIDUE-001`, and
  `SC-INFILE-MANAGEMENT-YAML-001` as present at the assessed commit.
- Narrative: `usersum/openwepp-canopy-phenology.md`, version 1.0 dated
  2026-07-29.
- Historical evidence: completed canopy implementation and CAL-01 through
  CAL-07F executions retain their recorded software/configuration identities.
- Fresh evidence: CAL-09 reconstructs only the compact synthesis result and
  candidate figures from retained inputs; it does not rerun or transfer the
  historical simulation population to current HEAD.
- Python: repository `.venv` Python 3.12 with standard-library scientific
  reconstruction; figure generation uses the environment already recorded by
  `candidate-figure-build.md`.
- V2 CLI: current `openwepp-assurance` crate at the assessed commit.

This freeze supports a retrospective draft. It is not a release-transfer
identity and does not claim every historical execution ran at the frozen
commit.

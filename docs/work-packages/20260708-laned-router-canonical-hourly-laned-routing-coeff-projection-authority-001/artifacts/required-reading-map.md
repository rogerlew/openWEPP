# Required Reading Map

Status: scaffolded.
Package:
`20260708-laned-router-canonical-hourly-laned-routing-coeff-projection-authority-001`

## Budget

Static byte count for the scaffolded core set measured on 2026-07-08:
`442,168` bytes. Disposition: `WARN`.

This package is contract-authority work. Execution should load the core files,
then use targeted excerpts from baseline Fortran and conditional contracts
rather than broad full-repo scans.

## Core Governance

| Path | Bytes | Purpose |
|---|---:|---|
| `AGENTS.md` | 10,269 | root governance |
| `docs/work-packages/AGENTS.md` | 16,364 | package governance |
| `docs/specifications/science-contracts/AGENTS.md` | 5,599 | contract governance |
| `docs/standards/AGENTS.md` | 3,328 | standards governance |
| `docs/standards/prompt-wording-guidance.md` | 9,780 | kickoff prompt wording |

## Core Authority

| Path | Bytes | Purpose |
|---|---:|---|
| `docs/ROADMAP.md` | 85,011 | queue authority |
| `docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md` | 162,146 | Lane D routing authority |
| `docs/specifications/wepp-input-files/specs/plant-file.spec.md` | 55,768 | native routing block / current no-inference rule |
| `docs/work-packages/20260708-laned-router-conditional-default-activation-001/package.md` | 7,890 | current default eligibility |
| `docs/work-packages/20260708-plant-file-native-lanuse-routing-doc-001/package.md` | 6,114 | native landuse/routing doc alignment |

## Baseline Code Authority

| Path | Bytes | Purpose |
|---|---:|---|
| `/workdir/wepp-forest_260430_baseline/src/frcfac.for` | 10,466 | legacy friction/cropland roughness surfaces |
| `/workdir/wepp-forest_260430_baseline/src/param.for` | 21,040 | erosion/rill/interrill parameter context |
| `/workdir/wepp-forest_260430_baseline/src/bigout.for` | 7,073 | runtime diagnostic surfaces including cover/roughness |
| `/workdir/wepp-forest_260430_baseline/src/watbal_hourly.for` | 41,320 | hourly water-balance/cropland runtime context |

## Conditional Reads

- `docs/specifications/science-contract-authoring-procedure.md`
- `docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `docs/specifications/science-contracts/contracts/SC-SED-001.md`
- `docs/specifications/science-contracts/contracts/SC-RUNOFFPART-001.md`
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `docs/specifications/science-contracts/contracts/SC-GWBASEFLOW-001.md`
- `docs/work-packages/20260707-laned-router-d16-hybrid-disturbed-route-coeff-source-acquisition-001/package.md`
- `docs/work-packages/20260707-laned-router-d16-hybrid-route-coeff-authoring-bridge-001/package.md`

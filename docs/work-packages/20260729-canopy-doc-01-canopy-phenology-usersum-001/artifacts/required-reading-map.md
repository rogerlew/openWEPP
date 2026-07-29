# Required Reading Map

Status: `complete`

Evidence mode: `Static`

## Core

| Path | Rationale | Read |
| --- | --- | --- |
| `AGENTS.md` | Repository governance and documentation truthfulness | read |
| `docs/work-packages/AGENTS.md` | Work-package execution and closure | read |
| `package.md` | Authorized objective, scope, coefficient contract, and gates | read |
| `docs/codex_exec_plans.md` | Autonomous living-plan and evidence requirements | read |
| `docs/standards/testing-and-gate-strategy.md` | Documentation validation lifecycle | read |
| `docs/standards/usersum-authoring-style-guide.md` | Narrative shape, claims, links, references, and versioning | read |
| `docs/planning/canopy-phenology-assurance-roadmap.md` | Campaign results, limitations, and DOC/ASSURE boundary | read |
| `docs/decisions/0034-management-file-lanuse-input-authority.md` | Native forest rationale and cropland-compatibility history | read |
| `docs/specifications/science-contracts/contracts/SC-PLANT-001.md` | GSI, canopy, LAI, height, allocation, and consumer authority | read |
| `docs/specifications/science-contracts/contracts/SC-RESIDUE-001.md` | Litter, decomposition, cover, depth, erosion, and frost authority | read |
| `usersum/README.md` | Public model-science catalog | read |
| `usersum/snow-frost-modeling-and-validation.md` | Canonical narrative example and canopy/winter cross-link context | read |

## Conditional

| Path | Trigger | Read |
| --- | --- | --- |
| `docs/specifications/science-contracts/contracts/SC-INFILE-MANAGEMENT-YAML-001.md` | Exact native YAML field and source-authority explanation | read |
| `crates/openwepp-management-schema/src/lib.rs` and `src/forest_litter.rs` | Exact active user field/domain inventory | read |
| downstream snow, ET, water-balance, routing, and sediment contracts | A causal statement cannot be supported from PLANT/RESIDUE authority alone | not triggered; PLANT/RESIDUE contracts support the qualitative statements |

## On demand

Read only the claim-bearing artifacts needed from completed
`CANOPY-PHENOLOGY-01` through `CANOPY-CAL-07F`, prioritizing source ledgers,
final dispositions, calibration ensembles, identifiability analyses, and
figure sidecars. Read primary literature objects when authoring citations.

Do not bulk-copy package prose or internal vocabulary into `usersum`.

Claim-bearing CAL-01, CAL-04B, CAL-05, CAL-07E, CAL-07F, and litter-source
authority artifacts were read selectively. Primary-source identities and
claim boundaries were checked against retained literature ledgers.

Review-triggered residue-lineage inspection also covered the native initial
seed and direct-production depth conversion in
`openwepp-hillslope-orchestrator` and `openwepp-runner`, plus the frost
residue-cover implementation evidence. This established that `cf` is active,
the conversion ratio is derived, and `diam` is currently branch-inert for
native forest `landuse=3`.

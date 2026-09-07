---
contract_id: SC-LANDSURFACEENERGY-001
title: Land-Surface Energy-Balance Process Contract
status: approved
maturity: active
owner: openWEPP maintainers + land-surface-energy/hydrology reviewer
contract_version: 32
producer_scope:
  - Future snow-free land-surface energy control-volume evaluator
  - Future post-snow receiving-surface evaluator after an atomic handoff cutover
  - Persistent Stage 3 snow--soil lower-boundary evaluator
consumer_scope:
  - Future soil-heat/frost boundary, evaporation, infiltration/runoff, and surface-water ledgers
evidence_level: static+independent_oracle+contract_vectors
last_reviewed: 2026-09-07
supersedes: []
superseded_by: []
contract_format: directory-v1
binding_index: SC-LANDSURFACEENERGY-001/binding-index.md#binding-exposure-index
---

# LSE contract

Candidate v32; adoption gates pending. Selective reading is authorized only for this
package's bounded candidate exercises until independent adoption closure.
Read the complete [shared interface](SC-LANDSURFACEENERGY-001/interface.md#interface) for every
task, then the applicable route and every conditional dependency. The entire normative
set remains binding. Sections include subsections to the next same/higher heading,
applicable marked definitions and their guard/test links; inspect each selected
chapter's Dependencies and introduction. Uncertainty expands reading.

## Document inventory
| Path | Kind | Purpose | Applicability |
|---|---|---|---|
| SC-LANDSURFACEENERGY-001/interface.md | normative | interface | selected |
| SC-LANDSURFACEENERGY-001/common-details.md | normative | common details | selected |
| SC-LANDSURFACEENERGY-001/audit-details.md | normative | audit details | selected |
| SC-LANDSURFACEENERGY-001/surface-energy.md | normative | surface energy | selected |
| SC-LANDSURFACEENERGY-001/soil-coupling.md | normative | soil coupling | selected |
| SC-LANDSURFACEENERGY-001/water-vapor.md | normative | water vapor | selected |
| SC-LANDSURFACEENERGY-001/solve-boundary.md | normative | solve boundary | selected |
| SC-LANDSURFACEENERGY-001/nonlinear-solve.md | normative | nonlinear solve | selected |
| SC-LANDSURFACEENERGY-001/terminal-support.md | normative | terminal support | selected |
| SC-LANDSURFACEENERGY-001/litter-phase.md | normative | litter phase | selected |
| SC-LANDSURFACEENERGY-001/soil-custody.md | normative | soil custody | selected |
| SC-LANDSURFACEENERGY-001/surface-custody.md | normative | surface custody | selected |
| SC-LANDSURFACEENERGY-001/map-custody.md | normative | map custody | selected |
| SC-LANDSURFACEENERGY-001/dependency-replay.md | normative | dependency replay | selected |
| SC-LANDSURFACEENERGY-001/qualification.md | normative | replay retention and experiments | selected |
| SC-LANDSURFACEENERGY-001/binding-index.md | normative | binding index | audit |
| SC-LANDSURFACEENERGY-001/history.md | historical | history | audit |

## Reading routes
| Task | Role/check | Initial material | Expansion trigger |
|---|---|---|---|
| Surface/soil rules | science review | [surface-energy](SC-LANDSURFACEENERGY-001/surface-energy.md#surface-energy) | Follow physical shared set and applicable mechanism dependencies. |
| Snow replay | correctness review | [dependency-replay](SC-LANDSURFACEENERGY-001/dependency-replay.md#dependency-replay) | All affected physics, errors/custody, support and frozen protocol. |
| Executable identity | identity verification | [qualification](SC-LANDSURFACEENERGY-001/qualification.md#qualification) | Frozen capture protocol; scientific claims expand physics/closure. |
| Frozen-litter closure | closure review | [litter-phase](SC-LANDSURFACEENERGY-001/litter-phase.md#litter-phase) | Physical operands, errors, support and receipt origin; implementation expands full evaluator/solver. |
| Other/uncertain | all | [binding index](SC-LANDSURFACEENERGY-001/binding-index.md#binding-exposure-index) | Expand full normative set and reconcile applicability. |

<a id="change-log"></a>
## Change Log
2026-09-07 v32: coherent directory presentation; scientific authority, production
HOLD and frozen identities unchanged. History: history.md.

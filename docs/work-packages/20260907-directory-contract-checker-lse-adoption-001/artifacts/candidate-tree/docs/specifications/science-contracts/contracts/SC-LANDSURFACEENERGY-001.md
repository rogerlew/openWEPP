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

# SC-LANDSURFACEENERGY-001 Land-Surface Energy-Balance Process Contract

Candidate adoption: NOT YET QUALIFIED. Whole-contract reading remains required until this package passes all adoption gates.
Scientific implementation authority, production activation and empirical/release qualification are distinct. This format-only revision changes none of them. Retained FAIL/HOLD and frozen kickoff requirements remain binding.

<a id="contract-scope"></a>
## Scope and cross-cutting requirements
Read [shared scope, signs, units, owners and qualification limits](SC-LANDSURFACEENERGY-001/interface.md#interface) for every task. The complete declared normative set remains binding. Unknown tasks or uncertain applicability require expansion and reconciliation, never omission. Other single-file contracts retain their whole-contract rules.

## Document inventory
| Path | Kind | Purpose | Applicability |
|---|---|---|---|
| SC-LANDSURFACEENERGY-001/interface.md | normative | interface | all |
| SC-LANDSURFACEENERGY-001/surface-energy.md | normative | surface energy | surface energy work |
| SC-LANDSURFACEENERGY-001/soil-coupling.md | normative | soil coupling | soil coupling work |
| SC-LANDSURFACEENERGY-001/water-vapor.md | normative | water vapor | water vapor work |
| SC-LANDSURFACEENERGY-001/nonlinear-solve.md | normative | nonlinear solve | nonlinear solve work |
| SC-LANDSURFACEENERGY-001/terminal-support.md | normative | terminal support | terminal support work |
| SC-LANDSURFACEENERGY-001/litter-phase.md | normative | litter phase | litter phase work |
| SC-LANDSURFACEENERGY-001/soil-custody.md | normative | soil custody | soil custody work |
| SC-LANDSURFACEENERGY-001/surface-custody.md | normative | surface custody | surface custody work |
| SC-LANDSURFACEENERGY-001/map-custody.md | normative | map custody | map custody work |
| SC-LANDSURFACEENERGY-001/dependency-replay.md | normative | dependency replay | dependency replay work |
| SC-LANDSURFACEENERGY-001/qualification.md | normative | qualification | qualification work |
| SC-LANDSURFACEENERGY-001/binding-index.md | normative | binding index | binding index work |
| SC-LANDSURFACEENERGY-001/history.md | historical | history | audit |

## Reading routes
| Task | Role/check | Initial material | Expansion trigger |
|---|---|---|---|
| Surface-energy and soil coupling | implementation/review | entry + [interface](SC-LANDSURFACEENERGY-001/interface.md#interface); [surface-energy](SC-LANDSURFACEENERGY-001/surface-energy.md#surface-energy), [soil-coupling](SC-LANDSURFACEENERGY-001/soil-coupling.md#soil-coupling), [water-vapor](SC-LANDSURFACEENERGY-001/water-vapor.md#water-vapor) | Changed evaluator adds nonlinear-solve; litter phase adds litter-phase; follow every applicable dependency. |
| Solver correctness and evaluator reuse | correctness reviewer | entry + [interface](SC-LANDSURFACEENERGY-001/interface.md#interface); [nonlinear-solve](SC-LANDSURFACEENERGY-001/nonlinear-solve.md#nonlinear-solve), [dependency-replay](SC-LANDSURFACEENERGY-001/dependency-replay.md#dependency-replay), [qualification](SC-LANDSURFACEENERGY-001/qualification.md#qualification) | Expand every affected evaluator, error-order and owner dependency; role never excludes physics. |
| Executable identity capture | identity verifier | entry + [interface](SC-LANDSURFACEENERGY-001/interface.md#interface); [qualification](SC-LANDSURFACEENERGY-001/qualification.md#qualification) | Frozen capture protocol remains mandatory; scientific result/closure claims add physical and operand chapters. |
| Physical closure reconstruction | scientific verifier | entry + [interface](SC-LANDSURFACEENERGY-001/interface.md#interface); [water-vapor](SC-LANDSURFACEENERGY-001/water-vapor.md#water-vapor), [soil-coupling](SC-LANDSURFACEENERGY-001/soil-coupling.md#soil-coupling), [litter-phase](SC-LANDSURFACEENERGY-001/litter-phase.md#litter-phase), [soil-custody](SC-LANDSURFACEENERGY-001/soil-custody.md#soil-custody), [surface-custody](SC-LANDSURFACEENERGY-001/surface-custody.md#surface-custody) | Expand radiative/turbulent operands and every affected receiver/lineage boundary. |
| Restart and owner custody | implementation/review | entry + [interface](SC-LANDSURFACEENERGY-001/interface.md#interface); [map-custody](SC-LANDSURFACEENERGY-001/map-custody.md#map-custody), [soil-custody](SC-LANDSURFACEENERGY-001/soil-custody.md#soil-custody), [surface-custody](SC-LANDSURFACEENERGY-001/surface-custody.md#surface-custody) | Transaction/rollback/publication changes add every affected owner. |
| Unknown or uncertain task | all | entry + [interface](SC-LANDSURFACEENERGY-001/interface.md#interface); [binding-index](SC-LANDSURFACEENERGY-001/binding-index.md#binding-index) | Read the entire normative set and reconcile applicability before action. |

[Binding index](SC-LANDSURFACEENERGY-001/binding-index.md#binding-exposure-index); [gaps/status](SC-LANDSURFACEENERGY-001/interface.md#gap-register-and-promotability-labels); [qualification](SC-LANDSURFACEENERGY-001/qualification.md#qualification); [original history](SC-LANDSURFACEENERGY-001/history.md#history).

<a id="change-log"></a>
## Change Log
2026-09-07, revision 32: directory-format relocation with preserved scientific meaning, logical IDs, qualification and activation limits; candidate adoption remains gated.

## Compatibility anchors
| Alias | Target |
|---|---|
| INV-LANDSURFACEENERGY-001 | SC-LANDSURFACEENERGY-001/interface.md#INV-LANDSURFACEENERGY-001 |
| INV-LANDSURFACEENERGY-002 | SC-LANDSURFACEENERGY-001/interface.md#INV-LANDSURFACEENERGY-002 |
| INV-LANDSURFACEENERGY-010 | SC-LANDSURFACEENERGY-001/interface.md#INV-LANDSURFACEENERGY-010 |
| INV-LANDSURFACEENERGY-011 | SC-LANDSURFACEENERGY-001/interface.md#INV-LANDSURFACEENERGY-011 |
| INV-LANDSURFACEENERGY-012 | SC-LANDSURFACEENERGY-001/interface.md#INV-LANDSURFACEENERGY-012 |
| INV-LANDSURFACEENERGY-013 | SC-LANDSURFACEENERGY-001/interface.md#INV-LANDSURFACEENERGY-013 |
| INV-LANDSURFACEENERGY-014 | SC-LANDSURFACEENERGY-001/interface.md#INV-LANDSURFACEENERGY-014 |
| INV-LANDSURFACEENERGY-015 | SC-LANDSURFACEENERGY-001/interface.md#INV-LANDSURFACEENERGY-015 |
| INV-LANDSURFACEENERGY-020 | SC-LANDSURFACEENERGY-001/interface.md#INV-LANDSURFACEENERGY-020 |
| INV-LANDSURFACEENERGY-021 | SC-LANDSURFACEENERGY-001/interface.md#INV-LANDSURFACEENERGY-021 |
| INV-LANDSURFACEENERGY-022 | SC-LANDSURFACEENERGY-001/interface.md#INV-LANDSURFACEENERGY-022 |
| INV-LANDSURFACEENERGY-030 | SC-LANDSURFACEENERGY-001/interface.md#INV-LANDSURFACEENERGY-030 |
| INV-LANDSURFACEENERGY-031 | SC-LANDSURFACEENERGY-001/interface.md#INV-LANDSURFACEENERGY-031 |
| INV-LANDSURFACEENERGY-032 | SC-LANDSURFACEENERGY-001/interface.md#INV-LANDSURFACEENERGY-032 |
| INV-LANDSURFACEENERGY-040 | SC-LANDSURFACEENERGY-001/interface.md#INV-LANDSURFACEENERGY-040 |
| INV-LANDSURFACEENERGY-041 | SC-LANDSURFACEENERGY-001/interface.md#INV-LANDSURFACEENERGY-041 |
| INV-LANDSURFACEENERGY-042 | SC-LANDSURFACEENERGY-001/interface.md#INV-LANDSURFACEENERGY-042 |
| INV-LANDSURFACEENERGY-043 | SC-LANDSURFACEENERGY-001/interface.md#INV-LANDSURFACEENERGY-043 |
| INV-LANDSURFACEENERGY-100 | SC-LANDSURFACEENERGY-001/soil-coupling.md#INV-LANDSURFACEENERGY-100 |
| INV-LANDSURFACEENERGY-101 | SC-LANDSURFACEENERGY-001/surface-energy.md#INV-LANDSURFACEENERGY-101 |
| INV-LANDSURFACEENERGY-102 | SC-LANDSURFACEENERGY-001/surface-energy.md#INV-LANDSURFACEENERGY-102 |
| INV-LANDSURFACEENERGY-103 | SC-LANDSURFACEENERGY-001/soil-coupling.md#INV-LANDSURFACEENERGY-103 |
| INV-LANDSURFACEENERGY-104 | SC-LANDSURFACEENERGY-001/water-vapor.md#INV-LANDSURFACEENERGY-104 |
| INV-LANDSURFACEENERGY-105 | SC-LANDSURFACEENERGY-001/water-vapor.md#INV-LANDSURFACEENERGY-105 |
| INV-LANDSURFACEENERGY-106 | SC-LANDSURFACEENERGY-001/soil-coupling.md#INV-LANDSURFACEENERGY-106 |
| INV-LANDSURFACEENERGY-107 | SC-LANDSURFACEENERGY-001/water-vapor.md#INV-LANDSURFACEENERGY-107 |
| INV-LANDSURFACEENERGY-108 | SC-LANDSURFACEENERGY-001/nonlinear-solve.md#INV-LANDSURFACEENERGY-108 |
| INV-LANDSURFACEENERGY-109 | SC-LANDSURFACEENERGY-001/nonlinear-solve.md#INV-LANDSURFACEENERGY-109 |
| INV-LANDSURFACEENERGY-110 | SC-LANDSURFACEENERGY-001/nonlinear-solve.md#INV-LANDSURFACEENERGY-110 |
| INV-LANDSURFACEENERGY-111 | SC-LANDSURFACEENERGY-001/nonlinear-solve.md#INV-LANDSURFACEENERGY-111 |
| INV-LANDSURFACEENERGY-112 | SC-LANDSURFACEENERGY-001/nonlinear-solve.md#INV-LANDSURFACEENERGY-112 |
| INV-LANDSURFACEENERGY-113 | SC-LANDSURFACEENERGY-001/nonlinear-solve.md#INV-LANDSURFACEENERGY-113 |
| INV-LANDSURFACEENERGY-114 | SC-LANDSURFACEENERGY-001/terminal-support.md#INV-LANDSURFACEENERGY-114 |
| INV-LANDSURFACEENERGY-115 | SC-LANDSURFACEENERGY-001/terminal-support.md#INV-LANDSURFACEENERGY-115 |
| INV-LANDSURFACEENERGY-116 | SC-LANDSURFACEENERGY-001/terminal-support.md#INV-LANDSURFACEENERGY-116 |
| INV-LANDSURFACEENERGY-117 | SC-LANDSURFACEENERGY-001/terminal-support.md#INV-LANDSURFACEENERGY-117 |
| INV-LANDSURFACEENERGY-118 | SC-LANDSURFACEENERGY-001/terminal-support.md#INV-LANDSURFACEENERGY-118 |
| INV-LANDSURFACEENERGY-119 | SC-LANDSURFACEENERGY-001/terminal-support.md#INV-LANDSURFACEENERGY-119 |
| INV-LANDSURFACEENERGY-120 | SC-LANDSURFACEENERGY-001/terminal-support.md#INV-LANDSURFACEENERGY-120 |
| INV-LANDSURFACEENERGY-121 | SC-LANDSURFACEENERGY-001/terminal-support.md#INV-LANDSURFACEENERGY-121 |
| INV-LANDSURFACEENERGY-122 | SC-LANDSURFACEENERGY-001/terminal-support.md#INV-LANDSURFACEENERGY-122 |
| INV-LANDSURFACEENERGY-123 | SC-LANDSURFACEENERGY-001/terminal-support.md#INV-LANDSURFACEENERGY-123 |
| INV-LANDSURFACEENERGY-124 | SC-LANDSURFACEENERGY-001/soil-coupling.md#INV-LANDSURFACEENERGY-124 |
| INV-LANDSURFACEENERGY-125 | SC-LANDSURFACEENERGY-001/soil-coupling.md#INV-LANDSURFACEENERGY-125 |
| INV-LANDSURFACEENERGY-126 | SC-LANDSURFACEENERGY-001/soil-coupling.md#INV-LANDSURFACEENERGY-126 |
| INV-LANDSURFACEENERGY-130 | SC-LANDSURFACEENERGY-001/water-vapor.md#INV-LANDSURFACEENERGY-130 |
| INV-LANDSURFACEENERGY-131 | SC-LANDSURFACEENERGY-001/nonlinear-solve.md#INV-LANDSURFACEENERGY-131 |
| INV-LANDSURFACEENERGY-138 | SC-LANDSURFACEENERGY-001/nonlinear-solve.md#INV-LANDSURFACEENERGY-138 |
| INV-LANDSURFACEENERGY-139 | SC-LANDSURFACEENERGY-001/nonlinear-solve.md#INV-LANDSURFACEENERGY-139 |
| INV-LANDSURFACEENERGY-140 | SC-LANDSURFACEENERGY-001/litter-phase.md#INV-LANDSURFACEENERGY-140 |
| INV-LANDSURFACEENERGY-141 | SC-LANDSURFACEENERGY-001/litter-phase.md#INV-LANDSURFACEENERGY-141 |
| INV-LANDSURFACEENERGY-142 | SC-LANDSURFACEENERGY-001/litter-phase.md#INV-LANDSURFACEENERGY-142 |
| INV-LANDSURFACEENERGY-143 | SC-LANDSURFACEENERGY-001/litter-phase.md#INV-LANDSURFACEENERGY-143 |
| INV-LANDSURFACEENERGY-144 | SC-LANDSURFACEENERGY-001/litter-phase.md#INV-LANDSURFACEENERGY-144 |
| INV-LANDSURFACEENERGY-145 | SC-LANDSURFACEENERGY-001/litter-phase.md#INV-LANDSURFACEENERGY-145 |
| INV-LANDSURFACEENERGY-146 | SC-LANDSURFACEENERGY-001/litter-phase.md#INV-LANDSURFACEENERGY-146 |
| INV-LANDSURFACEENERGY-147 | SC-LANDSURFACEENERGY-001/litter-phase.md#INV-LANDSURFACEENERGY-147 |
| INV-LANDSURFACEENERGY-148 | SC-LANDSURFACEENERGY-001/litter-phase.md#INV-LANDSURFACEENERGY-148 |
| INV-LANDSURFACEENERGY-149 | SC-LANDSURFACEENERGY-001/litter-phase.md#INV-LANDSURFACEENERGY-149 |
| INV-LANDSURFACEENERGY-150 | SC-LANDSURFACEENERGY-001/soil-custody.md#INV-LANDSURFACEENERGY-150 |
| INV-LANDSURFACEENERGY-151 | SC-LANDSURFACEENERGY-001/surface-custody.md#INV-LANDSURFACEENERGY-151 |
| INV-LANDSURFACEENERGY-152 | SC-LANDSURFACEENERGY-001/map-custody.md#INV-LANDSURFACEENERGY-152 |
| INV-LANDSURFACEENERGY-153 | SC-LANDSURFACEENERGY-001/surface-custody.md#INV-LANDSURFACEENERGY-153 |
| INV-LANDSURFACEENERGY-154 | SC-LANDSURFACEENERGY-001/terminal-support.md#INV-LANDSURFACEENERGY-154 |
| INV-LANDSURFACEENERGY-155 | SC-LANDSURFACEENERGY-001/soil-custody.md#INV-LANDSURFACEENERGY-155 |
| INV-LANDSURFACEENERGY-156 | SC-LANDSURFACEENERGY-001/litter-phase.md#INV-LANDSURFACEENERGY-156 |
| INV-LANDSURFACEENERGY-157 | SC-LANDSURFACEENERGY-001/litter-phase.md#INV-LANDSURFACEENERGY-157 |
| INV-LANDSURFACEENERGY-158 | SC-LANDSURFACEENERGY-001/surface-custody.md#INV-LANDSURFACEENERGY-158 |
| INV-LANDSURFACEENERGY-159 | SC-LANDSURFACEENERGY-001/map-custody.md#INV-LANDSURFACEENERGY-159 |
| INV-LANDSURFACEENERGY-160 | SC-LANDSURFACEENERGY-001/map-custody.md#INV-LANDSURFACEENERGY-160 |
| INV-LANDSURFACEENERGY-161 | SC-LANDSURFACEENERGY-001/map-custody.md#INV-LANDSURFACEENERGY-161 |
| INV-LANDSURFACEENERGY-162 | SC-LANDSURFACEENERGY-001/nonlinear-solve.md#INV-LANDSURFACEENERGY-162 |
| INV-LANDSURFACEENERGY-163 | SC-LANDSURFACEENERGY-001/nonlinear-solve.md#INV-LANDSURFACEENERGY-163 |
| INV-LANDSURFACEENERGY-164 | SC-LANDSURFACEENERGY-001/dependency-replay.md#INV-LANDSURFACEENERGY-164 |
| OBL-LANDSURFACEENERGY-C-001 | SC-LANDSURFACEENERGY-001/water-vapor.md#OBL-LANDSURFACEENERGY-C-001 |
| OBL-LANDSURFACEENERGY-C-002 | SC-LANDSURFACEENERGY-001/water-vapor.md#OBL-LANDSURFACEENERGY-C-002 |
| OBL-LANDSURFACEENERGY-C-003 | SC-LANDSURFACEENERGY-001/soil-coupling.md#OBL-LANDSURFACEENERGY-C-003 |
| OBL-LANDSURFACEENERGY-C-004 | SC-LANDSURFACEENERGY-001/interface.md#OBL-LANDSURFACEENERGY-C-004 |
| OBL-LANDSURFACEENERGY-C-005 | SC-LANDSURFACEENERGY-001/soil-custody.md#OBL-LANDSURFACEENERGY-C-005 |
| OBL-LANDSURFACEENERGY-C-006 | SC-LANDSURFACEENERGY-001/surface-custody.md#OBL-LANDSURFACEENERGY-C-006 |
| OBL-LANDSURFACEENERGY-C-007 | SC-LANDSURFACEENERGY-001/map-custody.md#OBL-LANDSURFACEENERGY-C-007 |
| OBL-LANDSURFACEENERGY-C-008 | SC-LANDSURFACEENERGY-001/surface-custody.md#OBL-LANDSURFACEENERGY-C-008 |
| OBL-LANDSURFACEENERGY-C-009 | SC-LANDSURFACEENERGY-001/terminal-support.md#OBL-LANDSURFACEENERGY-C-009 |
| OBL-LANDSURFACEENERGY-C-010 | SC-LANDSURFACEENERGY-001/soil-custody.md#OBL-LANDSURFACEENERGY-C-010 |
| OBL-LANDSURFACEENERGY-C-011 | SC-LANDSURFACEENERGY-001/litter-phase.md#OBL-LANDSURFACEENERGY-C-011 |
| OBL-LANDSURFACEENERGY-C-012 | SC-LANDSURFACEENERGY-001/litter-phase.md#OBL-LANDSURFACEENERGY-C-012 |
| OBL-LANDSURFACEENERGY-C-013 | SC-LANDSURFACEENERGY-001/surface-custody.md#OBL-LANDSURFACEENERGY-C-013 |
| OBL-LANDSURFACEENERGY-C-014 | SC-LANDSURFACEENERGY-001/map-custody.md#OBL-LANDSURFACEENERGY-C-014 |
| OBL-LANDSURFACEENERGY-C-015 | SC-LANDSURFACEENERGY-001/map-custody.md#OBL-LANDSURFACEENERGY-C-015 |
| OBL-LANDSURFACEENERGY-C-016 | SC-LANDSURFACEENERGY-001/map-custody.md#OBL-LANDSURFACEENERGY-C-016 |
| OBL-LANDSURFACEENERGY-C-017 | SC-LANDSURFACEENERGY-001/nonlinear-solve.md#OBL-LANDSURFACEENERGY-C-017 |
| OBL-LANDSURFACEENERGY-C-018 | SC-LANDSURFACEENERGY-001/nonlinear-solve.md#OBL-LANDSURFACEENERGY-C-018 |
| OBL-LANDSURFACEENERGY-C-019 | SC-LANDSURFACEENERGY-001/map-custody.md#OBL-LANDSURFACEENERGY-C-019 |
| OBL-LANDSURFACEENERGY-C-020 | SC-LANDSURFACEENERGY-001/dependency-replay.md#OBL-LANDSURFACEENERGY-C-020 |
| OBL-LANDSURFACEENERGY-P-001 | SC-LANDSURFACEENERGY-001/water-vapor.md#OBL-LANDSURFACEENERGY-P-001 |
| OBL-LANDSURFACEENERGY-P-002 | SC-LANDSURFACEENERGY-001/water-vapor.md#OBL-LANDSURFACEENERGY-P-002 |
| OBL-LANDSURFACEENERGY-P-003 | SC-LANDSURFACEENERGY-001/water-vapor.md#OBL-LANDSURFACEENERGY-P-003 |
| OBL-LANDSURFACEENERGY-P-004 | SC-LANDSURFACEENERGY-001/water-vapor.md#OBL-LANDSURFACEENERGY-P-004 |
| OBL-LANDSURFACEENERGY-P-005 | SC-LANDSURFACEENERGY-001/soil-custody.md#OBL-LANDSURFACEENERGY-P-005 |
| OBL-LANDSURFACEENERGY-P-006 | SC-LANDSURFACEENERGY-001/surface-custody.md#OBL-LANDSURFACEENERGY-P-006 |
| purpose | SC-LANDSURFACEENERGY-001/interface.md#purpose |
| scientific-scope-and-explicit-out-of-scope-boundaries | SC-LANDSURFACEENERGY-001/interface.md#scientific-scope-and-explicit-out-of-scope-boundaries |
| authority-anchors-with-top-down-citations | SC-LANDSURFACEENERGY-001/interface.md#authority-anchors-with-top-down-citations |
| variables-and-units-using-canonical-symbols-first | SC-LANDSURFACEENERGY-001/interface.md#variables-and-units-using-canonical-symbols-first |
| algorithm-state-surfaces | SC-LANDSURFACEENERGY-001/interface.md#algorithm-state-surfaces |
| algorithm-specification-with-step-sequence | SC-LANDSURFACEENERGY-001/interface.md#algorithm-specification-with-step-sequence |
| step-local-preconditions-intermediates-and-postconditions | SC-LANDSURFACEENERGY-001/interface.md#step-local-preconditions-intermediates-and-postconditions |
| branch-and-guard-table | SC-LANDSURFACEENERGY-001/interface.md#branch-and-guard-table |
| invariants-and-invariant-guard-map | SC-LANDSURFACEENERGY-001/interface.md#invariants-and-invariant-guard-map |
| invariant-guard-map | SC-LANDSURFACEENERGY-001/interface.md#invariant-guard-map |
| producer-obligations-and-consumer-obligations | SC-LANDSURFACEENERGY-001/interface.md#producer-obligations-and-consumer-obligations |
| symbol-alias-map | SC-LANDSURFACEENERGY-001/interface.md#symbol-alias-map |
| constants-and-parameters-with-provenance-anchors | SC-LANDSURFACEENERGY-001/interface.md#constants-and-parameters-with-provenance-anchors |
| unit-governance-map | SC-LANDSURFACEENERGY-001/interface.md#unit-governance-map |
| tolerance-and-numeric-notes | SC-LANDSURFACEENERGY-001/interface.md#tolerance-and-numeric-notes |
| calibration-and-identifiability | SC-LANDSURFACEENERGY-001/interface.md#calibration-and-identifiability |
| test-vector-obligations | SC-LANDSURFACEENERGY-001/interface.md#test-vector-obligations |
| child-2c-invariant-ids | SC-LANDSURFACEENERGY-001/interface.md#child-2c-invariant-ids |
| gap-register-and-promotability-labels | SC-LANDSURFACEENERGY-001/interface.md#gap-register-and-promotability-labels |
| openwepp_snow_free_lse_v1-constitutive-authority | SC-LANDSURFACEENERGY-001/surface-energy.md#openwepp_snow_free_lse_v1-constitutive-authority |
| selected-sources-and-domain | SC-LANDSURFACEENERGY-001/surface-energy.md#selected-sources-and-domain |
| exact-ownership-and-state | SC-LANDSURFACEENERGY-001/surface-energy.md#exact-ownership-and-state |
| shortwave-and-reciprocal-longwave | SC-LANDSURFACEENERGY-001/surface-energy.md#shortwave-and-reciprocal-longwave |
| neutral-turbulent-heat-and-vapor-network | SC-LANDSURFACEENERGY-001/surface-energy.md#neutral-turbulent-heat-and-vapor-network |
| surface-humidity-surface-enthalpy-litter-and-soil-heat | SC-LANDSURFACEENERGY-001/soil-coupling.md#surface-humidity-surface-enthalpy-litter-and-soil-heat |
| signed-vapor-and-liquid-enthalpy | SC-LANDSURFACEENERGY-001/water-vapor.md#signed-vapor-and-liquid-enthalpy |
| immutable-beginning-water-transaction-and-current-ingress | SC-LANDSURFACEENERGY-001/water-vapor.md#immutable-beginning-water-transaction-and-current-ingress |
| ordered-numerical-algorithm-active-branches-and-error-precedence | SC-LANDSURFACEENERGY-001/nonlinear-solve.md#ordered-numerical-algorithm-active-branches-and-error-precedence |
| independent-closure-and-errors | SC-LANDSURFACEENERGY-001/water-vapor.md#independent-closure-and-errors |
| v1-invariants-and-independent-fixtures | SC-LANDSURFACEENERGY-001/water-vapor.md#v1-invariants-and-independent-fixtures |
| scope | SC-LANDSURFACEENERGY-001/surface-energy.md#scope |
| terminal-receiver-remaining-support-amendment | SC-LANDSURFACEENERGY-001/terminal-support.md#terminal-receiver-remaining-support-amendment |
| child-2c-shared-carrier-and-successor-support-amendment | SC-LANDSURFACEENERGY-001/terminal-support.md#child-2c-shared-carrier-and-successor-support-amendment |
| snow-free-successor-chronology | SC-LANDSURFACEENERGY-001/terminal-support.md#snow-free-successor-chronology |
| child-2c-guards | SC-LANDSURFACEENERGY-001/terminal-support.md#child-2c-guards |
| version-8-persistent-snow--soil-boundary-amendment | SC-LANDSURFACEENERGY-001/soil-coupling.md#version-8-persistent-snow--soil-boundary-amendment |
| version-9-exact-liquid-reference-state-representation-amendment | SC-LANDSURFACEENERGY-001/water-vapor.md#version-9-exact-liquid-reference-state-representation-amendment |
| version-11-inactive-liquid-vapor-coordinate-domain-amendment | SC-LANDSURFACEENERGY-001/nonlinear-solve.md#version-11-inactive-liquid-vapor-coordinate-domain-amendment |
| version-12-exact-closed-bound-finite-difference-amendment | SC-LANDSURFACEENERGY-001/nonlinear-solve.md#version-12-exact-closed-bound-finite-difference-amendment |
| version-13-first-domain-valid-no-update-termination-amendment | SC-LANDSURFACEENERGY-001/nonlinear-solve.md#version-13-first-domain-valid-no-update-termination-amendment |
| version-14-snow-free-frozen-forest-litter-successor-amendment | SC-LANDSURFACEENERGY-001/litter-phase.md#version-14-snow-free-frozen-forest-litter-successor-amendment |
| retained-authority-and-adjudicated-constants | SC-LANDSURFACEENERGY-001/litter-phase.md#retained-authority-and-adjudicated-constants |
| v3-state-phase-free-solve-and-signed-vapor | SC-LANDSURFACEENERGY-001/litter-phase.md#v3-state-phase-free-solve-and-signed-vapor |
| bounded-kinetic-phase-and-fusion-energy-closure | SC-LANDSURFACEENERGY-001/litter-phase.md#bounded-kinetic-phase-and-fusion-energy-closure |
| ingress-wb14-identity-restart-receipts-and-failure-posture | SC-LANDSURFACEENERGY-001/litter-phase.md#ingress-wb14-identity-restart-receipts-and-failure-posture |
| v3-invariants-and-required-production-vectors | SC-LANDSURFACEENERGY-001/litter-phase.md#v3-invariants-and-required-production-vectors |
| version-15-receiver-owned-exact-soil-enthalpy-carry-amendment | SC-LANDSURFACEENERGY-001/soil-custody.md#version-15-receiver-owned-exact-soil-enthalpy-carry-amendment |
| version-16-lse-surface-enthalpy-exact-carry-amendment | SC-LANDSURFACEENERGY-001/surface-custody.md#version-16-lse-surface-enthalpy-exact-carry-amendment |
| openwepp_snow_free_lse_v2-v10-coupling-amendment | SC-LANDSURFACEENERGY-001/nonlinear-solve.md#openwepp_snow_free_lse_v2-v10-coupling-amendment |
| version-9-positive-support-admission-owner-amendment | SC-LANDSURFACEENERGY-001/terminal-support.md#version-9-positive-support-admission-owner-amendment |
| canonical-stage-3-accepted-map-boundary-amendment | SC-LANDSURFACEENERGY-001/map-custody.md#canonical-stage-3-accepted-map-boundary-amendment |
| profile-integration | SC-LANDSURFACEENERGY-001/map-custody.md#profile-integration |
| exact-surface-parent-local-chronology-amendment | SC-LANDSURFACEENERGY-001/surface-custody.md#exact-surface-parent-local-chronology-amendment |
| represented-snow-native-lse-cross-regime-amendment | SC-LANDSURFACEENERGY-001/terminal-support.md#represented-snow-native-lse-cross-regime-amendment |
| candidate-only-v2-soil-beginning-amendment | SC-LANDSURFACEENERGY-001/soil-custody.md#candidate-only-v2-soil-beginning-amendment |
| exact-v3-litter-phase-capacity-spill-amendment | SC-LANDSURFACEENERGY-001/litter-phase.md#exact-v3-litter-phase-capacity-spill-amendment |
| exact-heterogeneous-v3-surface-resource-join-amendment | SC-LANDSURFACEENERGY-001/litter-phase.md#exact-heterogeneous-v3-surface-resource-join-amendment |
| topology-ranked-v16-exact-surface-owner-amendment | SC-LANDSURFACEENERGY-001/surface-custody.md#topology-ranked-v16-exact-surface-owner-amendment |
| validated-in-memory-lse-custody-handoff-amendment | SC-LANDSURFACEENERGY-001/map-custody.md#validated-in-memory-lse-custody-handoff-amendment |
| snow-free-final-receipt-reseal-amendment | SC-LANDSURFACEENERGY-001/map-custody.md#snow-free-final-receipt-reseal-amendment |
| covered-nonfinal-physical-only-map-amendment | SC-LANDSURFACEENERGY-001/map-custody.md#covered-nonfinal-physical-only-map-amendment |
| stage-3-identity-anchor-jacobian-amendment | SC-LANDSURFACEENERGY-001/nonlinear-solve.md#stage-3-identity-anchor-jacobian-amendment |
| covered-leaf-maximum-demand-exact-reuse-amendment | SC-LANDSURFACEENERGY-001/nonlinear-solve.md#covered-leaf-maximum-demand-exact-reuse-amendment |
| carrier-parent-static-and-same-map-validation-once-amendment | SC-LANDSURFACEENERGY-001/map-custody.md#carrier-parent-static-and-same-map-validation-once-amendment |
| component-temperature-jacobian-dependency-replay-amendment | SC-LANDSURFACEENERGY-001/dependency-replay.md#component-temperature-jacobian-dependency-replay-amendment |
| eligibility-integrity-mismatch-and-error-outcomes | SC-LANDSURFACEENERGY-001/dependency-replay.md#eligibility-integrity-mismatch-and-error-outcomes |
| normative-fallibility-and-canonical-crossability-matrix | SC-LANDSURFACEENERGY-001/dependency-replay.md#normative-fallibility-and-canonical-crossability-matrix |
| prospective-stencil-aware-dependency-replay-experiment | SC-LANDSURFACEENERGY-001/qualification.md#prospective-stencil-aware-dependency-replay-experiment |
| exp-stage3-20260906-r-pc1-proof-closed-maximum-leaf-eligibility | SC-LANDSURFACEENERGY-001/qualification.md#exp-stage3-20260906-r-pc1-proof-closed-maximum-leaf-eligibility |
| exp-stage3-20260906-r-sg1-shared-wet-routing-guard-assurance | SC-LANDSURFACEENERGY-001/qualification.md#exp-stage3-20260906-r-sg1-shared-wet-routing-guard-assurance |

<a id="INV-LANDSURFACEENERGY-001"></a>
<a id="INV-LANDSURFACEENERGY-002"></a>
<a id="INV-LANDSURFACEENERGY-010"></a>
<a id="INV-LANDSURFACEENERGY-011"></a>
<a id="INV-LANDSURFACEENERGY-012"></a>
<a id="INV-LANDSURFACEENERGY-013"></a>
<a id="INV-LANDSURFACEENERGY-014"></a>
<a id="INV-LANDSURFACEENERGY-015"></a>
<a id="INV-LANDSURFACEENERGY-020"></a>
<a id="INV-LANDSURFACEENERGY-021"></a>
<a id="INV-LANDSURFACEENERGY-022"></a>
<a id="INV-LANDSURFACEENERGY-030"></a>
<a id="INV-LANDSURFACEENERGY-031"></a>
<a id="INV-LANDSURFACEENERGY-032"></a>
<a id="INV-LANDSURFACEENERGY-040"></a>
<a id="INV-LANDSURFACEENERGY-041"></a>
<a id="INV-LANDSURFACEENERGY-042"></a>
<a id="INV-LANDSURFACEENERGY-043"></a>
<a id="INV-LANDSURFACEENERGY-100"></a>
<a id="INV-LANDSURFACEENERGY-101"></a>
<a id="INV-LANDSURFACEENERGY-102"></a>
<a id="INV-LANDSURFACEENERGY-103"></a>
<a id="INV-LANDSURFACEENERGY-104"></a>
<a id="INV-LANDSURFACEENERGY-105"></a>
<a id="INV-LANDSURFACEENERGY-106"></a>
<a id="INV-LANDSURFACEENERGY-107"></a>
<a id="INV-LANDSURFACEENERGY-108"></a>
<a id="INV-LANDSURFACEENERGY-109"></a>
<a id="INV-LANDSURFACEENERGY-110"></a>
<a id="INV-LANDSURFACEENERGY-111"></a>
<a id="INV-LANDSURFACEENERGY-112"></a>
<a id="INV-LANDSURFACEENERGY-113"></a>
<a id="INV-LANDSURFACEENERGY-114"></a>
<a id="INV-LANDSURFACEENERGY-115"></a>
<a id="INV-LANDSURFACEENERGY-116"></a>
<a id="INV-LANDSURFACEENERGY-117"></a>
<a id="INV-LANDSURFACEENERGY-118"></a>
<a id="INV-LANDSURFACEENERGY-119"></a>
<a id="INV-LANDSURFACEENERGY-120"></a>
<a id="INV-LANDSURFACEENERGY-121"></a>
<a id="INV-LANDSURFACEENERGY-122"></a>
<a id="INV-LANDSURFACEENERGY-123"></a>
<a id="INV-LANDSURFACEENERGY-124"></a>
<a id="INV-LANDSURFACEENERGY-125"></a>
<a id="INV-LANDSURFACEENERGY-126"></a>
<a id="INV-LANDSURFACEENERGY-130"></a>
<a id="INV-LANDSURFACEENERGY-131"></a>
<a id="INV-LANDSURFACEENERGY-138"></a>
<a id="INV-LANDSURFACEENERGY-139"></a>
<a id="INV-LANDSURFACEENERGY-140"></a>
<a id="INV-LANDSURFACEENERGY-141"></a>
<a id="INV-LANDSURFACEENERGY-142"></a>
<a id="INV-LANDSURFACEENERGY-143"></a>
<a id="INV-LANDSURFACEENERGY-144"></a>
<a id="INV-LANDSURFACEENERGY-145"></a>
<a id="INV-LANDSURFACEENERGY-146"></a>
<a id="INV-LANDSURFACEENERGY-147"></a>
<a id="INV-LANDSURFACEENERGY-148"></a>
<a id="INV-LANDSURFACEENERGY-149"></a>
<a id="INV-LANDSURFACEENERGY-150"></a>
<a id="INV-LANDSURFACEENERGY-151"></a>
<a id="INV-LANDSURFACEENERGY-152"></a>
<a id="INV-LANDSURFACEENERGY-153"></a>
<a id="INV-LANDSURFACEENERGY-154"></a>
<a id="INV-LANDSURFACEENERGY-155"></a>
<a id="INV-LANDSURFACEENERGY-156"></a>
<a id="INV-LANDSURFACEENERGY-157"></a>
<a id="INV-LANDSURFACEENERGY-158"></a>
<a id="INV-LANDSURFACEENERGY-159"></a>
<a id="INV-LANDSURFACEENERGY-160"></a>
<a id="INV-LANDSURFACEENERGY-161"></a>
<a id="INV-LANDSURFACEENERGY-162"></a>
<a id="INV-LANDSURFACEENERGY-163"></a>
<a id="INV-LANDSURFACEENERGY-164"></a>
<a id="OBL-LANDSURFACEENERGY-C-001"></a>
<a id="OBL-LANDSURFACEENERGY-C-002"></a>
<a id="OBL-LANDSURFACEENERGY-C-003"></a>
<a id="OBL-LANDSURFACEENERGY-C-004"></a>
<a id="OBL-LANDSURFACEENERGY-C-005"></a>
<a id="OBL-LANDSURFACEENERGY-C-006"></a>
<a id="OBL-LANDSURFACEENERGY-C-007"></a>
<a id="OBL-LANDSURFACEENERGY-C-008"></a>
<a id="OBL-LANDSURFACEENERGY-C-009"></a>
<a id="OBL-LANDSURFACEENERGY-C-010"></a>
<a id="OBL-LANDSURFACEENERGY-C-011"></a>
<a id="OBL-LANDSURFACEENERGY-C-012"></a>
<a id="OBL-LANDSURFACEENERGY-C-013"></a>
<a id="OBL-LANDSURFACEENERGY-C-014"></a>
<a id="OBL-LANDSURFACEENERGY-C-015"></a>
<a id="OBL-LANDSURFACEENERGY-C-016"></a>
<a id="OBL-LANDSURFACEENERGY-C-017"></a>
<a id="OBL-LANDSURFACEENERGY-C-018"></a>
<a id="OBL-LANDSURFACEENERGY-C-019"></a>
<a id="OBL-LANDSURFACEENERGY-C-020"></a>
<a id="OBL-LANDSURFACEENERGY-P-001"></a>
<a id="OBL-LANDSURFACEENERGY-P-002"></a>
<a id="OBL-LANDSURFACEENERGY-P-003"></a>
<a id="OBL-LANDSURFACEENERGY-P-004"></a>
<a id="OBL-LANDSURFACEENERGY-P-005"></a>
<a id="OBL-LANDSURFACEENERGY-P-006"></a>
<a id="purpose"></a>
<a id="scientific-scope-and-explicit-out-of-scope-boundaries"></a>
<a id="authority-anchors-with-top-down-citations"></a>
<a id="variables-and-units-using-canonical-symbols-first"></a>
<a id="algorithm-state-surfaces"></a>
<a id="algorithm-specification-with-step-sequence"></a>
<a id="step-local-preconditions-intermediates-and-postconditions"></a>
<a id="branch-and-guard-table"></a>
<a id="invariants-and-invariant-guard-map"></a>
<a id="invariant-guard-map"></a>
<a id="producer-obligations-and-consumer-obligations"></a>
<a id="symbol-alias-map"></a>
<a id="constants-and-parameters-with-provenance-anchors"></a>
<a id="unit-governance-map"></a>
<a id="tolerance-and-numeric-notes"></a>
<a id="calibration-and-identifiability"></a>
<a id="test-vector-obligations"></a>
<a id="child-2c-invariant-ids"></a>
<a id="gap-register-and-promotability-labels"></a>
<a id="openwepp_snow_free_lse_v1-constitutive-authority"></a>
<a id="selected-sources-and-domain"></a>
<a id="exact-ownership-and-state"></a>
<a id="shortwave-and-reciprocal-longwave"></a>
<a id="neutral-turbulent-heat-and-vapor-network"></a>
<a id="surface-humidity-surface-enthalpy-litter-and-soil-heat"></a>
<a id="signed-vapor-and-liquid-enthalpy"></a>
<a id="immutable-beginning-water-transaction-and-current-ingress"></a>
<a id="ordered-numerical-algorithm-active-branches-and-error-precedence"></a>
<a id="independent-closure-and-errors"></a>
<a id="v1-invariants-and-independent-fixtures"></a>
<a id="scope"></a>
<a id="terminal-receiver-remaining-support-amendment"></a>
<a id="child-2c-shared-carrier-and-successor-support-amendment"></a>
<a id="snow-free-successor-chronology"></a>
<a id="child-2c-guards"></a>
<a id="version-8-persistent-snow--soil-boundary-amendment"></a>
<a id="version-9-exact-liquid-reference-state-representation-amendment"></a>
<a id="version-11-inactive-liquid-vapor-coordinate-domain-amendment"></a>
<a id="version-12-exact-closed-bound-finite-difference-amendment"></a>
<a id="version-13-first-domain-valid-no-update-termination-amendment"></a>
<a id="version-14-snow-free-frozen-forest-litter-successor-amendment"></a>
<a id="retained-authority-and-adjudicated-constants"></a>
<a id="v3-state-phase-free-solve-and-signed-vapor"></a>
<a id="bounded-kinetic-phase-and-fusion-energy-closure"></a>
<a id="ingress-wb14-identity-restart-receipts-and-failure-posture"></a>
<a id="v3-invariants-and-required-production-vectors"></a>
<a id="version-15-receiver-owned-exact-soil-enthalpy-carry-amendment"></a>
<a id="version-16-lse-surface-enthalpy-exact-carry-amendment"></a>
<a id="openwepp_snow_free_lse_v2-v10-coupling-amendment"></a>
<a id="version-9-positive-support-admission-owner-amendment"></a>
<a id="canonical-stage-3-accepted-map-boundary-amendment"></a>
<a id="profile-integration"></a>
<a id="exact-surface-parent-local-chronology-amendment"></a>
<a id="represented-snow-native-lse-cross-regime-amendment"></a>
<a id="candidate-only-v2-soil-beginning-amendment"></a>
<a id="exact-v3-litter-phase-capacity-spill-amendment"></a>
<a id="exact-heterogeneous-v3-surface-resource-join-amendment"></a>
<a id="topology-ranked-v16-exact-surface-owner-amendment"></a>
<a id="validated-in-memory-lse-custody-handoff-amendment"></a>
<a id="snow-free-final-receipt-reseal-amendment"></a>
<a id="covered-nonfinal-physical-only-map-amendment"></a>
<a id="stage-3-identity-anchor-jacobian-amendment"></a>
<a id="covered-leaf-maximum-demand-exact-reuse-amendment"></a>
<a id="carrier-parent-static-and-same-map-validation-once-amendment"></a>
<a id="component-temperature-jacobian-dependency-replay-amendment"></a>
<a id="eligibility-integrity-mismatch-and-error-outcomes"></a>
<a id="normative-fallibility-and-canonical-crossability-matrix"></a>
<a id="prospective-stencil-aware-dependency-replay-experiment"></a>
<a id="exp-stage3-20260906-r-pc1-proof-closed-maximum-leaf-eligibility"></a>
<a id="exp-stage3-20260906-r-sg1-shared-wet-routing-guard-assurance"></a>

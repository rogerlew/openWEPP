# EROD10 Intake Decision and Scope

Status: `completed`
Evidence mode: `Static + Ran`
Decision: `GO-WITH-HOLD-GATES`
Decision date: `2026-05-23`

Static:
- Reviewed package authority inputs from PL09 queue addendum, PL15 hold-lift
  decision, WB16 disposition, and canonical science contracts.
- Reviewed kernel governance authority from:
  - `docs/specifications/science-contract-authoring-procedure.md`
  - `docs/specifications/science-contracts/kernel-process-contract-profile.md`

Ran:
- Inspected package/dependency artifacts with repository commands (`ls`, `find`,
  `rg`, `sed`) in `/home/workdir/openWEPP`.

## Decision Summary

- `KERNEL-GAP-010` is converted from a deferral into an executable,
  dependency-gated sediment-kernelization roadmap.
- EROD10 is an intake/planning package only; no production erosion-kernel code
  is authorized in this package.
- Follow-on implementation execution is authorized only through explicit wave
  gates defined in EROD10 artifacts.

## Intake Baseline

| Baseline item | Evidence | Intake interpretation |
|---|---|---|
| `KERNEL-GAP-010` queued as `EROD10-sediment-kernelization-intake` | `pl08-hold-lift-work-package-queue.md` | Intake package must define executable follow-on erosion work waves. |
| PL08 hold remains retained after PL15 | `pl15-pl08-hold-lift-decision-record.md` | Erosion lane planning must preserve strict governance gating and no over-claim posture. |
| WB16 peak-runoff coupling is complete | `20260523-wb16-peak-runoff-kernel-001/artifacts/wb16_disposition.md` | Required hydrologic peak/duration inputs (`peakro`, `watdur`) are available as erosion-lane prerequisites. |
| Sediment/routing contracts contain non-promotable authority gaps | `SC-SED-001`, `SC-HYDRAULICS-001`, `SC-ROUTE-001`, `SC-WATBAL-001`, `SC-RUNOFFPART-001` | Wave-0 ownership closure is required before production erosion implementation. |

## Scope Boundary

Included:
- Intake decision and executable wave plan for sediment kernelization.
- Explicit dependency graph and package ownership/gates.
- Contract-authority mapping for erosion-lane producer/consumer boundaries.
- Kernel governance checklist for follow-on kernel-authoring packages.

Out of scope:
- Production erosion, routing, or watershed code changes.
- Contract promotion claims outside planning scope.
- PL08 hold-lift decision changes.

## High-Severity Authority/Ambiguity Register

| ambiguity_id | issue | evidence | disposition owner | gate |
|---|---|---|---|---|
| `EROD10-AH-001` | Alias ownership for erosion boundary symbols remains identity-only and non-promotable in companion contracts. | `GAP-SED-002`, `GAP-HYD-002`, `GAP-ROUTE-002`, `GAP-WATBAL-003`, `GAP-RUNOFFPART-002` | `EROD11-alias-and-boundary-ownership-closure-001` | `HOLD` until explicit canonical->runtime alias mapping and owners are ratified. |
| `EROD10-AH-002` | Cross-domain erosion closure semantics remain provisional across sediment/hydraulics/routing companion contracts. | `GAP-SED-003`, `GAP-HYD-003`, `GAP-ROUTE-003`, `GAP-RUNOFFPART-004` | `EROD12-cross-domain-contract-closure-001` | `HOLD` until companion contract closure and shared guard ownership matrix are ratified. |
| `EROD10-AH-003` | Downstream watershed production consumer path is not yet completed. | `WS10` package status `queued` | `WS10-channel-impoundment-production-kernels-001` + `EROD15-routing-boundary-coupling-001` | `HOLD` on erosion-to-watershed production coupling until WS10 exits HOLD. |

## Ratified Follow-On Package Set

| package_id | lane | objective | depends_on | acceptance gate |
|---|---|---|---|---|
| `EROD11-alias-and-boundary-ownership-closure-001` | governance+contracts | Close boundary alias ownership ambiguity for erosion/hydraulics/routing/watbal payloads. | `EROD10`, `WB16` | All non-promotable alias gaps re-owned with explicit canonical->runtime map and typed guard ownership. |
| `EROD12-cross-domain-contract-closure-001` | contracts | Close cross-domain ownership/guard semantics across `SC-SED-001`, `SC-HYDRAULICS-001`, and `SC-ROUTE-001` with dual review/disposition/verification artifacts. | `EROD11` | Companion non-promotable contract gaps resolved or explicitly risk-accepted under authority. |
| `EROD13-hillslope-core-erosion-kernel-001` | erosion-kernel | Implement Chapter-11 core continuity/detachment/deposition/transport-capacity runtime behavior with typed guards. | `EROD12`, `WB16`, `WB14`, `WB15` | `INV-SED-001..007` runtime guard + contract-test vectors passing. |
| `EROD14-multiofe-and-enrichment-kernel-001` | erosion-kernel | Implement OFE-strip routing + enrichment class conservation and branch logic. | `EROD13` | `INV-SED-008..009` closure and conservation vectors passing. |
| `EROD15-routing-boundary-coupling-001` | integration | Implement authoritative erosion export payload + routing consumer coupling for production pathways. | `EROD14`, `WS10` | `INV-SED-010` and routing payload guards pass on integration fixtures. |
| `EROD16-sediment-closeout-and-comparator-001` | closeout | Tiered comparator evidence and governance disposition for erosion lane claims. | `EROD15` | Comparator evidence complete, remaining gaps explicitly classified (`promotable-with-risk` vs `non-promotable`). |

## Intake Verdict

- Intake package objectives are satisfied.
- Erosion lane is executable with explicit package IDs, dependencies, and
  gate ownership.
- `HOLD` behavior is retained for follow-on execution when Wave-0 authority
  gates (`EROD10-AH-001..003`) are not closed.

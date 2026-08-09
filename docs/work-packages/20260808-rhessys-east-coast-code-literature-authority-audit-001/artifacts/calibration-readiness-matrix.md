# Calibration-Readiness Matrix

Status: `audit-complete / not calibration ready`

Evidence mode: `Static`

Global fields: `science_implementation_status=NOT_IMPLEMENTED`,
`calibration_evidence_status=NOT_CALIBRATION_READY`, and
`identifiability_status=NOT_ASSESSED`.

| Family | Science authority | Parameter/data authority | Observation/operator | Readiness disposition | Evidence |
| --- | --- | --- | --- | --- | --- |
| profile identity/schema | typed strict boundary admitted; implementation absent | 71x32 raw data licensed, but five mismatches and 53 hidden defaults | not applicable to raw parsing | `BLOCKED` | parameter matrix, `AUTH-RHEC-001/002` |
| radiation/optics | bounded literature candidates; mixed operator not admitted | profile cells lack locators/domains | no empirical role assigned | `BLOCKED` | `AUTH-RHEC-003` |
| liquid interception | bounded store/event candidates; complete cadence/release law absent | capacity values unadmitted | no calibration/validation split | `BLOCKED` | `AUTH-RHEC-004` |
| aerodynamic conductance | authority missing | geometry/sentinel parameters unadmitted | no operator | `BLOCKED` | `AUTH-RHEC-005` |
| stomatal/canopy conductance | domain-limited empirical family available; exact source chain not admitted | factor thresholds/scales lack cell provenance | no independent leaf/canopy conductance role assigned | `BLOCKED` | `AUTH-RHEC-006` |
| Penman-Monteith demand | primary equation family is supported, but the executed source gamma is dimensionally reconstructed and rejected | resistance/energy inputs unadmitted | ET partition operator absent | `BLOCKED` | `AUTH-RHEC-007` |
| C3 photosynthesis | core primary equations available; source adaptations incomplete | capacity/N constants and profile domains incomplete | flux-tower/gas-exchange roles absent | `BLOCKED` | `AUTH-RHEC-008` |
| phenology/LAI | current openWEPP GSI remains owner; RHESSys static profile route unadmitted | dates/SLA/LAI ratio unadmitted | phenology/LAI operator absent | `BLOCKED` | `AUTH-RHEC-009` |
| layer root demand | normalized literature candidate; canonical request/arbitration law incomplete | root profiles absent | layer-water observation operator absent | `BLOCKED` | `AUTH-RHEC-010` |
| respiration/allocation/turnover | family leads only | values/domains unadmitted | carbon/litter operators absent | `BLOCKED` for respiration; otherwise `NOT_APPLICABLE` to first boundary | `AUTH-RHEC-011` |
| canopy snow | canonical single-owner boundary only | no law/parameters | none | `NOT_APPLICABLE` to initial boundary | `AUTH-RHEC-012` |
| canopy/ground available energy | component owners and complete closing equation are not admitted; executed source heat/longwave/clamp behavior is rejected | storage depth, heat capacity, emissivity, and lower-boundary operands unadmitted | independent component-energy reconstruction absent | `BLOCKED` | `AUTH-RHEC-014` |
| compatible initial vegetation state | no admitted synthesis law connects LAI, SLA, C/N pools, allocation, deadwood ratio, and root depth | initializer constants and profile cells are unadmitted and SLA identity diverges | initial-state mass/LAI reconstruction absent | `BLOCKED` | `AUTH-RHEC-015` |
| reproducible definition acquisition | strict digest-bound local input is required; mutable network fallback is rejected | accepted bytes have pinned repository and file provenance only | acquisition-integrity check required, not an empirical operator | `BLOCKED` | `AUTH-RHEC-016` |

No measured data were assigned to calibration or independent validation, no
objective was run, and no synthetic recovery was performed. Profile table
values are not observations, priors, calibrated values, or physiological
bounds.

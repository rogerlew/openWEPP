# Calibration And Validation Readiness

Status: `PASS for EB-01 reconciliation`; no calibration or result-bearing
execution is authorized.

## Orthogonal Status

| Scope | `science_implementation_status` | `calibration_evidence_status` | `identifiability_status` | Rationale |
| --- | --- | --- | --- | --- |
| Sub-canopy longwave | `AUTHORITY_MISSING` | `NOT_CALIBRATION_READY` | `NOT_ASSESSED` | Load-bearing view partition and canopy temperature/emissivity authority are absent. |
| Energy-consistent sublimation | `NOT_IMPLEMENTED` | `NOT_CALIBRATION_READY` | `PARTIALLY_IDENTIFIABLE` | Contract-first composition is admitted; installed mass/persistence observations identify only some responses. |
| Combined factorial | `NOT_IMPLEMENTED` | `NOT_CALIBRATION_READY` | `PARTIALLY_IDENTIFIABLE` | EB-02/03 and decision thresholds are prerequisites; warm-maritime conifer transfer remains data-limited. |

These statuses describe the prospective mechanisms, not a blocked EB-01 gate.
EB-01's current-scope obligation is to identify and disposition the gaps. It
does not implement or calibrate a parameterized model.

## Science-Contract-Spec Obligations

| Obligation | Gate | Evidence | Rationale |
| --- | --- | --- | --- |
| typed/enumerable parameter surface | `NOT_APPLICABLE` | [authority gap ledger](authority-gap-ledger.csv) | EB-01 introduces no parameter or execution value; successors must type any admitted surface. |
| observation operator with units and scale | `PASS` | [observation ledger](observation-fixture-ledger.csv), [response ledger](response-operator-ledger.csv) | Sources, periods, strata, units, forcing uncertainty, and operators are frozen; unresolved thresholds explicitly hold EB-04. |
| deterministic candidate execution | `NOT_APPLICABLE` | `package.md`, excluded scope | EB-01 is reconciliation/design and runs no B/L/S/LS candidate. |
| objective reconstruction | `PASS` | [factorial design](factorial-design.md), [decision rules](decision-rules.csv) | Effects, interaction, direct-first ordering, and unresolved threshold owners are explicit. |
| sensitivity analysis | `NOT_APPLICABLE` | `package.md`, excluded scope | No parameter surface or empirical fitting exists in EB-01. |
| identifiability/confounding analysis | `PASS` | [observation figure](figures/snow-eb01-observation-discrimination.md), [role freeze](observation-role-freeze.csv) | Canopy contrasts, open controls, partial bindings, and missing transfer lane are separated. |
| boundary, saturation, and failure reporting | `PASS` | [stop-loss](stop-loss.csv), [rejected formulas](rejected-formulas.md) | Hard physical failures, inadmissible compensation, and campaign stop outcomes are explicit. |
| equifinality/uncertainty retention | `PASS` | [decision rules](decision-rules.csv), [observation ledger](observation-fixture-ledger.csv) | Forcing/representation limits and longwave–sublimation interaction are retained; no unique-fit claim is made. |
| synthetic recovery | `NOT_APPLICABLE` | `package.md`, excluded scope | No parameterized calibration machinery is implemented here. |
| additional-data inventory | `PASS` | [source acquisition list](source-acquisition-needed.csv) | Longwave authority and warm-maritime transfer needs are separately prioritized. |

There is no current-scope `BLOCKED` row. The `NOT_CALIBRATION_READY` mechanism
statuses are truthful prospective outcomes and are carried as successor holds,
not relabeled as EB-01 validation passes. Before EB-04 can execute, the
threshold/window rows in [decision-rules.csv](decision-rules.csv) must be
replaced by admitted, frozen values.

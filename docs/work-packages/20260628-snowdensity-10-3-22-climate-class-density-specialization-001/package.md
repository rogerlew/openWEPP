# SNOWDENSITY-10.3.22 Climate-Class Density Specialization

Status: `HOLD-AUTHORITY-GAP-NO-PROMOTION`  
Date: `2026-06-28`  
Contract: `SC-SNOWFREEZE-001` v106, `INV-SNOWFREEZE-077`,
`OBL-SNOWFREEZE-P-052`  
Selector: `OPENWEPP_SNOWDENSITY09_DENSITY_MODEL=physics_bulk_climate_class_density_v1`

## Objective

Scaffold and execute an opt-in comprehensive climate-class snow-density
specialization candidate using the full Sturm snow-class system. The candidate
must assign class from the run's own wind, precipitation, and air-temperature
climate, never from geography, site identity, observed residuals, or fixture
membership. Parameter authority must remain wholly in Sturm 1995/2010, with the
cross-SNOTEL forcing-robust rubric used only for validation.

The candidate scope includes all six Sturm 1995 class labels: tundra, taiga,
alpine, maritime, prairie, and ephemeral.

## Read-First Basis

- `docs/work-packages/20260628-snow-density-paradigm-assessment-001/artifacts/recommendation.md`
- `docs/work-packages/20260628-snow-density-paradigm-assessment-001/artifacts/adr-candidate-snow-density-paradigm.md`
- `docs/work-packages/20260628-snow-density-paradigm-assessment-001/artifacts/paradigm-comparison.md`
- `docs/planning/snow-frost-fidelity-strategy.md` section 10.2 item 7 and
  section 10.3
- `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
- ADR-0011, ADR-0025, ADR-0026, ADR-0027, ADR-0028
- `references/copyrighted/sturm2010_swe_climate_classes.pdf`
- R-59 Sturm 1995 and NSIDC-0768 class-system references
- `crates/openwepp-hillslope-orchestrator/src/hydrology/09_snow_density.rs`
- `DirectSnowLaneState` / direct snow partition runtime surfaces

## Execution Summary

This package was executed to the contract-first boundary. It reserved the
opt-in selector and implemented explicit-unit Sturm 2010 density trajectory
support for the five classes with local Table 4 parameters:

- alpine
- maritime
- prairie
- tundra
- taiga

The candidate remains fail-closed and non-promoted because two authority gaps
block an honest comprehensive implementation:

- The local authority set does not contain the numeric Sturm 1995 binary
  decision-tree thresholds needed to derive class from the run's own wind,
  precipitation, and air-temperature climate.
- Sturm 2010 Table 4 does not supply `rho_max`, `rho_0`, `k1`, and `k2`
  parameters for ephemeral snow; the paper states ephemeral measurements were
  excluded.

No thresholds, parameters, class mappings, or smoothing were fitted to
SNOTEL/cancov fixtures. NSIDC-0768 remains an independent class-system
cross-check only and is not used as a runtime geographic lookup.

## Implementation Notes

- `physics_bulk_climate_class_density_v1` is accepted only by the existing
  package-bound internal density selector.
- The current no-env default remains
  `coe_liquid_holding_capacity_v1 + physics_bulk_density_compaction_v1 +
  harder_pomeroy_hourly`.
- `legacy_wepp` rollback is preserved.
- The candidate fails closed without authoritative class assignment operands.
- Ephemeral fails closed because local Sturm 2010 density parameters are absent.
- Raw Sturm density-form fallback is flagged by the runtime outcome when used.
- SWE identity and the active `522 kg m^-3` density cap are preserved for
  explicit supported-class unit evidence.

## Gate Disposition

| Gate | Status | Evidence |
|---|---|---|
| Cross-SNOTEL forcing-robust rubric beats current default `15/179` | Missing | Not run because runtime class assignment authority is incomplete. |
| Bidirectional densification flip | Missing | Cannot be tested without authoritative class assignment. |
| No new bidirectional persistence tail | Missing | Cannot be tested without authoritative class assignment and real WAT run. |
| Conservation closes | Partial | Focused explicit-class unit evidence closes SWE identity; whole-model WAT conservation was not run. |
| No fixture fitting | Pass | No class threshold or parameter was derived from fixtures. |
| Default and rollback preserved | Pass | No-env default remains unchanged; rollback selector remains available. |

## Protected Boundaries

No fixture, public output schema, density cap, frost behavior, parser/runfile/user
CLI selector, `.run` control, Qwet/frzftp, compatibility runtime, melt, phase,
canopy, or radiation change is authorized by this package.

## Disposition

`HOLD-AUTHORITY-GAP-NO-PROMOTION`.

The right follow-on is an authority package that obtains or cites the numeric
Sturm 1995 decision-tree thresholds and supplies defensible ephemeral density
parameters or a separately ratified ephemeral fallback. Only then can
`physics_bulk_climate_class_density_v1` be run on the cross-SNOTEL primary gate.

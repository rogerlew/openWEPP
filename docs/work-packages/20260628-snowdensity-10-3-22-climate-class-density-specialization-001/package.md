# SNOWDENSITY-10.3.22 Climate-Class Density Specialization

Status: `HOLD-GATE-FAILURE-NON-PROMOTION`
Date: `2026-06-28`  
Contract: `SC-SNOWFREEZE-001` v107, `INV-SNOWFREEZE-077`,
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
- R-59 Sturm 1995, R-61 Sturm/Liston 2021, and NSIDC-0768 class-system references
- `crates/openwepp-hillslope-orchestrator/src/hydrology/09_snow_density.rs`
- `DirectSnowLaneState` / direct snow partition runtime surfaces

## Execution Summary

This package was rerun after the source PDFs became available. The authority gap
for the Sturm 1995 tree thresholds is closed by
`artifacts/sturm-thresholds-source-verification.md`; the first-pass extraction
is a navigation aid only.

The opt-in selector remains explicit and package-bound. The rerun now implements
forcing-derived class assignment from the run's own climatological normals:

- CDM from monthly mean air temperature with `Tc=10 degC`;
- SPR from mean monthly precipitation rate over months with `Ta < Tc`;
- winter wind from the same cold-month forcing subset.

Verified Sturm 1995 scanned source thresholds:

- CDM threshold values `30` and `125 degC-month`;
- SPR threshold `2 mm day^-1`;
- wind bracket `0.5-2.0 m s^-1`.

The actual-wind branch fails closed inside `0.5 < wind < 2.0 m s^-1` because
Sturm 1995 brackets but does not select a single wind-speed cutoff; the original
map used vegetation stature as a wind proxy. Rare deep-tundra/deep-taiga
branches also fail closed because they are not one of the six standard Sturm
1995 class labels.

The candidate carries explicit-unit Sturm 2010 density trajectory support for
the five classes with local Table 4 parameters:

- alpine
- maritime
- prairie
- tundra
- taiga

Ephemeral is part of the class-assignment tree, but Sturm 2010 excludes
ephemeral from Table 4. The candidate therefore retains the existing
process-first fresh-snow/Anderson compaction behavior for ephemeral as a
documented fallback rather than fabricating Sturm parameters.

Sturm/Liston 2021 is recorded as a cross-check, not substituted authority:
ephemeral CDM changes `30 -> 61 degC-month`, precipitation changes
`2 -> 4 mm day^-1`, `Taiga` is renamed `Boreal Forest`, and `Alpine` is renamed
`Montane Forest`. Runtime class labels remain the Sturm 1995 names paired with
Sturm 2010 parameters.

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
- The candidate fails closed without source-derived class assignment operands,
  for rare tree branches, and for wind-dependent branches in the unresolved
  1995 wind interval.
- Ephemeral uses the documented fresh-snow/Anderson fallback because local
  Sturm 2010 density parameters are absent.
- Raw Sturm density-form fallback is flagged by the runtime outcome when used.
- SWE identity and the active `522 kg m^-3` density cap are preserved for
  explicit supported-class unit evidence.

## Gate Disposition

The real coupled direct-production cross-SNOTEL+cancov WAT/trace rerun completed
for the current no-env default and the opt-in
`physics_bulk_climate_class_density_v1` candidate. Source authority is now
closed, and whole-model trace conservation closes, but the candidate fails the
primary promotion gates:

- activated default rerun profile: `15` robust fails / `179` robust score;
- climate-class candidate profile: `16` robust fails / `168` robust score;
- candidate improvements: `4` robust cells;
- candidate regressions: `13` robust cells;
- bidirectional densification flip: not achieved; `harvard_open` regressed on
  `seasonal_densification_trajectory`.

Conservation evidence from the candidate traces:

- candidate trace rows: `159986`;
- max snow-state residual: `4.440892098500626e-16 m`;
- max partition residual: `5.551115123125783e-17 m`;
- tolerance: `1e-9 m`.

Review and verification evidence:

- `artifacts/review.md`
- `artifacts/verification.md`
- `artifacts/line-count-governance.md`

Coverage is honest: all six Sturm 1995 classes are implemented by reference, but
the current corpus does not validate every class regime. Classes absent from the
SNOTEL/cancov climate corpus remain reference-covered, not rubric-validated.

| Gate | Status | Evidence |
|---|---|---|
| Source thresholds verified from source | Pass | `artifacts/sturm-thresholds-source-verification.md`; contract v107. |
| Cross-SNOTEL forcing-robust rubric beats current default `15/179` | Fail | Candidate `16/168` vs default `15/179`; `artifacts/climate-class-density-specialization-rubric.json`. |
| Bidirectional densification flip | Fail | No robust densification improvements; `harvard_open:seasonal_densification_trajectory` regressed. |
| No new bidirectional persistence tail | Fail | Candidate worsened `13` robust cells vs activated default. |
| Conservation closes | Pass | Candidate trace rows `159986`; max snow-state residual `4.440892098500626e-16 m`; max partition residual `5.551115123125783e-17 m`. |
| No fixture fitting | Pass | No class threshold or parameter was derived from fixtures. |
| Default and rollback preserved | Pass | No-env default remains unchanged; rollback selector remains available. |

## Protected Boundaries

No fixture, public output schema, density cap, frost behavior, parser/runfile/user
CLI selector, `.run` control, Qwet/frzftp, compatibility runtime, melt, phase,
canopy, or radiation change is authorized by this package.

## Disposition

`HOLD-GATE-FAILURE-NON-PROMOTION`.

No activation is authorized. The source authority gap is closed, the candidate is
available only through the package-bound opt-in selector, and the no-env default
plus rollback path remain unchanged. The observed-data rubric is the validation
authority, and this rerun shows the comprehensive Sturm climate-class density
candidate does not beat the current default.

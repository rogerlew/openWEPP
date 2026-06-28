# Snow-Density Paradigm Comparison

Evidence class: Static.

## Decision Frame

The current no-env default scores `15` robust fails / `179` on the
cross-SNOTEL forcing-robust rubric. SNOWDENSITY-10.3.21 decomposes the remaining
robust fails as:

| Residual cluster | Robust fails | Character | Mechanism signal |
| --- | ---: | --- | --- |
| Seasonal densification trajectory | 9 | Diffuse across climates, split sign | New density structure or regime-adaptive density trajectory |
| Mountain timing under-persistence | 4 | SNOTEL mountain timing | Wind redistribution or forcing representativeness |
| Humid-New-England depth-SWE geometry | 2 | `cancov_forest` continuity cluster | Canopy interception/sub-canopy longwave |

The package therefore scores options by the dominant density-trajectory residual
first, while preserving ADR-0028 constraints: defensible physics, forcing-robust
rubric improvement, no fixture fitting, conservation, and comparators as flags.

## Summary Matrix

Scores are relative to the next snow-density package, not to an unconstrained
research program.

| Dimension | Paradigm 1: climate-class specialization | Paradigm 2: multilayer physics | Baseline: accept current floor |
| --- | --- | --- | --- |
| Fit with current implementation | High | Low | Highest |
| Modeling-philosophy fit | Medium-high | High | Medium |
| Long-term robustness | Medium-high | High | Medium-low |
| Cost / effort / risk | Low-medium | Very high | Very low |
| Expected payoff on residual | Medium-high | High but delayed | Low for snow, useful for frost schedule |
| Overall next-step value | Best first candidate | Escalation path | Parallel frost-threshold path, not density remediation |

## 1. Fit With Current Implementation

| Option | Assessment |
| --- | --- |
| Paradigm 1 | Fits the scalar density lane. `09_snow_density.rs` already selects model-specific constants and updates one mass/density state, so a climate-class candidate can be an opt-in model that changes constants or coefficient multipliers by class while preserving the current SWE normalization and density cap. It does need a clean class source and trace field, but it does not require layer persistence or output schema changes if class is internal/diagnostic only. |
| Paradigm 2 | Conflicts with current scalar snow state. Crocus/SNOWPACK/SNTHERM-style physics would need layer mass, density, temperature, liquid, grain/metamorphism, and layer collapse. `DirectSnowLaneState` currently carries one SWE/depth/density tuple, so this is a runtime representation and consumer-projection project. It also expands ADR-0025 performance and closure scope. |
| Baseline | Requires no implementation work and preserves all current defaults and rollback selectors. It leaves the known split-sign density residual in place. |

## 2. Modeling-Philosophy Fit

| Option | Assessment |
| --- | --- |
| Paradigm 1 | Admissible, but only in the clean ADR-0028 form. Sturm 1995/2010 provide independent regime classes and class-specific density behavior; openWEPP should not fit the raw depth+DOY regression to SNOTEL/cancov fixtures. The process-consistent form is to make Anderson/SNOBAL-style compaction coefficients regime-adaptive by independently assigned snow class. |
| Paradigm 2 | Strongest process-first fit. It resolves the vertical gradients, layer loading, liquid state, and metamorphism variables that the residual suggests a bulk curve cannot represent. The cost is that detailed snow models remain parameterized and would need contract-first state semantics before implementation. |
| Baseline | Fits truthfulness and scope discipline if framed as a floor, not as a solved density problem. It supports frost-attribution scheduling, but it does not answer the package's density-structure question. |

## 3. Long-Term Robustness

| Option | Assessment |
| --- | --- |
| Paradigm 1 | More robust than one global bulk curve because it allows maritime, alpine, taiga, tundra, prairie, and ephemeral packs to follow different densification trajectories. It is still a class-based statistical abstraction, so it can be brittle near class boundaries or under climate regimes that shift outside the original classification envelope. NSIDC-0768 or local weather-driver classification can reduce site identity dependence. |
| Paradigm 2 | Best theoretical robustness because density emerges from temperature gradient, overburden, liquid water, and metamorphism rather than a preassigned class. It also creates reusable structure for future frost insulation, canopy snow, albedo, and layer-specific cold-content questions. The maintenance burden is much higher. |
| Baseline | Robust operationally because it is already validated as the current floor. Scientifically, it preserves a residual in frost-critical variables and gives no path to improve density-trajectory transferability. |

## 4. Cost / Effort / Risk

| Option | Assessment |
| --- | --- |
| Paradigm 1 | Low-to-medium implementation cost. Main risks are authority translation and class assignment: raw Sturm regression is empirical, and class selection must not become hidden site calibration. A focused opt-in candidate can test whether class-aware coefficients improve the primary rubric before any default decision. |
| Paradigm 2 | Very high cost and high integration risk. It requires new state, new conservation ledgers, layer-to-scalar projections, performance gates, trace evidence, and likely more than one work package before the cross-SNOTEL rubric can be run. It is the right escalation only if a cheap class-aware candidate fails or if frost/canopy decisions require layer physics explicitly. |
| Baseline | No engineering risk. Scientific risk is carrying density/depth uncertainty into frost attribution and later discovering that frost conclusions depend on exactly the unresolved density structure. |

## 5. Expected Payoff

| Option | Assessment |
| --- | --- |
| Paradigm 1 | Directly targets the dominant `seasonal_densification_trajectory` residual. The split-sign pattern is exactly what climate-class specialization is designed to address: one pack family needs less densification, another needs more. It is unlikely to solve the mountain timing or cancov geometry clusters by itself. |
| Paradigm 2 | The most complete answer to density structure and may also help layer-dependent frost insulation. It can represent gradients and internal structure that class coefficients cannot. Payoff is high but not immediate because the path first spends down architecture and validation debt. |
| Baseline | Payoff is schedule and clarity, not snow-density improvement. It allows frost threshold work to proceed with bounded uncertainty, consistent with 10.3.21's mixed read, but it knowingly leaves the dominant residual untreated. |

## Comparison Result

Paradigm 1 is the best first snow-density candidate because it is the only option
that simultaneously:

- targets the dominant split-sign density-trajectory residual;
- can be implemented as an opt-in extension of the current scalar bulk lane;
- has independent regime authority in Sturm 1995/2010 and NSIDC-0768;
- can fail cheaply on the same cross-SNOTEL forcing-robust rubric without
  committing openWEPP to a new multilayer runtime representation.

Paradigm 2 remains the preferred escalation path if Paradigm 1 fails to improve
the rubric, if frost attribution proves sensitive to unresolved vertical snow
structure, or if a future canopy/wind package needs layer-specific snow state.
The baseline floor remains valid for parallel frost-threshold work, but it is
not the recommended snow-density remediation path.


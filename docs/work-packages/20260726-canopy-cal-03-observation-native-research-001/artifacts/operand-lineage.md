# Research Trace Operand Lineage

Evidence class: `Prospective, before production edits`

All fields are diagnostic copies of authoritative production state or consumer
operands. They never feed simulation state.

| Field group | Units/basis | Producer authority | Classification |
| --- | --- | --- | --- |
| GSI indicators, instantaneous GSI, GSI21 | fractions; lane/day | `openwepp-plant-phenology` daily result | authoritative diagnostic copy |
| structural, evergreen, deciduous, total foliar, total live biomass | kg/m2 hillslope surface | native canopy realization | authoritative diagnostic copy |
| LAI, canopy, leaf-on, leaf-off | m2/m2, fraction, kg/m2/day | native canopy realization | authoritative diagnostic copy |
| snow canopy | fraction | day-input snow handoff | authoritative consumed operand |
| interception LAI/cover/live biomass and interception | m2/m2, fraction, kg/m2, m/day | interception compute input/state | authoritative consumed operand/result |
| ET LAI/cover | m2/m2, fraction | direct day frame ET inputs | authoritative consumed operand |
| litter, aggregate residue, decomposition loss, residue depth | kg/m2/day, kg/m2, m | decomposition input/result; loss independently reconstructed from state and fluxes | authoritative operands plus derived diagnostic |
| frost residue depth | m | frost compute input/day frame | authoritative consumed operand |
| runoff canopy/cover operands | fraction | direct hydrology/erosion day frame | authoritative consumed operand |
| shadow current/previous/old cohorts | kg/m2 | analysis-only recurrence from exact litter and declared decay | derived diagnostic |

Normalization is per hillslope horizontal surface area. Calendar identity is
the simulation date plus zero-based day/lane indices. Null means the named
consumer/process was inactive or inapplicable; it never means zero. Non-finite
values and serialization/I/O failures are errors.

Independent acceptance reconstructs daily live-plus-residue mass movement and
the shadow-cohort sum from retained source operands. Equality between copied
aliases alone is only wiring evidence, not mass closure.

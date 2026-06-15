# Numeric Equivalence

Static: production edits moved existing reads and assignments into private
helpers without changing arithmetic expressions, constants, threshold values, or
field assignments.

Static: no aggregation formula, denominator, area-weighting expression, unit
conversion, optional default, or alias mapping was changed.

Static: independent operand reconstruction is intentionally out of scope for
this behavior-preserving package because no publication formula or operand
lineage changed.

Ran: focused tests verify representative read-path values before and after the
refactor, including `Area`, `P`, `RM`, `Total-Soil`, `SoilWaterTotal`,
`ProfilePorosityCap`, `Tile`, `Irr`, optional all-null defaults, and invalid
area failure.

Ran: full workspace tests passed after the refactor.

Disposition: numeric equivalence preserved for the current-scope refactor.

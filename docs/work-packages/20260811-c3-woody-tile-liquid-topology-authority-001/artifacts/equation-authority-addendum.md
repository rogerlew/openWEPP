# Equation Authority Addendum

Status: `selected`

Evidence mode: `Static`

V2 retains V1 E01--E22 equations, fixed constants, numerical algorithms, and
typed exclusions. The amendment supplies the missing execution topology:

- E01--E03 remain tile-column local.
- E04 is evaluated per occupancy with complete initial/second drainage.
- E05--E15 consume occupancy-local radiation, wetness, forcing, warm starts,
  and authorization caps; nonlinear inputs are never pre-averaged.
- The potential column is solved top-to-bottom, its stand-basis water requests
  are arbitrated once, and the final capped column is rebuilt top-to-bottom from
  the original beginning state. Descendants consume final upstream release;
  authorization is immutable during that rebuild and any failure rolls back
  every owner.
- E16--E22 execute once per shared stratum after
  `GPP_s=sum_t(f_t*GPP_s,t)`,
  `R_leaf,s=sum_t(f_t*R_leaf,s,t)`, and finalized transpiration aggregation.
  Shared maintenance, turnover, allocation, and growth respiration execute
  once. Mineral-N requests remain keyed by stratum, layer, and species after
  occupancy aggregation; C/N pools are not duplicated by tile.

The only new mathematical mappings are explicit area transformations by
positive `f_t`, exact conditional area by `C_s`, and weighted linear aggregation
after local nonlinear solutions. These are `OPENWEPP_CANONICAL_SELECTION`, not
new empirical parameters.

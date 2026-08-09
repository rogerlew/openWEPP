# Initial State Authority

Status: `executed / BLOCKED`

Evidence mode: `Static + reused primary-source evidence`

Ford-era Coweeta observations provide partial dated peak LAI, basal area,
density, DBH, height, sapwood area, and leaf area for WS18 oak and adjacent WS17
pine. They do not provide a common area/date/topology or a compatible complete
state containing layer root fractions plus leaf, fine-root, livewood, deadwood,
storage, and transfer C/N pools.

The GIS initializers are rejected as authority: they use fixed row positions,
hard-coded `333.33` deadwood C:N and `0.05` allocation, unproved root depths,
and nonfinite-to-zero behavior; initializer and runtime SLA identities can
diverge. No allometry, carbon fraction, stoichiometry, cross-site composition,
or temporal transfer operator with uncertainty closes those gaps.

Observed operands remain `DIAGNOSTIC_ONLY`; no transformed or synthesized
state is admitted. `AUTH-RHEC-015` is blocked. The minimum lift is the target
route in `target-boundary-selection.md` plus independently reconstructible LAI,
geometry, root, and C/N mass vectors.

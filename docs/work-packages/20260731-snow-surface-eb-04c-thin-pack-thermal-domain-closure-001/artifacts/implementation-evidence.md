# Implementation Evidence

Evidence class: `Static + Ran`

The real Stage 3 production loop checks total ice mass before active/lower
partition. At total mass `<=1 kg m^-2`, it advances no thermal substep and
records unresolved duration/mass. For resolved total mass, it selects the
active/lower partition; a lower volume strictly `<1 kg m^-2` becomes a
whole-pack one-volume solve for the substep and publishes collapse
duration/mass. Exact lower equality remains coupled and two-volume.

Persistent layer mass, liquid, refrozen mass, and cold content remain owned by
the existing CoE/projection and liquid-routing paths. The suspended branch does
not clamp temperature, add a vapor-pressure epsilon, convert remnant snow to
melt, clear cold content, or introduce a user coefficient.

The real runner trace publishes:

- `stage3_thermal_domain_suspended_seconds`; and
- `stage3_minimum_unresolved_thermal_mass_kg_m2`.
- `stage3_lower_thermal_volume_collapsed_seconds`; and
- `stage3_minimum_collapsed_lower_mass_kg_m2`.

The exact 22-case production replay confirms that this consumer, rather than a
test-only or wrapper path, reads the branch.

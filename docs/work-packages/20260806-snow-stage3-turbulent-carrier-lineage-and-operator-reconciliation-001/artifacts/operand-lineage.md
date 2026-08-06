# Operand Lineage

Status: `result-blind draft / review required before implementation`.

Evidence class: `Static`.

All energy terms use `J m^-2` over explicit evaluated support or `W m^-2` as a
support-weighted mean. Mass uses `kg m^-2`; temperatures use degrees Celsius;
pressures use pascals; lengths use metres; velocities use `m s^-1`; specific
humidity uses `kg kg^-1`.

The authoritative source is the immutable post-CoE daily snapshot. Evaluation
state and all new fields are diagnostic only. The same-state operator's
endpoints must equal its source state. Sequential endpoints belong only to its
within-call clone.

Every external term is reconstructed as shortwave, longwave, sensible, latent,
or precipitation-advection. Internal active/lower conduction is separate.
Positive energy and vapor mass point toward snow. Known rejected aliases are
production Stage 3 temperature/energy, CoE melt terms, internal conduction as
snow-ground heat, calendar support, zero-filled N/A stability, and any adjacent
producer total used instead of consumer operands.

The reviewed implementation freeze must expand this artifact into a field-level
table before Rust edits.

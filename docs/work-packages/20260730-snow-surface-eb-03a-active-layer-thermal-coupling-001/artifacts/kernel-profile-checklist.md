# Kernel Profile Checklist

Status: `complete / pass`

Evidence mode: `Static + Ran`

| Kernel obligation | Disposition |
| --- | --- |
| A0 conservation | PASS: independent active, lower, whole-pack, vapor/latent, and liquid ledgers |
| A1 hard invariants | PASS: typed finite/domain guards; `T > -273.15 deg C`, `T <= 0 deg C`, positive pressure/conductivity/resistance |
| A3 constitutive relationship | PASS: exact libsnobal `KTS+efcon`, harmonic interface conduction, and fixed active-depth/timestep authority |
| Default protection | PASS: absent, empty, and explicitly disabled WAT and trace hashes are identical |
| Real consumer | PASS: direct-production B/L/S/LS all execute the active/lower path |
| Surrogate prohibition | PASS: no clamp, fitted limiter, air-temperature snow-state replacement, remote-data requirement, or new coefficient |
| Output closure | PASS: same-substep operands independently reconstruct `G_0` and resistance |

The candidate remains behind the existing default-off selectors. CoE melt,
phase, density, frost, fixture, and public-schema boundaries are unchanged.

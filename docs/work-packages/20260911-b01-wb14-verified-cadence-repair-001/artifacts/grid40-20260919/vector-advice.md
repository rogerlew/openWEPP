# Fixed noncaptured contract-vector advice

Attributable adviser: `/root/grid40_vector_advisor` (Astra/high). Root preserved
this transcription because the adviser role is read-only. It is methodological
advice, not independent acceptance. Actual Rust verification remains pending.

Static: inspected detached covered solve, existing V8 fixture and affected
thermal/inactive-coordinate authority. No source edits, captured-case evaluation
or parameter sweep. Ran: two independent reduced thermal calculations using
`.venv/bin/python`, both exit0. Two mistaken chapter-path lookups failed and were
resolved; included in the orchestrator's floor386.

Clone the noncaptured `covered_v8_block_matches_frozen_joint_solution` column,
retaining interval, thermal parameters, resistances, biochemical constants and
positive configured under-canopy geometry. Apply:

- Authority `V10NonpositiveAssimilation`; caps None.
- Occupancy physical sun/shade/stem areas, LAI, SAI, beginning canopy liquid,
  absorbed shortwave/PAR, top rain and ground terminal shortwave: positive zero.
  Rebuild consistent bound shortwave. Ground liquid zero.
- Keep configured `under_canopy_geometry.leaf_area_index=2.708333333333333`:
  aerodynamic configuration separately requires positive LAI. This tests the
  internal covered-column boundary, not a new runtime owner configuration.
- T=`273.15 + 2^-26 = 273.15000001490114` K. All eight temperature coordinates,
  soil beginning temperatures, column/ground air and ground warm start equal T.
- Ground enthalpy `3235.68 * (T - 273.15)`; column/ground atmospheric longwave
  `STEFAN_BOLTZMANN_W_M2_K4 * T.powi(4) - 1.0`.
- Four hydraulic potentials zero; both beta coordinates one. Canopy humidity
  and column/ground atmospheric humidity `0.001`.

Zero litter water gives zero relative humidity/ground vapor. Zero physical
occupancy areas remove canopy exchanges. Equal current/beginning temperatures
remove storage, conduction and sensible heat at the base. Every residual starts
at zero except ground energy, -1 W/m². Canonical inactive hydraulic replacement
and V10 scaling remain active: `v10_nonpositive_assimilation_active` includes
Inactive. No direction injection or changed physics.

Independent four-coordinate thermal finite-difference reconstruction (inward
ground stencil, changing normalizers) predicts:

| Quantity | Value |
| --- | ---: |
| Ground-canopy resistance s/m | 16.909963441236744 |
| Base infinity norm | 999900.0099990001 |
| Canopy delta K | -0.012949661580024889 |
| Ground delta K | -0.02338102809708071 |
| First soil delta K | -0.0006902664361870094 |
| Second soil delta K | -0.00003194781541959511 |
| b20 ground distance above lower bound K | -7.396749879262643e-9 |
| b21 ground distance above lower bound K | 3.752234079001937e-9 |
| b21 infinity norm | 999899.5330798278 |

Baseline must refuse at iteration zero; treatment must install its first step
at b21. Assert actual recorded direction, domain decisions, strict decrease and
next base. Full synthetic-solve acceptance is not required or expected. Freeze
fixture/assertions before changing control C's actual hard20 callsite to T.

Exact-bound variant T=273.15 predicts outward refusal through40 and a wholly
duplicate-rounded b40 trial, rejected for equal norm. Verify unchanged failed
solution and exhaustion contribution40 in Rust. Residuals must remain far above
tolerance despite tiny steps. This does not replace positive first-valid
witness/no-install, nonfinite/error or confinement checks.

Inspected source hashes at adviser return:
`solver_covered_solve.rs`: e728e32ece5555070034617ce26a1832d5d65ae23e0a1dab3309fff42c367d21;
`solver_tests.rs`: d6c1f78c0d5e11970848bac06133decff34ffcd3699f483d3cd8f2e0e9a1fb25.
These identify the advisory inspection, not the eventual candidate.

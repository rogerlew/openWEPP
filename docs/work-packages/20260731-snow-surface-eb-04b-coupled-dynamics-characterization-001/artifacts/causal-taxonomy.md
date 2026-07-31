# EB-04B Causal Taxonomy

Evidence: `Ran + Static`

## Thermal Family: 22 Cases

Proximate mechanism: `PROVEN`.

Every rejected control-volume slice contains one layer. Its ice mass is only
`1.2003e-7` to `1.0143e-5 m` SWE while positive cold content remains `79.13`
to `6056.36 J m^-2`. Independently evaluating

`T = -Q_cc / (SWE rho_w c_i)`

reproduces all 22 typed temperatures exactly. Seventeen states contain `1.015`
to `11.355` times the cold content compatible with the `0 K` boundary. Five
contain `0.975` to `0.992` times that boundary and are rejected slightly
earlier when ice saturation vapor pressure underflows.

The rejected slice mass is only `1.022e-6` to `5.957e-2` of the preceding
successful day's complete-pack SWE; the median ratio is `2.769e-5`. This
comparison establishes scale, not continuity between differently labeled
control volumes.

Producer ordering: `PROVEN`.

At each substep the production path constructs and validates the current
control volume, then evaluates the surface carrier, applies conduction, applies
surface energy, and finally removes sublimated mass. Negative energy can add
cold content without a physical-domain bound. The next substep's initial
control-volume construction is the first point that needs conductivity and
rejects an already-invalid or numerically unsupported temperature. The current
failing substep's flux is therefore not evaluated. Retained evidence does not
identify whether negative energy, mass depletion, or their coupled ordering
crossed the boundary during the unpublished preceding substeps.

Sublimation role: `STRONG ASSOCIATION AND MECHANISTIC CONTRIBUTOR; TERMINAL
AMPLIFICATION INCONCLUSIVE`.

Twenty thermal failures are S or LS, and all 20 carry prior-day sublimation
mass export together with negative latent energy. Two are L-only contrary
cases and reach the same terminal domain without the new sublimation selector.
The unpublished boundary-crossing substep prevents a causal amplification
estimate. A coefficient adjustment or sublimation-only patch would therefore
be unsupported and would not address the complete population.

Conductivity role: `DOWNSTREAM SENTINEL`.

For the 17 below-absolute-zero cases, the constitutive conductivity equation
does not generate the invalid state; typed temperature construction rejects
upstream. The five underflow cases are different: their temperatures remain
above `0 K`, but the SNOBAL ice-vapor-pressure term numerically underflows to
zero inside the conductivity dependency. EB-04C must determine whether that is
a valid-but-unsupported constitutive domain, a numerical formulation limit, or
a state that authoritative thin-pack treatment should never present.

Root-cause class:
`POSITIVE_COLD_CONTENT_ON_VANISHING_MASS_WITH_UNRESOLVED_CROSSING_DRIVER`.

The current implementation follows the contract's ordering and conservation
ledger, but the contract does not define the physically authoritative outcome
when a vanishing CoE-retained pack approaches the temperature or vapor-pressure
domain. Negative surface energy and sublimation/mass depletion are authority
hypotheses for EB-04C, not terminal drivers proven here. This is not evidence
for a clamp, temperature replacement, coefficient cap, or guard relaxation.

## Geometry Family: Two Cases

Proximate mechanism: `PROVEN`.

Both failures contain exactly one fragment whose SWE is about `5.26e-10 m`.
The production reconstruction excludes it using `mass_swe_m > 1e-9 m`, but the
fragment depths are `1.0078e-9 m` and `1.0882e-9 m`. Those depths exceed the
separate `1e-9 m` depth-closure tolerance because snow density is `522.0` and
`484.1 kg m^-3`, not water density.

The reported depth residual equals the excluded fragment depth in both cases.
The same fragment's SWE remains below the water-depth tolerance, explaining why
the mass aggregate passes while physical depth fails.

Root-cause class:
`DIMENSIONALLY_INCONSISTENT_FRAGMENT_FILTER_AND_DEPTH_CLOSURE`.

This is a layer-bookkeeping/numerical-boundary defect, not material conservation
loss. It does not justify silently increasing the depth tolerance.

## Contrary Cases And Limits

- All 22 paired B cells complete and reach the day before the corresponding
  thermal rejection. This rules out forcing absence and a universal baseline
  failure.
- Two L-only failures rule out sublimation as a necessary cause.
- Failures from day 13 through day 12,517 rule out a startup-only cause.
- Complete chronology quantifies active/lower mass, depth, conductivity,
  resistance, requested/applied/rejected `G_0`, and cadence. The terminal error
  payload does not label whether its one-layer slice is active or lower, so
  terminal lower-layer availability is `INCONCLUSIVE`, not inferred.
- The exact energy component that crosses the boundary inside the rejected day
  is not retained because validation occurs before the next carrier evaluation.
  The common state/ordering mechanism is proven; failure-substep attribution is
  intentionally not fabricated.

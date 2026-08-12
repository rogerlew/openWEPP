# Numerical Solver And Convergence Contract

Status: `selected`

Evidence mode: `Static`

All tolerances below are version constants, not site parameters. Comparisons
use `abs(r) <= atol + rtol*scale` with the stated physical residual scale.

| Solve | Algorithm | Bracket/domain | Tolerance | Limit/failure |
|---|---|---|---|---|
| quadratic roots | cancellation-safe smaller-root formula; linear limit when the exact quadratic coefficient is zero | discriminant nonnegative within `64 epsilon * max(b^2,|4ac|)` | algebraic roundoff only | negative discriminant: `VEG-E-NUM-001` |
| radiation defining integrals | adaptive Simpson with deterministic left-before-right recursion for both `mubar` and direct upscatter | `[0,1]`; removable zero denominators use analytic zero contribution | absolute `1e-14` | 20 bisection levels; `VEG-E-NUM-006` |
| leaf `ci` | Brent-Dekker with bisection safeguard | `[Gamma*, c_a]`; endpoints evaluated explicitly | `atol=1e-6 Pa`, `rtol=1e-10`, residual `1e-8 umol m-2 s-1` | 64 evaluations; `VEG-E-NUM-002` |
| leaf/wet/dry-stem temperatures + canopy-air nodes | damped Newton; centered finite-difference Jacobian step `sqrt(epsilon)*max(abs(x), unit_scale)`; pivoted LU; accept only strict infinity-norm residual decrease, halving otherwise; active wet-store cap stays inside the residual | liquid branch `273.15<=T<=373.15 K`, positive conductances/vapor domains; the bounds are the selected CLM liquid-polynomial domain, not a clamp | energy `1e-6 W m-2 + 1e-10*scale`; temperature step `1e-8 K` | 50 Newton steps, 20 halvings; pivot `<64 epsilon matrix_norm` or limit: `VEG-E-NUM-003` |
| hydraulic potentials/complementarity | damped semismooth Newton on the four CLM continuity equations plus each cap residual `r_i=q_i-min(A_i/dt,q_law_i(psi))`; centered finite-difference generalized Jacobian, pivoted LU and strict-decrease halving | finite potentials, positive vulnerability/path conductance, `0<=q_i*dt<=A_i`; any negative solved law flux is typed unsupported redistribution | water residual `1e-12 mm s^-1 + 1e-9*scale`; potential step `1e-7 mm` | 50 steps, 20 halvings; redistribution, singular, or nonconverged `VEG-E-NUM-004` |
| coupled outer system | one simultaneous residual vector for leaf `ci`, temperatures, canopy-air nodes, `beta_hyd`, equality of gas/energy and hydraulic transpiration, and hydraulic complementarity; an equivalent nested solve must converge the identical residuals and may not accept a relative mismatch | all component domains, `0<=beta_hyd<=1`, and one immutable forcing/state snapshot | every normalized component residual <=1, water-flux equality uses the hydraulic tolerance, and every step tolerance is met | 50 steps; `VEG-E-NUM-005` |

No last iterate, fallback flux, clamped exponent, conductance floor, zeroed
negative store, or partially updated state is admissible. Every solve operates
on candidate state. Any failure returns the typed error plus residual/iteration
diagnostics and leaves all owner-state bytes identical. Stable evaluation uses
`expm1` for small exponential differences and log-domain evaluation for peaked
temperature response; this changes numerical evaluation, not the equation.

For a quadratic discriminant in `[-64 epsilon*scale,0)`, the numerical
representation is treated as exact zero; more-negative values fail. The
`ci` residual is
`r_ci=ci-[ca-(1.4 rb+1.6 rs)R T An(ci) 1e-6]` for `An` in micromoles;
equivalently convert `rb/rs` from `s m^-1` through
`g_mol=g_ms Patm/(R T)` before forming the partial-pressure drawdown.
Alternate initial guesses are required to converge to the same solution within
the stated tolerances. All limits/tolerances are
`OPENWEPP_CANONICAL_SELECTION` constants chosen below double-precision
roundoff yet above platform noise; successor sensitivity vectors must repeat at
one-half/twice tolerance without branch or ledger changes.

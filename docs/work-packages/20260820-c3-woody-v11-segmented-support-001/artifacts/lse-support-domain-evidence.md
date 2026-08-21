# LSE positive-support evidence (Phase 1/2)

Base: `464cd506ad2fa789cc68a22e969646be639b50df` (clean, synchronized
`main`/`origin/main` at intake). The prior actual-stack 1 ns failure is retained
in `tiny-support-lse-authority-blocker.md`; no V10 source, tolerance, or
physical parameter was changed.

## Observed solver boundary

The covered actual V10 stack at 1 ns enters `OPENWEPP_SNOW_FREE_LSE_V1`,
`LsebE034 / NumericalIterationLimit`: 50 Newton iterations and 736
backtracks, matrix infinity norm `2.485478572575206e15`, pivot `108.64`.
The complete residual/step/backtracking trajectory, beginning identities and
configuration identities are retained in the blocker artifact and solver
diagnostics. A root-preserving pivot preconditioner changed the pivot to
`108639` but did not change the trajectory or acceptance, so no tuning is
admitted.

The deterministic boundary sweep used the ignored test
`v11_support_domain_evidence_sweep` and the actual covered forest fixture.
The 0.6 s and 0.601 s cases pass; the 0.06 s neighbourhood is
non-monotone (some isolated supports pass and others fail), and 1 ns fails.
This is why the released policy is a conservative fixed domain boundary rather
than an inferred last-successful duration. The same sweep command and raw
output are reproducible with:

```text
nix develop -c cargo test -p openwepp-hillslope-orchestrator \
  v11_support_domain_evidence_sweep -- --ignored --nocapture
```

The declared policy is `minimum_support_ns = 600000000` for the V11 actual
covered-forest adopter profile only. Open-mineral, litter, wet/dry and other
profiles are explicitly non-admitted by this package and require their own
profile-and-review cycle. It is an explicit admission boundary, not a hidden
solver floor; below-domain support is rejected before Newton and is never
retried at the boundary.

## Representation analysis

For binary64 temperatures in the declared 200--350 K LSE domain, adjacent
temperature spacing is `2.842170943040401e-14` K at 200 K and
`5.684341886080802e-14` K at 291.5--350 K. Representative finite-capacity
energy quanta (`C * ulp(T)`) and support needed to exceed the existing
`1e-6 W m^-2` absolute energy tolerance are:

| Areal capacity (J m^-2 K^-1) | Energy quantum (J m^-2) | Resolution support (s) |
|---:|---:|---:|
| 3,235.68 | 1.83927e-10 | 1.83927e-4 |
| 42,000 | 2.38742e-9 | 2.38742e-3 |
| 120,000 | 6.82121e-9 | 6.82121e-3 |
| 180,000 | 1.02318e-8 | 1.02318e-2 |
| 200,000 | 1.13687e-8 | 1.13687e-2 |

The independent storage-lattice floor is therefore below the observed
nonlinear transition. The 0.6 s policy includes a deterministic solver-domain
margin for the executed covered-forest adopter without changing residual
tolerances or constitutive equations. Open-mineral, litter, wet/dry and other
surfaces are not admitted by this package and require a separate authority
cycle. A future smaller or state-qualified domain likewise requires a new
authority cycle.

## Decision

Nanosecond chronology remains valid in coupled time. The physical adopter has
an explicit positive-support domain, a sealed support-admission receipt, and a
typed pre-Newton rejection. Event-boundary coalescing is deferred to the
independently reviewed snow/event contract and must preserve event-time, mass,
and energy bounds.

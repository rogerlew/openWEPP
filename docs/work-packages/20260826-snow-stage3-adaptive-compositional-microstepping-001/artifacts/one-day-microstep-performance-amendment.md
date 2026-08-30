# Canonical one-day microstep performance amendment

Evidence mode: `Ran + Static`.

Owner amendment date: `2026-08-28`.

## Objective and frozen work

The active package objective is the real canonical one-day adaptive workload.
Seasonal/archive qualification, archive representation, memory residency, and
general per-step optimization remain paused unless they are necessary to reduce
accepted/rejected microsteps. The pre-correction blocker is:

- 48 half-hour parents;
- 1,434 accepted adaptive microsteps and 1,435 retained publication supports;
- 1,529 rejected trials;
- accepted widths `60 s x 1,433` and `420 s x 1`;
- rejection classes: phase `1,403`, event `0`, other `126`;
- fixed-point evaluations/iterations/cap: `2,814 / 131,765 / 64`;
- body/wall time: `485.858 s / 507.67 s`.

The retained source log is
`/tmp/runner_cutover_telemetry/archive-v3-one-day-take-order.log`. It is the
comparison baseline, not current closure evidence.

## Diagnosis

Ran: default-off, in-process audit of the real covered owner showed that
supports at and above 120 seconds repeatedly entered an exact stable period-2
outer iteration between the coupled Stage-3 and soil-thermal images. Reaching
iteration 64 therefore meant cap exhaustion of a noncontracting Picard map,
not a support that was converging too slowly. Exact 60-second supports normally
converged in 33--45 iterations. No audit record is serialized, added to restart,
or written by production.

Ran: comparison audit on the unmodified adaptive controller recorded 152
direct-versus-composed field comparisons in the first five parents. The 126
soil-thermal mismatches were normally `0.0003--0.012 K` temperature truncation
error and `0.2--0.3%` energy truncation error. The 26 snow-owner mismatches
included one phase-threshold discontinuity of about `0.913 K`; that discrete
transition remains rejecting and must refine.

Ran: after later receipt-custody corrections, the canonical run regressed to
1,336 accepts and 1,426 rejects. A five-parent audit first exposed and corrected
an audit-only classification error (`ReceiptLineage` was reported although the
production comparator already excluded it). The corrected audit then found
87/87 exact mismatches at only
`surface_liquid.wb14_parent_working_state.per_ofe_authorities.*.receipts`.
The supposed `ReceiptOrdering` value was the map's 64-hex receipt digest keys:
transaction-local child-factorization identity, not physical order.

## Corrections

Static: for covered supports greater than 60 seconds, the coupled Stage-3 and
soil-thermal outer iteration now advances

```text
x_(n+1) = x_n + w * (F(x_n) - x_n)
w       = min(0.5, 120 seconds / support)
```

and tests convergence using the full residual `x_n` versus `F(x_n)`. The
60-second constitutive fallback remains raw by default. Only after authentic
Stage-3 iterates form an exact-discrete/native-unit `A/B/A` period-two cycle
may the unpublished floor solve use `w=0.5`; event, topology, density, and
other exact-surface changes cannot trigger or enter that blend. The relaxation
blends only continuous trial state. Structure, layer counts, density,
identities, initial conditions, active-set/phase posture, receipts, and
published owners remain exact. Every relaxed candidate is resealed and
validated before reuse. The fail-closed cap is 96 iterations.

Ran: the first exploratory full-state implementation converged a 120-second
support in 27 iterations and a 480-second support in 75 iterations, but
independent review rejected it because its unpublished soil digest and
Stage-3 thermodynamic/custody joins were incomplete. That result is diagnostic
only and is not qualification evidence.

Ran: after correction, the guarded real fixture passes. Same-posture
120-second supports converge in 18--25 iterations. The 180- and 420-second
proposals in the transition interval straddle the exact `1 kg m^-2`
resolved/terminal active set and therefore refine at the typed 96-iteration
cap; no owner state is interpolated across that phase boundary. The separate
stable 1800-second production fixture remains the ordinary-support proof.

Static: the exact Stage-3 lower-boundary/column operand-join closure error was
added to the existing typed trial-local numerical/refinement set above the
floor. At the 60-second floor the same failure is returned as typed failure.
Other component-closure, topology, custody, receipt, and conservation errors
remain non-refinable.

Static: an under-relaxed Stage-3 iterate is admitted only when schema,
terminal model, layer cardinality, settling, initial state,
precipitation/external-liquid inputs, and resolved/terminal/dormant posture are
exactly equal on both sides and the result preserves every per-layer
represented-mass predicate. Density is never blended: the authentic candidate
bit pattern is retained, and acceptance still requires exact density equality.
Layer temperature and thickness are recomputed from blended extensive
mass/cold content and that exact density. Cumulative water and energy are
independently reconstructed. Soil transaction lineage is
exact and both nested digests are canonically resealed. Each snow--soil heat
receipt is joined to the actual iterate's snow serialization and soil OFE
before its exact equal/opposite credit can be used. Any failed guard declines
relaxation and retains typed refinement; accepted publication is still an
exact raw-candidate replay.

Static: final `SnowSoilHeatReceiptV1` authority distinguishes the causal
receipt from its installed-endpoint convergence audit. The retained receipt
keeps bit-for-bit the energy actually consumed by both solvers, including the
exact equal/opposite snow debit and soil credit. Independent reconstruction
from the installed endpoint temperatures must differ by no more than
`1e-9 J m^-2` and `1e-8 K`; only then may the causal receipt reseal to the
exact installed candidate identities. Replaying the identity-only reseal must
reproduce both installed owners byte-for-byte. A nonfinite or larger residual
consumes another bounded iteration; exact-60 retries remain raw and cap
exhaustion remains fail-closed. These bounds are `TOL-SNOWENERGY-005`, do not
alter the applied receipt energy, and leave the independent
`1e-6 J m^-2` ledger-closure threshold unchanged.

Ran: the bitwise endpoint fixed-point audit stagnated with authentic
floating-point residuals of approximately `4e-11` to `1.6e-10 J m^-2`; raw
and damped outer retries returned to the same physically converged map image.
The final guarded five-parent real fixture passes in `105.54 s` with maximum
accepted receipt residuals of `9.66338120633736253e-10 J m^-2` and
`1.42108547152020037e-12 K`. Exact threshold-side tests reject the first
binary64 value above either `1e-9 J m^-2` or `1e-8 K`, negative values, and
nonfinite values. The default-off qualification audit emits both maxima and
does not participate in persisted production state.

Static: adaptive complete-owner truncation-error tolerances were changed only
where the audit justified them. Snow/LSE relative energy is `5e-3`, soil
thermal relative energy is `1.5e-2`, temperature absolute error is `1e-2 K`,
snow mass absolute error is `5e-6 kg m^-2`, and the existing exact discrete
predicates remain exact. These are direct-versus-composed controller bounds;
they do not change any constitutive residual or ledger closure tolerance.

Static: the surface-liquid owner finalizer now derives the receiver from the
candidate's effective zero-duration surface-liquid state instead of a stale
frame shadow. Final committed Stage-3 liquid canonicalizes only the exact first
binary64 value above `273.15 K` to the exact reference value, matching the
canonical enthalpy datum. The next value above that, every value below the
reference, and every non-reference temperature are unchanged.

Static: `SC-SURFACELIQUID-001@13` now classifies exactly the WB14 per-OFE
digest-keyed child receipt map and its child ordinal as per-trial
`ReceiptLineage`. Each direct or composed trial still seals and independently
validates its complete exact receipt map, predecessor chain, support ordering,
payloads, and replay digest; only cross-factorization physical comparison
ignores the necessarily different child count/identities. All other receipt
ordering, events, topology, membership, custody, rollback, and fail-closed
poisons remain exact. The post-correction five-parent gate emitted 30
comparisons instead of 94, zero exact-discrete mismatches, and candidate widths
through 1,800 seconds; its body fell from 220.17 to 122.09 seconds.

## Canonical one-day replacement evidence

Ran: the final exact-head canonical one-day fixture passed all 48 parents, the
committed qualification snapshot, downstream publication consumer, archive
fold, and output transaction. The retained stdout and external time logs are
`/tmp/adaptive_microstep_amendment/one-day-final-v16-exact-head.log` and
`/tmp/adaptive_microstep_amendment/one-day-final-v16-exact-head.time`. This
supersedes v12 after the broad gate exposed final-child-only accepted-terminal
precipitation/resource custody. The
earlier 497/206 v7 result predates exact chronology, positive-solid forcing,
accepted-carrier, and partition corrections and is historical only.

| Measure | Baseline | Replacement | Delta |
|---|---:|---:|---:|
| accepted adaptive microsteps | 1,434 | 588 | -846 (-59.00%) |
| rejected trials | 1,529 | 320 | -1,209 (-79.07%) |
| retained publication supports | 1,435 | 1,078 | -357 (-24.88%) |
| fixed-point evaluations | 2,814 | 2,026 | -788 (-28.00%) |
| fixed-point iterations | 131,765 | 54,753 | -77,012 (-58.45%) |
| maximum fixed-point iterations | 64 | 96 | bounded cap changed as specified |
| optimized model/test body | 485.858 s | 420.11 s | -65.748 s (-13.53%) |
| external wall time | 507.67 s warm optimized build | 489.99 s incremental optimized rebuild | compilation-inclusive values are not directly comparable |
| compilation-inclusive peak RSS | not retained in baseline log | 3,935,368 KiB | includes the incremental optimized compiler and is not model-residency evidence |

The accepted-width histogram is `60 s x 139`, `120 s x 111`, `180 s x 320`,
`240 s x 12`, `300 s x 1`, `420 s x 3`, `900 s x 1`, and `1800 s x 1`. It
contains exactly 588 accepted steps and covers exactly 86,400 seconds. Thus
449 of 588 accepted steps (76.36%) are larger than the 60-second fallback
floor.

The controller recorded 180 phase rejections, zero event rejections, zero
combined phase/event rejections, and 140 comparison rejections. The limiting-
cause audit recorded 124 fixed-point nonconvergences and 140 scaled direct-
versus-composed comparison rejections: snow deposition `5`, refrozen liquid
`95`, snow-layer temperature `6`, persistent surface-liquid mass `1`, and
WB14 parent-working surface-liquid mass `33`. Exact-discrete comparison
rejections were zero. These counters are emitted only by the ignored
qualification test and are absent from production persistence and restart.

Ran: independent ledger validation checked 2,037 ledgers. Maximum absolute
mass residual was `3.55271367880050093e-15 kg m^-2` against the unchanged
`1e-9 kg m^-2` threshold. Maximum absolute energy residual was
`1.39698386192321777e-9 J m^-2` against the unchanged `1e-6 J m^-2`
threshold. The causal receipt audit found maximum endpoint reconstruction
residuals of `9.98625182546675205e-10 J m^-2` and
`1.06297193269710988e-11 K`, within `TOL-SNOWENERGY-005`.

Disposition: the owner's primary accepted/rejected-count objective passes,
ordinary stable supports are substantially larger than the floor, and strict
ledger/event/custody publication gates pass. The comparable optimized test
body fell 13.53%; the incremental-build external wall and peak RSS remain build-
inclusive and are not used as solver-speed or residency evidence. The owner
amendment explicitly paused per-step optimization and prioritized the real
step-count blocker; no further archive, memory, or generic per-step
optimization was performed for this disposition.

# EB-04R Prospective Decision Protocol

Status: `FROZEN BEFORE EXECUTION`

Evidence class: `Static`

## Population And Roles

The immutable matrix contains five SNOTEL open controls; Harvard open and
hardwood; Marcell open, deciduous, and conifer; and Sleepers South open plus
Sleepers W9 hardwood diagnostic frost lanes. Each receives B, L, S, and LS
exactly once. The first ten lanes retain `INDEPENDENT_VALIDATION`; both Sleepers
lanes remain `DIAGNOSTIC_ONLY`.

B disables both target mechanisms. L enables only
`dilley_unsworth_subcanopy_v1`. S enables only
`neutral_bulk_stage3_v1`. LS enables both. All cells use
`physics_bulk_multilayer_density_v1`, `harder_pomeroy_hourly`,
`coe_liquid_holding_capacity_v1`, and `layered_thermal_liquid_v1`.

## Sanitized Runtime Environment

Before every subprocess, remove every inherited environment key beginning
`OPENWEPP_`. Install exactly the four non-target selectors, the two target
selectors, and `OPENWEPP_R7H_SNOW_TRACE_PATH`. Record removed key names only and
the exact effective seven-key mapping. Any extra, missing, or mismatched key is
a physical/provenance failure before scoring.

## Physical And Consumer Gates

The independently reconstructed daily snow-mass tolerance is `1e-9 m`.
Surface energy, cold-content energy, and daily/hourly energy identities use
`1e-6 J m^-2`; the hourly latent mass-energy identity also admits the larger
of that absolute threshold and 16 floating-point epsilons of its operand sum.
Daily vapor aggregation uses `1e-12 kg m^-2`; vapor-to-sublimation identity
uses `1e-9 kg m^-2`. WAT-to-trace SWE/depth and complete layer-vector sums use
`1e-9 m`; layer density uses `1e-4 kg m^-3`; layer and aggregate cold content
use `1e-6 J m^-2`. Numeric operands must be finite, hourly vectors must contain
exactly 24 values, represented-layer temperatures must satisfy
`-273.15 deg C < T <= 0 deg C`, and layer mass/depth/density/cold content must
remain in their contracted nonnegative domains. All 48 cells must complete and
pass. No partial or failed cell may be scored. The consumer completes and
records the full 48-cell physical and provenance pass before it calls any
observation loader. Longwave and
sublimation must each be nonzero in all 24 enabled cells; their disabled
counterparts must remain zero. On every row, sublimation is nonnegative, vapor
mass exchange is nonpositive, latent energy is nonpositive, and active
sublimation requires strictly negative vapor and latent terms.

The real retained-output consumer must reject deleted represented fragments,
aggregate-only layer substitutions, and wrong selectors. The independent
energy consumer must reject wrong signs and omitted energy. Both control sets
must pass before the protocol may freeze.

## Observation And Timing Operators

- Use the installed normalized observation bytes and their frozen EB-04 role.
- Retain the exact EB-04 lane IDs, fixture hashes, observation hashes, filters,
  strata, climates, and independent-validation/diagnostic roles.
- Score the canonical `INV-SNOWFREEZE-050` ordinal labels:
  `fail=0`, `marginal=1`, `pass=2`, `strong=3`.
- Snow-present is SWE `>1 mm`. Disappearance is the first day at or below
  `1 mm` followed by seven consecutive days at or below `1 mm`.
- Each runoff timing window is water-year October 1 through the last persistent
  disappearance day, or September 30 if unresolved.
- Ties use the earliest date. Missing observations remain unavailable and are
  not imputed.

## Eight-Part Promotion Rule

Only LS is eligible. The outcome is `GO_TO_EB05_PROMOTION_ASSESSMENT` only if:

1. every LS physical, provenance, and trace gate passes;
2. LS raises the independent-lane forcing-robust ordinal sum over B by at least one;
3. LS reduces the forcing-robust fail count relative to B;
4. no independent lane gains a new forcing-robust fail;
5. neither open-control nor canopy protected-group ordinal sum is below B;
6. longwave and sublimation reach all enabled cells with contracted mass/energy identities;
7. no opposite marginal degradations are hidden by an aggregate combined gain; and
8. no forcing, coefficient, fixture, observation, rubric, or post-result operator changes.

Otherwise, valid complete evidence closes as
`CLOSE_NONPROMOTION_EMPIRICAL_RULE`; a failed physical/provenance gate closes
as `HOLD_PHYSICAL_OR_PROVENANCE_GATE`. A tie is not improvement.

The inherited reducer and a separately implemented package-local reducer must
agree exactly on ordinal sums, fail counts, new failures, protected-group
scores, compensation findings, and final criteria. Disagreement is `HOLD`.

## Freeze And Interruption

`--execute` must run all self-checks, confirm EB-04E remains a recorded PASS,
require clean and content-hashed `crates/` and `tests/` trees, and reconcile the
source, executable, tool, protocol, dependency, population, and predecessor
identities to the frozen JSON receipt. Each completed cell is registered
atomically. Any launcher interruption cancels work that has not started,
discovers already completed provenance records, finalizes the attempt as
`INTERRUPTED_HOLD`, and permanently forbids a retry under this protocol.

## Claim Limits

- Harvard/Marcell paired contrasts support only their installed cold/humid
  continental inference.
- SNOTEL open controls cannot identify canopy longwave.
- Sleepers results are diagnostic-only.
- Warm-maritime conifer transfer remains withheld.
- Comparator similarity is not correctness authority.

## Stop-Loss

This is one fresh round. Do not tune or start another round from its results.
If the promotion rule fails, proceed to campaign assurance closeout unless new
authoritative process science, discriminating data, or an independently
testable formulation is separately authorized.

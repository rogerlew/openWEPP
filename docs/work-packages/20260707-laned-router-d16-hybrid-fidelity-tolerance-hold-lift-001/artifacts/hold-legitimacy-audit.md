# Hold Legitimacy Audit

Status: EXECUTED-HOLD-COHORT-AUTHORITY. Evidence mode: Static + Ran.

## Hold Condition

The D16 hybrid default-promotion fidelity hold cannot be lifted because the
required broad active-runnable cohort does not exist in the current repo or
session.

## Evidence Proving The Hold

- D16 already rejected H2637-only tolerance ratification.
- `SC-OFEROUTE-002#INV-OFEHYB-008` requires named ratified fidelity/timing
  tolerances before promotion.
- `tools/owcmp/suites/*.json` are `cohort-inventory` manifests. They pass
  `owcmp env`, but `owcmp manifest run` refuses them as preflight
  declarations, not runnable comparison pairs.
- Searching repo-local fixtures and the three external owcmp run roots for
  `routing_coefficients` in management files returned zero matches.
- Active preflight on four repo-local runfiles produced no active-runnable
  candidate: three fail closed on missing `route_*` authority symbols and one
  fails earlier on non-finite climate input.

## Why This Is Outside The Package Envelope

The package could safely predeclare comparison surfaces and audit existing
cohort availability. It could not safely:

- invent production tolerances from H2637 alone,
- add surrogate routing coefficients to existing cohorts,
- treat `cohort-inventory` manifests as executable comparator suites,
- or flip the hybrid default selector without cohort-backed contract authority.

Creating the missing active-runnable cohort requires a separate package because
it changes validation-suite posture and likely creates or exports new
source-authorized run inputs. That package must run anti-evasion guards if it
touches required-case bindings or external-authority suite posture.

## First Actionable Follow-On

Scaffold `D16-HYB-COHORT-AUTHORITY`:

1. Select a minimum active-hybrid promotion cohort from the existing owcmp
   inventories plus H2637 or explicitly justify replacements.
2. For each hillslope, create openWEPP runfiles and source-authorized
   `routing_coefficients` from the same native management authority used by
   D11/D15A. Do not synthesize surrogate route operands.
3. Add an executable comparator/timing suite manifest that runs active plain
   and active explicit hybrid and reports the tolerance surfaces named in
   `artifacts/tolerance-surface-design.md`.
4. Run anti-evasion guards if the package changes cohort fixtures,
   required-case bindings, or external-authority suite posture.
5. Return to D16 default promotion only after that cohort exists and passes.

No partial selector promotion was made.

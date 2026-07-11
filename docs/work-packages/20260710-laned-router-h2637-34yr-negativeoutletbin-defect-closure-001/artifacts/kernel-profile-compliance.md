# Kernel Process Contract Profile Compliance

Status: `EXECUTED`

Evidence mode: `Static + Ran`

Applicability: this package changes production kinematic-wave boundary flux
behavior, so `kernel-process-contract-profile.md` applies.

| Required item | Evidence | Result |
| --- | --- | --- |
| Canonical `SC-*` authority updated | `SC-OFEROUTE-001` rev 51, not a package-local surrogate | pass |
| Required schema sections present | purpose/scope, authority anchors, variables/units, state surfaces, numbered algorithm, branch/guard table, invariants and map, aliases, constants, unit governance, tolerances, vectors, gaps | pass |
| Algorithm and branch table updated | Algorithm 5 specifies finite/nonnegative actual stage faces and exact-zero predictor outlet lower bound before the available-water cap; the KWE/TVD guard row matches | pass |
| Guard/error alignment | non-finite raw face maps to `NonFiniteState`; downstream negative-bin defense remains `NegativeOutletBin`; no guard is loosened | pass |
| Unit governance for touched surfaces | predictor/corrector face and outlet units remain `m2/s` per unit width; integration and bins remain `m2`; existing registry/scalar posture is unchanged and explicitly represented in the contract | pass |
| Test-vector obligation implemented | source-quiet wet-penultimate/dry-outlet vector failed before and passes after; closure is reconstructed from committed depths; a separate test retains the defensive guard | pass |

The changed behavior is a hard physical domain boundary and conservative
accounting invariant. No external constitutive Level-4 suite is defined for
this downstream boundary stencil; acceptance therefore uses the canonical
contract vector, plain-path oracle/conservation tests, the selected production
cohort, and the full canonical H2637 endpoint. Legacy/comparator evidence is
treated as a signal and is not the sole acceptance authority.

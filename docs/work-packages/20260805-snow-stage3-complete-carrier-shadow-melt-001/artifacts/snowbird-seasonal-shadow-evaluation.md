# Snowbird Seasonal Shadow Evaluation

Evidence class: Ran, real direct-production consumer, 2026-08-05.

## Execution

- Exact binary: `target/release/openwepp-cli-hill`
- Binary SHA-256:
  `9d8dc882825da4b1935c13f3dd4fc2755f4d216a49d13c5f3a47e9475f4f875b`
- Binary size: `11,296,824` bytes
- Fixture: retained `snotel_snowbird_ut` direct-production fixture from the
  20260804 mass-transition adjudication.
- Runtime: Stage 3 layered thermal liquid enabled, Harder-Pomeroy hourly phase,
  multilayer density, CoE authoritative melt, Dilley-Unsworth subcanopy
  longwave, and the default-off complete-carrier shadow explicitly enabled.
- Trace:
  `target/snow_stage3_complete_carrier_shadow_melt/runs/snotel_snowbird_ut/snowbird-shadow-v2.snow.jsonl`
- Trace SHA-256:
  `c20566f7bc55d7f587c107d59ee6ed4272d988ba78edb91392c0458840ae53ce`

The reconstruction uses the frozen Snowbird primary cohort, water years
1990-2024 (`35` windows), and sums each window from October 1 through the
observed SNOTEL peak date. Reconstructing the retained carrier's absorbed
shortwave yields median `223.2500438 MJ m^-2`, independently recovering the
frozen `223.25 MJ m^-2` operand and therefore the intended window boundary.

## Median Per Primary Window

| Operand | Median |
| --- | ---: |
| Absorbed shortwave | `223.2500 MJ m^-2` |
| Complete shadow energy | `+169.6840 MJ m^-2` |
| Signed cold-energy change | `-28.7911 MJ m^-2` |
| Positive excess before fusion | `196.5771 MJ m^-2` |
| Shadow melt | `589.3481 kg m^-2` (`0.5893 m` SWE) |
| Terminal unallocated energy | `1.87e-15 MJ m^-2` (numerical zero) |
| Authoritative CoE raw melt | `0.4101 m` |

The maximum emitted substep energy-closure residual was
`1.409e-9 J m^-2`, below the `1e-6 J m^-2` guard. No evaluated resolved
substep or primary window retained positive terminal energy above `1e-6 J
m^-2`. The run stops shadow phase evaluation at the unresolved `1 kg m^-2`
thermal boundary, so this is a resolved-domain terminal result, not a
residual-snow disposition.

## Gate Disposition

`FAIL` — the prescribed-state, resolved-domain complete-carrier plausibility
screen did not pass. The median remains strongly positive rather than near
zero. This is not a coherent post-cutover seasonal energy balance because the
shadow reinitializes daily from authoritative post-CoE state.

`NOT EVALUABLE` — terminal meltout. The resolved-domain terminal-energy
operand is zero, but the shadow reaches the pre-existing thin-pack boundary
without localizing exhaustion or dispositioning residual snow and post-snow
energy. CoE retirement and Stage 3 authoritative melt remain blocked. No flux,
geometry, window, or threshold was tuned after inspection.

## Interpretation Boundary

The shadow advances independently across every stability substep within a
day, reserves sublimation before melt availability, debits shadow ice, and
records exact cold, fusion, and terminal-energy operands. It is reinitialized
from the authoritative post-CoE pack at each daily runner call. Therefore the
seasonal sums above are a real-consumer terminal-energy and carrier
characterization, not a coherent post-cutover seasonal SWE trajectory. They
cannot adjudicate the frozen peak-SWE, peak-date, midwinter-melt, or spring-rate
directions. Persistent cross-day shadow state, hourly precipitation-to-shadow
chronology, and same-substep liquid refreeze/retention/routing remain required
before those predictions can be evaluated.

The retained `unused_positive_energy` and the new
`Q_unallocated_after_exhaustion` have different lineages. The former records
positive current-carrier energy left after cold-content application without a
bounded fusion/ice debit. The latter records complete-carrier excess after
cold warming and bounded latent-fusion consumption over evaluated shadow
substeps. Their numerical change cannot be interpreted as a like-for-like
improvement.

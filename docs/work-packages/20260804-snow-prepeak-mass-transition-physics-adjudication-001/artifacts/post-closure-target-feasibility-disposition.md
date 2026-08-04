# Post-Closure Target Feasibility Review Disposition

Status: `post-closure disposition complete / prospective forcing-target lane reopened / original closure unchanged`

Evidence mode: `Ran` plus `Static`

Disposition date: `2026-08-04`. The reviewed artifact was authored after the
package closed at `b44e75c1`.

## Standing

This is a post-closure supplement. It does not enter or revise the package's
prospectively frozen evidence set, terminal write-set identity, dual reviews,
dual verifications, or original disposition. The exact execution result
`UPSTREAM_GENERATION_PRIORITY` remains valid in its stated narrow sense: among
the audited current-model mass-transition boundaries, authoritative pre-peak
solid-pack loss is generated on the upstream CoE state-mutation path rather
than by Stage-3 disposition or a second SWE debit.

The review does expose a campaign-level omission. The closed package did not
compare that current-model path against a complete all-phase input ceiling or
an audited observation/representativeness operator. Therefore its result must
not be used to rank CoE physics ahead of forcing or target mismatch. The next
prospective lane is reopened around input/target feasibility and input-versus-
loss discrimination before any CoE equation correction is selected.

## Reproduction And Additional Evidence

The review's local table was independently reproduced from the three named,
hash-bound repository inputs. Including `initial_swe_m` does not change any
displayed site median because its median contribution is zero. The following
values reproduce exactly:

| Site | fixture/raw PREC | raw PREC/pillow | fixture/pillow | current peak ratio |
|---|---:|---:|---:|---:|
| Mica Creek | `0.966` | `1.562` | `1.539` | `0.702` |
| Niwot | `0.963` | `1.237` | `1.198` | `0.504` |
| Paradise | `0.906` | `1.331` | `1.105` | `0.518` |
| Snowbird | `0.847` | `0.915` | `0.823` | `0.382` |

The EB-04W2 magnitude/chronology values in F-7 also reproduce from the retained
summary CSV.

The raw Snowbird `PREC` result is not, however, evidence that the station's
observations are internally inconsistent. The
[NRCS data-management handbook](https://directives.nrcs.usda.gov/sites/default/files2/1720456725/Chapter%206%20-%20Data%20Management.pdf)
states that snow-pillow and precipitation-gauge increments are not one-to-one
at most SNOTEL sites and identifies snowfall undercatch, plugging/capping, and
sensor lag as expected causes. It instructs editors not to force increments to
match absent an identified sensor problem.

Snowbird also publishes `PRCPSA`, which the official
[AWDB element catalog](https://wcc.sc.egov.usda.gov/awdbRestApi/services/v1/reference-data?referenceType=elements)
defines as derived “Snow Adjusted Total Precipitation.” A fresh read-only AWDB
query for daily Snowbird `PRCPSA` from `1989-10-01` through `2024-09-30`
returned `434,932` bytes at SHA-256
`fb6358fffc7abdeccbc1d9fe6352f33c1d04829d44473aee36574133ec26e2d1`.
Using the package's exact primary windows gives:

- `35/35` complete Snowbird years;
- median snow-adjusted precipitation / pillow peak SWE `1.2507`;
- no primary year below `1.0`; and
- median fixture / snow-adjusted precipitation `0.6563`.

`PRCPSA` is derived and therefore cannot be treated as independent validation
of the pillow or as correction authority without its adjustment lineage. It
does falsify the claim that raw `PREC/pillow = 0.915` establishes a physically
impossible station record or a `0.915` ceiling for gauge-matched forcing.

## Finding Disposition

| Finding | Disposition | Reason |
|---|---|---|
| F-1 observation pair internally inconsistent | `PARTIALLY ACCEPTED / HEADLINE REJECTED` | The `PREC/pillow` calculation is correct and flags a material raw-sensor mismatch. NRCS explicitly says the sensors are generally not one-to-one, and Snowbird's derived `PRCPSA` restores positive mass headroom. The raw pair is not proven internally invalid. |
| F-2 all-phase mass ceiling absent | `ACCEPTED` | The arc computed only post-phase effective input. `initial SWE + all-phase fixture precipitation` is the required zero-loss ceiling for the exact no-lateral-import fixture, and it was absent. Snowbird's median `0.823` proves the current fixture cannot attain raw pillow parity even with zero modeled loss. |
| F-3 multiplier buys down infeasible target | `PARTIALLY ACCEPTED / CAUSAL LABEL REJECTED` | The arithmetic partitions the current raw gap into about `0.177` input-ceiling shortfall and `0.441` modeled loss. It does not prove the target is infeasible or uniquely split the nonlinear `2.0` response. The first term is current-input/target mismatch, not necessarily observation error. |
| F-4 feasibility normalization removes outlier | `PARTIALLY ACCEPTED` | `0.382 / 0.823 = 0.464` is a valid fraction of the current-input ceiling. Calling it feasibility normalization is too strong, and Snowbird remains the lowest of four descriptive values. It shows reduced separation from Niwot/Paradise, not a common failure mode. |
| F-5 affirmative evidence near-tautological | `ACCEPTED AS CLAIM LIMIT` | The gross-positive/pack-loss ratio is an adjacent-ledger identity, as the package already states. It localizes current modeled loss but supplies no independent evidence that CoE magnitude dominates forcing or target mismatch. |
| F-6 forcing branch closed on budget | `ACCEPTED / FOLLOW-UP` | EB-04W2's `2.0` boundary was an experiment-budget stop, not physical saturation. “Forcing branch closed” was procedural and must not be read as evidentiary exclusion. A new discriminant lane is warranted; an unfocused W3 scalar extension is not. |
| F-7 input scalar co-closes magnitude and chronology | `PARTIALLY ACCEPTED / DOMINANCE INFERENCE REJECTED` | The response values are correct and demonstrate equifinality: forcing mass can remove both reported errors inside this model. Added mass also delays pack exhaustion, so co-closure does not discriminate against excessive melt. The warm `Tmax` bias and truncated event tail remain relevant unresolved forcing signals. |
| F-8 wet-compaction duplicate alias open | `ACCEPTED / FOLLOW-UP` | The duplicate `2 * state_loss + released_rain` data flow is already confirmed. It is excluded as a direct SWE debit but remains an unresolved geometry/cold-state interaction and must be bounded before those states support a coupled correction. |

## Prospective Campaign Disposition

The original package remains `COMPLETE`; none of its executed numbers, closure
proofs, Stage-3 consumer exclusions, or no-correction boundary changes.

Its prospective recommendation is qualified as follows:

1. Do not proceed directly to a CoE equation correction or Stage-3 retention
   tuning.
2. Scaffold a new read-only
   `SNOW-ACCUMULATION-TARGET-FEASIBILITY-AND-INPUT-LOSS-DISCRIMINATION`
   package first.
3. Freeze three distinct quantities without collapsing their authority:
   - `CURRENT_INPUT_MASS_CEILING = (initial SWE + all-phase .cli
     precipitation) / observed peak SWE`;
   - raw `PREC/pillow` as a sensor-comparison diagnostic, not a physical
     ceiling; and
   - `PRCPSA/pillow` as a derived snow-adjusted diagnostic, not independent
     validation.
4. Explicitly bind the modeled hillslope versus station elevation, footprint,
   redistribution, canopy, and no-lateral-import assumptions.
5. Use event-scale discriminants rather than another undifferentiated scalar:
   compare cold-storm observed `WTEQ` increments with modeled snowfall input,
   and compare dry-interval observed `WTEQ` changes with modeled CoE pack loss.
   Preserve missingness, gauge adjustment, and scale mismatch as separate
   uncertainty classes.
6. Carry the EB-04W2 response shape, Snowbird SNOTEL-forcing experiment,
   `Tmax` bias, and event-tail deficit into the frozen candidate set.
7. Bound the wet-compaction alias under explicit operand authority before
   relying on layer geometry or cold content to select a coupled correction.

Only after that lane should the campaign choose between a forcing/operator
correction family, upstream CoE/total-snow-water physics, or a genuinely
multi-factor successor.

## Claim Limits

- No production model or fixture was changed or rerun.
- The live `PRCPSA` query is post-closure diagnostic evidence and is not added
  to the original package evidence manifest.
- Neither raw `PREC` nor derived `PRCPSA` is promoted as truth, a correction,
  calibration input, or independent validation.
- No existing calibration, forcing, phase, or physics parameter is authorized.
- The original package verdict remains a current-code-path localization result,
  not a complete causal ranking of the observed SWE deficit.

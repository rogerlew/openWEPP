# Verification of openWEPP's Daily Linear Groundwater-Reservoir Recurrence

> Architecture prototype: this hand-authored manuscript is nonpublic ASSURE-02
> design evidence. It is not an approved openWEPP report and is not part of the
> `usersum` catalog.

Prepared by Codex as an internal architecture prototype under maintainer-
directed ASSURE-02. A human report lead and scientific approver must be assigned
before a production successor enters review.

## Key Findings

- openWEPP reproduces the authorized two-day linear-reservoir recurrence with a
  maximum absolute residual of `1.78 × 10^-15 m3`, against a
  `1.0 × 10^-12 m3` implementation-test allowance, including the one-day
  timing of storage debits.
- An independently reconstructed 731-day production case closes both terminal-
  storage identities within `4.26 × 10^-11 m3`, against allowances greater
  than `1.20 × 10^-7 m3`.
- Generated baseflow and deep seepage reach the watershed consumer through the
  production hillslope-pass boundary; the enabled branch does not substitute
  the separate channel baseflow coefficient (`cbase`) or feed groundwater into
  surface routing.

These findings resolve respectively to claims `GW-P01`–`GW-P04` and `GW-P09`,
`GW-P05`, and `GW-P06`–`GW-P07` plus `GW-P09` in the prototype's
claim-evidence matrix.

## Plain-Language Summary

openWEPP represents delayed groundwater discharge with a reservoir that receives
deep drainage from the soil and releases fixed daily fractions as baseflow and
deep seepage. We checked whether the software follows the published equations,
keeps the day-to-day accounting in the correct order, rejects physically
inconsistent coefficient combinations, and delivers the calculated baseflow to
the watershed model. A two-day calculation matched independently computed
values. A 731-day production run balanced groundwater recharge, storage, and
discharge to much less than a billionth of a cubic meter. These results show that
the assessed software realization correctly carries out and transfers this
specific recurrence. They do not show how accurately baseflow will be predicted
for an untested watershed; that requires observations and site-appropriate
parameters in a separate empirical evaluation.

## Abstract

Groundwater baseflow can sustain streamflow after rapid surface and lateral
responses decline. The WEPP linear-reservoir extension represents this process
with daily storage forced by deep percolation and proportional releases to
baseflow and deep seepage. We evaluated whether the assessed openWEPP
implementation realizes the authorized recurrence, enforces its domain, and
preserves generated groundwater fluxes through the watershed consumer. The
method combined formulation traceability, an independently calculable two-day
vector, negative domain tests, production consumer tests, and reconstruction of
a 731-day run with 19 overland-flow elements (OFEs). The two-day vector produced
storage of `12.0` and
`14.2 m3`, baseflow of `1.2` and `1.42 m3`, and deep seepage of `0.6` and
`0.71 m3`, matching the analytical recurrence. In the production case, terminal
pre-export storage reconstructed to within `4.25 × 10^-11 m3`; the complete
post-export ledger differed by `4.25 × 10^-11 m3`. Tests also demonstrate that
generated baseflow and deep seepage traverse the hillslope-pass boundary and
that the enabled reservoir branch does not substitute the separate channel
baseflow coefficient (`cbase`). We
conclude that the assessed realization is verified for this bounded daily
recurrence and tested consumer path. Field performance, coefficient
transferability, uncertainty in deep-percolation forcing, and fitness for a
particular watershed were not evaluated.

## 1. Introduction

Streamflow in forest watersheds can contain surface runoff, lateral subsurface
flow, and delayed groundwater baseflow. Baseflow is especially important during
dry periods, but it was absent from the original WEPP formulation used for
surface-runoff-dominated applications. Srivastava et al. (2013) coupled WEPP
deep percolation to a linear groundwater reservoir. In a calibrated evaluation
of the coupled model at Priest River Experimental Forest, they reported that
Nash-Sutcliffe efficiency increased from `0.57` to `0.67`, while runoff-volume
deviation fell from `47%` to `7%` when the baseflow routine was included.

Those calibration-conditioned statistics describe a complete coupled-model
application, including its forcing, parameter choices, and interacting
processes. They motivate the formulation but neither isolate the recurrence's
effect nor constitute current openWEPP results. Porting the routine creates a
separate question that must be answered before new empirical work is
interpreted: does the software implement the accepted equations and move their
outputs through the production model without loss or double counting?

This study evaluates that bounded question for one frozen openWEPP realization.
It distinguishes groundwater-reservoir baseflow from lateral subsurface export
and from the separate unit-area channel baseflow coefficient (`cbase`).

## 2. Model Formulation

For day `i`, recharge `D_i` is the hillslope volume of WEPP deep percolation.
Storage is updated by adding current recharge and removing the preceding day's
baseflow and deep seepage:

`S_i = S_(i-1) + D_i - Qb_(i-1) - Qs_(i-1)`.

Current-day baseflow and deep seepage are proportional to the accepted storage
over the daily step:

`Qb_i = kb S_i Δt`,

`Qs_i = ks S_i Δt`,

where `S`, `D`, `Qb`, and `Qs` are daily volumes in cubic meters and `kb` and
`ks` have units of inverse days, and `Δt = 1 d`. The contract and code use the
equivalent shorthand `Q = kS` because the daily interval is fixed. Initial
storage is an input depth multiplied by hillslope area.

Current openWEPP authority admits finite `kb >= 0` and `ks >= 0`. For a
positive accepted storage, the combined daily exports may not exceed that
storage; a coefficient combination violating this recurrence-level boundary
fails closed at runtime even though the parser assigns no independent upper
bound to either coefficient. Negative `ks`, which would represent upward
exchange from a lower aquifer in the broader modeling lineage, is outside the
current authority. The formulation, coefficient lineage, branch behavior, and
publication identities are specified in `SC-GWBASEFLOW-001` and trace to
Srivastava (2013), Srivastava et al. (2013), and the pinned legacy WEPP source.

## 3. Materials And Methods

### 3.1 Assessed realization

The integrated evidence was generated at Git commit
`de520f1ff867ca5c65b1f82dfe32a19c213ae18c`. ASSURE-02 checked that the
groundwater implementation, runner path, and H2637 test path were unchanged at
the documentation intake commit
`773eb3c56f0afcbc7f605d49c9a09d391e8113a5`. This static comparison is a
currency observation, not a substitute for a release-candidate rerun.

### 3.2 Analytical recurrence test

The two-day vector uses `1,000 m2` of area, `0.010 m` initial storage depth,
`kb = 0.10 d^-1`, `ks = 0.05 d^-1`, and recharge of `2.0` then `4.0 m3`.
Expected values were computed directly from the equations before comparison to
the implementation. The test also checks that current-day exports are not
removed until the following storage update.

The analytical acceptance rule is an absolute difference no greater than
`1.0 × 10^-12 m3` for each storage or export value. This is the coded
implementation-test tolerance for binary floating-point multiplication and
subtraction, not a hydrologic accuracy threshold. The rule is defined in
`direct_runtime.rs` and precedes this prototype's interpretation.

### 3.3 Domain and integration tests

Negative tests admit nonnegative coefficients at the input boundary but reject
a combination whose baseflow and deep-seepage fractions would export more than
the accepted reservoir storage. Consumer tests follow generated volumes from
the direct runtime through hillslope publication, hillslope binary pass (HBP)
serialization and parsing, watershed contribution construction, and channel
routing. They separately test the area threshold and reject generated
groundwater fields when the enabling coefficient authority is absent.

### 3.4 Production recurrence reconstruction

The retained H2637 case spans 731 daily steps and 19 OFEs. The run manifest
publishes initial storage, cumulative recharge, cumulative exports, terminal
pre-export storage, and terminal-day exports. We independently reconstructed:

`SN = S0 + sum(D) - [sum(Qb) - QbN] - [sum(Qs) - QsN]`,

and the post-export identity:

`SN - QbN - QsN = S0 + sum(D) - sum(Qb) - sum(Qs)`.

The coded acceptance allowance for the first identity is
`1.0 × 10^-9 × max(SN, 1 m3)` and for the second is
`1.0 × 10^-9 × max(|SN - QbN - QsN|, 1 m3)`. These storage-scaled ledger
criteria admit floating-point accumulation roundoff. They do not represent
measurement uncertainty, numerical convergence, or a calibrated error target.

The latest runoff-event HBP record was not used as terminal-day evidence because
the last runoff event need not occur on the last simulation day.

## 4. Results

### 4.1 Two-day analytical vector

**Table 1. Independent two-day recurrence vector and openWEPP outputs.**

The maximum observed absolute residual was
`1.7763568394002505 × 10^-15 m3`, below the
`1.0 × 10^-12 m3` allowance: **PASS**.

| Day | Recharge (`m3`) | Storage before (`m3`) | Storage after (`m3`) | Baseflow (`m3`) | Deep seepage (`m3`) |
| ---: | ---: | ---: | ---: | ---: | ---: |
| 1 | 2.0 | 10.0 | 12.0 | 1.20 | 0.60 |
| 2 | 4.0 | 12.0 | 14.2 | 1.42 | 0.71 |

The second-day storage is `12.0 + 4.0 - 1.2 - 0.6 = 14.2 m3`, confirming
that the first day's exports, rather than the second day's newly computed
exports, are debited in the recurrence.

### 4.2 Production run reconstruction

**Table 2. Retained H2637 groundwater-ledger operands and independently
reconstructed residuals.**

Table 2 gives the retained H2637 operands and independent residuals.
The stored floating-point digits are shown so the numerical identities can be
reconstructed; they do not imply equivalent hydrologic measurement precision.

| Quantity | Value (`m3`) |
| --- | ---: |
| Initial storage, `S0` | 0.0 |
| Cumulative recharge | 3668.610172576748 |
| Cumulative baseflow | 3547.636225849919 |
| Cumulative deep seepage | 0.0 |
| Terminal pre-export storage, `SN` | 126.01452784040274 |
| Terminal-day baseflow, `QbN` | 5.04058111361611 |
| Terminal-day deep seepage, `QsN` | 0.0 |
| `SN` minus recurrence reconstruction | `-4.249045559845399 × 10^-11` |
| Post-export ledger residual | `-4.250466645316919 × 10^-11` |

The first residual passed its `1.2601452784040276 × 10^-7 m3` allowance; the
post-export residual passed its `1.2097394672678664 × 10^-7 m3` allowance.
Both reconstruction results are **PASS**.

The residual magnitudes are approximately `1.2 × 10^-14` of cumulative
recharge. This result is evidence of recurrence and publication-ledger
consistency, not numerical-solution convergence, solver-error characterization,
or a measure of agreement with observed baseflow.

### 4.3 Guards and downstream consumption

The over-export vector (`kb = 0.80 d^-1`, `ks = 0.30 d^-1`) fails before a
physically inconsistent state is accepted. Separate tests show that the HBP
payload exposes nonzero generated baseflow and deep seepage, the watershed
linear-reservoir branch consumes generated HBP baseflow instead of `cbase`, the
area threshold suppresses below-threshold side contributions, and a generated
payload without groundwater coefficient authority fails closed.

## 5. Discussion

The evidence supports a bounded, positive conclusion: the assessed openWEPP
realization implements the authorized daily recurrence, handles its tested
domain boundary, publishes the operands needed for independent reconstruction,
and connects generated groundwater fluxes to a real watershed consumer. This is
more informative than a test count because the equations, timing convention,
units, operands, residuals, and consumer are visible.

The production reconstruction also exposed an important evidence-design rule.
The latest runoff-event HBP baseflow was `5.032033091000001 m3`, whereas the
terminal-day baseflow was `5.04058111361611 m3`. Treating the event record as
the terminal state would therefore have produced a plausible but wrong
reconstruction. Publishing timing-qualified operands made that alias detectable.

The original Priest River study supplies calibration-conditioned,
coupled-model observational evidence that motivates the scientific formulation
and demonstrates why the routine matters. It does not isolate the recurrence
from forcing, parameter, or interacting-process effects. A current openWEPP
empirical study must rerun the assessed realization against admitted
observations, document calibration/evaluation separation, characterize forcing
and measurement uncertainty, and report graphical and quantitative performance
over a declared domain. None of those questions can be answered by exact
recurrence closure.

## 6. Limitations

- The analytical vector is synthetic and tests the recurrence rather than the
  environmental adequacy of a linear reservoir.
- H2637 is one retained production case; its exact ledger does not establish
  multi-watershed predictive performance.
- Deep-percolation recharge is treated as an input to this study. Error or
  uncertainty in that upstream process propagates into baseflow.
- The H2637 coefficients produced zero deep seepage, so the integrated run does
  not characterize nonzero deep-seepage magnitude; nonzero transfer is covered
  by focused tests.
- Current authority excludes negative `ks` and therefore excludes upward
  lower-aquifer recharge. Adding that process requires separate scientific,
  contract, parser, implementation, and evaluation work.
- Combined daily exports may not exceed accepted storage. The runtime guard is
  a daily-recurrence admissibility boundary, not an independently established
  physical upper bound for either coefficient.
- Coefficient calibration, identifiability, equifinality, and transferability
  were not assessed.
- No observed streamflow or baseflow-separation dataset was evaluated for the
  current openWEPP realization.
- No conclusion is made about a particular watershed, management decision, or
  acceptable consequence of error.

## 7. Conclusions

For the equations, vectors, production case, and consumer branches examined,
openWEPP's daily linear groundwater-reservoir realization is verified: it
follows the authorized recurrence and preserves generated fluxes through the
tested watershed handoff. The next distinct evidence step is empirical
corroboration of a frozen release realization against independently admitted
watershed observations. A practitioner should not treat the present result as a
site-specific baseflow prediction warranty.

## Open Research And Reproduction

The exact claim-to-evidence identities and SHA-256 values are listed in
`groundwater-claim-evidence-matrix.md`. The exact frozen-to-intake path
comparison, focused test command and output, and independent two-day arithmetic
are retained in `groundwater-current-tree-confirmation.md`. The focused
analytical tests are in
`crates/openwepp-hillslope-orchestrator/src/tests/tests_mod/direct_runtime.rs`.
The production reconstruction and logs are retained under
`docs/work-packages/20260713-integrated-validation-campaign-001/artifacts/`.
The assessed H2637 test command is:

```text
cargo nextest run --test laned_shadow_h2637 --run-ignored ignored-only \
  -E 'test(=h2637_native_active_owner_routes_and_closes)'
```

ASSURE-02 also reran one exact current-tree nextest `quick` selection: three
recurrence/domain tests, three typed watershed tests, and one direct HBP
serialization test all passed. This confirms the focused paths after the frozen
campaign; it does not replace a fresh H2637 or full release-candidate run.

The report prototype does not claim that a temporary run directory is a
preservation repository. A publishable successor must snapshot or regenerate
the declared input and result objects and bind them to a release identity.

## References

- Srivastava, A. (2013). *Modeling Hydrological Processes in Three
  Mountainous Watersheds in the U.S. Pacific Northwest*. Ph.D. dissertation,
  Washington State University.
- Srivastava, A., Dobre, M., Wu, J. Q., Elliot, W. J., Bruner, E. A., Dun, S.,
  Brooks, E. S., and Miller, I. S. (2013). Modifying WEPP to improve streamflow
  simulation in a Pacific Northwest watershed. *Transactions of the ASABE*,
  56(2), 603–611. [doi:10.13031/2013.42691](https://doi.org/10.13031/2013.42691).
- U.S. Environmental Protection Agency (2009). [Guidance on the Development,
  Evaluation, and Application of Environmental Models](https://www.epa.gov/sites/default/files/2015-04/documents/cred_guidance_0309.pdf).

## About This Prototype

- Proposed report identity: `linear-groundwater-reservoir-recurrence`.
- Assessed realization: `de520f1ff867ca5c65b1f82dfe32a19c213ae18c`.
- Prototype date: 2026-07-15 UTC.
- Publication state: nonpublic architecture evidence; not scientifically
  approved or release-transferred.
- Review: internal coding-agent review and verification records are retained
  with the ASSURE-02 package; external scientific peer review is not claimed.
- Supersession: a production ASSURE-05 report, if approved, replaces this
  prototype rather than promoting it unchanged.

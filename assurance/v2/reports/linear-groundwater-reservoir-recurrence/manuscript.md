# Verification of openWEPP's Daily Linear Groundwater-Reservoir Recurrence

Internal architecture-fixture author: Codex (AI coding agent). Accountable human
report lead: unassigned. Scientific approver: unassigned.

## Key Findings

- openWEPP reproduced the authorized two-day linear-reservoir recurrence with
  a maximum absolute residual of `1.78 × 10^-15 m3`, below the
  `1.0 × 10^-12 m3` implementation-test allowance, including the one-day
  timing of storage debits.
- An independently reconstructed 731-day production case closed both terminal-
  storage identities within `4.26 × 10^-11 m3`, against allowances greater
  than `1.20 × 10^-7 m3`.
- Generated baseflow and deep seepage reached the watershed consumer through
  the production hillslope-pass boundary; the enabled branch did not
  substitute the separate channel baseflow coefficient (`cbase`) or return
  groundwater to surface routing.

### Authorship and review disclosure

This manuscript is the internal, nonpublic ASSURE-04A source-contract fixture
derived from the ASSURE-02 architecture prototype. Codex drafted both versions
under maintainer direction. The exact model/configuration used for the
ASSURE-02 draft was not retained; this incomplete agent provenance is disclosed
in the structured source and blocks review entry. ASSURE-02 received internal
coding-agent architecture review, not external scientific peer review. No human
report lead or scientific approver has accepted this manuscript. It demonstrates
how evidence is presented and identified; it is not approved for publication
and does not replace accountable human authorship or scientific review of a
future production report.

## Plain-Language Summary

openWEPP represents delayed groundwater discharge with a reservoir that
receives deep drainage from the soil and releases fixed daily fractions as
baseflow and deep seepage. We checked whether the software follows the
authorized equations, keeps the day-to-day accounting in the correct order,
rejects physically inconsistent coefficient combinations, and delivers the
calculated baseflow to the watershed model. A two-day calculation matched
independently computed values. A 731-day production run balanced groundwater
recharge, storage, and discharge to much less than a billionth of a cubic
meter. These results show that the assessed software realization correctly
carries out and transfers this specific recurrence. They do not show how
accurately baseflow will be predicted for an untested watershed; that requires
observations and site-appropriate parameters in a separate empirical study.

## Abstract

Groundwater baseflow can sustain streamflow after rapid surface and lateral
responses decline. The WEPP linear-reservoir extension represents this process
with daily storage forced by deep percolation and proportional releases to
baseflow and deep seepage. We evaluated whether the assessed openWEPP
implementation realizes the authorized recurrence, enforces its domain, and
preserves generated groundwater fluxes through the watershed consumer. The
method combined formulation traceability, an independently calculable two-day
vector, negative domain tests, production consumer tests, and reconstruction
of a 731-day run with 19 overland-flow elements (OFEs). The two-day vector
produced storage of `12.0` and `14.2 m3`, baseflow of `1.2` and `1.42 m3`, and
deep seepage of `0.6` and `0.71 m3`, matching the analytical recurrence. In the
production case, the terminal-storage identities closed within
`4.26 × 10^-11 m3`. Generated baseflow and deep seepage also traversed the
hillslope-pass boundary. We conclude that the assessed realization is verified
for this bounded daily recurrence and tested consumer path. Field performance,
coefficient transferability, uncertainty in deep-percolation forcing, and
fitness for a particular watershed were not evaluated.

## 1. Introduction

Streamflow in forest watersheds can contain surface runoff, lateral subsurface
flow, and delayed groundwater baseflow. Srivastava et al. (2013) coupled WEPP
deep percolation to a linear groundwater reservoir. In a calibrated evaluation
at Priest River Experimental Forest, the authors reported improved streamflow
performance when the baseflow routine was included.

Those calibration-conditioned statistics describe a complete coupled-model
application, including its forcing, parameters, and interacting processes.
They motivate the formulation but do not establish that a new software
realization implements the recurrence correctly. This study therefore asks a
bounded prior question: does openWEPP perform the accepted calculation and
move its outputs through the production model without loss, substitution, or
double counting?

## 2. Model Formulation

For day `i`, recharge `D_i` is the hillslope volume of WEPP deep percolation.
Storage is updated by adding current recharge and removing the preceding day's
baseflow and deep seepage:

`S_i = S_(i-1) + D_i - Qb_(i-1) - Qs_(i-1)`.

Current-day baseflow and deep seepage are proportional to accepted storage:

`Qb_i = kb S_i Δt`, and `Qs_i = ks S_i Δt`,

where `S`, `D`, `Qb`, and `Qs` are daily volumes in cubic meters, `kb` and `ks`
have units of inverse days, and `Δt = 1 d`. The contract and code use the
equivalent shorthand `Q = kS` because the interval is fixed at one day.

Current openWEPP authority admits finite nonnegative coefficients. For positive
accepted storage, combined daily exports may not exceed storage. Negative
`ks`, which could represent upward exchange in broader modeling lineages, is
outside current authority.

## 3. Materials and Methods

### 3.1 Assessed realization and authority

The integrated evidence was generated at Git commit
`de520f1ff867ca5c65b1f82dfe32a19c213ae18c`. ASSURE-02 confirmed that the
declared implementation and test paths were unchanged at its documentation
intake. The recurrence and coefficient domain are identified by
`SC-GWBASEFLOW-001`; exact source and evidence identities are retained in the
supplement and structured source manifest.

### 3.2 Analytical recurrence test

The two-day vector uses `1,000 m2` of area, `0.010 m` initial storage depth,
`kb = 0.10 d^-1`, `ks = 0.05 d^-1`, and recharge of `2.0` then `4.0 m3`.
Expected values were computed directly from the equations. The absolute
acceptance allowance is `1.0 × 10^-12 m3` for each storage or export value.
This is a floating-point implementation-test tolerance, not a hydrologic
accuracy threshold.

### 3.3 Domain and integration tests

Negative tests reject coefficient combinations whose combined exports exceed
accepted storage. Consumer tests follow generated volumes through direct
runtime publication, hillslope binary serialization and parsing, watershed
contribution construction, and channel routing. Separate assertions distinguish
generated groundwater baseflow from the channel `cbase` contribution and from
surface-runoff routing.

### 3.4 Production recurrence reconstruction

The retained H2637 case spans 731 daily steps and 19 OFEs. We independently
reconstructed the terminal pre-export identity

`SN = S0 + sum(D) - [sum(Qb) - QbN] - [sum(Qs) - QsN]`

and the complete post-export ledger

`SN - QbN - QsN = S0 + sum(D) - sum(Qb) - sum(Qs)`.

Storage-scaled acceptance allowances accommodate floating-point accumulation;
they are not measurement uncertainty, convergence criteria, or calibrated
error targets.

## 4. Results

### 4.1 Two-day analytical vector

| Day | Recharge (`m3`) | Storage before (`m3`) | Storage after (`m3`) | Baseflow (`m3`) | Deep seepage (`m3`) |
| ---: | ---: | ---: | ---: | ---: | ---: |
| 1 | 2.0 | 10.0 | 12.0 | 1.20 | 0.60 |
| 2 | 4.0 | 12.0 | 14.2 | 1.42 | 0.71 |

The maximum absolute residual was
`1.7763568394002505 × 10^-15 m3`, below the
`1.0 × 10^-12 m3` allowance. Second-day storage equals
`12.0 + 4.0 - 1.2 - 0.6 = 14.2 m3`, confirming the prior-day debit timing.

### 4.2 Production ledger reconstruction

| Quantity | Value (`m3`) |
| --- | ---: |
| Initial storage, `S0` | 0.0 |
| Cumulative recharge | 3668.610172576748 |
| Cumulative baseflow | 3547.636225849919 |
| Cumulative deep seepage | 0.0 |
| Terminal pre-export storage, `SN` | 126.01452784040274 |
| Terminal-day baseflow, `QbN` | 5.04058111361611 |
| Terminal-day deep seepage, `QsN` | 0.0 |
| Recurrence residual | `-4.249045559845399 × 10^-11` |
| Complete post-export ledger residual | `-4.250466645316919 × 10^-11` |

Both identities passed their storage-scaled allowances, which exceeded
`1.20 × 10^-7 m3`. The residuals characterize ledger consistency, not
agreement with observed baseflow.

### 4.3 Guards and downstream consumption

The over-export vector (`kb = 0.80 d^-1`, `ks = 0.30 d^-1`) failed before an
inconsistent state was accepted. Separate tests showed nonzero generated
groundwater fields in the hillslope-pass payload, consumption by the watershed
linear-reservoir branch, suppression below the declared contributing-area
threshold, and rejection when enabling coefficient authority was absent.

## 5. Discussion

The evidence supports a bounded positive conclusion: the assessed realization
implements the authorized daily recurrence, handles its tested domain boundary,
publishes operands needed for reconstruction, and connects generated fluxes to
a real watershed consumer. Showing equations, timing, units, operands,
residuals, and consumer behavior makes that conclusion independently auditable.

The latest runoff-event record differed from the terminal-day baseflow because
the final runoff event need not occur on the final simulation day. Publishing
timing-qualified operands prevented that plausible but incorrect alias from
entering the reconstruction.

The Priest River study is observational evidence for the coupled formulation,
not empirical validation of this openWEPP realization. A current empirical
study must rerun a frozen realization against admitted observations, document
calibration/evaluation separation, characterize forcing and measurement
uncertainty, and report performance over a declared domain.

## 6. Limitations and Intended Use

This evidence applies to the daily linear-reservoir recurrence, admitted
coefficient domain, named implementation realization, and tested publication
and consumer paths. It does not establish parameter transferability,
observational accuracy, timing at subdaily scales, uncertainty bounds,
numerical convergence, or suitability for a specific watershed decision.

The appropriate use is software and formulation assurance preceding empirical
evaluation. Users should not treat this bounded verification as a claim that
baseflow predictions are accurate everywhere or that a watershed application
is fit for purpose.

## 7. Conclusions

For the equations, vectors, production case, and consumer branches examined,
the assessed openWEPP realization follows the authorized daily groundwater-
reservoir recurrence and preserves generated fluxes through the tested
watershed handoff. That is a positive, bounded software-verification result.
The next distinct evidence step is empirical corroboration of a frozen release
realization against independently admitted watershed observations. A
practitioner should not treat this result as a site-specific baseflow-prediction
warranty.

## 8. Reproducibility

The supplement maps every finding to stable claim, method, dependency, result,
and reference identities. Machine-readable result objects retain the exact
two-day residual and H2637 ledger operands. ASSURE-04A validates these sources
offline and verifies their content hashes; later packages own dependency
planning, document assembly, review locks, and publication.

## References

Srivastava, A., Dobre, M., Wu, J. Q., Elliot, W. J., Bruner, E. A., Dun, S.,
Brooks, E. S., and Miller, I. S. 2013. Modifying WEPP to improve streamflow
simulation in a Pacific Northwest watershed. *Transactions of the ASABE*,
56(2), 603–611. DOI: `10.13031/2013.42691`.

openWEPP Science Contract `SC-GWBASEFLOW-001`, daily groundwater-reservoir
recurrence and publication authority.

## About This Report Source

- Report identity: `linear-groundwater-reservoir-recurrence`, source version
  `0.1.0`.
- Assessed realization: `de520f1ff867ca5c65b1f82dfe32a19c213ae18c`.
- Source role: nonpublic ASSURE-04A architecture fixture; not scientifically
  approved, release-transferred, exported, or vendored.
- Accountable human report lead and scientific approver: unassigned; review
  entry is blocked.
- Agent assistance: disclosed in the source manifest; the exact ASSURE-02
  model/configuration was not retained, so provenance is incomplete.
- Review: internal coding-agent architecture review only; external scientific
  peer review is not claimed.
- Supersession: a revised and approved ASSURE-05 report replaces this fixture
  rather than promoting it unchanged.

# Verification of openWEPP's Daily Linear Groundwater Reservoir

*Version 1.0 draft — 2026-07-16*

Prepared with disclosed Codex assistance for openWEPP scientific-assurance
maintainers. An accountable human report lead and independent human reviewers
must accept the exact source before publication; this draft is available for
that review but is not public authority.

## Key Findings

- The focused Rust recurrence test passed its preregistered
  `{{quantity:GW-VALUE-TWO-DAY-ALLOWANCE}}` assertion allowance for the
  two-day case. Separately, an independent binary64 recurrence differed from
  decimal arithmetic by at most
  `{{quantity:GW-VALUE-MAX-RESIDUAL-SUMMARY}}`.
- In the {{quantity:GW-VALUE-H2637-DURATION}},
  {{quantity:GW-VALUE-H2637-OFE-COUNT}} H2637 production case, independent
  reconstruction of the terminal groundwater ledger differed from the
  published storage by at most
  `{{quantity:GW-VALUE-H2637-POST-RESIDUAL-FIGURE}}`, compared with a
  storage-scaled allowance of
  `{{quantity:GW-VALUE-H2637-POST-ALLOWANCE-FIGURE}}`.
- Separate tests verified the production HBP writer/parser contract and the
  watershed consumer's handling of a generated-groundwater payload. They
  separated groundwater from lateral subsurface flow, channel `cbase`, and
  active surface-routing water. A single fresh nonzero-groundwater execution
  across the complete CLI adapter chain remains to be demonstrated.

## Plain-Language Summary

openWEPP represents delayed groundwater discharge as a reservoir. Deep drainage
from the soil enters storage; fixed fractions of that storage leave each day as
baseflow and deep seepage. We asked whether the software performs that daily
calculation in the specified order, rejects inconsistent inputs, reports enough
information to audit the water ledger, serializes the calculated groundwater
volumes, and consumes those fields correctly when supplied to the watershed
model.

The focused two-day Rust test passed its numerical assertion, and independent
arithmetic reconstructed both the analytical case and the 731-day production
ledger within floating-point allowances many orders of magnitude smaller than
the simulated volumes. Separate interface tests showed that generated fields
are serialized and that the watershed consumer uses them when supplied without
substituting a different baseflow term or returning them to surface routing.

This is a strong, bounded software-verification result. It does not show how
accurately openWEPP will predict groundwater or streamflow at an untested site.
That judgment requires observations, site-appropriate parameters, forcing and
measurement uncertainty, and a separate empirical evaluation.

## Abstract

Groundwater baseflow can sustain streamflow after rapid surface and shallow
subsurface responses decline. The WEPP groundwater extension represents this
delayed response with a daily linear reservoir forced by deep percolation. We
evaluated whether the assessed openWEPP realization implements the authorized
storage recurrence, enforces its coefficient and storage domain, exposes
auditable run-level operands, and transfers generated groundwater volumes
at its production-writer, strict-parser, and watershed-consumer interfaces. The study combined
formulation traceability, an independently calculated two-day vector, negative
domain tests, typed serialization and consumer tests, and independent
reconstruction of a {{quantity:GW-VALUE-H2637-DURATION}},
{{quantity:GW-VALUE-H2637-OFE-COUNT}} production run. The Rust analytical test
passed its `{{quantity:GW-VALUE-TWO-DAY-ALLOWANCE}}` assertion; independent
binary64 arithmetic differed from decimal arithmetic by at most
`{{quantity:GW-VALUE-MAX-RESIDUAL-SUMMARY}}`. The production recurrence and
complete post-export identities each closed within
`{{quantity:GW-VALUE-H2637-POST-RESIDUAL-FIGURE}}`, against allowances near
`{{quantity:GW-VALUE-H2637-POST-ALLOWANCE-FIGURE}}`. Separate production-writer,
strict-parser, and watershed-consumer tests preserved the generated fields and
their meanings; missing authority, threshold conditions, and over-export cases
failed closed. The assessed realization is therefore verified for the daily
recurrence and those named interfaces. A single fresh nonzero-groundwater run
through the complete CLI-to-watershed chain, predictive accuracy, parameter
transferability, subdaily behavior, and fitness for a particular watershed
were not evaluated.

## 1. Introduction

Forest-streamflow hydrographs can include rapid surface runoff, lateral
subsurface flow, and delayed groundwater baseflow. The delayed component matters
most when streamflow persists after rainfall or snowmelt inputs have declined.
A model that omits or mishandles this storage-and-release behavior can reproduce
short peaks while missing recession and low-flow periods.

Srivastava et al. (2013) coupled WEPP deep percolation to a linear groundwater
reservoir and evaluated the combined formulation at Priest River Experimental
Forest. They calibrated on 2005-2006 data and evaluated 2007-2009. Across the
complete 2005-2009 study period, the authors reported an
overall Nash-Sutcliffe efficiency of 0.67 and runoff-volume deviation of 7%
with baseflow, compared with 0.57 and 47% without it. They fitted baseflow and
deep-seepage daily coefficients of 0.0156 and 0.00026. Those results show why the
formulation is scientifically relevant, but they belong to that coupled model,
site, period, forcing, and calibration. They are not performance statistics for
the openWEPP realization assessed here.

Before empirical evaluation can be interpreted, the software calculation and
its interfaces must be correct. This study therefore asks: does openWEPP
realize the authorized daily recurrence, preserve its timing and units, reject
inadmissible states, publish the operands needed for independent audit, and do
the separately tested writer/parser and watershed consumer honor the generated
groundwater fields?

## 2. Model Formulation

For day `i`, `D_i` is the hillslope volume of WEPP deep percolation entering the
groundwater reservoir. `S_i` is accepted storage before current-day exports;
`Qb_i` and `Qs_i` are the baseflow and deep-seepage volumes generated from that
storage. The one-day recurrence is:

```text
S_i  = S_(i-1) + D_i - Qb_(i-1) - Qs_(i-1)
Qb_i = kb S_i
Qs_i = ks S_i
```

Storage is in cubic meters. `D`, `Qb`, and `Qs` are daily-integrated cubic-meter
volumes, while `kb` and `ks` have units of inverse days. The contract's
`Q = kS` form includes the fixed one-day interval; no subdaily integration is
implied. Initial storage depth is converted to volume from hillslope area
before the first recurrence.

The current authority admits finite, nonnegative initial storage and
coefficients. The implementation rejects non-finite or negative states and a
coefficient combination that would export more than accepted storage in one
day. A missing optional groundwater-coefficient sidecar disables the reservoir;
it does not authorize inferred defaults. Generated groundwater baseflow is
distinct from lateral subsurface flow and from the channel unit-area coefficient
named `cbase`.

## 3. Materials and Methods

### 3.1 Assessed realization and preregistration

The assessed repository realization was frozen before result execution at Git
commit `01ed70550a4e371e99afe35c4bdd4d9b667e812c`. The 12 declared groundwater
producer, publication, serialization, test, and watershed-consumer paths were
byte-identical to the earlier integrated realization. The H2637 runner binary
was rebuilt for the frozen commit before accepted production evidence was run.

`SC-GWBASEFLOW-001` supplied the equations, units, branch authority, consumer
obligations, and test vectors. The study protocol fixed the equations,
operation order, two-sided tolerances, operand lineage, and rejected aliases
before fresh execution. A change to any bound implementation, fixture, method,
or result object requires new evidence and review.

### 3.2 Independent two-day calculation

The analytical case used a {{quantity:GW-VALUE-AREA}} hillslope,
`{{quantity:GW-VALUE-INITIAL-STORAGE-DEPTH}}` initial storage depth,
`kb = {{quantity:GW-VALUE-KB}}`, `ks = {{quantity:GW-VALUE-KS}}`, and daily
recharge of `{{quantity:GW-VALUE-DAY1-RECHARGE}}` then
`{{quantity:GW-VALUE-DAY2-RECHARGE}}`. A standard-library Python procedure,
which neither imports nor calls openWEPP, evaluated the recurrence with decimal
and binary64 arithmetic. The absolute allowance for each calculated state or
flux was `{{quantity:GW-VALUE-TWO-DAY-ALLOWANCE}}`. This allowance measures
floating-point implementation agreement, not hydrologic error.

### 3.3 Domain and transfer checks

A focused nextest selection executed the recurrence, over-export guard,
multi-OFE recharge aggregation, contributing-area threshold, missing-authority
failure, hillslope-pass serialization, and watershed consumption. The transfer
contract assessed across separate tests was:

```text
daily groundwater producer
  -> direct publication state
  -> hillslope binary pass (HBP)
  -> strict typed HBP parser
  -> watershed hillslope contribution
  -> channel/baseflow consumer
```

The writer/strict-parser test and the hand-constructed `HillslopeContribution`
consumer test verify adjacent interfaces; they are not one end-to-end execution
of a nonzero groundwater payload through the actual CLI adapter. Separate
assertions rejected lateral subsurface flow, `cbase`, active-router surface
source, and producer-only state as substitutes for generated groundwater.

### 3.4 H2637 production reconstruction

H2637 used the production hillslope runner for
{{quantity:GW-VALUE-H2637-DURATION}} and
{{quantity:GW-VALUE-H2637-OFE-COUNT}}. Its
groundwater inputs were zero initial storage, a daily `kb` of 0.04, zero `ks`,
and a 1 ha groundwater-contributing-area threshold. The test ran the same
native-management fixture with the active owner disabled, active by default,
and explicitly active. Default and explicit-active HBP and pass-Parquet bytes
were required to match.

The independent procedure read the produced explicit-active manifest and
reconstructed two timing-qualified identities:

```text
S_N = S_0 + sum(D) - [sum(Qb) - Qb_N] - [sum(Qs) - Qs_N]
S_N - Qb_N - Qs_N = S_0 + sum(D) - sum(Qb) - sum(Qs)
```

The first compares terminal pre-export storage. The second compares terminal
post-export storage. Each two-sided allowance was `1e-9` times the magnitude of
the corresponding storage, with a one-cubic-meter minimum scale. The procedure also
checked produced HBP and Parquet hashes against the manifest before using its
operands.

### 3.5 Evidence classification

The two-day case combines a Rust assertion test with independent arithmetic
reconstruction. H2637 is deterministic production recurrence and conservation
evidence. The separate writer/parser and consumer tests are interface-contract
evidence, not a continuous-path execution. Negative tests are domain and
fail-closed evidence. The Priest River study is external prior empirical
evidence for a related coupled formulation. No observation is used as an
empirical referent in the present study.

## 4. Results

### 4.1 Daily recurrence

{{table:GW-TABLE-TWO-DAY}}

{{figure:GW-FIGURE-TWO-DAY}}

Day 1 accepted storage was `{{quantity:GW-VALUE-DAY1-STORAGE}}`, producing
`{{quantity:GW-VALUE-DAY1-BASEFLOW}}` of baseflow and
`{{quantity:GW-VALUE-DAY1-DEEP-SEEPAGE}}` of deep seepage. Day 2 storage was
`{{quantity:GW-VALUE-DAY2-STORAGE}}`, which equals the preceding storage plus
`{{quantity:GW-VALUE-DAY2-RECHARGE}}` recharge minus the preceding day's two
exports. The maximum binary64-versus-decimal residual was
`{{quantity:GW-VALUE-MAX-RESIDUAL-EXACT}}`, below the preregistered allowance.

The over-export case (`kb = {{quantity:GW-VALUE-GUARD-KB}}`,
`ks = {{quantity:GW-VALUE-GUARD-KS}}`) was rejected before an inconsistent
state was accepted.

### 4.2 Production groundwater ledger

{{table:GW-TABLE-H2637}}

{{figure:GW-FIGURE-H2637}}

Across the production run, cumulative recharge was
`{{quantity:GW-VALUE-H2637-CUM-RECHARGE}}` and cumulative baseflow was
`{{quantity:GW-VALUE-H2637-CUM-BASEFLOW}}`. Terminal pre-export storage was
`{{quantity:GW-VALUE-H2637-TERMINAL-STORAGE}}`; the independently reconstructed
value was `{{quantity:GW-VALUE-H2637-RECURRENCE-RECONSTRUCTED}}`. Their signed
difference was `{{quantity:GW-VALUE-H2637-RECURRENCE-RESIDUAL-EXACT}}`.

After the terminal export, storage was
`{{quantity:GW-VALUE-H2637-POST-EXPORT-STORAGE}}`; reconstruction from the
full-run recharge and export totals gave
`{{quantity:GW-VALUE-H2637-FULL-RUN-STORAGE}}`. The signed difference was
`{{quantity:GW-VALUE-H2637-POST-RESIDUAL-EXACT}}`. Both residual magnitudes
were less than four ten-thousandths of their respective allowances.

The active surface-routing ledger also closed independently. Its residual was
`{{quantity:GW-VALUE-H2637-SURFACE-RESIDUAL}}`, or
`{{quantity:GW-VALUE-H2637-SURFACE-RELATIVE}}` of routed source water, below
the preregistered `{{quantity:GW-VALUE-H2637-SURFACE-ALLOWANCE}}` allowance.

### 4.3 Transfer, threshold, and authority behavior

The focused checks showed that the production writer serializes the generated
HBP fields and the strict parser preserves them. A separate consumer test
constructed a `HillslopeContribution` with those fields and exercised the
watershed linear-reservoir branch. Below-threshold contributing area suppressed
the applicable channel contribution. A generated-groundwater payload without
`gwcoeff` authority failed closed. Tests also distinguished the generated
contribution from `cbase`, which is a separate channel parameter. These results
verify both interface contracts but do not establish a single fresh execution
across the adapter between them.

The latest runoff-event HBP baseflow was not used as `Qb_N`: the final runoff
event need not be the final simulated day. The timing-qualified manifest value
was required for terminal reconstruction. This negative alias check materially
changed how the ledger could be audited.

## 5. Discussion

The analytical and production results answer the recurrence question
positively for the named realization. The software applies the one-day storage
debit in the specified order, generates proportional exports, rejects the
tested invalid domain, and publishes complete terminal operands. The writer,
parser, and watershed consumer separately honor the generated-groundwater
contract; complete adapter traversal is an open integration claim. The verified
results are inspectable because the equations, units, timing, source paths,
produced operands, tolerances, and reconstruction procedure are retained
together.

The residuals are numerical bookkeeping quantities. Their small size shows
that independent arithmetic agrees with produced state and totals; it does not
measure error against nature. Likewise, a real production path is stronger than
a producer-only unit test for integration assurance, but it is not an observed
watershed evaluation.

The Priest River study supports the scientific usefulness of a linear
groundwater reservoir when groundwater contributes materially to streamflow.
It also illustrates why empirical performance is conditional: the reported
coefficients were fitted for that study, and the performance statistics reflect
the coupled model, forcing, site, years, and observations. H2637 instead used
a daily `kb` of 0.04 and zero `ks` as a deterministic software fixture. Agreement
of its ledger cannot transfer Priest River's predictive statistics to
openWEPP.

The fail-closed cases are relevant scientific-assurance evidence. They show
that absent parameter authority is not silently replaced and that an excessive
daily export is not accepted merely because the algebra produces a number.
They do not establish that all nonnegative coefficients are plausible for all
watersheds; parameter plausibility remains an application and empirical-study
question.

## 6. Limitations

- No streamflow, groundwater-level, lysimeter, tracer, or other environmental
  observation was compared with openWEPP output.
- H2637 is a deterministic integration fixture, not an empirical watershed
  sample. Its result does not quantify model-form, forcing, parameter, or
  measurement uncertainty.
- H2637 used `ks = 0`; nonzero deep seepage was exercised by the analytical
  vector and serialization/domain checks, not by the production recurrence.
- The fixed one-day recurrence was assessed. No subdaily solution, timestep
  convergence, or alternate nonlinear groundwater formulation was evaluated.
- The evidence applies to the frozen implementation and named consumer path.
  A change to the producer, serialization, consumer, science authority, or
  result method requires impact review and, where material, rerun.
- The production writer/parser and watershed consumer were verified in
  separate tests. A single fresh nonzero-groundwater execution through the
  actual CLI adapter into the watershed model was not performed.
- The current authority excludes negative `ks` and therefore does not represent
  upward exchange from a deeper aquifer.
- Verification of implementation and transfer cannot establish parameter
  transferability or predictive accuracy for another climate, soil,
  topography, management, or watershed.

## 7. Conclusions

For the specified daily linear reservoir, analytical vector, and H2637
production case, the assessed openWEPP realization performs the authorized
recurrence. The terminal storage and complete run ledgers reconstruct within
preregistered floating-point allowances, and tested invalid or unauthorized
conditions fail closed. The production writer/parser and watershed consumer
also preserve the generated-groundwater fields in separate interface tests;
one fresh execution across their actual adapter remains open.

This conclusion is deliberately bounded to software and integration
verification. It does not claim that openWEPP baseflow predictions are accurate
for a particular watershed. A decision owner should combine this evidence with
an independently designed empirical evaluation, site inputs and parameter
provenance, uncertainty, and the consequences of error.

## 8. Open Research and Reproduction

The {{link:supplement|technical supplement}} gives the claim-to-evidence map,
exact commands, realization and binary identities, output hashes, and review
boundary. Public-safe research objects include the
{{link:research-object:GW-OBJECT-TWO-DAY-INPUT|analytical inputs}},
{{link:research-object:GW-OBJECT-TWO-DAY|analytical result}},
{{link:research-object:GW-OBJECT-H2637|production result}}, and
{{link:research-object:GW-OBJECT-REPRODUCTION-PROCEDURE|independent analysis procedure}}.
The {{link:research-object:GW-OBJECT-SCIENCE-CONTRACT|portable science contract}}
provides the process authority. The
{{link:usersum:hillslope-hydrology-and-sediment-physics.md|model-science narrative}}
explains how groundwater fits within broader hillslope and watershed hydrology.

The complete study can be challenged by rebuilding the frozen runner, rerunning
the named nextest cases, and applying the retained independent procedure to the
new manifest and outputs. Expected hashes are currency checks; changed hashes
require semantic comparison rather than automatic rejection.

## References

{{reference:GW-REF-SRIVASTAVA-2013}}

{{reference:GW-REF-SCIENCE-CONTRACT}}

## About This Report

- Report identity: `linear-groundwater-reservoir-recurrence`, version `1.0.0`.
- Assessed realization: `01ed70550a4e371e99afe35c4bdd4d9b667e812c` plus the
  exact rebuilt runner identity in the supplement.
- Study type: formulation, code-verification, integration/consumer, and
  realization-transfer evaluation; no current empirical evaluation.
- Agent assistance: Codex drafted and mechanically analyzed the report under
  the retained ASSURE-05 prompt, protocol, inputs, and evidence. Ordinary
  reproduction and builds do not invoke an agent.
- Human accountability: report lead and independent scientific,
  reproduction/publication, assurance-steward, and release approvals remain to
  be supplied for the exact reviewed root. Internal coding-agent review is not
  external peer review or publication approval.
- Publication, approval, and supersession metadata are populated only after the
  lifecycle gates pass.

## Revision Log

| Version | Date | Changes |
| --- | --- | --- |
| 0.1 | 2026-07-15 | Established the architecture fixture and deterministic assembly source. |
| 1.0 draft | 2026-07-16 | Reframed the source as a genuine scientific study, preregistered the method, required fresh evidence and independent reconstruction, quantified prior Priest River evidence without attributing it to openWEPP, and made the approval boundary explicit. |

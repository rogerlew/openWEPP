# openWEPP Verification And Validation Strategy

Status: proposed v2 reframe — ASSURE-02 acceptance gate

Owner: openWEPP maintainers and designated scientific reviewers

## Purpose

openWEPP verification and validation exists to make the model's scientific
basis, implementation, evaluated behavior, uncertainty, and limitations visible
enough that hydrologists, soil scientists, researchers, and practitioners can
assess the evidence for their own work.

The strategy produces two different kinds of outcome:

- **Verification** answers specified questions about formulation, code,
  numerical solution, integration, conservation, publication, and release
  realization. These questions can be hard-gated pass or fail.
- **Empirical model evaluation** asks how model results correspond to an open
  natural system for a named quantity, scale, domain, dataset, forcing, and
  method. Its outcome is a nonterminal body of corroborating, mixed, contrary,
  or missing evidence—not universal proof that the model is “valid.”

Application fitness is a third decision. It belongs to the named user or
institution that knows the site, decision, accuracy need, alternatives, and
consequence of error.

## Public Product

The primary public product is a conventional
[scientific model-evaluation report](../standards/scientific-model-evaluation-report.md).
It leads with a scientific question and findings, not lifecycle state, aggregate
grades, hashes, or test counts. A report explains:

1. why the study matters;
2. what process, quantity, scale, domain, and realization were evaluated;
3. what formulation, data, referent, and method were used;
4. what quantitative results were observed;
5. what the results mean relative to prior knowledge;
6. what uncertainty, contrary evidence, and limitations remain;
7. what can and cannot be transferred; and
8. how the study can be reproduced or challenged.

A public technical supplement carries detailed method and traceability. A
version-bound public research-object surface carries safe claim-bearing data,
procedures, and reproduction material. An internal machine assurance bundle
carries protected objects, typed identities, dependencies, review locks, build
records, and snapshots. None substitutes for the scientific argument. The
[v2 architecture](scientific-assurance-v2-architecture.md) defines the boundary.

## Why Verification And Validation Are Both Needed

A model can reproduce observations for the wrong reason: compensating errors,
calibration leakage, biased forcing, parameter non-identifiability, or a broken
consumer path may produce plausible outputs. Empirical agreement therefore does
not prove that the specified equations were solved or published correctly.

A model can also implement its equations perfectly and still omit important
processes, use uncertain inputs, or perform poorly outside the tested domain.
Verification therefore does not establish correspondence with nature.

Both are needed because they address different failure modes. They remain
separate in every claim, report, review, and release record.

## Evidence Dimensions

| Dimension | Question | Typical referent | Decision posture |
| --- | --- | --- | --- |
| Conceptual/formulation evaluation | Is the process representation scientifically defensible for the stated scope? | Theory, literature, experiments, science contract | Reviewed claim, not code proof |
| Code verification | Does the implementation realize the specified formulation? | Analytical/manufactured solution, hand calculation, independent oracle, property | Hard pass/fail for bounded claim |
| Numerical-solution verification | Is discretization or solver error characterized and acceptable for the claim? | Convergence, stability, order, resolution study | Hard gate where material |
| Integration/consumer verification | Does the real downstream path consume and publish the quantity correctly? | End-to-end reconstruction, boundary and consumer tests | Hard pass/fail |
| Empirical corroboration | How does the model correspond to observations in the declared domain? | Admitted observations independent of evaluation choices | Graded, nonterminal evidence |
| Comparative evidence | How does another implementation or model differ? | Legacy or alternate model | Diagnostic; not truth by itself |
| Release transfer | Does the evidence apply to the exact release realization? | Frozen rerun, semantic reconstruction, impact analysis | Release-specific gate |
| Application assessment | Is the evidence adequate for this named decision? | Report evidence plus local context | Decision-owner judgment |

No aggregate status may collapse these dimensions. A verification gap blocks
only conclusions for which that verification is materially required. A useful
code-verification study is not headlined as empirically insufficient when it
does not claim predictive performance.

## Claim Envelope

Every conclusion is bounded by:

- process and quantity of interest;
- spatial and temporal support;
- tested climate, soil, topography, management, state, and numerical regimes;
- software, configuration, parameter, and dependency realization;
- referent and evidence type;
- calibration and evaluation roles;
- uncertainty and sensitivity treatment;
- observed result and acceptance rationale; and
- known limitation and prohibited transfer.

Evidence for annual soil loss does not establish event timing. Snow-water
equivalent evidence does not establish snow depth. Conservation does not
establish predictive accuracy. Agreement with a legacy comparator does not
establish truth. A report uses precise verbs—verified, reproduced,
reconstructed, corroborated, contradicted, not evaluated—only for the bounded
claim supported.

## Verification Philosophy

Verification is requirements-based and can be binary. Its rigor is proportional
to the consequence of a defect and normally includes:

- formulation traceability from science contract to implementation;
- independent expected values rather than producer-derived self-comparison;
- numerical refinement, convergence, or stability where discretization matters;
- conservation with independently reconstructed operands;
- domain and fail-closed behavior;
- deterministic serial/parallel semantics where claimed;
- real producer-to-consumer proof; and
- exact release realization or justified semantic transfer.

Passing tests are evidence only when their referent, operands, tolerances,
branches, and real consumer are visible. Test count is never a scientific
conclusion.

## Empirical Evaluation Philosophy

Environmental evaluation is purpose- and domain-specific and continues over the
model life cycle. A credible study:

- states the intended quantity, scale, domain, and decision context;
- characterizes observation quality, representativeness, alignment, and sample
  count;
- separates calibration, mechanism selection, debugging, and evaluation data;
- distinguishes forcing-limited from model-form-limited inference;
- selects graphical and quantitative diagnostics for the quantity rather than
  applying one universal score;
- reports bias, residual structure, regimes, contrary cases, and uncertainty;
- compares with prior knowledge and alternatives where informative; and
- concludes only within the tested envelope.

Natural variability and measurement error place a floor under apparent
performance. A scientifically advanced model can still show large pointwise
residuals when the referent is noisy or spatially mismatched. Honest evaluation
therefore explains the data and uncertainty rather than using a single
threshold as a licensing verdict.

## Calibration And Evidence Independence

The report discloses every way observations influenced model design, parameter
selection, input adjustment, metric choice, thresholds, or defect correction.
Data used for any of those purposes are not independent evaluation data for the
same claim unless a prospectively justified method says otherwise.

When independent data are unavailable, the study may still report formulation,
verification, sensitivity, comparison, or exploratory empirical evidence. It
must name that limitation and avoid a favorable independent-corroboration claim.

## Comparator Posture

The pinned legacy WEPP baseline provides implementation provenance and a useful
diagnostic comparator. Under ADR-0017 it is a flag, not a truth target.

- Agreement can support migration consistency when the baseline path is itself
  trusted for that bounded behavior.
- Disagreement opens investigation; it is not automatically an openWEPP defect.
- Independent science contracts, invariants, analytical solutions,
  observations, and conservation decide the disposition.
- A report labels comparative evidence separately from empirical corroboration.

## Transparency And Reproducibility

Transparency means a domain reader can understand the choices and limitations;
auditability means a reviewer can trace and reproduce the material claims. Both
are required.

The manuscript remains canonical for interpretation. Claim-bearing values,
tables, and figures resolve to retained result identities with units, methods,
software, and source data. Data and software availability, access conditions,
licenses, computational cost, and known nondeterminism are disclosed. Restricted
evidence is reviewed without publishing protected paths or content.

Every safely redistributable project-owned object needed to inspect a published
claim appears on the report's public research-object surface. An internal bundle
is not a substitute for public availability.

The future build system mechanizes identities, dependencies, values, tables,
figures, review locks, staging, drift, and snapshots. It does not select methods,
interpret results, invoke agents, or approve science.

## Review And Publication

A report becomes public only after:

1. independent scientific review;
2. independent reproduction or reconstruction of material results;
3. publication and accessibility review;
4. complete finding disposition;
5. named human scientific approval;
6. deterministic public build and catalog-consumer checks; and
7. binding to an exact release or standalone publication snapshot.

Drafts and review candidates remain in staging. A report can publish negative or
mixed results when the study is scientifically useful. A placeholder that only
says evidence is missing does not publish as a report.

Coding-agent review is disclosed as internal review. It does not become external
peer review by being independent of the producing agent.

## Application Fitness

The report developer supplies evidence, tested domain, uncertainty, and
limitations. The application owner compares these with the proposed watershed,
inputs, quantity, decision threshold, acceptable error, alternatives, and
consequence of misuse.

The application assessment records that comparison separately. openWEPP neither
authorizes nor rejects an unnamed use by attaching a universal validation grade
to the model.

## Portfolio And Roadmap

The project prioritizes reports by user importance, scientific readiness,
evidence quality, production relevance, and confounding—not by which subsystem
is easiest to label. The v2 sequence is:

1. freeze this documentation architecture using a hand-authored, real-evidence,
   bounded non-snow prototype;
2. retire the v1 public SNOTEL candidate while preserving its provenance and a
   neutral zero-report public state;
3. implement only the minimal deterministic builder demonstrated by the
   prototype;
4. publish the bounded pilot after full scientific and reproduction review;
5. synthesize the extensive snow/frost evidence into one or more conventional
   reports; and
6. expand the portfolio across hydrology, erosion, sediment, plant, channel, and
   watershed questions.

WEPPcloud vendoring remains a mandatory but deferred transfer gate immediately
before the openWEPP beta release in WEPPcloud.

## Prohibited Shortcuts

- Calling openWEPP or a subsystem “validated” without a complete claim envelope.
- Publishing an evidence gap as a status-first scientific report.
- Treating verification, conservation, comparator agreement, or a mature
  formulation as empirical accuracy.
- Treating empirical fit as proof of correct implementation.
- Calibrating on evaluation data without disclosure.
- Using one metric or threshold as a universal verdict.
- Hiding failed cases, uncertainty, limitations, or application assumptions.
- Claiming a downstream or public path with producer-only evidence.
- Letting a builder, agent, or generic project status decide scientific meaning
  or application fitness.

## Governing Documents

- [ADR-0038](../decisions/0038-manuscript-first-scientific-assurance-publication.md)
- [V2 architecture](scientific-assurance-v2-architecture.md)
- [Report lifecycle and ownership](scientific-assurance-dossier-lifecycle.md)
- [Source, build, and dependency contract](scientific-assurance-v2-source-build-contract.md)
- [Scientific model-evaluation report standard](../standards/scientific-model-evaluation-report.md)

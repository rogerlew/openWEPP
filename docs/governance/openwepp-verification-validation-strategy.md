# openWEPP Verification and Validation Strategy

Status: `active-strategy`

Document type: governance philosophy and target-state requirements

Implementation maturity: `planned`

Last reviewed: `2026-07-13`

Related authority:

- [correctness authority model](../specifications/correctness-authority-model.md);
- [science-contract index](../specifications/science-contracts/index.md);
- [ADR-0017 comparator posture](../decisions/0017-re-pin-operational-distrust-comparator-is-flag-not-target.md);
- [ADR-0028 observed-data admission](../decisions/0028-observed-data-admission-authority.md); and
- [draft release procedure](openwepp-release-procedure-draft.md).

## Executive Position

openWEPP will treat verification and validation (V&V) as a first-class
scientific assurance subsystem. The subsystem will connect requirements,
science contracts, source and binary identities, test and observational data,
executed evidence, uncertainty, review, and release decisions in one traceable
evidence graph.

The unit of V&V is a **bounded claim**, not "the model." A claim states that a
named openWEPP version and configuration is sufficiently credible for a named
use, quantity of interest, scale, and application domain. Evidence outside that
envelope may inform a new claim, but it does not silently extend the old one.

Verification and validation answer different questions:

- **Verification:** Does the implementation and each computed solution satisfy
  its specified mathematical, software, numerical, interface, and publication
  requirements?
- **Validation:** Does the conceptual and computational model represent the
  relevant real-world system adequately for the claim's intended use?
- **Use qualification:** Is the combined evidence, uncertainty, applicability,
  and known limitation profile sufficient to permit that particular use?

Both are required for predictive claims about reality, with depth graded to the
claim and its consequences. A correctly implemented inadequate model is
precisely wrong.
A model that agrees with selected observations despite implementation defects,
data leakage, numerical error, or compensating biases is not predictive.

This strategy is active governance for future V&V work. It does not assert that
the target subsystem is implemented, that every current test has been
reclassified, or that openWEPP as a whole is validated for release.

## Purpose and Scope

This strategy establishes:

1. the language openWEPP uses for V&V claims;
2. the evidence classes that may support those claims;
3. minimum governance requirements for planning, executing, reviewing, and
   preserving V&V evidence;
4. the target code and artifact architecture for machine-auditable evidence and
   human-readable reports;
5. the path from current contract and integration evidence to process,
   subsystem, integrated, and application-level V&V; and
6. the V&V basis eventually required for a public release.

The strategy applies to process kernels, coupled hillslope and watershed
systems, input and output transformations, numerical solvers, runner behavior,
and scientific results. It also applies to the data, tools, and reports used to
judge those surfaces.

This strategy does not:

- adopt nuclear licensing rules or claim nuclear-grade qualification;
- replace canonical `SC-*` science authority;
- make a legacy binary a correctness oracle;
- prescribe universal goodness-of-fit thresholds;
- equate ordinary input checking with scientific model validation; or
- authorize production code changes without the normal contract-first and
  work-package governance.

## Governing Philosophy

### Claims are use-specific

No release, executable, crate, or process receives an unqualified "validated"
label. Each claim must identify at least:

- the decision or use being supported;
- source, binary, configuration, and schema identities;
- modeled processes and couplings;
- quantities of interest and their units;
- spatial and temporal scales;
- forcing, soil, climate, topography, management, land-use, and topology
  regimes where relevant;
- required accuracy, uncertainty, and failure behavior; and
- explicit exclusions and extrapolations.

The claim envelope is the intersection of these dimensions. Passing evidence
for annual hillslope soil loss does not validate event timing, watershed
sediment delivery, snow-dominated hydrology, or an untested management regime.

### V&V accumulates evidence, not truth

Models are purposeful simplifications. Validation can establish that results
are sufficiently consistent with reality for a specified use; it cannot prove
that the model is true. New data may strengthen, narrow, or overturn an earlier
assessment. Negative evidence must remain visible.

Evidence records are therefore immutable. Later runs or assessments supersede
earlier ones through explicit relationships; they do not rewrite history.

### Verification precedes reliance on validation results

Empirical agreement is interpretable only after the code path, numerical
solution, inputs, units, consumer path, and output lineage have been verified.
Verification and validation may iterate during development, but a validation
claim cannot close on an unverified result surface.

### Scientific consistency is necessary but not empirical validation

Dimensional consistency, conservation, monotonic response, boundedness, and
contract conformance are powerful correctness evidence. They can invalidate a
model or implementation, and are hard gates where applicable. Alone, they do
not show that openWEPP predicts field behavior with adequate accuracy.

### Comparator agreement is a flag, not a target

ADR-0017 remains controlling. Agreement with the pinned WEPP baseline is
regression or equivalence evidence. A difference is an investigation trigger.
Neither agreement nor disagreement determines physical validity without
independent authority and like-for-like lineage.

### Uncertainty is part of the result

A result without its material uncertainty, applicability, and caveats is an
incomplete result. V&V must distinguish, where relevant:

- measurement and sampling uncertainty;
- natural or aleatory variability;
- input and forcing uncertainty;
- parameter uncertainty;
- conceptual and model-form uncertainty;
- numerical and finite-precision uncertainty; and
- uncertainty introduced by transformations, aggregation, or publication.

Unknown uncertainty is recorded as unknown; it is not encoded as zero.

### Rigor is graded by risk of misuse

The required depth of evidence depends on the consequence of a wrong result,
the influence of openWEPP on the decision, novelty of the application, and
distance from existing evidence. Tailoring may reduce irrelevant work, but it
may not waive a required evidence class without recording the resulting claim
limitation or hold.

### Evidence is designed for challenge

Reproduction, independent review, defect traceability, and explicit gaps are
features of the subsystem. The objective is not to produce a persuasive report;
it is to make the strongest defensible claim and make that claim easy to audit,
dispute, reproduce, and revise.

## Terminology and Evidence Classification

The following classifications are normative for new V&V work. Historical
artifacts retain their names, but must be mapped to these classes when they are
used in a new claim.

| Class | Question | Typical openWEPP evidence |
| --- | --- | --- |
| Requirements verification | Are requirements complete, consistent, traceable, and testable? | `SC-*` reviews, obligation maps, schema and invariant linkage |
| Code verification | Is the specified algorithm implemented correctly? | analytical solutions, manufactured solutions, hand calculations, property tests, independent implementations |
| Solution verification | Is this computed solution numerically adequate? | refinement studies, convergence, timestep sensitivity, iterative residuals, roundoff and determinism studies |
| Integration verification | Are couplings, ordering, units, lineage, and real consumers correct? | direct-path tests, independent closure, serial/parallel equivalence, fail-closed publication tests |
| Conceptual validation | Does the process abstraction adequately represent the real system for the intended use? | literature synthesis, competing hypotheses, process-importance review, expert scientific review |
| Empirical validation | Do predictions agree adequately with independent observations or an accepted real-world referent? | laboratory, plot, hillslope, watershed, and operational observations with uncertainty |
| Uncertainty and sensitivity | What drives the result, and how uncertain is it? | sensitivity analysis, ensembles, uncertainty propagation, regime and structural alternatives |
| Comparative evidence | How does openWEPP differ from another implementation? | pinned legacy comparison, independent solver comparison, alternative-model ensemble |
| Use qualification | Is the evidence profile sufficient for the declared use? | claim assessment, limitations, independent review, release disposition |

The referent determines the meaning of a comparison. Comparison with an exact
solution is verification. Comparison with measured reality is validation.
Comparison with a legacy binary is comparative evidence unless that binary's
result has separate authority for the exact claim.

Calibration is parameter or model adjustment using data. Calibration can be a
legitimate development activity, but fit to calibration data is not independent
validation evidence. Ordinary parser or domain "validation" remains input
validation and must not be counted as model validation.

## Claim-Centered V&V Model

V&V will be represented as a traceable chain:

```text
intended use -> claim -> evidence obligations -> executed evidence
             -> claim assessment -> use/release disposition
```

Defects, limitations, uncertainty, and supersession relationships attach to
that chain. Reports are views of it, not independent narratives.

### Claim requirements

| ID | Requirement |
| --- | --- |
| `VV-CLAIM-001` | Every acceptance statement must reference a stable claim ID and version. |
| `VV-CLAIM-002` | A claim must define its intended use, quantities of interest, application envelope, acceptance criteria, and exclusions before verdict-bearing evidence is run. |
| `VV-CLAIM-003` | A claim must bind exact contract invariants, interfaces, configurations, and output lineage relevant to the result. |
| `VV-CLAIM-004` | A claim must state the consequence of error, model influence on the decision, novelty or extrapolation, and resulting rigor class. |
| `VV-CLAIM-005` | Each required evidence class must be marked `PASS`, `FAIL`, `BLOCKED`, or `NOT_RUN`; absence of evidence is never a pass. |
| `VV-CLAIM-006` | Claim assessment outcomes are `SUPPORTED`, `SUPPORTED_WITH_LIMITATIONS`, `NOT_SUPPORTED`, or `INSUFFICIENT_EVIDENCE`; the unqualified outcome `VALIDATED` is prohibited. |
| `VV-CLAIM-007` | Expanding a claim envelope requires a new claim version and evidence for the newly covered domain. |
| `VV-CLAIM-008` | A claim must expose known counter-evidence, unresolved defects, and material evidence gaps. |

### Rigor classes

The detailed scoring rubric belongs in the future subsystem specification. The
minimum strategy-level classes are:

| Class | Intended role | Minimum posture |
| --- | --- | --- |
| `R0-exploratory` | diagnosis, development, or hypothesis generation | reproducible identity and explicit non-release status |
| `R1-component` | process or component development claims | requirements and code verification plus relevant process evidence |
| `R2-production` | production-path and user-facing scientific results | end-to-end verification, uncertainty treatment, representative validation, and independent review |
| `R3-release` | release-defining use claims or material extrapolation | complete claim profile, application-domain evidence, independent review and verification, release snapshot, and explicit residual-risk decision |

No numeric average of evidence dimensions determines sufficiency. A missing
required factor remains visible and governs the disposition. Maturity profiles
may guide investment, but they are not acceptance oracles.

## Hierarchical Strategy

Complex systems require both bottom-up and top-down evidence. Component evidence
localizes errors; integrated evidence exposes coupling, ordering, scale, and
emergent behavior. Neither substitutes for the other.

### Verification ladder

1. **Requirements and design:** contracts, equations, units, algorithms,
   guards, interfaces, and publication semantics are coherent and traceable.
2. **Code:** implementations reproduce exact, analytical, manufactured,
   independently reconstructed, or otherwise authoritative solutions.
3. **Solution:** each relevant calculation demonstrates adequate numerical
   error, convergence, timestep or resolution behavior, finite-precision
   posture, and determinism where promised.
4. **Coupled execution:** kernels, phase scheduling, hillslope and watershed
   coupling, real consumers, aggregation, parallel execution, and output
   publication preserve the required meaning.
5. **Release realization:** the exact built binaries, runtime dependencies,
   schemas, inputs, and evidence bundle are the verified configuration.

### Validation ladder

1. **Conceptual/process:** the selected process abstraction and governing
   responses are defensible for the intended use.
2. **Separate-effects/component:** individual hydrology, plant, snow/frost,
   hydraulics, erosion, and routing responses are compared with focused
   observations where confounding is controlled.
3. **Integral-effects/subsystem:** coupled hillslope or watershed subsystems are
   compared with experiments or observations that exercise important
   interactions.
4. **Application/field:** end-to-end quantities of interest are evaluated across
   representative soils, climates, topographies, managements, scales, and
   event magnitudes.
5. **Post-audit:** predictions are compared with observations acquired after the
   assessment or release, and the claim is revised when warranted.

Higher-level success cannot erase a failed important process. Lower-level
success cannot establish coupled or application performance.

### Process importance and coverage

Each `R2-production` or `R3-release` claim must include a process and response
importance assessment analogous in purpose to the nuclear Phenomena
Identification and Ranking Table. For openWEPP it must:

- begin with the intended use and quantities of interest;
- identify relevant processes, state transitions, couplings, regimes, and
  scales;
- rank importance and knowledge/evidence gaps separately;
- map each important item to verification, validation, and uncertainty
  obligations; and
- record the scientific basis and review for each ranking.

This assessment prioritizes evidence. It does not convert expert ranking into
correctness authority and does not permit low-ranked mechanisms to violate
conservation or fail-closed requirements.

## openWEPP-Specific Validation Design

### Separate forcing from model response

Validation against field observations must distinguish at least two lanes:

1. **Reconstruction lane:** use measured or quality-controlled forcing to test
   process and response fidelity while minimizing climate-generation error.
2. **Operational lane:** use the forcing products and workflow expected in
   practice to evaluate total predictive performance.

Forcing-robust signatures and forcing-limited magnitudes must remain separate,
consistent with ADR-0028. If forcing uncertainty prevents an absolute-magnitude
verdict, the result is reported but cannot close that magnitude claim.

### Keep calibration and evaluation independent

Dataset roles must be declared before model adjustment:

- development and debugging;
- calibration or parameter estimation;
- validation or held-out evaluation; and
- post-audit or future observations.

Splits must prevent leakage across sites, plots, watersheds, climates,
management histories, or repeated measurements when those dependencies would
inflate apparent generalization. Random row splitting is insufficient when
rows share the same physical system or event history.

### Respect environmental variability

Runoff and erosion observations contain measurement error, sampling effects,
natural variability, and scale mismatch. Replicate-plot variability or another
defensible observation-error model should be used where available. Exact match
to one noisy observation is not the objective, and poor agreement cannot be
excused merely by saying that field data are variable.

### Use quantity- and regime-specific metrics

Acceptance criteria must be selected for the claim, not inherited as universal
thresholds. Evidence should normally include more than one of:

- bias and signed residuals;
- absolute and relative error with declared behavior near zero;
- event magnitude and timing;
- distributional or exceedance behavior;
- seasonal and annual aggregation;
- spatial deposition and erosion patterns;
- rank or management-treatment discrimination;
- conservation and closure; and
- uncertainty coverage and calibration.

One aggregate efficiency statistic cannot hide failed extremes, timing,
regimes, or conservation. Results must be stratified over relevant event size,
climate, soil, management, slope, scale, and topology regimes.

### Address scale and extrapolation explicitly

Plot, hillslope, multi-OFE, channel, and watershed observations do not transfer
automatically. Each validation claim must identify:

- the scale of the referent and simulation;
- aggregation and normalization bases;
- geometry and boundary-condition differences;
- missing or distorted processes;
- support of the data relative to the proposed application domain; and
- whether the conclusion is interpolation, guarded extrapolation, or outside
  the evidence domain.

## Evidence Artifact Requirements

The V&V subsystem will use a machine-readable source of record and generate
human reports from it. The conceptual provenance model should remain compatible
with the W3C PROV ideas of entities, activities, agents, generation, use, and
derivation. A future export may use RO-Crate, but neither standard is imposed as
the initial internal wire format.

### Required artifact types

| Artifact | Purpose |
| --- | --- |
| Claim definition | Versioned statement of intended use, envelope, criteria, and obligations |
| Evidence plan | Scenario matrix, process-importance map, data partition, metrics, and required runs |
| Dataset record | Origin, rights, collection method, uncertainty, transformations, partition role, and hashes |
| Execution manifest | Exact software, binary, environment, command, inputs, outputs, metrics, logs, and status |
| Assessment | Evidence-to-claim reasoning, applicability, uncertainty, limitations, findings, and disposition |
| Review record | Reviewer identity or role, independence, scope, findings, and finding disposition |
| Release snapshot | Immutable set of current claims, evidence, limitations, binaries, schemas, and hashes |
| Human report | Generated summary of supported uses, gaps, uncertainty, defects, and reproduction pointers |

### Evidence record requirements

| ID | Requirement |
| --- | --- |
| `VV-EVID-001` | Every execution record must bind claim and obligation IDs, source commit, dirty-state status, binary hashes where applicable, tool and schema versions, and host/runtime context. |
| `VV-EVID-002` | Inputs, fixtures, observational datasets, transformation code, outputs, and retained logs must be content-identified or explicitly marked unavailable. |
| `VV-EVID-003` | Metrics must carry units, quantity lineage, scale, aggregation basis, uncertainty, acceptance-criterion ID, and exact evaluation status. |
| `VV-EVID-004` | Dataset provenance must cover collection, quality control, transformations, calibration/evaluation role, rights, and known limitations. |
| `VV-EVID-005` | Evidence must identify the real production consumer for direct-path or publication claims and independently reconstruct conservation-sensitive outputs. |
| `VV-EVID-006` | Failed, blocked, invalidated, and superseded evidence must remain discoverable and must not contribute to a current pass. |
| `VV-EVID-007` | A rerun cannot silently replace a failure; the new record must link to the prior record and state why the earlier result is not current. |
| `VV-EVID-008` | Large or restricted payloads may remain external, but the tracked manifest must preserve stable location, access classification, content hash, and transformation provenance. |
| `VV-EVID-009` | Producer-derived expected values are insufficient where an independent reconstruction, analytical solution, measurement, or alternative implementation is practicable. |
| `VV-EVID-010` | Machine and human outputs must be rendered from the same assessment and evidence records; manually maintained duplicate verdict tables are prohibited. |

### Evidence freshness and invalidation

Evidence is current only for its recorded dependency set. Claim definitions must
identify relevant source modules, contracts, schemas, fixtures, metric code,
and runtime/build configuration. A change-impact tool should classify evidence
as:

- `current`: no material dependency changed;
- `review-required`: a dependency changed but automated impact is uncertain;
- `stale`: a material dependency changed; or
- `superseded`: a newer accepted assessment replaced it.

Passing historical evidence may inform planning but cannot close a release gate
when stale. Conversely, unrelated documentation edits should not force an
expensive scientific rerun merely because the commit SHA changed; the dependency
and review record must explain reuse.

## Governance Requirements

| ID | Requirement |
| --- | --- |
| `VV-GOV-001` | Canonical `SC-*` contracts remain the authority for intended openWEPP physics; V&V artifacts provide evidence and use qualification, not replacement authority. |
| `VV-GOV-002` | Every V&V campaign must be authorized by a work package with a fixed claim set, source/configuration freeze policy, intended write set, evidence plan, and exact exit states. |
| `VV-GOV-003` | Acceptance criteria and dataset roles must be frozen before verdict-bearing execution; later changes require a new plan version and bias review. |
| `VV-GOV-004` | Verification, validation, comparative evidence, uncertainty, and software QA must be reported as distinct dimensions. |
| `VV-GOV-005` | Important negative results, extrapolations, missing process evidence, and known defects must be visible in machine and human reports. |
| `VV-GOV-006` | Review independence and scientific expertise must scale with rigor class; `R3-release` requires independent scientific review and independent evidence verification. |
| `VV-GOV-007` | No weighted score or pass-count majority may override a failed mandatory obligation. |
| `VV-GOV-008` | A comparator delta verdict must preserve ADR-0017's like-for-like and independent-authority burden of proof. |
| `VV-GOV-009` | Calibration against evaluation data, post hoc threshold changes, selective case removal, fixture editing, and tolerance relaxation are closure-blocking unless the claim is reset and independently reassessed. |
| `VV-GOV-010` | V&V defects must link into the existing defect-closure workflow and remain associated with every affected claim until fixed, bounded, or retired. |
| `VV-GOV-011` | External-authority level, test coverage, and V&V maturity are separate concepts and must not be substituted for one another. |
| `VV-GOV-012` | Public language must match evidence: use "verified," "supported for," "corroborated over," or "not assessed" with the named envelope; avoid whole-model validity claims. |

## Target Code and Tooling Architecture

The V&V subsystem must be integrated with openWEPP without placing filesystem,
reporting, or acceptance-policy concerns inside process kernels.

### Target boundaries

| Boundary | Responsibility |
| --- | --- |
| `docs/specifications/subsystems/verification-validation/` | Canonical subsystem contract, schemas, status vocabulary, and claim/evidence rules |
| `crates/openwepp-vv` | Typed IDs, manifests, criteria results, provenance relationships, validation of records, and deterministic report inputs |
| `tools/vv/` or a dedicated CLI | Campaign planning, execution adapters, evidence collection, impact analysis, linting, assessment assembly, and report rendering |
| Domain test adapters | Analytical, manufactured-solution, property, refinement, comparator, observational, and production-consumer harnesses |
| Work-package and release artifacts | Immutable execution manifests, reviews, assessments, reports, and release snapshots |

Exact names may change during subsystem specification. The ownership boundaries
are the strategy requirement.

### Code integration requirements

| ID | Requirement |
| --- | --- |
| `VV-CODE-001` | Process kernels must not depend on evidence storage, report rendering, network access, or release policy. |
| `VV-CODE-002` | Production surfaces needed for V&V must expose typed, unit-bearing state or outputs through normal consumers or explicitly diagnostic interfaces; no hidden cwd sentinels. |
| `VV-CODE-003` | Evidence collection must observe the production path and must not change physics, solver choices, ordering, or tolerances unless the claim explicitly assesses that alternative. |
| `VV-CODE-004` | Schema parsing and status transitions must fail closed on unknown versions, missing required provenance, invalid units, malformed criteria, or contradictory verdicts. |
| `VV-CODE-005` | Mechanical criteria may be evaluated automatically; qualitative scientific adequacy and use qualification require an explicit assessment and review record. |
| `VV-CODE-006` | The toolchain must support a dry planning mode, an execution mode, an assessment mode, and deterministic machine/human report generation. |
| `VV-CODE-007` | The subsystem must query evidence by claim, contract invariant, process, quantity, dataset, source path, release, status, regime, and freshness. |
| `VV-CODE-008` | Report generation must be reproducible from retained manifests without rerunning the scientific simulation. |
| `VV-CODE-009` | Current external-authority registries and work-package evidence must be migrated or referenced; the subsystem must not create a second conflicting authority ladder. |
| `VV-CODE-010` | The subsystem itself requires schema tests, migration tests, round-trip tests, anti-evasion tests, and evidence-lineage verification. |

The first implementation should prefer a small, versioned JSON or YAML schema
and typed Rust model over a database service. A database or RO-Crate export can
be added after the object model and audit requirements prove stable.

## Human and Agent Reports

The same evidence graph serves two views.

An agent-facing report must provide stable IDs, exact statuses, unmet
obligations, dependency paths, reproduction commands, hashes, machine-readable
metrics, and permitted next actions. It should allow an agent to answer: "What
claim is affected by this change, what evidence is stale, and what exact work
would restore support?"

A human-facing report must lead with:

1. the intended uses that are supported and not supported;
2. source/binary/configuration identity;
3. the application envelope and data coverage;
4. the verification, validation, uncertainty, and review profile;
5. material biases, failures, defects, and extrapolations;
6. the meaning and consequence of residual uncertainty; and
7. links to reproducible evidence rather than raw log dumps.

Reports must show the profile by evidence dimension. A single badge, maturity
score, or aggregate pass rate may be a navigation aid, but cannot replace the
profile or hide the weakest required dimension.

## Relationship to Current openWEPP Governance

This strategy extends rather than replaces the repository's strongest current
controls:

- `SC-*` contracts already provide traceable requirements and science
  authority.
- ADR-0017 correctly prevents comparator agreement from becoming validation.
- ADR-0028 already separates observed-data admission, forcing-robust evidence,
  calibration avoidance, and conservation.
- external-authority suites already preserve citations, fixture hashes,
  provenance, lane posture, and anti-evasion obligations.
- work packages already require frozen scope, direct evidence, independent
  reconstruction, consumer-path proof, review, and verification.
- the integrated validation campaign demonstrates fixed-source end-to-end
  production, conservation, publication, fail-closed, and release evidence.

Under the new taxonomy, much of the current integrated campaign is strong
**integration and release verification** plus selected science-authority
evidence. It is not by itself broad empirical validation of openWEPP predictions
across application regimes. Historical status names need not be rewritten, but
future release claims must not infer that broader meaning from them.

The principal gaps this subsystem must close are:

- no canonical claim and application-envelope registry;
- inconsistent use of the word "validation" across input checking, contract
  conformance, comparative testing, and empirical evaluation;
- no unified code/solution/integration/conceptual/empirical evidence profile;
- limited systematic numerical solution verification;
- fragmented machine provenance and hand-maintained human summaries;
- no dependency-aware evidence freshness model; and
- insufficient cross-regime, held-out empirical validation for several public
  quantities of interest.

## Adoption Strategy

### Phase 1: Canonicalize claims and language

- Author the V&V subsystem specification and a short ratifying ADR.
- Define claim, obligation, dataset, execution, assessment, and release-snapshot
  schemas with stable IDs and status vocabularies.
- Inventory current evidence without relabeling it more strongly than its
  referent permits.
- Select one representative pilot claim spanning a process, a production
  consumer, and an observed quantity.

Exit condition: the pilot has a reviewed claim envelope and evidence plan, and
existing evidence can be classified without ambiguity.

### Phase 2: Implement the evidence spine

- Add typed schema support, linting, evidence ingestion, supersession, and
  report generation.
- Connect current contract tests, external-authority suites, release gates, and
  work-package artifacts through adapters rather than rewrites.
- Generate the pilot's machine manifest and human report from one source.
- Implement dependency-aware freshness and fail-closed schema validation.

Exit condition: a source change can identify affected pilot obligations, run or
reuse evidence with review, and generate non-divergent machine and human views.

### Phase 3: Deepen verification and process validation

- Build analytical, manufactured-solution, property, refinement, and numerical
  uncertainty coverage for important kernels and solvers.
- Build the process-importance and hierarchical validation matrix for major
  domains.
- Admit observational datasets through explicit QA, uncertainty, rights,
  partition, and transformation records.
- Run separate-effects and subsystem pilots before expanding to integrated
  claims.

Exit condition: each release-relevant process has an explicit evidence profile
or a visible, owned gap; component and coupled claims no longer rely primarily
on legacy agreement.

### Phase 4: Integrated application and release qualification

- Execute fixed-source hillslope and watershed campaigns across representative
  regimes and scales.
- Quantify sensitivity, uncertainty, and guarded extrapolation for public
  quantities of interest.
- Generate an immutable release V&V snapshot and independent assessment.
- Establish a post-audit process for new field and operational observations.

Exit condition: every proposed release use is `SUPPORTED` or
`SUPPORTED_WITH_LIMITATIONS` at `R3-release`, or is explicitly excluded from
the release claim. No high-importance failed or insufficient obligation is
hidden by an aggregate result.

## Eventual Release Basis

An openWEPP release may be technically buildable before it has a sufficient V&V
basis. Release qualification requires more than green CI.

At minimum, the release V&V snapshot must provide:

- exact source, binary, dependency, schema, and runtime identities;
- supported intended uses and explicit exclusions;
- claim-by-claim verification, validation, uncertainty, applicability, and
  review profiles;
- process-importance coverage and owned gaps;
- current code, solution, integration, and release-realization verification;
- representative empirical evidence for public scientific quantities of
  interest, including extremes and important regimes;
- calibration/evaluation separation and dataset pedigree;
- uncertainty and sensitivity treatment proportional to the claim;
- unresolved-defect and limitation registers;
- independent scientific review and evidence verification; and
- reproducible machine artifacts and a generated human report.

The release statement should say what openWEPP is supported for, over what
domain, with what uncertainty and limitations. It should not claim that the
entire model has been proven valid.

## Prohibited Shortcuts

The following cannot close a V&V claim:

- test count, line coverage, or lint success standing in for scientific
  evidence;
- exact agreement with a value calculated by the same producer and operands;
- legacy parity standing in for physical validity;
- conservation standing in for empirical performance;
- empirical fit standing in for code or solution verification;
- calibration and evaluation on the same effective data;
- one favorable aggregate metric hiding failed regimes or quantities;
- a report without source, binary, input, and transformation provenance;
- rerunning until green without preserving and explaining failures;
- carrying evidence to a changed model, schema, or application without an
  impact assessment; or
- labeling missing evidence as not applicable merely because it is expensive.

## Research Basis

This strategy adapts, rather than adopts wholesale, practices from high-
consequence and environmental modeling:

- [NRC Regulatory Guide 1.203](https://www.nrc.gov/docs/ML0535/ML053500170.pdf)
  provides the intended-use-first evaluation-model process: determine required
  capability and important phenomena, establish an assessment base, assess
  separate and integrated behavior, address scaling and uncertainty, use
  configuration control and independent review, and document the whole model.
- [NASA-STD-7009B](https://standards.nasa.gov/standard/NASA/NASA-STD-7009)
  separates code and solution verification, conceptual and empirical
  validation, permissible use, data pedigree, uncertainty, results assessment,
  defects, and reporting over the model life cycle.
- [EPA environmental-model guidance](https://www.epa.gov/sites/production/files/2015-04/documents/cred_guidance_0309.pdf)
  emphasizes that model quality is meaningful only for an application,
  evaluation is continuous, corroboration is not proof of truth, observational
  uncertainty matters, and independent data are needed to judge robustness.
- [ASME V&V 20](https://www.asme.org/codes-standards/find-codes-standards/standard-for-verification-and-validation-in-computational-fluid-dynamics-and-heat-transfer)
  anchors comparison at specified validation variables and points, with
  uncertainty in both simulation and experiment, and treats interpolation or
  extrapolation beyond those points as an additional engineering judgment.
- [Sandia's V&V and predictive-capability report](https://doi.org/10.2172/809603)
  motivates PIRT-like prioritization, code and solution verification,
  hierarchical validation, numerical error estimation, statistical validation
  metrics, and coordination between simulation and experiment.
- [Sandia's Predictive Capability Maturity Model](https://doi.org/10.2172/976951)
  separates representation, physics fidelity, code verification, solution
  verification, validation, and uncertainty while warning that maturity does
  not itself establish application acceptance.
- [The VERA-CS V&V plan](https://www.ornl.gov/publication/vera-cs-verification-validation-plan-0)
  demonstrates hierarchical multiphysics practice: establish acceptable
  evidence for single-physics components before relying on coupled-system V&V,
  while automating standardized result tables and figures.
- [Nearing's erosion-model evaluation method](https://doi.org/10.1002/1096-9837%28200008%2925%3A9%3C1035%3A%3AAID-ESP121%3E3.0.CO%3B2-B)
  shows why model-observation differences must be interpreted relative to
  variability in measured erosion data.
- [Wang et al.'s multi-regime WEPP evaluation](https://doi.org/10.1016/j.iswcr.2022.10.004)
  illustrates the value of large cross-climate, soil, topography, and management
  datasets, separate calibrated and uncalibrated results, multiple temporal
  scales, and explicit failure at event extremes.
- [W3C PROV-O](https://www.w3.org/TR/prov-o/) and the
  [RO-Crate specification](https://www.researchobject.org/ro-crate/specification.html)
  provide useful provenance and research-object concepts for connecting
  entities, activities, agents, workflows, software, data, and human-readable
  views without making prose the only audit surface.

Repository bibliography entries `R-114` through `R-124` record these sources,
their roles, and their rights posture.

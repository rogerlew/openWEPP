# ASSURE-05 Internal Domain-Science Review

Date: 2026-07-16

Review role: coding-agent internal scientific assurance, from a hydrology and
model-evaluation perspective

Evidence class: Static + limited ran reconstruction

Disposition: **HOLD-REVISION-AND-EVIDENCE**

This is an internal defect-finding review. It is not external peer review,
human scientific approval, publication approval, or application-fitness
approval.

## Review Charge And Identity

The charge was to examine scientific correctness, evidence classification,
claim bounds, prior-study attribution, method adequacy, uncertainty and
limitations, and the eight-question minimum useful-publication test. The review
covered the manuscript, supplement, source manifest, preregistered protocol,
operand lineage, fresh execution evidence, retained results, reproduction
procedure, focused test implementations, production consumer adapter, and
`SC-GWBASEFLOW-001`.

The principal reviewed identities were:

| Object | SHA-256 |
| --- | --- |
| `manuscript.md` | `7cbb158edef0d44b788e35feb9520cc20401fa0bac65b335377c611572f2c295` |
| `supplement.md` | `13adcca3f760434301b7341ae2ebd360b7d1ad92e9e1334d951dd7799534b163` |
| `report.yaml` | `3ba828d7ec61b27b796a9d18d6f92548f08fe455f658653b133f4d08e86b66c6` |
| `study-protocol.md` | `ec747399ea66b868b03fbac5ec003ded94129f5d7e7393fcc9b3fbf484557491` |
| `operand-lineage.md` | `0704d730050c9b245b5cabefe87e82cf56583ed892fb1921376f8c774544b010` |
| `assure05-production-evidence.json` | `54829b7198769ee9180938a98d4d118e1991a6984264f744e8d615bcf45ebee6` |
| `two-day-recurrence.json` | `8d6659d9e60de5c9dace531cbe2d3f74df3e7a5dd9f1b64be4430449cfc4c9d8` |
| `h2637-ledger.json` | `95e7bbbce3f4a1c8b7f67a9189b0c02f010658d9539c2b3a77811ecc48ff1a56` |
| `SC-GWBASEFLOW-001.md` | `97ee00e87df4a87221aa34fc1f44c77176f43922bcfac96c69d4b6de8e230d60` |

Static inspection included the named focused tests and the production
`HbpLatestEventState` to `HillslopeContribution` adapter. Ran reconstruction was
limited to the retained Python procedure. Both analytical and H2637 outputs were
semantically equal to their retained result JSON. H2637 reconstruction used the
still-present transient explicit-active files; no Rust test suite or production
run was independently rerun for this review.

The prior-study check used the primary
[U.S. Forest Service record](https://research.fs.usda.gov/treesearch/43824)
and its [paper PDF](https://research.fs.usda.gov/download/treesearch/43824.pdf).

## Overall Verdict

The draft is scientifically careful in its central distinction: it reports
software and integration verification, not empirical validation. The recurrence
equations, two-day expected values, H2637 ledger arithmetic, coefficient guard,
and most uncertainty and transfer limitations are stated correctly. The cited
Priest River performance values and fitted coefficients are also accurate.

The report is nevertheless not ready for scientific or publication disposition.
Three major evidence-to-claim defects remain: the headline analytical residual
is not an openWEPP implementation residual, the asserted end-to-end production
consumer traversal was not executed as a continuous chain by the retained fresh
tests, and the H2637 reproduction procedure depends on transient files that are
not durable research objects. A prior-study period attribution and a units
statement also require correction.

## Findings

### DS-01 — Major: the headline analytical residual is not an implementation residual

**Evidence.** The manuscript uses `1.7763568394002505e-15 m3` to support the
claim that openWEPP reproduced the independently calculated vector. The source
manifest calls it the “maximum absolute implementation residual,” and the figure
caption calls it the “maximum implementation residual.” The independent Python
procedure does not ingest any Rust/openWEPP observations. Its `observed` values
are calculated by the procedure itself with Python binary64 arithmetic, while
its `expected` values are calculated by the same procedure with `Decimal`.
Consequently, the retained residual measures Python binary64-versus-decimal
arithmetic for the published equations.

The focused Rust test independently provides useful implementation evidence: it
executes `DirectGroundwaterRunState`, asserts the six expected state/export
values, and uses an absolute `1.0e-12` tolerance. However, the retained focused
result contains only the passing-test count, not the Rust observations or their
residuals. The exact `1.7763568394002505e-15 m3` value therefore cannot be
attributed to openWEPP from the retained evidence.

**Scientific consequence.** The qualitative two-day timing verification remains
supported at the Rust test's `1.0e-12 m3` assertion level. The tighter headline
number, its “implementation residual” classification, and prose saying that the
analytical case “matched within” that number overstate the evidence. This affects
a key finding, abstract result, result definition, and figure.

**Recommendation.** Retain machine-readable openWEPP observations for every
two-day state and export, and have an independent procedure compare those values
with the decimal oracle. Report the resulting implementation residual. If that
evidence is not added, relabel `1.7763568394002505e-15 m3` exclusively as the
independent procedure's binary64-versus-decimal arithmetic residual and state
only that the Rust test passed its `1.0e-12 m3` per-value allowance.

### DS-02 — Major: the claimed continuous production consumer traversal is not freshly demonstrated

**Evidence.** The main report states that generated groundwater volumes
“traversed the production hillslope-pass and watershed-consumer path” and that
the focused checks passed HBP fields “through the strict parser and into the
watershed linear-reservoir branch.” The retained fresh tests prove adjacent
components, but not that continuous execution:

- the HBP writer test starts from a synthetic publication frame, serializes
  nonzero baseflow and deep seepage, parses the bytes, and stops at the parsed
  payload;
- the watershed consumer test manually constructs `HillslopeContribution` with
  nonzero groundwater fields and proves branch, threshold, and `cbase`
  separation behavior; and
- the actual production adapter in `openwepp-cli-watershed` that copies parsed
  `HbpLatestEventState` fields into `HillslopeContribution` is not exercised with
  nonzero groundwater by the seven retained fresh tests.

The H2637 execution is a hillslope active-owner test rather than a watershed CLI
consumer run, and its deep-seepage coefficient is zero. Static source inspection
shows the intended adapter assignments, so the missing link is plausible, but
static plausibility plus seam-isolated tests is not the same evidence class as an
executed end-to-end production transfer. A defect in the adapter could leave all
seven selected tests passing.

**Scientific consequence.** Serialization/parsing and watershed-kernel
consumption are individually supported. The stronger claim that nonzero
generated baseflow and deep seepage traversed the real production producer,
HBP, adapter, and watershed consumer is not closed by the current fresh evidence.
That distinction is material under the report standard's real-consumer rule.

**Recommendation.** Add and freshly run a test that begins with claim-bearing
producer/publication output or its production HBP, passes nonzero baseflow and
deep seepage through the production pass inventory and CLI adapter, executes the
watershed branch, and asserts the final generated-baseflow and deep-seepage
diagnostics. Include `cbase`, threshold, authority, and no-event timing checks as
appropriate. Alternatively, narrow the manuscript and `GW-P06` to component and
static bridge evidence and do not say the volumes traversed the complete
production path.

### DS-03 — Major: the retained H2637 reproduction path depends on temporary, unpreserved files

**Evidence.** The H2637 procedure requires three inputs: the produced manifest,
HBP, and pass-Parquet files. The supplement directs a researcher to use retained
paths recorded by the execution evidence. The normalized evidence instead says
that raw binary outputs were not committed, records only filenames and hashes,
and delegates transient absolute paths to the package execution record. Those
files currently exist under `/tmp/laned_shadow_h2637_active_on_473038`, but a
temporary run directory is not a preservation repository.

The retained `h2637-ledger.json` is sufficient to inspect the reported operands
and residuals, but the current procedure cannot regenerate it from durable
public-safe inputs in the repository. The prose can instruct a fresh rebuild and
rerun, but it does not provide a deterministic wrapper that returns the newly
created paths and performs the comparison, and this is different from retaining
the exact claim-bearing acquisition objects. The source standard explicitly
states that temporary run directories are not preservation repositories.

**Scientific consequence.** An informed reader can understand and manually
check the arithmetic, but cannot execute the documented H2637 reproduction
procedure against the exact retained acquisition after the temporary directory
is removed. Minimum useful-publication question 8 is therefore not yet answered
as claimed.

**Recommendation.** Preserve the small public-safe manifest, HBP, and Parquet
acquisition objects in a version-bound research-object location, or provide a
durable normalized manifest input that contains every procedure operand plus the
original file identities and make the procedure support it. If regeneration is
the intended policy, add one deterministic command that rebuilds, runs, locates
the outputs, reconstructs the result, and compares it with the retained object;
revise the text so it no longer claims that unpreserved paths are retained.

### DS-04 — Moderate: the Priest River calibration/evaluation period is mischaracterized

**Evidence.** The manuscript describes the cited results as belonging to a
“calibrated 2005-2009 application.” Srivastava et al. state that observed daily
streamflow from 2005-2006 was used for calibration and the remaining 2007-2009
record was used for model-performance evaluation. The reported overall values
of NSE `0.672` versus `0.570`, runoff-volume deviation about `7%` versus `47%`,
and fitted coefficients `0.0156 d^-1` and `0.00026 d^-1` are correctly rounded
and attributed, but the period description erases the calibration/evaluation
split. The overall statistics span both portions and are not evaluation-only
statistics.

**Scientific consequence.** This does not affect the openWEPP verification, but
it understates the prior study's design while also risking ambiguity about what
the overall performance statistics represent.

**Recommendation.** State that the coupled model was calibrated on 2005-2006,
evaluated on 2007-2009, and that the cited overall 2005-2009 metrics combine the
full period. Keep the existing warning that none of these values are openWEPP
performance results.

### DS-05 — Minor: storage and daily integrated flux volumes are assigned the same dimensional wording

**Evidence.** The model-formulation section says, “All storage and flux terms are
cubic metres per hillslope day.” `SC-GWBASEFLOW-001` instead defines `S_i` as a
storage state in `m3` and `D_i`, `Qb_i`, and `Qs_i` as volumes in `m3` integrated
over the current daily timestep. Only `kb` and `ks` have inverse-day units. The
supplement's “hillslope-day volumes” wording is less problematic but could still
be read as a rate.

**Scientific consequence.** The equations and numeric calculations use the
intended one-day volume convention, so this is a presentation defect rather than
an arithmetic error. It can nevertheless cause a hydrologist to interpret
`Qb_i` as `m3 d^-1` while the pass field is a daily integrated `m3` value.

**Recommendation.** Say that `S_i` is storage in `m3`; `D_i`, `Qb_i`, and `Qs_i`
are daily integrated volumes in `m3` over one hillslope timestep; and `kb` and
`ks` are `d^-1`, with the fixed one-day interval implicit in the discrete
release equations.

## Evidence And Interpretation Assessment

The following aspects are adequately bounded and should be preserved during
revision:

- The daily prior-export recurrence matches `SC-GWBASEFLOW-001`, and the two-day
  expected values are arithmetically correct.
- Both H2637 terminal reconstructions reproduce the retained result. Their nearly
  identical residuals are expected because the post-export identity is the
  pre-export identity with terminal exports debited; they are two timing views of
  one cumulative ledger, not two independent hydrologic corroborations.
- H2637 is correctly classified as deterministic integration and conservation
  evidence rather than an observation-based watershed evaluation.
- The report explicitly distinguishes floating-point closure from error against
  nature and does not infer parameter transferability or predictive accuracy.
- The zero H2637 deep-seepage coefficient and the resulting limit on production
  evidence are disclosed in the main limitations section.
- Missing authority, over-export, `cbase`, lateral subsurface flow, active surface
  routing, and latest-event aliases are treated as distinct concerns rather than
  collapsed into groundwater baseflow.
- Environmental, forcing, measurement, parameter, model-form, subdaily, and
  alternate-formulation uncertainties are accurately described as unevaluated.
- The report remains visibly `DRAFT` and correctly says coding-agent review cannot
  provide human scientific or publication approval.

The H2637 closure is a strong self-consistency check on published cumulative and
terminal operands. It should continue to be described as bookkeeping and
integration evidence. It does not, by itself, verify every daily H2637 recurrence
value or exclude compensating day-level errors; the two-day vector supplies the
direct timing check for the tested recurrence.

## Minimum Useful-Publication Test

| Question | Assessment | Basis |
| --- | --- | --- |
| 1. Why was the study needed? | Yes | The software-before-empirical-evaluation rationale is clear. |
| 2. What exact process, quantity, scale, domain, and realization were assessed? | Yes, with minor correction | The recurrence, synthetic case, H2637 fixture, parameters, daily scale, and commit are recoverable; fix the units wording. |
| 3. What referent and method were used, and why are they appropriate? | No | The decimal oracle is appropriate, but the exact residual is not an implementation comparison and the continuous consumer method is not executed as claimed. |
| 4. What quantitative results were observed? | Partial | The vector and ledger values are inspectable, but the headline implementation residual is misclassified. |
| 5. What do the results mean in relation to prior knowledge? | Partial | The conditional interpretation is good; correct the Priest River calibration/evaluation split. |
| 6. What important evidence contradicts or limits the conclusion? | Yes | Empirical absence, deterministic-fixture scope, zero `ks`, daily resolution, and transfer limits are prominent. |
| 7. What may and may not be inferred for another application? | Yes | The report clearly rejects predictive, parameter-transfer, and site-fitness inference. |
| 8. Where can the study be reproduced or challenged? | No | Analytical reproduction is durable; exact H2637 procedure inputs remain only in a transient directory. |

The report does not pass the eight-question test in its current form.

## Required Disposition

Before this internal review can be dispositioned as resolved:

1. correct or replace the analytical implementation-residual claim and its
   result metadata;
2. add real continuous consumer-path execution evidence or narrow every affected
   claim consistently;
3. make the H2637 reproduction inputs or deterministic regeneration workflow
   durable and truthful;
4. correct the Priest River calibration/evaluation attribution; and
5. correct the storage and daily-volume units language.

After those changes, rerun quantitative reconstruction and focused evidence,
rebind all affected source and research-object identities, and obtain a new
internal domain-science review. Named human scientific, reproduction,
publication-steward, assurance, and release decisions remain separate required
steps.

## Remediation Verification — 2026-07-16

Review mode: coding-agent internal remediation verification

Evidence class: Static + ran lightweight reconstruction

This section appends to, and does not erase, the original defect record. It
reviews only the requested DS-01 through DS-05 remediations. It is not external
peer review, human scientific approval, publication approval, or application-
fitness approval.

### Renewed Review Identity And Checks

The remediated principal source identities were:

| Object | SHA-256 |
| --- | --- |
| `manuscript.md` | `3bc9b1fed50387e1efb33d1af18bdf331008ff29beb6b542e2a741def767fc79` |
| `supplement.md` | `9f6ba23f228a74822f54d0fbfd855e242e00b5ecd85bc449b6efe28b067f6562` |
| `report.yaml` | `ffbda40acfa6c8663c79d18e7a56b66bf26febbeef4c372fda61590f1f4ce3cf` |
| `assure05-production-evidence.json` | `4618a1491ecaf3e473564f7efc8f4df2250cb1068969b4984f80a6f297996e14` |

Lightweight ran checks used the repository's retained Python procedure, not a
new Rust or production execution:

- analytical mode output was semantically equal to the retained
  `two-day-recurrence.json`;
- H2637 mode accepted the repository-retained manifest, HBP, and pass-Parquet
  files and produced output semantically equal to `h2637-ledger.json`;
- retained H2637 object digests were
  `756e324e5b4f055ea45c33b0d5f679ab2fc9f4b958e853dc0b70f17aeb592208`
  for the manifest,
  `378a8c1d80a22c9452fb256cf9a95eab09035f3a6cd387c6d626ab26c426c453`
  for the HBP, and
  `915f3b99c2ff20e3e0632b4e90a6ceb1cb8e7fee58f0d3e29b41de10c540f550`
  for the pass-Parquet file; and
- those three digests agree across the files, normalized production evidence,
  and `report.yaml` research-object bindings.

### DS-01 — Resolved

The remediated source no longer attributes the
`1.7763568394002505e-15 m3` value to observed openWEPP output. The key findings,
abstract, method, result semantics, table, figure, and supplement now classify it
as the independent procedure's binary64-versus-decimal arithmetic residual.
They separately state that the Rust recurrence test passed its `1.0e-12 m3`
assertion allowance and explicitly decline to infer an observed Rust-oracle
residual. This is scientifically faithful to the retained evidence.

Disposition: **RESOLVED BY CLAIM AND EVIDENCE-CLASS CORRECTION.** No new Rust
observation object was added, and none is now claimed.

### DS-02 — Resolved By Scope Narrowing; Integration Gap Remains Open

The remediated source consistently describes separate adjacent-interface tests:
the production writer/strict-parser test and the hand-constructed watershed-
consumer test. The key findings, abstract, methods, evidence classification,
results, discussion, limitations, conclusion, supplement claim map, `GW-P06`,
and `GW-METHOD-CONSUMER` all disclose that no fresh nonzero groundwater payload
traversed the actual CLI adapter in one execution. The positive claim is now
limited to interface-contract behavior rather than complete production-path
traversal.

Disposition: **RESOLVED BY CLAIM NARROWING.** The missing end-to-end adapter test
remains a declared future integration obligation; it no longer invalidates the
bounded interface conclusion.

### DS-03 — Resolved

The exact accepted H2637 manifest, HBP, and pass-Parquet files are now durable
report research objects under `evidence/h2637/`. `report.yaml` binds each object,
the supplement names them as procedure inputs, and normalized execution evidence
records both their paths and identities. The reproduction procedure executed
successfully against those repository paths and regenerated the retained H2637
result semantically.

Disposition: **RESOLVED BY DURABLE OBJECT RETENTION AND EXECUTABLE
RECONSTRUCTION.** The original dependence on `/tmp` is removed from the public
reproduction path.

### DS-04 — Resolved

The manuscript now states that Srivastava et al. calibrated on 2005-2006 data,
evaluated on 2007-2009, and reported the cited overall statistics across the full
2005-2009 period. It retains the important boundary that those fitted
coefficients and performance statistics belong to the cited coupled-model study,
not openWEPP.

Disposition: **RESOLVED BY CORRECT PRIOR-STUDY ATTRIBUTION.**

### DS-05 — Resolved

The manuscript now defines storage in `m3`, defines `D`, `Qb`, and `Qs` as daily-
integrated `m3` volumes, and assigns inverse-day units only to `kb` and `ks`. The
supplement uses the same storage-versus-daily-volume distinction.

Disposition: **RESOLVED BY DIMENSIONALLY PRECISE WORDING.**

### Renewed Useful-Publication Assessment

The DS-01 correction restores a faithful answer to questions 3 and 4: the reader
can distinguish the independent arithmetic referent from the Rust assertion and
can tell exactly what each quantitative value measures. The DS-04 correction
restores the calibration/evaluation context required by question 5. Durable,
executable H2637 research objects now answer question 8. The openly declared CLI
adapter gap remains visible under questions 6 and 7 rather than being hidden in
an affirmative transfer claim.

No unresolved DS-01 through DS-05 defect remains in the remediated source.

### Superseding Internal-Agent Verdict

**INTERNAL-AGENT-REVIEW-CLEAR-FOR-REQUIRED-HUMAN-REVIEW.** The original
`HOLD-REVISION-AND-EVIDENCE` is superseded for DS-01 through DS-05 by the
remediation evidence above. This verdict means only that the identified internal
domain-science defects were corrected or honestly scoped. It does not close the
declared end-to-end CLI adapter evidence gap, authorize public release, establish
empirical validation, supply application-fitness judgment, or substitute for any
named human scientific, reproduction, publication-steward, assurance, or release
decision.

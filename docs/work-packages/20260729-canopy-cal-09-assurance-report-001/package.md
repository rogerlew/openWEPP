# CAL-09 / CANOPY-ASSURE-01 Native-Forest Canopy-Phenology Assurance Report

Status: `executed hold — source complete; canonical V2 admission and
accountable human approval unavailable`

Evidence mode: `retrospective synthesis + deterministic reconstruction + Ran
reproduction and publication checks`

Intent: `scientific model evaluation, independent reproduction, and assurance
publication; no new calibration or process implementation`

This ExecPlan is a living document governed by `docs/codex_exec_plans.md`.
Keep `Progress`, `Surprises & Discoveries`, `Decision Log`, and `Outcomes &
Retrospective` current throughout execution.

## Purpose / Big Picture

Turn the completed native-forest canopy-phenology implementation and CAL-01
through CAL-07F research record into a conventional scientific
model-evaluation report. A hydrologist, forest manager, model developer, or
WEPP user should be able to understand what was evaluated, reproduce the
material results, see favorable and contrary evidence together, and judge
whether the evidence is relevant to a named application.

`CAL-09` is the work-package name for roadmap Order 9. Its scientific
publication product is `CANOPY-ASSURE-01`, with proposed V2 report ID
`native-forest-canopy-phenology-evaluation`.

The package must not read as a chronological digest of CAL work packages.
Those packages supply evidence and provenance. The manuscript is organized
around the scientific question:

> Does openWEPP's native-forest canopy formulation produce internally correct
> seasonal states, and where is that behavior corroborated, bounded, or
> contradicted by available evidence?

## Objective

Author and bind a manuscript-first V2 assurance source consisting of a main
report, technical supplement, strict result objects, public-safe research
objects, accessible tables and figures, and complete reproduction procedures.
Independently reconstruct every material quantitative result and advance the
source only as far through review, approval, publication, and release transfer
as authenticated human authority permits.

If independent accountable human scientific and reproduction/publication
approvals are unavailable, execution must finish truthfully at
`HOLD-HUMAN-APPROVAL`. Internal agent review cannot substitute for human
approval or authorize a public report.

## Scientific Claim Envelope

The report evaluates:

- the weather-and-latitude to GSI, foliage, LAI, canopy, litter, residue, and
  downstream-consumer chain;
- mathematical, implementation, mass, state, chronology, and real-consumer
  verification;
- Hubbard Brook calibration and partial identifiability;
- independent Harvard timing transfer and its contrary result;
- litter-source and decomposition calibration readiness;
- within-model canopy-gradient congruence and bounded snow/frost response;
- synthetic hemispheric phase behavior and independent Southern Hemisphere
  observational behavior;
- the Bezà tropical dry-forest chronology contradiction and stop-loss;
- Elliot-source reproduction as comparative evidence; and
- current downstream evidence for interception, ET, snow, frost, runoff, and
  erosion.

Every conclusion must preserve process, quantity, units, temporal and spatial
support, tested domain, software realization, referent, method, uncertainty,
limitation, and application boundary. No aggregate verdict may erase a
contradicted site, forest class, season, observation product, or missing
process.

## Included Scope

- Freeze a claim-evidence matrix before drafting claim-bearing prose.
- Freeze the figure and table plan with the operator before result-bearing
  asset production.
- Content-identify every admitted predecessor result and distinguish
  `CALIBRATION`, `INDEPENDENT_VALIDATION`, `DIAGNOSTIC_ONLY`,
  `LEGACY_COMPARISON`, `MODEL_OUTPUT`, and `SOFTWARE_VERIFICATION`.
- Reconstruct a compact strict result object from retained evidence using a
  deterministic, independently executable procedure.
- Author a scientific manuscript following
  `docs/standards/scientific-model-evaluation-report.md`.
- Author a technical supplement containing detailed methods, identities,
  parameter tables, complete negative evidence, and reproduction instructions.
- Bind claims, methods, datasets, results, values, tables, figures, references,
  agent assistance, and public-safe research objects in a production-domain V2
  descriptor.
- Register the source in `assurance/v2/catalog.yaml` without exposing a draft
  through public `usersum`.
- Validate, plan, normalize-check, build, and check in unrelated disposable
  staging roots, with byte-for-byte deterministic output.
- Independently reproduce every material result and audit all numerical,
  conservation, and transformation claims against source operands.
- Complete two independent internal reviews, disposition every finding, and
  complete two terminal verifications.
- When authenticated human authority is available, follow the V2 lifecycle
  through exact-root approval, release transfer, and publication. Otherwise
  preserve the public boundary and close on hold.

## Excluded Scope

- New canopy, phenology, litter, decomposition, snow, frost, hydrology, or
  erosion process physics.
- New calibration, refitting, parameter search, or reopening the CAL-07F
  stop-loss.
- New literature- or observation-acquisition campaigns.
- Treating search grids, examples, single-site fits, or accepted ensembles as
  universal physiological or typical ranges.
- Predictive evergreen needle or fine-woody litter-source laws.
- Using downstream residuals to select upstream canopy parameters.
- Treating Elliot numerical outputs or legacy agreement as correctness
  authority.
- Hiding unavailable erosion outputs, the Harvard SWE identity contradiction,
  poor Harvard transfer, or the Bezà contradiction in pooled summaries.
- Application approval for a particular site, management decision, or accuracy
  requirement.
- Assigning human roles, competence, approval, or signatures without
  authenticated participation.
- Publishing a draft, hand-copying staging output into tracked `usersum`, or
  bypassing review locks and release-transfer controls.
- Production Rust, science-contract, schema, fixture, or runtime changes
  without a reviewed package amendment.

## Manuscript Spine

The main report uses this scientific sequence:

1. title, authorship, key findings, plain-language summary, and abstract;
2. introduction and bounded evaluation question;
3. model formulation and forcing-to-consumer chronology;
4. evaluation design, sites, evidence roles, methods, and verdict vocabulary;
5. implementation, mass, state, and real-consumer assurance;
6. temperate calibration, identifiability, and independent transfer;
7. litter sources, decomposition, and forest-floor state;
8. canopy-gradient and bounded downstream evaluation;
9. Southern Hemisphere synthetic and observational evaluation;
10. Elliot comparison and retained staged methodology;
11. integrated claim matrix;
12. discussion;
13. limitations and application guidance;
14. conclusions;
15. open research, reproduction, references, and report metadata.

The detailed section contract lives in
`artifacts/manuscript-and-supplement-outline.md`.

## Deliverables

1. Production-domain V2 source under
   `assurance/v2/reports/native-forest-canopy-phenology-evaluation/`.
2. `manuscript.md`, `supplement.md`, and a schema-valid `report.yaml`.
3. Deterministic reconstruction procedures and strict result objects.
4. Complete public-safe research-object manifest and retained objects.
5. Accessible, question-driven result figures with source data, generation
   procedure, captions, and text or tabular alternatives.
6. Machine-readable claim-evidence and operand-lineage records.
7. Review, finding-disposition, verification, approval-boundary,
   deterministic-build, and release-transfer evidence.
8. Roadmap, work-package catalog, and internal assurance-source catalog
   updates appropriate to the achieved lifecycle state.

## Intended Write Set

- `docs/planning/canopy-phenology-assurance-roadmap.md`
- `docs/work-packages/README.md`
- `docs/work-packages/20260729-canopy-cal-09-assurance-report-001/**`
- `assurance/v2/catalog.yaml`
- `assurance/v2/README.md`
- `assurance/v2/identity.lock.json`
- generated transaction records created by the typed V2 workflow
- `assurance/v2/reports/native-forest-canopy-phenology-evaluation/**`
- disposable staging, logs, and scratch outputs outside tracked public trees

The following remain read-only unless the package is prospectively amended and
the amendment is reviewed before edits:

- `usersum/**`, including `usersum/assurance/**`;
- `assurance/generated/**` and legacy `assurance/catalog.yaml`;
- production Rust and tests;
- V2 schemas, builder code, principals, and existing report sources;
- canopy science contracts, runtime code, fixtures, and predecessor packages;
- export, release-snapshot, vendor, and WEPPcloud trees.

Publication to an explicit external root is a lifecycle operation, not
permission to edit tracked public paths. If authenticated lifecycle authority
requires a source path not listed above, amend the write set before touching
it.

## Dependencies

### Communication and lifecycle authority

- `docs/standards/scientific-model-evaluation-report.md`
- `docs/governance/scientific-assurance-v2-architecture.md`
- `docs/governance/scientific-assurance-dossier-lifecycle.md`
- `docs/governance/scientific-assurance-v2-source-build-contract.md`
- `docs/decisions/0038-manuscript-first-scientific-assurance-publication.md`
- `assurance/v2/README.md`
- `assurance/v2/schemas/report.schema.json`

### Scientific and narrative authority

- `docs/planning/canopy-phenology-assurance-roadmap.md`
- `usersum/openwepp-canopy-phenology.md`
- `SC-PLANT-001`
- `SC-RESIDUE-001`
- `SC-INFILE-MANAGEMENT-YAML-001`
- downstream consumer contracts actually cited by the report

### Evidence

- completed `CANOPY-PHENOLOGY-01` and `CANOPY-PHENOLOGY-02`;
- completed CAL-01 through CAL-07F package evidence;
- retained source-authority, observation, calibration, reconstruction,
  gradient, Southern Hemisphere, review, and verification objects;
- the exact active source realization selected during Phase 1.

Work-package artifacts provide evidence and provenance; they do not replace
science contracts or primary scientific authority.

## Progress

- [x] (2026-07-29) Operator selected the high-level manuscript argument and
  authorized CAL-09 scaffolding.
- [x] (2026-07-29) Scaffolded the package, kickoff prompt, manuscript outline,
  claim-map seed, and provisional figure-planning surface.
- [x] (2026-07-29) Opened a bounded figure-candidate phase and built eight
  deterministic time-series-forward candidate SVGs with sidecars and retained
  source bindings; F9 was dropped, F10 became the integrated claim table, and
  F6 was selected in principle.
- [x] (2026-07-29) Reviewed and froze the scientific figure and table plan
  after operator inspection and requested layout corrections.
- [x] (2026-07-29) Froze the report question, claims, evidence roles, exact realization,
  and protected public baseline.
- [x] (2026-07-29) Reconstructed the strict synthesis result from seven retained
  CSV sources and proved byte-equal repeated execution.
- [x] (2026-07-29) Authored the manuscript, supplement, schema-valid report
  descriptor, references, tables, figures, and public-safe research objects.
- [x] (2026-07-29) Confirmed canonical V2 validation/build is blocked: the tool has typed
  operations for existing reports but no typed new-report admission operation;
  direct schema, reference, path, arithmetic, and existing-catalog checks pass.
- [x] (2026-07-29) Reproduced the strict result and figure set; canonical
  canopy-specific disposable staging is not runnable before typed admission.
- [x] (2026-07-29) Completed two internal reviews, full finding disposition,
  two independent terminal verifications, exact-diff reconciliation, and
  truthful executed-hold disposition.
- [x] (2026-07-29) Recorded that authenticated human approvals and release
  transfer are unavailable; preserved the internal draft and public boundary.

## Execution Phases

### Phase 1 — Intake, Protected Boundary, And Study Freeze

Record the exact worktree and assessed realization, protected public baseline,
instruction chain, evidence inventory, report-decomposition decision, evidence
roles, claim envelopes, uncertainties, and exclusions. Confirm that one
integrated report can keep verification, calibration, independent evaluation,
comparison, and downstream response separate. Split before drafting if it
cannot.

Acceptance: the study protocol, evidence inventory, claim matrix, realization
freeze, and protected-public baseline are complete and content-identify every
claim-bearing input.

### Phase 2 — Figure, Table, And Result Contract Freeze

Finalize `artifacts/figure-plan.md` with the operator. For each selected figure
and table, state the scientific question, source rows, quantities, units,
aggregation, uncertainty, sample count, intended comparison, caption point,
accessibility alternative, and generation procedure. Freeze the set before
building the strict result object so the procedure produces the exact operands
needed by prose, tables, and figures without ornamental assets or duplicated
numbers.

Acceptance: every planned asset answers a scientific question, has identified
source data, and is mapped to at least one bounded claim. Unselected candidates
are visibly rejected or deferred with rationale.

### Phase 3 — Deterministic Scientific Reconstruction

Create a standard-library or otherwise repository-authorized deterministic
procedure that reads only declared, content-identified evidence and emits
strict result objects. Reconstruct rather than transcribe material values.
Preserve raw operands, units, sample counts, missing-value semantics,
observation products, calibration/evaluation roles, and adverse cases.

For mass, closure, and transformation claims, retain operand lineage, rejected
aliases, independent reconstruction, and real magnitude or closure evidence.
Exact self-consistency and one-sided bounds are supporting evidence only.

Acceptance: fresh procedure output exactly matches retained results; every
claim-bearing number, table row, and figure datum resolves to a result identity;
an independent reproduction reaches the same values without relying on the
manuscript's reported summaries.

### Phase 4 — Manuscript, Supplement, And V2 Binding

Author the manuscript using the frozen scientific sequence. Keep the broad
model explanation concise and link logically to the native-forest narrative.
Place extended methods, parameter tables, failed cases, full acceptance cells,
identities, and commands in the supplement. Bind all claims, methods, values,
tables, figures, references, and research objects in the V2 descriptor.

Lead with quantitative findings and ordinary scientific language. Preserve the
poor Harvard transfer result, the predictive litter-source gap, bounded
canopy-gradient evidence, unavailable erosion output, Harvard SWE exclusion,
and Bezà ecosystem-model limitation. Do not convert lifecycle state into a
scientific conclusion.

Acceptance: the report validates as production-domain `DRAFT`; all numerical
directives resolve; the report can be understood without reading YAML or work
packages; and every safely redistributable project-owned claim-bearing object
is declared for the public research surface.

### Phase 5 — Reproduction, Reader Audit, And Internal Review

Run report validation and planning, normalize-check American English, then
build and check in two unrelated disposable staging roots. Compare complete
trees byte-for-byte. Independently reproduce material results and conduct a
reader audit from hydrology, forestry, model-development, and WEPP-user
perspectives.

Complete one internal domain-science review and one independent
reproduction/publication review. Disposition every finding as `accepted`,
`rejected`, `deferred`, or `follow-up`; correct and recheck all accepted
findings. Then complete two terminal verifications.

Acceptance: deterministic staging, numerical reproduction, accessibility,
link portability, research-object completeness, validation, review,
disposition, and verification all pass with no undispositioned finding.

### Phase 6 — Human Authority, Release Transfer, And Disposition

If authenticated, competent, independent humans accept the defined roles,
follow the V2 event and lock workflow for review entry, findings, approvals,
assurance-steward authorization, and release-owner transfer. Bind all decisions
to the exact report, supplement, evidence, result, figure, reference, agent
packet, software realization, and release roots. Run the deterministic public
build and release verification only when those prerequisites pass.

If those prerequisites are unavailable, do not manufacture them. Preserve the
source as an internal draft, prove the public boundary is unchanged, and close
the package at `HOLD-HUMAN-APPROVAL` with exact next human actions.

Acceptance: either the exact approved report is release-transferred and
published through the governed external-root workflow, or the package records
a truthful hold with no public exposure.

## Validation Plan

The exact terminal diff determines the final command set. At minimum, retain:

1. deterministic execution of the package reconstruction procedure;
2. exact comparison of fresh and retained strict result objects;
3. independent reconstruction of all conservation and transformation claims;
4. `cargo run --quiet -p openwepp-assurance -- validate --report
   native-forest-canopy-phenology-evaluation`;
5. `cargo run --quiet -p openwepp-assurance -- plan --report
   native-forest-canopy-phenology-evaluation`;
6. the active typed American-English normalization check for the draft;
7. two unrelated disposable `build` and `check` runs plus byte comparison;
8. report-specific focused assurance V2 and publication-contract tests selected
   directly under `docs/standards/testing-and-gate-strategy.md`;
9. Markdown, link, catalog, schema, placeholder, research-object, and generated
   drift checks;
10. protected-public hash and file-inventory comparison;
11. `git diff --check` and exact-diff/write-set reconciliation;
12. dual internal reviews, complete finding disposition, dual terminal
    verification, and `.rs` line-count governance; and
13. campaign-closure or release-transfer commands only when the achieved
    lifecycle boundary actually selects them.

No Rust, schema, or builder gate is selected merely because the repository
contains Rust. If the terminal diff touches those surfaces after an authorized
amendment, apply the conservative requirements selected by the testing
strategy. Critical campaign/release and heavy full-workspace runs must use the
required comparator-suite runner when available.

## Exit Criteria

- The manuscript and supplement answer the bounded scientific question using
  the frozen outline and active report standard.
- Every material claim has recoverable process, quantity, units, scale, domain,
  realization, referent, method, result, uncertainty, limitation, and
  application boundary.
- Verification, calibration, independent evaluation, comparative evidence,
  downstream response, release transfer, and application fitness remain
  distinct.
- Every quantitative statement, table, and figure resolves to retained source
  data and a deterministic generation or reconstruction procedure.
- Favorable, adverse, unavailable, and development-influenced evidence remain
  visible without pooled erasure.
- The Elliot record remains a bounded comparative result and methodological
  precedent, not correctness authority or the manuscript's organizing spine.
- The report does not reopen calibration or obscure the CAL-07F stop-loss.
- Research objects are complete, public-safe, licensed or restriction-marked,
  portable, and reproducible.
- All current-scope validation has direct evidence; no failed or blocked
  increment gate is retroactively deferred.
- Two independent internal reviews, complete finding disposition, and two
  terminal verifications pass.
- Exact terminal diff matches the declared write set and lifecycle state.
- Publication occurs only with valid independent human approvals and exact
  release transfer; otherwise the draft and public boundary are preserved and
  the package closes on explicit human-approval hold.

## Security And Production Impact

The scaffold changes documentation only. Later execution may add internal V2
scientific sources and public-safe research objects. It must not expose
protected evidence, secrets, personal data, restricted locations, absolute
workspace paths, or unlicensed material. Symlinks and undeclared external paths
are forbidden in the report source and public research surface.

No production model behavior, defaults, inputs, schemas, or scientific
contracts change. Public assurance, export, snapshot, vendor, and WEPPcloud
surfaces remain unchanged until the exact governed publication boundary passes.

## Delegated Review Authorization

Subagent authorization: this package explicitly authorizes
spawning/delegating to one internal domain-science reviewer, one independent
reproduction/publication reviewer, and two terminal verifier subagents for
read-only review of the final report source, retained results, research
objects, reproduction evidence, lifecycle boundary, and package disposition.
Expected outputs are compact findings, commands checked, numerical
reconstructions, and review text returned for the named package artifacts;
write access is read-only.

If campaign/release closure, a broad comparator, or full-workspace correctness
run is selected, this package explicitly authorizes and requires a
`comparator_suite_runner` subagent for those heavy commands, with writes
limited to scratch space and package log artifacts. No subagent may create or
claim human identity, competence, peer review, approval, release authority, or
application fitness.

Higher-precedence session policy may still require explicit user authorization
before spawning. If unavailable during execution, record that boundary and use
an allowed local equivalent only when package governance permits it.

## Idempotence And Recovery

All reconstruction and build steps are deterministic and offline. Write
generated scientific objects to temporary paths and replace retained objects
only after schema and equality checks pass. Disposable staging roots may be
deleted and rebuilt. Use typed V2 transactions for identity and lifecycle
changes; never repair drift by manually copying hashes or locks.

Preserve unrelated user changes. Do not recover from a failed publication by
copying draft output into tracked `usersum`, weakening validation, or fabricating
authority.

## Surprises & Discoveries

- Observation: the active V2 schema renders only
  `linear_magnitude_bars`; the reviewed CAL-09 scientific assets are
  time-series SVGs.
  Evidence: `assurance/v2/schemas/report.schema.json` fixes the visualization
  enum, while the operator-reviewed F1-F8 objects are retained SVGs.
- Observation: the active CLI cannot admit a new report into generated
  identity state.
  Evidence: `openwepp-assurance --help` exposes typed amendments for existing
  reports but no add/admit operation; after adding the catalog entry,
  `validate --report native-forest-canopy-phenology-evaluation` fails first on
  the stale catalog identity. Hand-editing `identity.lock.json` would violate
  the package and V2 transaction contract.
- Observation: leaving the unadmitted catalog row would invalidate the
  existing V2 source root without admitting the canopy report.
  Evidence: removing the row restores `validate --all` for both existing
  reports, while canopy-specific validation truthfully reports an unknown
  report ID.

## Decision Log

- Decision: use `CAL-09` as the work-package name and
  `CANOPY-ASSURE-01` as the scientific product identity.
  Rationale: this preserves the operator's requested sequence while retaining
  the roadmap's established report name.
  Date/Author: 2026-07-29, operator/Codex.
- Decision: plan one integrated process-chain report, subject to a Phase 1
  split test.
  Rationale: canopy state connects foliage, litter, and downstream consumers,
  but the report must split if distinct datasets, scales, or conclusions cannot
  remain legible.
  Date/Author: 2026-07-29, Codex.
- Decision: freeze figures and tables with the operator before strict result
  construction and manuscript drafting.
  Rationale: assets should answer scientific questions and determine retained
  result operands, rather than being selected after conclusions are written.
  Date/Author: 2026-07-29, operator/Codex.
- Decision: retain Elliot reproduction as a compact comparative section.
  Rationale: its staged methodology informed the study, but its numerical
  outputs are not native correctness or calibration authority.
  Date/Author: 2026-07-29, Codex.
- Decision: drop the Elliot figure and communicate its result through prose or
  a compact table; convert the claim-domain map from a figure to the integrated
  claim table.
  Rationale: the operator prioritized model time-series trends,
  observed-versus-simulated comparisons, and coefficient effects.
  Date/Author: 2026-07-29, operator/Codex.
- Decision: build candidate figures before freezing the final inventory.
  Rationale: several layout and aggregation choices cannot be judged reliably
  without seeing the retained series.
  Date/Author: 2026-07-29, operator/Codex.
- Decision: select F1, F3, F4, F5, F6, and F8 for the main report; retain F2
  and F7 in the supplement.
  Rationale: this keeps coefficient response, process propagation,
  observed-versus-modeled timing, equifinality, canopy/snow response, and the
  product-consistent Bezà contradiction in the main causal spine without
  losing the broader seasonal context.
  Date/Author: 2026-07-29, operator/Codex.
- Decision: declare the reviewed time-series figures as version-bound
  research objects and include one schema-native Harvard range figure.
  Rationale: preserving the scientific assets is preferable to changing the
  schema or flattening time-series evidence into bars.
  Date/Author: 2026-07-29, Codex.
- Decision: do not hand-edit or locally synthesize a successor V2 identity
  lock for new-report admission.
  Rationale: generated identity and receipt state is owned by typed
  transactions; no current typed operation authorizes this transition.
  Date/Author: 2026-07-29, Codex.
- Decision: retain the complete canopy source outside the V2 catalog until a
  typed new-report admission operation exists.
  Rationale: this preserves the work while keeping the existing admitted V2
  catalog and identity root valid.
  Date/Author: 2026-07-29, Codex.

## Outcomes & Retrospective

Current outcome: CAL-09 has a complete manuscript, technical supplement,
deterministically reproduced 32-value strict result, 39 public-safe research
objects, eight deterministic time-series figures with linked sidecars, and a
schema-valid production-domain draft descriptor. Source-level gates and
domain-science review pass.

Canonical report-specific validation, planning, staging, build, and check
cannot begin because the typed V2 workflow cannot admit a new report. The
source is therefore retained outside the catalog; existing admitted reports
still validate, protected public surfaces are unchanged, and no human approval
or publication is claimed. The next action is a separately authorized
new-report admission capability, followed by canonical build and accountable
human scientific and reproduction/publication review.

# CANOPY-CAL-01 Source And Target Ledger

Package ID: `20260726-canopy-cal-01-source-target-ledger-001`

Status: `COMPLETE / PASS`

Date opened: `2026-07-26`

Execution mode: `package-end-to-end`

Package type: scientific campaign intake, source preservation, and
characterization authority.

This ExecPlan is a living document maintained under
`docs/codex_exec_plans.md`. The `Progress`, `Surprises & Discoveries`,
`Decision Log`, and `Outcomes & Retrospective` sections must remain current
during execution.

## Purpose / Big Picture

The canopy-phenology campaign needs one inspectable record of what Bill Elliot
received, assumed, changed, ran, and reported before any native openWEPP
parameter can be calibrated from his work. After this package, a reader can
distinguish field observations, Bill-derived assumptions, WEPP management
operands, chart-derived estimates, and model outputs; verify the preserved
source hashes; and determine exactly what evidence CANOPY-CAL-02 may reproduce.

This package changes no model behavior. It establishes evidence authority and a
fail-closed reproduction-admission record.

## Progress

- [x] (2026-07-26) Scaffolded the package from operator direction and the
  dedicated canopy-phenology assurance roadmap.
- [x] (2026-07-26) Operator supplied seven literature PDF/Markdown files and a
  clean clone of the Hubbard Brook synthesis book; amended the package before
  source installation to include a separate literature reference subtree.
- [x] (2026-07-26) Admitted base
  `f56c3fb541784903bdf6c7df6428fa43f44e42a2`, reconciled the inherited
  instruction set, and recorded the intent/gate plan before installing sources.
- [x] (2026-07-26) Preserved and independently hash-verified the commissioned
  Elliot sources, supplied literature, synthesis-book sources, and two
  officially acquired USDA Treesearch papers.
- [x] (2026-07-26) Audited the report's numeric targets, assumptions, model operands, and
  outputs into a typed ledger.
- [x] (2026-07-26) Audited primary-source lineage and comparison scales,
  excluding misclassified, unsourced, and cross-boundary quantities.
- [x] (2026-07-26) Resolved the CAL-02 input inventory and issued
  `READY_BOUNDED`; exact reproduction is not authorized.
- [x] (2026-07-26) Completed independent review and disposition; all scientific
  findings were corrected, with third-party publication rights retained as an
  explicit package-level hold.
- [x] (2026-07-26) Completed two independent terminal verifications and final
  exact-diff reconciliation; both returned `PASS_WITH_PUBLICATION_HOLD`.
- [x] (2026-07-26) Operator subsequently confirmed redistribution permission
  for the retained third-party sources and directed commit/push; the
  publication hold is superseded.

## Objective

Bind the commissioned April 2026 William J. Elliot report and management files
to exact source identities; produce a unit-, scale-, material-, and
provenance-aware target ledger; preserve the Hubbard Brook `dropfc=0.92` versus
delivered `0.95` discrepancy as two explicit branches; exclude unsourced or
cross-scale values from validation authority; and issue a deterministic
admission record for CANOPY-CAL-02.

## Rationale

Bill's analysis contains valuable process targets, but its report, management
files, literature context, manual parameter choices, and screenshots do not all
have the same evidentiary status. Reproduction started without a typed ledger
could silently turn an assumption into an observation, compare unlike spatial
scales, or choose one side of the 92/95 discrepancy. A dedicated intake package
makes those errors mechanically visible before model runs begin.

## Context And Orientation

The governing plan is
`docs/planning/canopy-phenology-assurance-roadmap.md`. Its first ordered package
requires a source and target ledger before reproduction or calibration.

The commissioned sources are currently preserved in the sibling WEPPcloud
repository:

```text
../wepppy/docs/work-packages/
  20260626_deciduous_mixed_forest_managements/references/
```

That directory contains:

- `bill_elliot_2026_modeling_hardwood_and_mixed_forests_in_wepp.pdf`;
- `bill_elliot_2026_hardwood_forest.man`; and
- `bill_elliot_2026_santee_mixed_forest.man`.

The WEPPcloud reference README records redistribution permission and exact
SHA-256 identities. The openWEPP campaign will preserve exact source bytes
under `references/canopy_phenology/elliot_2026/` so later assurance evidence
does not depend only on a mutable sibling checkout. The PDF must use the
repository's existing `references/**/*.pdf` Git LFS rule.

Bill's report gives field-context targets for Hubbard Brook and Santee, then
iteratively changes perennial WEPP parameters until long-run live biomass and
current/previous/old residue resemble those targets. Some reported quantities
are direct literature values, some are Bill's assumptions, some are approximate
readings from figures, and some hydrology/sediment comparisons mix hillslope and
watershed scales. These categories must never share one undifferentiated
"observed" label.

## Included Scope

- Copy the three commissioned source files byte-for-byte into the declared
  openWEPP reference directory with normalized filenames.
- Move the operator-supplied literature PDF/Markdown files from `~/Downloads`
  into a separate openWEPP literature directory, preserving source-native bytes
  and normalized bibliographic filenames.
- Preserve the relevant source-native Hubbard Brook synthesis-book chapter,
  bibliography, repository identity, and license/provenance metadata from the
  clean clone at `../synthesisbook`.
- Acquire the remaining references cited by Bill from legitimate primary,
  publisher, DOI, institutional, or author-hosted sources when they are not
  already present in openWEPP or the operator-supplied set.
- Record source and destination SHA-256 identities, sizes, media types,
  copyright/redistribution statement, and provenance.
- Verify Git LFS handling for the PDF before it is staged.
- Extract every material numeric quantity needed by CAL-02, including units,
  spatial and temporal basis, site, material class, source citation, uncertainty
  or chart-resolution status, and evidence role.
- Classify every quantity as one of:
  `FIELD_OBSERVATION`, `LITERATURE_CONTEXT`, `BILL_DERIVED_ASSUMPTION`,
  `MANAGEMENT_OPERAND`, `MODEL_OUTPUT`, `CHART_DIGITIZED`, or
  `UNSOURCED_CONTEXT`.
- Record the source-to-management parameter mapping for `beinp`, `dropfc`,
  `spriod`, `oratea`, `orater`, `bb`, `xmxlai`, canopy height, root depth,
  root mass, plant spacing, and stem diameter.
- Preserve separate Hubbard Brook `dropfc=0.92` and `dropfc=0.95` branches.
- Distinguish leaf, needle, twig/branch, fine-woody, and aggregate litter
  wherever the evidence permits; record `UNRESOLVED_MATERIAL_CLASS` otherwise.
- Record comparison scale and process boundary for hillslope surface runoff,
  watershed discharge, lateral flow, baseflow, hillslope sediment delivery,
  and channel/watershed sediment.
- Inventory all run artifacts required for exact or bounded reproduction.
- Issue a machine- and human-readable CAL-02 admission verdict.

## Excluded Scope

- Running WEPP or openWEPP.
- Reproducing Bill's charts or reported results.
- Selecting native canopy parameters.
- Fitting GSI thresholds, canopy operands, litter sources, or decomposition.
- Amending `SC-PLANT-001`, `SC-RESIDUE-001`, management schemas, production
  code, or public output schemas.
- Treating Bill's management parameters or unsourced contextual values as field
  observations.
- Silently resolving the `0.92`/`0.95` discrepancy.

## Dependencies

- Operator direction on 2026-07-26.
- `docs/planning/canopy-phenology-assurance-roadmap.md`.
- Completed `20260717-canopy-phenology-gsi-kernel-001`.
- Completed `20260719-canopy-phenology-native-integration-001`.
- The commissioned sources and permission record in the sibling WEPPcloud
  repository.

If a source hash does not match the WEPPcloud ledger, stop source installation,
retain the mismatch evidence, and issue `HOLD-SOURCE-IDENTITY`. Do not
canonicalize, re-export, or repair the file.

## Declared Write Set

- `docs/work-packages/README.md`
- `docs/work-packages/20260726-canopy-cal-01-source-target-ledger-001/**`
- `references/canopy_phenology/**`

All production code, science contracts, tests, existing fixtures, existing
references, and the sibling WEPPcloud repository are read-only.

## Required Deliverables

Execution must create and maintain:

- `artifacts/source-manifest.json`
- `artifacts/source-provenance.md`
- `artifacts/target-ledger.csv`
- `artifacts/target-ledger-schema.md`
- `artifacts/parameter-lineage.md`
- `artifacts/comparison-scale-ledger.md`
- `artifacts/discrepancy-register.md`
- `artifacts/missing-source-bundle.md`
- `artifacts/cal02-admission.json`
- `artifacts/cal02-admission.md`
- `artifacts/intent-plan.md`
- `artifacts/gate-evidence.md`
- `artifacts/review-agent-a.md`
- `artifacts/review-agent-b.md`
- `artifacts/finding-disposition.md`
- `artifacts/verification-agent-a.md`
- `artifacts/verification-agent-b.md`
- `artifacts/final-disposition.md`

The installed reference directory must contain its own README and SHA-256
manifest. The commissioned and literature subtrees must remain distinct so a
later user cannot mistake Bill's report for independent authority.

## Phase Plan

1. Admit governance, source inventory, intent plan, and exact write boundary.
2. Preserve and hash-verify commissioned, operator-supplied, synthesis-book,
   and legitimately acquired literature source bytes.
3. Build and validate target, parameter, scale, and discrepancy ledgers.
4. Audit primary-source lineage and classify comparison admissibility,
   explicitly recording references that remain unavailable.
5. Inventory reproduction inputs and issue the CAL-02 admission verdict.
6. Reconcile the exact diff, review, verify, and disposition the package.

## CAL-02 Admission Verdict

`artifacts/cal02-admission.json` must emit exactly one verdict:

- `READY_EXACT`: Bill's exact management, climate, soil, slope, run controls,
  constant-cover comparator, executable/version identity, and machine outputs
  are present or independently reconstructable without judgment.
- `READY_BOUNDED`: enough exact input and target evidence exists for a declared
  bounded reconstruction, but one or more source-native run artifacts are
  missing. Every substitute and resulting claim limit is enumerated.
- `BLOCKED_SOURCE_BUNDLE`: execution would require inventing, silently
  converting, or guessing a load-bearing run input or comparison surface.

`READY_BOUNDED` is not exact reproduction authority. CAL-02 must carry the same
scope limitation. `BLOCKED_SOURCE_BUNDLE` prevents CAL-02 execution until a
new exact admission record supersedes it.

## Plan Of Work

### Milestone 1: source identity and preservation

Read the WEPPcloud permission/hash ledger, independently hash each source, copy
exact bytes into the declared openWEPP reference directory, and re-hash the
destinations. Verify the PDF is represented through Git LFS and that the
management files remain ordinary text objects. A reader must be able to compare
source and destination hashes without trusting the copy procedure.

### Milestone 2: target and parameter ledger

Inspect the report visually and by text extraction. Record every quantity used
to establish biomass, litterfall, forest-floor mass, canopy, LAI, runoff,
sediment, or return-period claims. Bind each row to a report page, cited source,
unit, scale, material class, evidence role, and admissibility. Cross-check the
two management files rather than transcribing report prose alone.

### Milestone 3: primary-source and scale audit

Trace load-bearing literature values to retained bibliographic identities.
Separate hillslope from watershed outputs and exclude AI-attributed or
untraceable numbers from calibration and validation. Record unresolved
questions instead of converting them into point targets.

### Milestone 4: reproduction admission

Inventory the exact files and software identity needed to rerun Bill's Windows
analysis and WEPPcloud comparisons. Compare that inventory with what is
available. Emit one deterministic CAL-02 admission verdict and a plain-language
explanation. Then complete review, verification, and final disposition.

## Concrete Steps

Work from the openWEPP repository root.

1. Run `tools/agents/find-agents --for` over the exact terminal write set and
   update `artifacts/required-reading-map.md`.
2. Admit an intent plan before installing source bytes.
3. Use `shasum -a 256` or an equivalently recorded SHA-256 tool on both sibling
   sources and installed destinations.
4. Inspect the report using the repository-supported PDF render/extraction
   workflow; retain page references but do not commit temporary page renders.
5. Validate the target CSV against its documented schema and required
   enumerations.
6. Independently reconstruct the CAL-02 admission verdict from the source
   inventory.
7. Run documentation lint/validation, reference-manifest verification, Git LFS
   checks, local-link checks, placeholder scans, and `git diff --check`.
8. Complete two independent reviews, finding disposition, and two independent
   terminal verifications.

Exact commands and outputs must be recorded in `artifacts/gate-evidence.md` as
the package is executed; this scaffold does not claim they have run.

## Exit Criteria And Validation

The package may close `COMPLETE / PASS` only when:

- all three installed reference hashes exactly match the commissioned source
  ledger;
- the PDF is under the intended LFS rule and no pointer/source confusion
  remains;
- every load-bearing numeric target has units, site, time basis, area/scale
  basis, material class, source/page, evidence role, and admissibility;
- approximate chart readings are explicitly uncertain and never represented as
  exact machine outputs;
- report prose and delivered management operands are cross-checked;
- the `0.92` and `0.95` Hubbard branches remain separately executable;
- leaf litter is not silently equated with Bill's added twig/branch transfer;
- unsourced or AI-attributed values are excluded from calibration/validation;
- hillslope, watershed, and channel quantities remain distinct;
- CAL-02's required and missing source artifacts are enumerated;
- the machine and human admission verdicts agree and are independently
  reconstructable;
- every selected documentation, manifest, LFS, path, and security gate passes;
- two independent reviews and two independent verifications have no
  undispositioned finding; and
- the exact terminal diff is contained by the declared write set.

Rust tests, workspace Clippy, Nextest, coverage, and CRAP are `NOT APPLICABLE`
unless execution amends the package before source work to authorize Rust or test
changes. This documentation/reference intake package must not run broad Rust
gates merely to create evidence volume.

## Security Impact

No executable, network, authentication, secret, or runtime behavior is changed.
Treat imported files as untrusted data: reject symlinks, special files, path
escape, unexpected media types, HTML error pages, and hash mismatches. The PDF
and management files are read-only evidence; no embedded content is executed.

Security-impact gate: source inventory, file type, path confinement, hash
identity, and absence of credentials must pass before closeout.

## Delegation

Subagent authorization: this package explicitly authorizes spawning/delegating
to two independent read-only reviewers and two independent read-only terminal
verifiers for source/target classification, comparison-scale review,
admission-verdict reconstruction, and final path/hash validation; expected
outputs are compact package artifacts with exact paths and findings; write
access is read-only. No heavy comparator or workspace gate is selected.

## Idempotence And Recovery

Source installation is idempotent only when hashes match. Re-running may
replace no existing file unless its bytes are already identical. On mismatch,
retain both identities in package evidence and stop; never overwrite a
conflicting reference. Ledger generation must use stable row keys and ordering
so reruns either produce identical bytes or a reviewable source-driven diff.

## Surprises & Discoveries

- Observation: the operator supplied both the scanned 1987 WEPP user
  requirements and a Markdown transcription, plus one uncited 2022 Santee
  hydro-meteorology paper useful for comparison-boundary context.
  Evidence: pre-installation media-type, metadata, and SHA-256 inventory.
- Observation: preliminary text inspection indicates the `7.6 Mg/ha` Yang et
  al. value is standing foliage biomass rather than annual leaf fall.
  Evidence: Yang et al. Table 2, visually and textually checked, labels the
  quantity `Foliage Biomass (Mg ha-1)`; it is excluded as annual-flux authority.
- Observation: Coates et al.'s `58/42` split is basal-area composition, not
  total-fuel composition, and Gresham's litterfall totals include woody and
  reproductive material.
  Evidence: Coates Table 2 and Gresham abstract/measurement definitions.
- Observation: the delivered Santee management retains the same
  `oratea=orater=0.0021` as Hubbard even though the report says residue decay
  was greater in the warmer Santee climate.
  Evidence: exact delivered management operands and report page 22.
- Observation: the existing Hubbard `p10.man` fixture is seasonal deciduous
  with leaf-off winter retention, not constant cover.
  Evidence: fixture management header and canopy-retention operand; the CAL-02
  admission now uses it only as a baseline control.
- Observation: Bill's WEPPcloud daily-return table is labeled hill streamflow
  and includes a lateral-flow explanation, so it is not hillslope surface
  runoff.
  Evidence: report pages 20–21 and the corrected 12-row return-period boundary.

## Decision Log

- Decision: make CAL-01 a non-model-changing source and characterization
  package.
  Rationale: reproduction and calibration must not manufacture authority while
  inventorying evidence.
  Date/Author: 2026-07-26 / Codex from operator direction.
- Decision: vendor exact commissioned bytes into openWEPP rather than rely only
  on a sibling checkout.
  Rationale: assurance research objects need stable identities and independent
  repository availability; redistribution permission is already recorded.
  Date/Author: 2026-07-26 / Codex.
- Decision: preserve `0.92` and `0.95` as separate branches.
  Rationale: the report and delivered management conflict, and reproduction
  must adjudicate rather than conceal that discrepancy.
  Date/Author: 2026-07-26 / Codex.
- Decision: separate commissioned artifacts from independent literature under
  `references/canopy_phenology/`.
  Rationale: source roles are materially different and must remain visible in
  the assurance evidence graph.
  Date/Author: 2026-07-26 / Codex from operator-supplied literature amendment.
- Decision: admit CAL-02 as `READY_BOUNDED`.
  Rationale: a deterministic common-forcing process experiment can use the
  exact existing Hubbard physical fixture and prescribed management-only
  replacements. Missing site-specific inputs and outputs prevent exact,
  Santee-site, 100-year, constant-cover, or result-equivalence claims.
  Date/Author: 2026-07-26 / Codex.
- Decision: initially close scientific intake under
  `HOLD-PUBLICATION-RIGHTS`, then supersede that hold on explicit operator
  confirmation.
  Rationale: the review snapshot lacked a redistribution basis beyond Bill's
  files. The operator subsequently confirmed redistribution permission for the
  retained third-party sources and directed commit/push.
  Date/Author: 2026-07-26 / Codex after independent review and operator
  confirmation.

## Outcomes & Retrospective

Executed. The package preserves 19 hash-bound sources, a 140-row typed target
ledger, parameter and comparison-scale lineage, the report/file `0.92` versus
`0.95` branch, and a deterministic `READY_BOUNDED` admission. Post-closeout
source recovery upgraded the admitted experiment to exact report-linked
site-specific 100-year WEPPcloud forcing and selected outputs for both sites.
CAL-02 still may not claim Bill's exact Windows rerun or Windows chart/table
equivalence. The recovered source-native `p1.man` and `p2.man` files are
admitted as the analytical constant-cover comparators.

No additional literature gap presently blocks the scientific analysis. Exact
reproduction still requires Bill's full per-site Windows run bundles. The
initial publication hold is superseded by the operator's explicit
redistribution-permission confirmation.

## Revision Note

2026-07-26: initial scaffold created to implement roadmap Order 1. Before source
installation, operator-supplied literature expanded the declared reference
boundary from the commissioned subtree to the complete canopy-phenology
reference family; production and contract boundaries remain unchanged.

2026-07-26: post-closeout operator evidence identified the report-linked
WEPPcloud run IDs and the BLARHG Windows executable. Exact selected source
fixtures were preserved and the bounded CAL-02 admission was superseded from
common forcing to site-specific 100-year source forcing. The verdict remains
`READY_BOUNDED` because Bill's manually converted Windows projects and outputs
are absent.

2026-07-26: the operator confirmed the Windows project is not required for the
campaign and directed use of buildable constant-cover files. Source inspection
established that recovered `p1.man` and `p2.man` already encode the
site-specific constant-cover mechanism described in Bill's report. They are
now admitted as exact analytical comparators; missing Windows serialization
and outputs limit only historical byte/output equivalence.

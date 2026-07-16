# ASSURE-06 — Snow/Frost Flagship Scientific Synthesis

Status: HOLD-HUMAN-APPROVAL

This ExecPlan is a living document governed by `docs/codex_exec_plans.md`.
Keep `Progress`, `Surprises & Discoveries`, `Decision Log`, and `Outcomes &
Retrospective` current throughout execution.

## Purpose / Big Picture

Turn the extensive existing openWEPP snow and frozen-soil evidence into a
recognizable scientific model-evaluation report for hydrologists, soil
scientists, researchers, and practitioners. The report must explain why the
process representation is credible, how it was evaluated, what the quantitative
results show, what remains uncertain, and how a reader can reproduce or
challenge the analysis. It must put the science first without hiding adverse or
mixed evidence behind lifecycle labels.

ASSURE-05 remains `HELD` and its groundwater source remains `DRAFT`. The
operator explicitly authorized ASSURE-06 drafting and internal evidence work in
parallel with that hold on 2026-07-16. This package does not infer that the
ASSURE-05 lifecycle was accepted and does not weaken any human-approval,
publication, release-transfer, export, or vendoring gate.

## Objective

Create a production-domain V2 snow/frost flagship source from retained
precipitation-phase, snow water equivalent (SWE), snow depth, bulk density,
frost-depth, soil-temperature, conservation, negative-mechanism, and production
evidence. Freeze a defensible study decomposition before authoring. Use one
integrated report only if the report can keep each dataset, scale, method,
uncertainty treatment, and conclusion explicit; otherwise create separate
reports. Finish at `HOLD-HUMAN-APPROVAL` unless authenticated, competent,
accountable humans actually complete the governed review and release records.

## Frozen Base And Authority

- Frozen package base: `47c2cf9e`.
- Scientific authority: `SC-SNOWFREEZE-001`, ADR-0017, ADR-0026 through
  ADR-0029, the admitted observation manifests, and the cited primary
  literature.
- Communication authority: the V2 architecture, lifecycle/source-build
  contracts, and scientific model-evaluation report standard.
- Existing work-package artifacts are retained evidence, not scientific or
  contract authority by themselves.
- The retired V1 dossier and method may be inspected only as a failure record.
  They are not a source template, public assessment, or conclusion authority.

## Included Scope

- Freeze the scientific questions, claim envelopes, study decomposition,
  datasets, software realizations, metrics, uncertainty treatment, and
  limitations before drafting conclusions.
- Reconstruct a compact strict result object from identified retained evidence
  with a deterministic, independently executable procedure.
- Author a conventional manuscript and technical supplement in American
  English.
- Bind claims, methods, results, values, tables, figures, references, and
  public-safe research objects in a production-domain V2 descriptor.
- Register the source in `assurance/v2/catalog.yaml` without publishing it.
- Validate, plan, normalize-check, build, and check the report in unrelated
  disposable staging roots; prove deterministic output.
- Perform two independent internal reviews, disposition every finding, and
  perform two terminal verifications.
- Preserve `usersum/assurance`, tracked exports, release snapshots, and vendor
  trees byte-for-byte unless valid human approval and release-transfer records
  unexpectedly become available.

## Excluded Scope

- New or changed snow/frost physics, tuning, calibration, thresholds, default
  selectors, fixtures, observations, kernel/runtime code, or public schemas.
- Rerunning broad historical mechanism-development campaigns merely to replace
  content-identified retained evidence. A current focused currency check may be
  run when needed to bound the assessed realization.
- Restoring or adapting the retired V1 dossier/compiler.
- Calling any builder output a scientific conclusion or aggregate validation
  grade.
- Assigning Anurag, Erin, or any other person a role without their authenticated
  participation.
- Public publication, WEPPcloud vendoring, or application-fitness decisions.

## Intended Write Set

- `docs/ROADMAP.md`
- `docs/work-packages/README.md`
- `docs/work-packages/20260716-assure06-snow-frost-flagship-synthesis-001/**`
- `assurance/v2/catalog.yaml`
- `assurance/v2/README.md`
- `assurance/v2/reports/snow-and-frozen-soil-process-evaluation/**`
- `tests/integration/assurance_dossier_build_contract.rs`
- `tests/integration/assurance_v2_assembly_contract.rs`
- `tests/integration/assurance_v2_planner_contract.rs`
- `tests/integration/assurance_v2_source_contract.rs`

Everything else is read-only. The four integration-test paths were added after
the focused gate proved that their one-report fixture assumptions reject a
valid second catalog entry; changes there are limited to report-count,
named-vs-all, and multi-report staging semantics. If study decomposition
requires more than one
report directory, amend this write set before creating it and record the reason
in `Decision Log`. Kernel, fixture, test, schema, builder, public `usersum`,
export, release, vendor, and WEPPcloud paths require a reviewed package amendment
before any edit.

## Required Reading

The tiered path map and byte budget live in
`artifacts/required-reading-map.md`. Read every Core item before source edits.
Read scientific authority and retained evidence on demand for the claim being
authored. Run `tools/agents/find-agents --for` over the final write set and
record the applicable chain before edits.

## Progress

- [x] (2026-07-16) Operator authorized ASSURE-06 drafting while preserving the
  ASSURE-05 human-review hold.
- [x] (2026-07-16) Scaffolded package directories, execution prompt, artifact
  map, and prospective queue change.
- [x] (2026-07-16) Froze study decomposition, protocol, evidence inventory, realization,
  and claim-specific uncertainty rules.
- [x] (2026-07-16) Produced and independently reproduced 188 strict retained result values.
- [x] (2026-07-16) Authored and bound the manuscript, supplement, tables, figures, references,
  research objects, and disclosed agent-assistance packet.
- [x] (2026-07-16) Validated, planned, normalize-checked, staged twice, checked, compared, and performed
  the reader audit.
- [x] (2026-07-16) Completed dual internal review and dispositioned every finding without waiver.
- [x] (2026-07-16) Completed dual terminal verification with no actionable
  findings and closed at `HOLD-HUMAN-APPROVAL`.

## Phase Plan

### Phase 1 — Intake And Study Freeze

Record the protected public baseline, required-reading chain, exact evidence
inventory, assessed realizations, and decomposition decision. The protocol must
distinguish four evidence families: precipitation phase against Jennings et al.;
seasonal SWE/depth/density against the admitted SNOTEL and canopy-site records;
frost depth/soil temperature against the five admitted non-SNOTEL sites; and
software/conservation/production-path verification. It must state which
observations influenced model development or activation and therefore cannot be
represented as untouched held-out validation.

Acceptance: `study-protocol.md`, `evidence-inventory.md`,
`realization-freeze.md`, and `protected-public-baseline.md` identify every
claim-bearing source and explain why the chosen report decomposition does not
hide differences among data, scale, method, or conclusion.

### Phase 2 — Mechanical Scientific Result

Create a deterministic standard-library procedure that reads only declared,
content-identified evidence and emits a compact strict JSON result. Include
sample counts, confusion-matrix operands, phase accuracy, humidity-threshold
behavior, current snow-profile counts, named residual families, frost-site
paired counts and residual extrema, and production partition closure. Preserve
raw counts and units. Do not turn an ordinal development rubric into a universal
accuracy score.

Acceptance: the retained result exactly equals fresh procedure output; every
reported value has source lineage and an independent reconstruction or direct
operand trace; plausible aliases and misleading aggregates are named in
`operand-lineage.md`.

### Phase 3 — Scientific Manuscript And V2 Binding

Write the manuscript in the order required by the report standard: title and
authorship, key findings, plain-language summary, abstract, introduction,
formulation, data and methods, results, discussion, limitations, conclusions,
open research/reproduction, references, and about-this-report. Lead with the
strongest bounded evidence, not `DRAFT`, `CANDIDATE`, `PASS`, or
`INSUFFICIENT_EVIDENCE`. Report mixed and adverse findings in ordinary
scientific language. Keep verification, empirical comparison, model-development
selection, release currency, and site fitness distinct.

Acceptance: the descriptor validates as production-domain `DRAFT`, all
claim-bearing values resolve mechanically, tables/figures are accessible and
scientifically purposeful, research objects are complete, agent assistance is
disclosed, and human accountability remains unassigned.

### Phase 4 — Disposable Consumer And Reader Audit

Run validation and planning, check American English, then build and check in two
unrelated disposable staging roots seeded with the snow/frost model narrative.
Compare the complete staged trees byte-for-byte. Read the staged report as a
hydrologist or soil scientist and answer the eight minimum-useful-publication
questions without consulting machine YAML.

Acceptance: both staging trees are identical; no unresolved directive,
workspace-only link, status-first headline, inaccessible figure, or unbound
number remains. Staging is explicitly not publication.

### Phase 5 — Review, Verification, And Disposition

Dispatch one independent internal domain-science reviewer and one independent
reproduction/publication reviewer. After accepted findings are fixed and
rechecked, dispatch two terminal verifiers. Disposition every finding as
`accepted`, `rejected`, `deferred`, or `follow-up` with rationale. Verify the
Gate Evidence Non-Deferral Rule, protected public state, source identities,
line-count governance, and absence of unauthorized scientific or lifecycle
claims.

Acceptance: all technical gates pass and no finding is undispositioned. Unless
valid human records exist, close `HOLD-HUMAN-APPROVAL`: the exact source remains
`DRAFT`, review `not_started`, public report count remains zero, and the handoff
names the human actions required for review entry and publication.

## Validation And Gates

Run and record:

1. the package-specific reproduction procedure and exact JSON comparison;
2. `cargo run --quiet -p openwepp-assurance -- validate --report snow-and-frozen-soil-process-evaluation`;
3. `cargo run --quiet -p openwepp-assurance -- plan --report snow-and-frozen-soil-process-evaluation`;
4. `cargo run --quiet -p openwepp-assurance -- normalize --report snow-and-frozen-soil-process-evaluation --language en-US --check`;
5. two independent `build` plus `check` runs and a byte-for-byte tree comparison;
6. focused assurance V2 and report-contract tests selected by Nextest;
7. `cargo fmt --check` and strict Clippy if any Rust source changes after an
   authorized amendment; otherwise record `NOT APPLICABLE — no Rust edits`;
8. Markdown/path/link checks available in this repository;
9. protected-public hash and file-inventory comparison;
10. dual internal review, finding disposition, dual terminal verification, and
    `.rs` line-count governance (`NOT APPLICABLE` is allowed only when no Rust
    file changes).

This is documentation/scientific-source work, so the adjudicated CRAP gate is
exempt unless production Rust changes after a package amendment.

## Subagent Authorization

Subagent authorization: this package explicitly authorizes spawning/delegation
to one internal domain-science reviewer, one independent reproduction and
publication reviewer, and two terminal verifier subagents for read-only review
of the final source and package evidence. Expected outputs are compact findings,
commands checked, identity/gate dispositions, and review text returned to the
parent for package artifacts; write access is read-only. If a heavy full-suite
run becomes required after a reviewed amendment, this package also explicitly
authorizes a heavy-gate-runner subagent with writes bounded to package log
artifacts and scratch space. No subagent may create human identity, approval,
peer review, release authority, or application fitness.

## Exit Criteria

- The V2 report is scientifically readable and technically complete for human
  assignment, without claiming that formal human review has started.
- Every quantitative statement is bound to retained evidence with units,
  sample counts, method, realization, uncertainty, and claim limits.
- Strong, mixed, adverse, unavailable, and development-influenced evidence are
  visible in the manuscript without an aggregate status headline.
- Reproduction and deterministic staging pass.
- The current public report count remains zero and protected paths are
  unchanged.
- Dual reviews and dual verifications are complete and every finding is
  dispositioned.
- The terminal package state truthfully reflects the human-authority boundary.

## Surprises & Discoveries

- The roadmap originally serialized ASSURE-06 behind completion of the
  ASSURE-05 human lifecycle. The operator explicitly chose parallel drafting;
  the scientific and public approval gates remain serial and unchanged.
- The focused assurance gate encoded a one-report catalog assumption in four
  integration-test surfaces. Adding the second production-domain draft exposed
  six failures in named/all equivalence, temporary-catalog counts, and staging
  setup even though named validation and the disposable report build passed.
- The first domain-science draft still over-aggregated three evidence families:
  it underdisclosed phase exclusions, treated correlated snow rubric cells too
  much like a success rate, and pooled frozen-soil sites whose adverse outcomes
  were materially different. Site-resolved tables and explicit selection rules
  were necessary for a manuscript a scientist could audit.
- The first conservation reconstruction authenticated prior residuals without
  retaining all operands, and a later revision verified rows while still
  reading duplicate headline fields. Review forced the final fail-closed design:
  one compact operand object, one staged source log, and summaries derived only
  from the reconstructed-row map.

## Decision Log

- Decision: permit ASSURE-06 internal drafting while ASSURE-05 remains held.
  Rationale: the user explicitly redirected the queue; waiting for human review
  need not idle scientific synthesis, but publication authority cannot be
  inferred. Date/Author: 2026-07-16, operator/Codex.
- Decision: begin with one integrated report ID but require a pre-authoring split
  test. Rationale: snow phase, snowpack, and frozen-soil response form one
  coupled process chain, while the report standard forbids hiding materially
  different datasets or conclusions. Phase 1 must split before authoring if an
  integrated manuscript cannot preserve those boundaries. Date/Author:
  2026-07-16, Codex.
- Decision: amend the write set to the four assurance integration tests named
  above. Rationale: multi-report catalog support is existing V2 behavior, and
  leaving tests pinned to one report would make ASSURE-06 structurally
  unshippable. Assertions must preserve named-report isolation, deterministic
  all-report behavior, and zero-public-report gates rather than merely changing
  expected counts. Date/Author: 2026-07-16, Codex.
- Decision: retain one integrated process-chain manuscript after the split test,
  but prohibit cross-family grading and render phase, snow, frost-tube,
  isotherm, and conservation conclusions separately. Rationale: snow state is a
  necessary boundary condition for frost interpretation, while separate methods
  and site tables preserve the distinct referents and claim envelopes.
  Date/Author: 2026-07-16, Codex.
- Decision: treat the pooled snow rubric and pooled isotherm rate as secondary
  arithmetic context only. Rationale: correlated site-by-signature cells and
  heterogeneous sites do not support independent-trial or portable performance
  inference. Date/Author: 2026-07-16, Codex.

## Outcomes & Retrospective

ASSURE-06 produced a production-domain V2 source titled *Observational
Evaluation of openWEPP Snow and Frozen-Soil Processes*. It remains `DRAFT`,
formal review is `not_started`, and public report count is zero. The manuscript
leads with bounded scientific findings and provides a plain-language summary,
model formulation, explicit study design, site-resolved results, adverse
evidence, limitations, citations, and reproduction instructions.

The deterministic result contains 188 values. The rendered report contains
seven tables, two accessible figures, and 16 public-safe research objects,
including the exact phase scorer, selected-row conservation operands and source
log, and dataset-provenance/reacquisition record. Dual internal review findings
were all accepted and closed. Two independent terminal verifiers then reproduced
the retained result, checked the scientific and governance evidence, confirmed
the protected public boundary, and returned PASS with no actionable findings.

The required terminal disposition is `HOLD-HUMAN-APPROVAL`. Advancement requires an
accountable human report lead and scientific approver (Anurag or Erin as the
operator suggested, if they accept), independent human reproduction/publication
review under the V2 contract, exact-subject approval and lock, assurance-steward
approval, release-owner transfer to an exact candidate realization, and the
publication gates. No export, release snapshot, vendoring, or WEPPcloud action
is authorized by this package.

## Idempotence And Recovery

All analysis is offline and deterministic. Write generated evidence first to a
temporary path and replace the retained result only after exact validation.
Disposable staging roots may be deleted and rebuilt. Never recover by copying a
draft into tracked `usersum`, by weakening descriptor identities, or by
fabricating review principals. Existing unrelated worktree changes remain user
owned and must be preserved.

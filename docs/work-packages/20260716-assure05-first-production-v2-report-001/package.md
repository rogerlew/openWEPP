# ASSURE-05 — First Production V2 Groundwater Report

Status: HOLD-HUMAN-APPROVAL

Package ID: `20260716-assure05-first-production-v2-report-001`

Frozen base: `01ed70550a4e371e99afe35c4bdd4d9b667e812c`

This ExecPlan is maintained under `docs/codex_exec_plans.md`. The `Progress`,
`Surprises And Discoveries`, `Decision Log`, and `Outcomes And Retrospective`
sections are living execution records.

## Objective

Convert the accepted linear-groundwater architecture fixture into the first
genuine v2 scientific model-evaluation report. The report must let a
hydrologist, soil scientist, researcher, or practitioner understand why the
daily linear reservoir is used, what exact openWEPP realization was assessed,
how its recurrence and production transfer were evaluated, what the numerical
results show, and what they do not establish.

The package regenerates current evidence rather than promoting ASSURE-02
transcriptions. It preserves exact inputs, outputs, logs, result objects,
analysis procedures, and software identity; independently reconstructs every
material quantitative statement; revises the manuscript and supplement from
the evidence; and exercises the real deterministic staging consumer.

Coding agents may prepare and internally review the study, but they cannot
authenticate people, establish domain competence, or create human scientific,
reproduction/publication, assurance-steward, or release-owner approval. If
valid named human records are not supplied, the terminal disposition is an
honest `HOLD-HUMAN-APPROVAL` with a complete review-ready staging handoff. No
draft or agent-only review enters public navigation, a release snapshot, or a
vendor tree.

## Scientific Question And Claim Envelope

Primary question: does the assessed openWEPP realization correctly implement
the authorized one-day linear groundwater-storage recurrence, serialize its
generated baseflow/deep-seepage quantities through the production hillslope
pass, and admit those quantities at the bounded watershed-consumer interface?
The fresh evidence tests those adjacent interfaces separately; it does not run
a nonzero groundwater payload through the complete CLI adapter in one
execution.

The report is a combined formulation, code-verification, integration, and
release-transfer study. It is not an empirical evaluation of streamflow,
groundwater level, or predictive accuracy. Priest River evidence is prior
knowledge about the related coupled WEPP formulation and must not be reported
as performance of the assessed openWEPP realization.

The bounded quantities are daily pre-export groundwater storage, daily
baseflow and deep-seepage volumes, cumulative recharge and exports, and the
serialized consumer operands. The analytical case uses a synthetic 1,000 m2
hillslope over two one-day steps. The production case uses the 731-day H2637
fixture and its declared OFE geometry and management. Application fitness for
another watershed remains with the named decision owner.

## Authority

Binding authority is ADR-0038; the active v2 architecture, lifecycle,
source/build contract, and report standard; `SC-GWBASEFLOW-001`; the active
ASSURE-05 roadmap row; and the completed ASSURE-04D publication mechanics.
The pinned legacy baseline is provenance only where the science contract cites
it. This package does not amend groundwater physics or contract authority.

## Included Scope

- freeze the current Git realization and exact groundwater producer,
  publication, serialization, and watershed-consumer paths;
- record an operand-lineage and rejected-alias table before evidence work;
- rerun the independent two-day recurrence calculation and the executable
  recurrence, guard, coefficient-domain, threshold, serialization, and real
  consumer checks;
- rerun the ignored H2637 active-owner production fixture under nextest process
  isolation and retain its exact manifest, relevant output identities, and
  execution log;
- independently reconstruct the two terminal groundwater identities from the
  produced manifest, with explicit units, timing, signs, tolerances, and
  rejected latest-event/diagnostic aliases;
- preserve public-safe inputs, results, analysis procedures, source/output
  identities, and reproduction commands as version-bound research objects;
- replace ASSURE-02-era path/test-count objects and stale realization identity
  with current ASSURE-05 evidence;
- revise the manuscript and supplement as scientific communication, including
  quantitative key findings, prior knowledge, contrary evidence, uncertainty,
  limitations, and challenge/reproduction routes;
- move the source from a `test_only` architecture fixture toward a production
  report only to the lifecycle state justified by accountable human records;
- build and check exact deterministic staging output, figures, tables,
  citations, supplement, public research objects, and portable links;
- perform internal domain-science critique, independent reconstruction,
  publication/accessibility review, dual implementation review, finding
  disposition, heavy closure, and dual terminal verification; and
- publish, snapshot, and cross-link only if valid production-domain human
  approvals and release transfer are supplied and mechanically verified.

## Excluded And Protected Scope

- no new groundwater, routing, snow/frost, erosion, or watershed physics;
- no calibration, parameter fitting, field-data accuracy claim, or application
  fitness verdict;
- no attribution of Srivastava et al. performance statistics to openWEPP;
- no agent-generated or inferred human identity, competence, independence,
  approval, approval date, or release authorization;
- no public lifecycle headline, aggregate validation grade, or empty evidence
  inventory;
- no tracked `usersum/assurance` or model-narrative backlink before verified
  `PUBLISHED` authority;
- no WEPPcloud vendoring or ASSURE-08 work;
- no production builder/schema/kernel/test-fixture mutation unless a blocking
  defect is recorded and this package is amended before the edit; and
- no committed temporary run directory, tool cache, secret, or copyrighted
  article text.

## Declared Write Set

- `assurance/README.md`
- `assurance/v2/README.md`
- `assurance/v2/catalog.yaml`
- `assurance/v2/principals.yaml`
- `assurance/v2/reports/linear-groundwater-reservoir-recurrence/**`
- `Cargo.toml` for a report-specific reproduction integration-test target
- `tests/integration/assurance_v2_{source,planner,assembly}_contract.rs` for
  current production-source expectations and renamed current evidence objects
- `tests/integration/assurance_v2_publication_contract.rs` for synthetic
  publication fixtures that must derive their test-only/production variants
  from the converted `1.0.0` production source
- `tests/integration/assurance_v2_groundwater_report_contract.rs` for the
  independent report reproduction and claim/evidence contract
- `usersum/hillslope-hydrology-and-sediment-physics.md`, conditional on a
  verified published report only
- `usersum/assurance/**`, conditional on the approved publication operation
  only; zero-report bytes remain protected otherwise
- `docs/ROADMAP.md`
- `docs/planning/scientific-assurance-v2-implementation-roadmap.md`
- `docs/work-packages/README.md`
- `docs/work-packages/20260716-assure05-first-production-v2-report-001/**`

All Rust, schemas, tests, release tooling, other science contracts, other
reports, export/release/vendor trees, and WEPPcloud paths are read-only. A
blocking builder defect requires an explicit write-set amendment before code.

The five test paths above are the sole test exception. They may update stale
fixture expectations and prove the report's independent procedure, but may not
change kernel behavior, schemas, builder semantics, or scientific thresholds.

## Required Deliverables

1. `artifacts/required-reading-map.md` and an exact realization/path freeze.
2. A preregistered study protocol and operand-lineage/rejected-alias table.
3. Fresh execution logs, retained input/output identities, current result
   objects, and an independently implemented reproduction procedure.
4. A revised manuscript, technical supplement, report descriptor, catalog,
   principal registry, and public-safe research-object manifest.
5. Deterministic staging output and semantic-difference record against the
   architecture fixture.
6. Internal domain-science review, independent reproduction review,
   publication/accessibility review, and complete finding disposition.
7. Dual implementation reviews, independent heavy-gate evidence, fresh
   adjudicated CRAP when implementation files are touched, line-count
   governance, and dual terminal verification.
8. Either a verified production publication/snapshot/receipt plus narrative
   cross-link, or an exact review-ready `HOLD-HUMAN-APPROVAL` handoff that keeps
   public surfaces unchanged.

## Phase Plan

### Phase 1 — Intake, Freeze, And Preregistered Method

Confirm the frozen base and a clean starting tree. Record applicable
instructions, exact producer/consumer/source paths, release realization,
fixture identity, claim envelope, expected equations, tolerances, uncertainty
categories, operand lineage, rejected aliases, and preservation plan before
running or revising results.

### Phase 2 — Fresh Evidence And Independent Reconstruction

Run the focused groundwater suite and H2637 production fixture with exact logs.
Retain the H2637 manifest and hashes for claim-bearing outputs. Independently
calculate the analytical recurrence and both H2637 terminal identities without
calling the production recurrence function or copying its asserted residual.
Create strict current result objects and a deterministic reproduction procedure
that fails on missing units, changed operands, wrong aliases, or tolerance
failure.

### Phase 3 — Manuscript And Source Revision

Revise the report from the fresh evidence. Lead with the scientific question
and quantitative results. Preserve the formulation, prior-knowledge context,
method rationale, contrary/negative cases, uncertainty, limitations, and
application boundary in the main manuscript. Keep machine lifecycle details in
the final metadata section and supplement. Bind every material number to one
result object; do not hand-copy competing sources.

### Phase 4 — Deterministic Staging And Reader Checks

Validate, plan, build, and check the named report in unrelated disposable
staging roots. Compare repeated builds byte-for-byte. Prove every figure,
table, citation, research object, narrative link, and reproduction instruction
is accessible and portable. Apply the report standard's eight-question minimum
useful publication test from the main report.

### Phase 5 — Independent Review And Finding Closure

Obtain separate internal domain-science, reproduction, and
publication/accessibility reviews plus two implementation/audit reviews.
Coding-agent records must say `internal coding-agent review`, not peer review or
human approval. Disposition every finding as `accepted`, `rejected`,
`deferred`, or `follow-up`; accepted findings are fixed and reverified before
the review root is frozen.

### Phase 6 — Human Approval Boundary And Conditional Publication

Inspect the production principal registry and exact-root review records. If a
human report lead, independent scientific reviewer, independent
reproduction/publication reviewer, assurance steward, and release owner have
supplied valid identities, competence/independence attestations, approvals, and
release transfer, validate and publish to unrelated external public/snapshot
roots, verify the receipt, prove exact release transfer, and add the approved
model-narrative cross-link.

If any human authority is absent, do not populate it. Record the exact subject
root, open human decisions, recommended review charge, staging commands, and
public zero-state hashes; disposition the package `HOLD-HUMAN-APPROVAL`.

### Phase 7 — Heavy Closure And Dual Terminal Verification

Run the package-required root gates through the delegated heavy runner. Because
the planned change is scientific source/evidence rather than Rust
implementation, the adjudicated CRAP gate is required only if production Rust
or tests are amended. Always run format, strict Clippy, full Nextest, deny,
assurance validation/planning/build checks, Markdown validation, link/path
checks, and `git diff --check`. Obtain two terminal verifications over the final
source and truthful disposition.

## Validation And Acceptance

Required commands, adjusted only with recorded equivalent evidence:

```bash
cargo nextest run --workspace --profile quick \
  -E 'test(/gwbaseflow|r6a_direct_hbp_writer_serializes_groundwater_payload_operands/)'
cargo nextest run --test laned_shadow_h2637 --run-ignored ignored-only \
  -E 'test(=h2637_native_active_owner_routes_and_closes)'
cargo run --quiet -p openwepp-assurance -- validate \
  --report linear-groundwater-reservoir-recurrence
cargo run --quiet -p openwepp-assurance -- plan \
  --report linear-groundwater-reservoir-recurrence --format json
cargo run --quiet -p openwepp-assurance -- build \
  --report linear-groundwater-reservoir-recurrence --staging-root <absolute-root>
cargo run --quiet -p openwepp-assurance -- check \
  --report linear-groundwater-reservoir-recurrence --staging-root <absolute-root>
cargo fmt --check
cargo clippy --workspace --all-targets -- -D warnings
cargo nextest run --workspace --profile full
cargo deny check
markdown-doc lint --path docs/work-packages/20260716-assure05-first-production-v2-report-001 \
  --path assurance/v2/reports/linear-groundwater-reservoir-recurrence \
  --path docs/ROADMAP.md --path docs/work-packages/README.md
git diff --check
```

Scientific acceptance requires that a domain reader can answer all eight
minimum-useful-publication questions from the main report, all material values
reconstruct from retained objects, the production consumer reads the claimed
path, limitations remain visible, and prior Priest River evidence is accurately
bounded.

`PASS-ASSURE-05-PUBLISHED` additionally requires valid named human approvals,
exact-root locks, release transfer, public build, snapshot, receipt, catalog,
narrative cross-link, and unchanged vendoring posture. Without them, the only
truthful terminal result is `HOLD-HUMAN-APPROVAL`; a technically complete draft
is not relabeled as published.

## Delegation Authorization

Subagent requirement: **REQUIRED**. This package explicitly authorizes and
requires subagent spawning/delegation to:

- one heavy-gate/fixture runner for H2637, full workspace, deny, and any fresh
  CRAP closure; expected output is compact metrics, exits, timings, hashes, and
  log paths; write access is limited to named package artifacts and scratch;
- one read-only domain-science reviewer and one read-only independent
  reproduction/publication reviewer; expected output is findings with evidence
  and a clear internal-agent limitation; and
- two read-only terminal verifiers after finding closure.

Subagents cannot create or populate human approval or release-authority records.

## Amendments

### 2026-07-16 — Report-Specific Reproduction Test Surface

Before test edits, the package added `Cargo.toml`, the three existing v2 source/
planner/assembly contracts, and one new groundwater-report contract to the
write set. The accepted architecture tests hardcode ASSURE-02 filenames,
`test_only`, and `fixture_only`; converting the source without updating those
real consumers would leave CI asserting the retired fixture state. The new test
must execute the standard-library analysis procedure against retained objects
and prove claim-bearing values, not merely file presence. This amendment does
not authorize production Rust, schema, kernel, fixture, or release-tool edits.
Because Rust tests are touched, fresh adjudicated CRAP evidence is required at
closure even though the scientific algorithm is unchanged.

### 2026-07-16 — Publication-Fixture Consumer Compatibility

After strict Clippy passed, full Nextest exposed 22 failures in
`assurance_v2_publication_contract`: its synthetic 04D helper copied the
converted production `1.0.0` report but still required literal `test_only`,
`fixture_only: true`, and `0.1.0` source assumptions. Before editing that test,
the package added it to the declared write set. The amendment authorizes only
deriving explicit synthetic test-only and production fixtures from the current
production source and updating version/path expectations; it does not authorize
publication implementation, schema, or authority weakening. Fresh workspace
and CRAP closure remain mandatory.

### 2026-07-16 — American-English Report Normalization

The user directed that reports use American English and explicitly requested
`uk2us`. The manuscript, supplement, and generated-report metadata were
normalized from `metre/metres` to `meter/meters`; dependent manuscript,
supplement, agent-packet, descriptor, catalog, source-root, and staging-manifest
identities were rebound. The corresponding source-contract mutation literal
was updated without changing its fail-closed assertion.

Proportional renewal exposed a preexisting timing weakness in the assembly
rollback test: polling could miss the short-lived installed-backup directory
and fail to inject the intended source drift. The test-only fixture now uses an
8 MiB declared implementation dependency and a 60-second observation deadline
to make the post-install rehash interval observable. Production assembly code
is unchanged. The final affected five-suite run passed 59/59 under strict
Clippy, and both terminal verifiers renewed the normalized source.

## Security, Idempotence, And Recovery

All ordinary build and analysis steps are local and deterministic. External
literature lookup is read-only and citation-limited. H2637 runs use nextest
process isolation. Scratch roots must be absolute, unrelated to the repository,
and removable. Retained objects are copied only after hashes and provenance are
recorded. Repeated analysis and staging must produce identical semantic results
and bytes. A failed review or gate preserves evidence and returns to the last
source root; it never mutates public navigation.

Rollback before publication is deletion of disposable staging roots. Rollback
after a valid publication uses the 04D withdrawal/supersession contract and
retains the immutable snapshot and receipt. Vendoring remains deferred to
ASSURE-08.

## Progress

- [x] (2026-07-16 UTC) User authorized ASSURE-05 after ASSURE-04D was committed
  and pushed at `01ed7055`.
- [x] (2026-07-16 UTC) Scaffolded the autonomous package, explicit approval
  boundary, declared write set, reading map, active prompt, and artifact set.
- [x] (2026-07-16 UTC) Froze realization `01ed7055`, the unchanged 12-path
  implementation/consumer set, H2637 fixture identity, study protocol, operand
  lineage, rejected aliases, tolerances, and prior-knowledge boundary before
  fresh execution.
- [x] (2026-07-16 UTC) Regenerated focused and clean-build H2637 production
  evidence; rejected a stale-provenance acquisition and independently
  reconstructed the accepted analytical and production ledgers.
- [x] (2026-07-16 UTC) Revised the manuscript, supplement, descriptor, results,
  current evidence objects, exact H2637 inputs, reproduction procedure, and
  archived agent-assistance record as a production-domain `DRAFT`.
- [x] (2026-07-16 UTC) Passed source validation, 34 focused v2 contracts, two
  byte-identical seeded staging builds/checks, staged analytical and H2637
  reproduction, and the minimum-useful-publication check.
- [x] (2026-07-16 UTC) Accepted and remediated internal domain-science and
  reproduction/publication findings; retained the missing continuous CLI-
  adapter execution as an explicit future evidence obligation.
- [x] (2026-07-16 UTC) Confirmed that named human approval and release-transfer
  records are absent; publication remains prohibited and the terminal package
  disposition must be `HOLD-HUMAN-APPROVAL`.
- [x] (2026-07-16 UTC) Passed terminal technical closure on the third complete
  attempt: full Nextest 2,049/2,049, deny, deterministic build/check, Markdown,
  diff hygiene, and fresh CRAP with 0 actionable rows. Earlier Clippy and full-
  suite fixture failures remain preserved. Dual terminal internal verification
  passed after the sole low-severity archival-summary finding was corrected and
  reverified; neither verifier lifted the human-approval hold.

## Surprises And Discoveries

- The accepted 04D mechanics deliberately require publication roots unrelated
  to the repository. ASSURE-05 therefore cannot treat a generated external
  preview as authority to hand-copy report bytes into tracked `usersum`.
- The current fixture still describes ASSURE-02 transcriptions and an old
  software identity. Fresh production evidence is a scientific prerequisite,
  not an editorial version bump.
- The first nominal release build exited successfully but reused an existing
  `target/release/openwepp-cli-hill`; its runtime sidecar did not bind the
  frozen source and its recorded binary digest disagreed with the executable.
  The associated H2637 run is retained as explicit non-acceptance evidence.
  Cargo success alone is therefore insufficient build provenance for a
  claim-bearing production run.
- Fresh focused evidence proved the production writer/parser and the watershed
  consumer from hand-constructed contributions, but did not traverse the
  intervening production CLI adapter with nonzero groundwater in one execution.
  The report therefore narrows its conclusion instead of treating adjacent
  seam tests as end-to-end evidence.
- The original H2637 reproduction route depended on transient scratch files.
  Internal review caught the defect before root freeze; the exact accepted
  manifest, HBP, and pass-Parquet objects are now retained, authenticated, and
  executable from the staged report.
- Full workspace execution showed that the 04D publication-contract fixture
  was coupled to the retired `test_only` `0.1.0` source. The failure is useful
  consumer evidence: converting a real report requires every synthetic
  publication fixture to derive its trust domain and paths explicitly rather
  than relying on stale source literals.
- A spelling-only follow-on exposed a nondeterministic rollback-test polling
  window. The contradictory pass/fail runs were preserved, and the harness was
  strengthened at the test-fixture boundary rather than weakening the
  transactional source-drift assertion.

## Decision Log

- Decision: declare missing human accountability as a terminal hold boundary
  before evidence execution.
  Rationale: ADR-0038 forbids coding agents and builders from manufacturing
  scientific judgment; predeclaring the boundary prevents a technically strong
  draft from being mislabeled as an approved publication.
  Date/Author: 2026-07-16 / Codex.
- Decision: retain H2637 manifest operands and independent reconstruction as
  production integration evidence, but do not claim empirical accuracy.
  Rationale: the study question is implementation and transfer correctness;
  field predictive performance is a different claim envelope.
  Date/Author: 2026-07-16 / Codex.
- Decision: amend the package before changing the test consumers that pin the
  architecture fixture's retired names and trust state.
  Rationale: the real validation/planning/assembly consumers must recognize the
  new production-source posture, and an executable reproduction contract is
  stronger than a prose-only procedure. The amendment is test-only and triggers
  the fresh CRAP closure gate.
  Date/Author: 2026-07-16 / Codex.
- Decision: accept only an H2637 run built and executed under a new isolated
  `CARGO_TARGET_DIR`, with the release runner, debug test executable, runtime
  sidecar, source commit, and produced-output identities recorded together.
  Rationale: a clean target prevents stale artifact reuse and turns executable
  provenance into a checked input rather than an inference from Cargo's exit
  status.
  Date/Author: 2026-07-16 / Codex.
- Decision: resolve the missing continuous CLI-adapter test by narrowing the
  report to the two interfaces actually executed and declaring the stronger
  traversal as future work.
  Rationale: standing evidence rules prohibit joining adjacent seam proofs into
  an unexecuted end-to-end claim; a smaller truthful report is reviewable now.
  Date/Author: 2026-07-16 / Codex.
- Decision: retain the exact accepted H2637 manifest, HBP, and pass-Parquet as
  public-safe, version-bound research objects.
  Rationale: the scientific report must remain reproducible after transient run
  directories disappear, including authentication of the claim-bearing raw
  inputs before independent reconstruction.
  Date/Author: 2026-07-16 / Codex.

## Outcomes And Retrospective

ASSURE-05 produced a scientifically organized, review-ready production-domain
`DRAFT` rather than promoting the status-first architecture fixture. Material
values are bound to retained evidence; the analytical and H2637 procedures are
executable from staged research objects; two unrelated staging roots are byte-
identical; and internal domain and reproduction/publication findings are closed.

Terminal technical closure passed only after strict Clippy exposed exact-float
test assertions and full Nextest exposed stale 04D `test_only` fixture
assumptions. Both failed attempts are retained, the affected tests were fixed
without changing production publication behavior, and the final full suite and
fresh adjudicated CRAP passed.

The terminal disposition is `HOLD-HUMAN-APPROVAL`. No valid named human report
lead, scientific reviewer, reproduction/publication reviewer, assurance
steward, or release owner is present for the exact source root. Accordingly the
report remains `DRAFT`; tracked public `usersum`, release snapshots, exports,
and vendoring remain unchanged. ASSURE-06 remains queued until accountable
humans review this pilot and the complete publication lifecycle is accepted.

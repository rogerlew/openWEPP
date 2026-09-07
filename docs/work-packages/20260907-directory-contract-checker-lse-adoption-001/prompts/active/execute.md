# Execute: directory-contract checker and LSE adoption

## Mission and explicit authorization

Work in the existing `rogerlew/openWEPP` checkout, normally `/workdir/openWEPP`.
Resolve its actual root. Scaffold and execute ONE work-package:

`docs/work-packages/20260907-directory-contract-checker-lse-adoption-001/`

Execution mode: `package-end-to-end`.
Implementation intent: `directory-v1 checker implementation + meaning-preserving LSE contract migration + validated selective reading`.

This request authorizes implementation and adoption, not another specification-only package. It supplies the separate authorization required by the completed directory-format specification and its handoff. Complete scaffolding, checker implementation, the LSE pilot, affected-consumer reconciliation, reading exercises, independent reviews, finding resolution, independent verification, and truthful disposition. Do not stop at a design, working checker, split document, or intermediate milestone while safe authorized work remains.

Use the specification at:
`docs/specifications/science-contract-directory-format.md`

Reviewed reference checkpoint:
`b932db101cce07d0860b45b5ecaa8ddb7f455b58`

That checkpoint is an audit anchor, not a reset target. Inspect actual HEAD and intervening changes. Continue an existing package implementing this exact authorization rather than duplicating it. Preserve unrelated work. Do not reopen the completed specification package merely to execute its already-defined adoption gates.

This work must make scientific engineering less context-intensive without reducing its obligations. The output is a usable modular LSE contract, not a generalized document platform or a new prerequisite for unrelated modeling.

## Protected boundaries

Preserve scientific meaning: equations, constants, tolerances, domains, assumptions, branch and failure precedence, guards, ownership, chronology, transaction/restart/serialization semantics, producer/consumer duties, qualification limits, and source/test authority. Preserve contract-first sequencing, applicable authority gates, real-consumer proof, independent conservation reconstruction, and anti-evasion.

No production solver/runtime changes, new physical interpretations, model activation, calibration, performance qualification, or resurrection of rejected Stage-3 candidates are authorized. Existing scientific test expectations and numerical fixtures remain unchanged; test edits are limited to preserving checks through reference/loader reconciliation. Only LSE is being migrated; do not bulk-migrate snow or other contracts.

Preserve existing logical contract/invariant/obligation IDs and historical FAIL/HOLD dispositions. Do not rewrite commit-pinned references, frozen historical package bytes, or previous evidence identities. Format adoption does not supersede a frozen kickoff or waive its reading/acceptance requirements.

Do not change agent effort defaults, recreate TESTGATE, introduce a semantic dependency engine, automate scientific-equivalence decisions, or build a policy service. Small shared parsing/loading helpers and focused tests are authorized where directly necessary.

Preserve unrelated staged, unstaged, and untracked files. No blanket reset, clean, stash, staging, or branch switching. Isolated temporary fixture trees outside the checkout are authorized. Use `.venv/bin/python`. Installing the repository's declared development/test dependencies in that local environment is authorized; do not upgrade global environments or silently replace missing tools. No deployment, remote publication, or infrastructure provisioning.

Create owned local scaffold/checkpoint/completion commits as required. Do not push. Previously pushing the specification does not authorize pushing this implementation.

## Intake and bounded required reading

Record HEAD, branch, relevant dirty/index state, tool availability, and the applicable instruction chain before edits. Use:

`tools/agents/find-agents --for <actual-intended-write-paths>`

Read applicable root/local instructions and the complete directory-format specification. Read the relevant parent obligations in:

- `docs/specifications/science-contract-spec.md`
- `docs/specifications/science-contract-provenance-spec.md`
- `docs/specifications/science-contract-authoring-procedure.md`
- `docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `docs/specifications/science-contracts/AGENTS.md`
- applicable unit-governance, testing/evidence-reuse, and package role guidance.

Start predecessor orientation at:
`docs/work-packages/20260907-directory-contract-format-spec-001/artifacts/worker-handoff.md`

Read its gate results only as needed. They distinguish three directly executed Python assertion functions from an unavailable pytest runner and explicitly certify no directory checker or migration. Do not reinterpret that evidence as implemented support.

Inspect the current checker and tests:

- `tools/check_sc_binding_exposure.py`
- `tests/python/test_check_sc_binding_exposure.py`

For this migration, the author and preservation reviewers must inspect the COMPLETE starting LSE authority, including amendments, tables, addenda, and its Binding Exposure Index:

`docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001.md`

Reading less is the prospective benefit, not permission to migrate material nobody inspected. Follow specifically relevant external-contract boundaries and provenance; do not preload unrelated contracts, all work-packages, or raw historical experiments. Record necessary expansion honestly.

## Phase 0 — freeze source, scope, preservation, and measurement

Scaffold the package and required artifacts. Freeze its substantive acceptance and independent assurance obligations before implementation. Do not retrospectively weaken a failed requirement.

Capture the starting LSE bytes, commit/path, content hash, version, incoming live references, checker behavior, and relevant consumer/test inventory. At the reviewed checkpoint LSE is version 31; inspect actual state and increment according to the specification rather than overwriting newer work. Preserve distinct scientific authority, activation authorization, and empirical/release status.

Create a source-bound clause inventory and proposed mechanism boundaries before moving content. Include binding material without IDs, not only regex matches. Record old-to-new AND new-to-original coverage for equations, qualifiers, exceptions, guards, units, ordering, ownership, assumptions, citations, test bindings, gaps, and qualification limits. Preserve original-source references using the original commit/path/anchor and line ranges where necessary.

Freeze representative reading tasks and a baseline answer/rule rubric from the old authority before seeing the new reading results. The tasks must cover surface/soil coupling, solver correctness, executable identity verification, and scientific closure reconstruction. Keep task scope and required conclusions equivalent before and after. Baseline expectations are evaluation evidence, not hints supplied to the fresh agents.

Enumerate exact write paths. Authorized families are the checker and narrowly necessary shared helpers/tests; the LSE entry and declared sibling chapters; directly affected documentation/validator/test consumers; minimal adoption-status/routing updates; and this package. Inspect at least the unit-governance reader `tools/release/check_sc_unit_compliance.py` and source-coupled Rust consumers such as `tests/integration/solver_architecture_authority_contract.rs`; discover all other applicable readers rather than assuming these are exhaustive.

Prefer existing dependency anchors. Minimal nonsemantic anchor/link corrections in explicitly named neighboring contracts are allowed only when necessary to resolve LSE references; observe their version/identity rules. This is not authority to restructure or amend their science. Add newly discovered paths before editing them and reconcile impact.

Use the completed format as authority. A small implementation-exposed ambiguity may receive a narrowly scoped, prospectively documented and independently reviewed clarification. Do not redesign the schema, bypass adoption gates, invent science, or turn one parser detail into a successor specification project.

## Phase 1 — implement and test directory-v1 checking

Extend the existing checker. Preserve its public legacy invocation, meaningful legacy outcomes, and documented exit semantics. Retain recognized single-file support indefinitely. Use one bounded directory-format implementation rather than divergent parsers embedded in multiple validators.

Dispatch deliberately: absent format metadata means legacy; `directory-v1` means directory validation. Unknown, duplicate, malformed, or unsupported format metadata must not trigger a fallback that treats directory material as legacy. Report structural failures with path/anchor diagnostics, not unhandled tracebacks or network lookups.

Implement the specification's actual grammar and conformance cases, including:

### Membership and resolution

- Full required entry metadata, one parent-owned version, one declared BEI, and exact inventory membership. The entry is implicitly normative and is not repeated in the inventory.
- Every chapter-directory file is declared once with the correct kind. Enumeration may detect undeclared files; it must not automatically make them normative.
- Case-sensitive POSIX member paths confined to the same-named chapter directory; reject prohibited components, absolute paths, symlinks, duplicates, unreadable files, unsupported entries, and malformed UTF-8.
- Dependency links may use the specification's permitted normalized repository-internal relative paths. Validate confinement and symlink restrictions before reading targets. Do not fetch external citations.
- Resolve required local paths and anchors for dependencies, coverage, binding definitions, BEI, and compatibility aliases. Reject missing/ambiguous anchors and normative authority targets that resolve only to history.
- Check required chapter parent links, table shapes, nonempty required fields, and concrete declared applicability data. Do not pretend a structural check proves dependency completeness.

### Actual definitions, not mention counting

Recognize canonical definitions only using the specification's marked Markdown table-row grammar: the first cell contains the exact explicit HTML anchor, one space, and the matching backtick-delimited ID, in the correct invariant/obligation table with required nonempty columns.

Support existing role-qualified obligation IDs. Registry entries, BEI rows, aliases, prose mentions, headings, and copied ID lists are not definitions. There must be exactly one canonical normative definition and exactly one binding-definitions registry row for each current binding ID, with correct targets in both directions.

Ignore fenced examples using the specified backtick/tilde fence rules, including longer delimiters and info strings. Test matching closure behavior. Reject unsupported definition-shaped declarations, mismatched anchor/ID values, duplicate normative definitions, missing definitions, and definition padding. Preserve historical IDs as history; their presence cannot satisfy a current definition or become a second current authority.

### Coverage, provenance, aliases, and verdicts

Validate exactly the 32 coverage keys defined by this format revision: `artifact.section.01`–`18` and `kernel.section.01`–`14`, once each. Each applicable target must resolve to its real normative section; N/A needs the schema-permitted rationale. This is structural coverage, not certification that a heading contains adequate science.

Preserve BEI statuses, classifications, qualified binding references, sidecar provenance, supersession pointers, and owned unresolved-residue semantics. Do not silently promote historical or undecidable material.

Implement entry compatibility aliases as explicit non-defining aliases with unique real targets. No chains, cycles, duplicates, or alias-as-definition acceptance. Keep per-file source locations in diagnostics.

Preserve verdicts: PASS exits 0; hard FAIL exits 1; usage errors exit 2; PASS-DEFERRED exits 0 normally and 1 with `--strict`. Strict directory PASS is necessary but not sufficient for adoption. Never represent a legacy PASS as directory qualification.

### Conformance evidence

Add focused pytest fixtures and CLI tests covering the full specification matrix. Include a valid two-mechanism contract, qualified obligations, section dependencies, and compatibility aliases; original legacy tests; and default/strict deferred cases.

Negative cases must include removed definitions with IDs still present in registries/BEI; duplicate definitions; fenced fake definitions; missing/duplicate/unlisted chapters; history-only targets; malformed front matter/tables/UTF-8; unsafe paths and symlinks; broken dependencies/aliases; missing coverage keys; and invalid provenance/supersession/status fields. Test public CLI behavior, not only internal functions. Use isolated external fixture directories and sentinel files to detect prohibited outside reads.

Maintain an explicit semantic counterexample: removal of a qualifier while all structural IDs remain can pass structural lint but must fail preservation review. Do not add keyword matching and call it scientific verification.

Run the actual supported test runner. Resolve the earlier missing pytest dependency through the local development setup where possible. Do not carry forward hand-invoked test functions as a substitute for new pytest collection, fixtures, parametrization, or CLI coverage. A genuinely unavailable required runner is an explicit unmet requirement, not PASS.

## Phase 2 — migrate the complete LSE authority

After checker conformance passes, implement the LSE pilot using the reviewed clause inventory. Retain:

`docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001.md`

as the small canonical entry. Put chapters under its same-named sibling directory. Establish actual boundaries from the source; illustrative filenames in the specification are not a required empty-file scaffold.

Organize by coherent scientific concern. Keep a mechanism's equations, units, assumptions, domains, constants, guards, ordering, invariants, provenance, and test duties together. Shared definitions have one canonical home and precise dependencies. Avoid separate global equations/constants/guards bins that force every task to read everything.

Implement the entry metadata, inventory, reading routes, cross-cutting requirements, BEI locator, gap/status distinctions, and history links. Implement chapter parent links and conditional dependencies, the 32-key schema coverage map, actual definition tables, and BEI preservation mappings. Keep the detailed migration evidence in the package rather than making the entry another full catalog.

Resolve amendments into current effective rules only when existing authority establishes their supersession and applicability. In particular, the starting LSE purpose preserves earlier context and specifies limited later supersession; do not globally treat old wording as obsolete. Preserve surviving restrictions and distinguish admitted implementation authority from actual runtime/selector/qualification status.

Existing unindexed binding clauses may be assigned stable IDs through the specified flagged-binding-addition review, without inventing their meaning. An uncertain scientific interpretation remains binding and blocks adoption of the affected migration; it must not be hidden in historical material or replaced with a convenient paraphrase.

Detailed derivations may be separate, but assumptions and citations needed to justify current equations remain normative. Historical material retains its required provenance and original meaning. Preserve old logical IDs and incoming anchors through valid aliases or complete reconciliation of unpinned live consumers. Do not rewrite frozen historical packages.

Publish one coherent contract-set revision. Do not independently version chapters, upgrade maturity/qualification, or claim the contract is scientifically improved because its layout changed.

## Phase 3 — reconcile all affected consumers

Find whole-file assumptions in documentation, instruction routes, templates, Python validators, Rust `include_str!`/filesystem readers, schema/unit checks, and evidence identities. Inspect their actual data flow, not just filenames.

Move each live consumer to the declared authoritative modules/sections or a bounded shared contract-set loader, preserving its original substantive checks and diagnostics. The existing unit-governance checks must still inspect applicable unit and alias content after it leaves the entry. Schema/profile and Rust assertions must not pass by checking only navigation text.

Do not retain a second handwritten monolith, copy test-expected sentences into the entry, remove failing assertions, or silently skip directory contracts. An ID mention is not proof that the protected clause survived. Include a regression demonstrating that deleting an applicable obligation/unit mapping from a chapter still fails the corresponding consumer check.

Use a compatibility aggregate only when genuinely necessary, generated deterministically from canonical membership and never maintained as a second authority. Do not make it the default agent reading path or claim its unconditional loading is a saving.

Reconcile relevant live guidance and prospective templates atomically with adoption. Legacy contracts retain their whole-contract rules. Frozen kickoffs retain their original bindings unless separately amended through their authorized process; this package does not quietly reauthorize current Stage-3 experiments.

## Phase 4 — prove selective-reading usability and measured reduction

Run the specification's bounded exercises with fresh, non-forked, read-only agents that did not author the migration and do not inherit the parent's transcript. This request explicitly authorizes those exercise agents in addition to required assurance roles. Keep their number and returned output small; group compatible tasks, but record carried context between exercises rather than claiming repeated freshness.

Give each agent its task, applicable repository instructions, the candidate entry, and access to the primary modules. Do not give it the old monolith, completed answer rubric, author's preservation conclusions, or a summary that supplies the answers being tested. These are candidate-format usability exercises, not production authorization before adoption.

Exercise: (1) surface-energy/soil-coupling rule selection; (2) solver correctness with affected evaluators and error-order dependencies; (3) executable-identity verification; and (4) physical closure reconstruction. Require exact applicable rules/anchors, boundaries, prohibited shortcuts, necessary inputs/tests, and expansion reasons. Identity-only and scientific-closure verification must not collapse into the same reading requirement.

Independent assessment compares answers and selected dependencies with the frozen original-authority rubric. A missing guard, owner boundary, qualifier, closure operand, or normative dependency fails usability even if fewer bytes were read. Fix routes and repeat only affected exercises. Do not tune the rubric to bless incorrect selections.

Measure the same tasks and roles before/after: entry and module bytes, full files versus section ranges, unique and repeated exposure, automatically required instructions, recursively applicable normative dependencies, bootstrap and later expansion, and document/read counts where observed. Cross-contract entry/interface requirements still count. Cycles resolve to a finite union, not infinite repeated reading or omission.

Report LSE-specific reduction separately from total science-bootstrap reduction. The remaining contracts, history, or frozen kickoff can still dominate the total. Do not relabel mandatory immediate reads as optional expansion, count relocation as deletion, claim unobserved workflow totals, or convert bytes to a Pro-quota percentage. Acceptance needs a smaller sufficient set for bounded tasks, not an arbitrary percentage or smaller entry alone.

If the runtime cannot provide genuinely fresh non-forked sessions, record the failed capability and its acceptance impact. Do not silently substitute inherited-context reviews and call the exercise passed.

## Phase 5 — validation, independent assurance, and adoption

Apply current validation policy to declared intent and the exact diff. Start with inexpensive checks and preserve complete logs with concise results. At minimum execute:

- Actual pytest collection/execution for `tests/python/test_check_sc_binding_exposure.py` and all added/affected test modules.
- Directory-v1 conformance and public CLI negative cases; original legacy behavior and strict/default deferred semantics.
- `.venv/bin/python tools/check_sc_binding_exposure.py --strict docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001.md` against the migrated set.
- Applicable schema/profile, provenance, reference, and unit-governance checks, including the affected unit checker invocation with `--path` for LSE.
- Affected Rust/governance tests, selecting their actual Cargo/nextest targets and supported environment from the repository; formatting/lint for changed code as applicable.
- Clause-preservation completeness, independent semantic counterexamples, reading exercises, and comparable context reports.
- Exact write-set/terminal-diff, identities, historical-reference preservation, and no unauthorized production/science change checks.

Do not select a full scientific campaign merely because Markdown moved. Do not skip an independently applicable critical/authority requirement. Record reasons, exact commands, working directories, source identities, exit codes, and limitations. Use required runner delegation for selected heavy checks; record actual unavailability before an allowed command-execution fallback.

This user request explicitly authorizes subagent spawning/delegation to two independent reviewers and two independent terminal verifiers. Reviewers/verifiers may write only their assigned artifacts; the command runner may write only declared logs/results. Use existing scoped effort profiles and compact findings. Parent self-review is not independence.

Reviewer A owns scientific obligation preservation, bidirectional clause mapping, effective-rule/supersession reasoning, cross-contract boundaries, and authority-versus-activation status. Reviewer B owns parser/path safety, actual-definition checks, legacy compatibility, consumer validation, reading usability, and measurement honesty. Both inspect primary evidence and challenge ID-only or heading-only preservation. Neither may merely ratify the author's conclusions.

Fix and disposition every finding. Use focused re-review of changed material and dependencies rather than restarting unchanged reviews for editorial updates. Freeze the corrected substantive cut; obtain two independent terminal verifications of actual checker behavior, preserved authority, affected tests, fresh-agent exercises, exact identities, and final claims. Verifiers should independently run selected negative cases, including missing definitions with retained mentions and a consumer check against corrupted chapter content.

Only after ALL adoption gates pass enable LSE's selective-reading route and publish the coherent revision. Reconcile the final activation/routing diff and candidate hashes against the reviewed set so an unreviewed rule change cannot slip in at publication. Avoid circular self-hashing or repeated re-review solely for final report packaging.

If a genuine scientific ambiguity or external capability prevents adoption, keep/reinstate the original canonical LSE and whole-contract reading route using only package-owned changes. Retain clearly labeled candidate work and valid checker progress as evidence; report HOLD, not complete adoption. Do not leave a half-migrated authoritative contract or lift a scientific HOLD. A blocker in this pilot does not block unrelated authorized architecture work.

## Artifact economy and delivery

Use the existing package scaffold and role templates. Maintain one clause-preservation map, one checker/consumer conformance record, one paired context/usability record, required independent assurance artifacts, and final disposition/handoff. Generated tables may be rendered from one source; do not maintain contradictory copies or repeat the full scientific narrative in every report. Keep full authority available where the preservation work needs it, not in every bootstrap.

Final response must state:

1. Disposition: adopted, or exact unmet gates with retained legacy posture.
2. Local commit(s), baseline and final LSE revision, owned changed surfaces; no push.
3. Checker/legacy outcomes and actual pytest, unit/schema, and Rust checks run.
4. Clause/ID/qualifier preservation and independent review/verification results.
5. Fresh-agent reading-exercise results and measured LSE/task reductions, with unresolved overall-context/runtime limits.
6. Exact next-agent entry point and canonical checker invocation.

Deliver the working checker and usable LSE adoption in this package. Do not stop after another approved design or a checker that has never consumed the migrated real contract.


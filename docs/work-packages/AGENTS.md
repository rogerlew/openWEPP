# docs/work-packages/AGENTS.md
> Agent playbook for openWEPP work packages.

## Authorship
**This document and all AGENTS.md documents are maintained by GitHub Copilot / Codex / Claude Code, which retain full authorship rights for all AGENTS.md content revisions. Agents may author and revise AGENTS.md documents when and where they see fit.** Revisions must preserve applicable user direction, package scope, review expectations, and higher-precedence governance.

## Mission Snapshot
- Own autonomous execution specs under `docs/work-packages/<id>/`.
- Preserve package governance: scope, evidence, gates, review, verification, line-count disposition, and closure truthfulness.
- Keep package artifacts as evidence, not replacement authority for canonical `SC-*` contracts.
- Make packages right-sized: coherent closure slices, not one-symbol diagnostic relays.
- Treat defect-closure (`DC`) packages as autonomous closure work by default:
  diagnose, correct, validate, review, and disposition inside the declared
  authority envelope unless a proven boundary prevents correction.

## Primary Assets / Key Files
- `docs/work-packages/README.md` — package catalog and process map.
- `docs/codex_exec_plans.md` — base ExecPlan expectations.
- `docs/defect_closure_execplans.md` — DC-ExecPlan requirements.
- `docs/standards/mechanical-refactor-authoring-guide.md` — mechanical refactor closure loop and artifact expectations.
- `docs/standards/code-quality-refactor-authoring-guide.md` — metric-driven behavior-preserving CQR requirements.
- `docs/standards/kernel-work-package-preparation.md` — required kernel package preparation procedure.
- `docs/standards/prompt-wording-guidance.md` — required prompt wording standard.
- `docs/work-packages/cqr-nightly-burndown-execplan.md` — operator-facing rolling CQR nightly process.
- `docs/work-packages/templates/cqr-nightly-package.md` — per-target CQR nightly package template.
- `docs/work-packages/templates/cqr-nightly-kickoff-prompt.md` — per-target CQR nightly kickoff prompt template.
- Package-local `package.md`, `prompts/`, and `artifacts/` directories.

## Standard Workflow
1. Confirm the package is authorized by queue, decision, or user direction.
2. Read root `AGENTS.md`, this file, package-local `package.md`, and any nested `AGENTS.md` for files in the write set.
3. Run `tools/agents/find-agents --for <declared-write-set>` before edits and
   record the applicable instruction files in `artifacts/required-reading-map.md`
   or equivalent package intake evidence.
4. For kernel-affecting work, also read `docs/specifications/science-contracts/AGENTS.md` before edits.
5. Execute package phases end-to-end unless a declared hard blocker is reached.
6. Update artifacts truthfully as work proceeds; label `Static:` vs `Ran:` evidence.
7. Complete dual independent reviews, explicit finding disposition, dual verification, line-count governance, and final disposition before closure.

## Gate Evidence Non-Deferral Rule
- A package, phase, or staged increment is complete only when every required
  increment-scope exit criterion and gate has direct evidence in the current
  artifact set. Boundary assignment follows declared pre-implementation intent
  and exact-diff terminal reconciliation under
  `docs/standards/testing-and-gate-strategy.md`.
- A campaign-owned obligation may remain `DEFERRED` only when declared before
  implementation with a named later boundary, owner, trigger, and rationale in
  the package/campaign record. Deferred is not passed, waived, or evidence for
  the current claim.
- If an increment-scope required gate can be proven only by a later
  phase/increment, the current phase is not complete. It must be marked `HOLD`
  / `executed-hold` with the later dependency named as the blocker.
- For DC-ExecPlans, this rule is not permission to stop early. If a gate is
  unmet because implementation, validation, or source reading remains inside
  the declared envelope, continue executing. `HOLD` is valid only after the
  package records why the missing evidence cannot be produced in-envelope.
- Do not reclassify an unmet current requirement as "next increment scope" after
  execution has started. Allowed alternatives are:
  1. execute the missing evidence in the current scope,
  2. amend the package/plan before implementation with explicit review that the
     gate is no longer current-scope acceptance, or
  3. hold with a named blocker and a defect-shaped follow-on.
- Validation tables must classify each required criterion as `PASS`, `FAIL`,
  `BLOCKED`, or `NOT RUN`; any `FAIL`, `BLOCKED`, or unjustified `NOT RUN`
  prevents `complete` disposition.
- Review and verification artifacts must check this rule explicitly. A review
  that verifies artifact presence but not requirement legitimacy is incomplete.
- Handoff language such as "lands in the next increment" is valid only for work
  that was not a current required gate. If it was a current required gate, that
  phrase must be paired with a hold disposition and blocker rationale.

## Science Implementation And Calibration Readiness

- ADR-0042 distinguishes science authority, data authority, and calibration
  readiness. When authoritative process science exists and is in scope,
  implement it even if observations are insufficient for unique calibration.
- Data limitations constrain empirical calibration, identifiability,
  validation, uncertainty, and transferability claims. They do not authorize
  proxy physics, silent defaults, invented physiological bounds, or an
  implementation `HOLD`.
- Parameterized-science packages must declare intent before edits:
  `implementation`, `calibration-readiness`, `empirical-calibration`,
  `independent-validation`, or an explicit combination.
- When empirical calibration cannot close for lack of suitable data, continue
  through applicable readiness work: typed/enumerable parameters, a
  unit/scale-defined observation operator, deterministic candidate execution
  and objective reconstruction, sensitivity/identifiability analysis,
  boundary/failure/equifinality reporting, and synthetic recovery where
  structurally meaningful.
- Synthetic recovery proves only that suitable information can pass through
  the implementation and calibration machinery. It is not empirical
  calibration, real-world identification, external validation, or
  transferability evidence.
- Values or bounds introduced only to execute a demonstration must be labeled
  `ASSUMED_FOR_EXECUTION`; never label them observations, probability priors,
  physiological bounds, or calibrated results.
- Report three orthogonal fields in `artifacts/calibration-readiness-matrix.md`
  or its machine-readable equivalent:
  `science_implementation_status` (`IMPLEMENTED`, `NOT_IMPLEMENTED`,
  `AUTHORITY_MISSING`), `calibration_evidence_status`
  (`EMPIRICALLY_CALIBRATED`, `CALIBRATION_READY_DATA_LIMITED`,
  `NOT_CALIBRATION_READY`, `NOT_APPLICABLE`), and
  `identifiability_status` (`IDENTIFIED`, `PARTIALLY_IDENTIFIABLE`,
  `NONIDENTIFIABLE`, `NOT_ASSESSED`, `NOT_APPLICABLE`).
- The readiness matrix must disposition every obligation named by
  `docs/specifications/science-contract-spec.md` as `PASS`, `BLOCKED`, or
  `NOT_APPLICABLE`, with evidence path and rationale. A required current-scope
  `BLOCKED` row forces `NOT_CALIBRATION_READY` and package `HOLD`.
- Assign measured observations prospectively to `CALIBRATION`,
  `INDEPENDENT_VALIDATION`, or `DIAGNOSTIC_ONLY`. Do not reuse calibration data
  as independent validation; a reviewed exception forfeits independence.
- Missing, sparse, correlated, interval-censored, scale-mismatched, or
  non-identifying data alone is not a legitimate hold boundary when
  authoritative implementation and in-scope readiness work remain possible.
- ADR-0024 and ADR-0028 authority-admission routes remain available. Hold for
  missing science authority only when no applicable route has succeeded.

## Manual Validation Planning And Tool Friction

- Agents select and execute applicable requirements directly from this file,
  the canonical testing strategy, package authority, and affected contracts.
- Validation planning has no prospective executable, planner, receipt,
  lifecycle state, or repair prerequisite. Use the manual route in ADR-0043.
- Record useful tool defects as ordinary debt. A known unmet underlying
  requirement still prevents truthful closure.
- Run cheap deterministic checks before expensive work: package/write-set
  reconciliation, diff hygiene, documentation/schema checks, required artifact
  presence, prompt state, and line-count governance.
- When overriding `TMPDIR`, follow
  [Temporary Directory Placement](../standards/local-ci-gate-selection.md#temporary-directory-placement):
  use an absolute scratch directory outside the checkout, never `target/` or
  another repository descendant. Assurance publication fixtures intentionally
  reject repository/staging/public/snapshot root overlap.

## Consumer-Path Closure Rule
- A package that claims `endpoint`, `direct`, `cutover`, `publication`,
  `ready`, `activation`, or equivalent production-readiness language must prove
  the real downstream consumer reads the new path.
- Producer-only, skeleton-only, counter-only, shadow-only, and
  direct-runtime-internal evidence is iteration evidence. It cannot close a
  consumer-facing gate unless the package is explicitly characterization-only
  and makes no readiness or cutover claim.
- Required consumer-path evidence includes a current package artifact naming:
  producer source, in-memory state/frame object, runner handoff, downstream
  consumer call site, output or API surface, and the negative proof that the old
  compatibility path is not used for that claim.
- Historical direct skeleton/shadow transition modes such as
  `DirectSkeletonNoop`, `DirectSkeletonShadowOnly`, and
  `DirectPublicationFrameCutover` are deleted runtime selections. Do not revive
  them for new evidence; use production direct execution or an explicitly named
  diagnostic harness instead.
- Before closure, run a "what still reads the old path?" check over the named
  downstream consumers. If any current-scope consumer still reads compatibility
  state, runtime symbols, writeback payloads, stale logical state, or a wrapper
  around those structures, the package must close in `HOLD` or continue until
  the consumer is moved.
- If that check exposes a blocker outside the package envelope, immediately
  scaffold or update a hold-lift package with the concrete blocker, write set,
  consumer-path proof requirements, and first actionable implementation step.
  Do not leave only a narrative handoff note.

## Conservation / Publication Acceptance Rule
- For packages that create, correct, or aggregate conservation-sensitive output
  surfaces (water, sediment, energy, mass, routed runoff, or closure ledgers),
  author an operand-lineage table before production edits. Record field name,
  units, normalization/denominator, area or volume basis, source authority, and
  whether each operand is authoritative or diagnostic.
- Regression fixtures must separate every plausible alias that could mask a
  wrong formula. The expected value must differ from rejected candidates such as
  adjacent diagnostic columns, publication areas, internal state areas, per-OFE
  sums, and legacy/interchange aliases when those candidates are in scope.
- Exact self-consistency checks and one-sided bounds are sanity evidence only.
  They cannot close a conservation/output acceptance gate by themselves.
- Acceptance must include independent reconstruction from produced outputs and
  a real closure or magnitude audit on the target fixture/cohort. Include
  two-sided magnitude/ratio checks when a physical range is known, and anchor
  checks for protected output surfaces.
- Reviews and verification must explicitly check anti-tautology: the gate must
  not restate the producer formula with the same operands, and metadata/schema
  descriptions must match the accepted operand lineage.

## Subagent Delegation Authorization
- Work packages that require delegated review, verification, comparator execution, or parallel agent work must explicitly authorize subagent spawning/delegation in `package.md` and the active kickoff prompt.
- Use direct wording: `Subagent authorization: this package explicitly authorizes spawning/delegating to <role> subagents for <scope>; expected outputs are <artifacts>; write access is <read-only|bounded write-set>.`
- Naming a role, saying `dispatch`, or listing an agent config path is not sufficient; include `explicitly authorizes subagent spawning/delegation` so tool policies can recognize user-approved delegation.
- Higher-precedence tool policy may still require explicit user/session
  authorization before any spawn. Package authorization is necessary repo
  governance, but it may not be sufficient by itself in newer tool sessions.
  Use the standing authorization wording in
  `docs/standards/prompt-wording-guidance.md` for recurring openWEPP launch
  prompts so agents do not need a per-task reminder.
- If a package lacks explicit authorization, do not claim delegated work occurred. Either run the gate locally when equivalent, or record the missing authorization as a package-documentation defect/blocker and update the package before delegated closure.
- If package authorization is present but session-level tool authorization is
  absent or blocked, ask for one-time authorization before spawning, or record
  the tool-policy block and run an equivalent local gate only when the package
  allows local substitution.

## Work-Package Authoring Requirements
- Use directory format `YYYYMMDD-<slug>-001` under `docs/work-packages/`.
- Add or update `docs/work-packages/README.md` so intent is discoverable.
- Scaffold `package.md`, `prompts/active/`, `prompts/archived/`, and `artifacts/` with queued placeholders.
- Encode status, objective, rationale, included/excluded scope, deliverables, dependencies, intended write set, phase plan, exit criteria, and security-impact gate.
- Encode exit criteria so each required gate is measurable inside the package or
  explicitly declared as a hold boundary before work starts. Do not author
  staged plans where an increment's required gate depends on a later increment's
  evidence while still allowing the earlier increment to close as complete.
- For conservation-sensitive output work, encode the Conservation /
  Publication Acceptance Rule as a current-scope gate unless the package is
  explicitly characterization-only.
- Encode explicit subagent authorization when package-required work depends on delegated reviewers, verifiers, comparator runners, or other role agents.
- Require dual reviews with finding disposition: `accepted`, `rejected`, `deferred`, or `follow-up`.
- Require `.rs` line-count governance: 2000+ lines is `WARN`; 3000+ nonexempt files require refactor before closure.

## DC-ExecPlan Requirements
- Use a Defect-Closure ExecPlan when closing an observed invariant violation, fail-closed event on valid input, or conservation residual.
- Declare the Correction Authority Envelope: defect IDs, observed failures, in-scope contracts/source files, allowed edit classes, validation surfaces, acceptance criteria, and protected boundaries.
- If the root cause is in-envelope and expected behavior is supported by canonical `SC-*` authority, pinned-baseline provenance, or a contract-authorized physical invariant, land the contract-first correction in the same package.
- Close in `HOLD` only at a declared boundary: out-of-envelope mechanism,
  missing/contradictory authority, invalid upstream input with correct typed
  guard, unavailable evidence, or different process family/contract authority.
  Diagnostic uncertainty, implementation effort, large edit size, or a
  partially working compatibility path are not hold boundaries.
- Before a DC package may hold, record a `HOLD legitimacy audit` artifact or
  section that names the boundary, cites the evidence proving it, lists the
  in-envelope correction route that was considered, and explains why that route
  cannot close the defect in the current package.
- If review finds that the package used a shortcut, wrapper, skeleton, shadow
  path, compatibility bridge, or incomplete direct path while an authority-backed
  production correction was in scope, the finding is closure-blocking and must
  be fixed before disposition.
- For kernel/process-physics defects, the acceptable production correction is
  baseline-authoritative or contract-authorized physics. Do not introduce
  surrogate, provisional, proxy, empirical stand-in, or heuristic process
  physics into production paths. If actual physics authority is missing, hold
  for authority; if it is present and in scope, implement the actual physics.
- The handoff's first actionable item must be `close defect <id>`, not a vague trace/inspect step.

## Mechanical Refactor Requirements
- Follow `docs/standards/mechanical-refactor-authoring-guide.md` for structural, behavior-preserving work.
- Required terminal validation follows the declared intent, exact terminal
  diff, and canonical testing strategy. Critical refactors and campaign/release
  boundaries retain full-workspace correctness. Coverage/CRAP is observational
  unless the package is explicitly CQR or module test enhancement. Focused,
  quick, frost, and erosion profiles claim only the surfaces they execute.
- Fall back to `cargo test --workspace` only for libtest-specific behavior or explicitly required legacy harness checks, and label that as a compatibility run rather than the default closure path.
- Package-required validation overrides generic ambient instructions to skip tests.
- Reconcile tests mechanically only; do not hide semantic changes inside refactor diffs.

## Observational Quality And Explicit Metric Packages

- Workspace coverage and adjudicated CRAP are observational quality evidence.
  Their absence, staleness, or debt verdict does not block ordinary
  implementation-package, campaign, or release closure. A valid optional QA
  report has `closure_eligible=false`.
- An explicitly authorized CQR/module-test-enhancement package retains binding
  package-local metrics. Its declared eligible coverage/obligation thresholds
  and owned actionable CRAP target must pass or the package holds. Unrelated
  workspace debt remains visible and non-blocking.
- Raw rows above 30 remain visible. A raw row is non-actionable only when it
  matches an exact, current entry in
  `tools/release/adjudicated_crap_exceptions.json`. Wildcards, filename-based
  exclusions, inline package waivers, and unreviewed additions are forbidden.
- Changing an adjudicated function's host-file hash, semantic role, complexity,
  public behavior, or consumer posture invalidates its prior disposition.
  Registry changes require an authorized package and two independent reviews
  applying ADR-0021's symbol-level taxonomy.
- Fresh measurement and package-local metric closure are
  canonical-registry-only and source-snapshot-bound. The
  before/after/final source manifests must match; a source or Git-index change
  during metric collection invalidates the run.

## CQR Nightly Burndowns

- Before the first module implementation edit in a multi-package CQR batch,
  scaffold and commit one aggregate admission package when campaign closeout
  will require one exact terminal diff. Its base-commit write set must cover
  the master plan, all module package trees, intended source/test paths, and
  closeout evidence. Per-module packages remain mandatory and one-module-only.
  Missing aggregate authority is a pre-implementation blocker; never repair it
  by retroactively widening an older package. Commit a package-local batch
  manifest with the aggregate scaffold; bind its master ExecPlan and complete
  module/path inventory in every module scaffold. Commit each module scaffold,
  then require a retained PASS from
  `tools/local_ci/check_cqr_aggregate_admission.py` before its first
  implementation edit.
- Operator phrasing such as `execute cqr nightly for 8 modules` means: read
  `docs/work-packages/cqr-nightly-burndown-execplan.md`, consume a verified
  current quality-observatory report, select the requested number of eligible
  production modules, and scaffold one package per module from
  `docs/work-packages/templates/cqr-nightly-package.md`, and execute each package
  end-to-end.
- CQR Nightly intake must run
  `tools/local_ci/cqr_quality_evidence.py inspect` against the exact compact
  observation, complete control receipt, and expected evidence ID. Only a
  `CURRENT` receipt may seed selection. Fresh recollection is allowed only
  after `authorize-recollection` binds a typed `STALE`/`INVALID` receipt to an
  explicit operator CQR directive.
- Treat each new imperative CQR-nightly request as authorization for a **fresh,
  separately numbered batch**, even when a completed nightly batch is already
  present in the worktree or recent history. A prior batch may inform exclusions,
  but it never satisfies or suppresses the new request. Interpret the request as
  status-only or audit-only only when the operator explicitly asks to inspect,
  summarize, verify, or avoid rerunning an existing batch.
- Each scaffolded package must also copy
  `docs/work-packages/templates/cqr-nightly-kickoff-prompt.md` into
  `prompts/active/` and fill in `Execution mode`, `Autonomy`, tiered required
  reading, required-reading budget/map, and required heavy-run subagent wording.
- CQR nightly packages are behavior-preserving maintenance. They may add
  characterization tests and decompose high-CRAP functions, but they must not
  change science formulas, contract authority, thresholds, serialization,
  fail-closed semantics, or public output meaning.
- When characterization tests are added or materially changed, record ADR-0021
  coverage closure: tier assignment, line/region threshold status,
  per-function region-floor disposition, and obligation-to-test binding.
- Before selecting or suppressing a CRAP row, apply ADR-0021's symbol-level
  eligibility taxonomy. Preserve raw and actionable counts separately; default
  hand-authored behavior to eligible; require exact evidence and dual-review
  acceptance for every retained exception or denominator exclusion. Filename,
  module role, wildcard, prior disposition, and “hard to test” are insufficient.
- Commit discipline is part of the process: create a scaffold commit before
  implementation edits for each selected module, then create either a completion
  commit or a hold-evidence commit before moving to the next selected module.
- Local target holds roll back only that package's implementation edits, preserve
  and commit hold evidence, and may continue to the next target. Global/process
  holds stop the nightly batch.

## Validation Checklist
- Package-specific gates from `package.md`.
- Gate evidence non-deferral: each required current-scope gate has current
  direct evidence, or the package/phase is held with a named blocker.
- Declared implementation intent, exact terminal-diff reconciliation, and exact
  commands/results for every applicable current-scope requirement.
- Selected full-workspace correctness regression at critical, campaign, and
  release boundaries; quality remains observational unless explicitly owned.
- Package-local coverage/CRAP only for explicit CQR/module-test-enhancement
  objectives.
- Conservation/publication acceptance rule when output magnitude or closure
  evidence is in scope.
- Doc-path integrity checks when moving documentation or required-reading lists.
- Source-level anti-evasion guards when touching external-authority suite posture, cohort fixtures, or required-case bindings.
- Dual review, review-disposition, dual verification, line-count governance, worker handoff, and disposition artifacts.

## Release-Binary Evidence Provenance
- For timing, comparator, release-candidate, or acceptance evidence that invokes
  a release CLI binary, build the exact binary target before running evidence;
  do not assume generic workspace `cargo build --release` relinks every
  non-default binary member.
- For openWEPP runner CLI evidence, the canonical broad build is
  `cargo build --release -p openwepp-runner --bins`; a narrower package may use
  explicit `--bin` names only when it records the exact binaries required.
- Record the build command, binary path, mtime/size or hash, and evidence run
  command in the package artifact before accepting output hashes, timings, or
  comparator deltas. If the binary provenance is stale, missing, or ambiguous,
  rerun the evidence after rebuilding.
- When a runfile or fixture hardcodes output paths, record that behavior and
  sequence/hash the actual output directories. Do not infer that an
  `--output-dir` flag relocates every produced artifact unless the fixture
  proves it.

## Common Pitfalls
- Do not close a package while accepted review findings remain unfixed or undispositioned.
- Do not mark gates as run when they were reasoned about or partially executed.
- Do not mark an increment complete when one of its required acceptance gates is
  waiting on a later increment's evidence.
- Do not convert a DC package into a diagnostic relay because the direct fix is
  larger than expected. Widen within the declared envelope or amend the package
  before implementation; after execution starts, continue to closure unless a
  legitimate boundary is proven.
- Do not implement compatibility wrappers, adapter detours, or skeleton paths
  when the package objective is direct production adoption and the actual
  consumer can be moved.
- Do not add surrogate or proxy physics to production code. Missing authority is
  a hold boundary; known in-scope physics is an implementation obligation.
- Do not close conservation-sensitive output work on exact self-consistency,
  one-sided bounds, or tests where wrong formulas alias the expected value.
- Do not use package artifacts to override canonical contract authority.
- Do not split a package solely to defer a known in-envelope correction.

## References
- Root guidance: `AGENTS.md`.
- Science contracts: `docs/specifications/science-contracts/AGENTS.md`.
- Prompt/procedure standards: `docs/standards/AGENTS.md`.

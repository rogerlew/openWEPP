# specialized-workflows

Read only sections triggered by the assigned mechanism or package type. Their explicit mandatory references remain binding. Relocated obligations, no waiver.

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

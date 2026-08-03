# Independent QA Review — Agent B

Status: **findings / HOLD before complete disposition**

Evidence mode: **Static + Ran (read-only integrity checks)**

Static: reviewed the terminal tracked/untracked diff, governing contracts,
production call sites, tests, package plan, and package evidence. Ran: checked
`git diff --check`, compared the frozen W2A scientific-rule projection, verified
the freeze/receipt/results/adjudication hashes and release-binary hashes, and
inspected retained Nextest JUnit metadata. I did not rerun Rust test or lint
suites; the retained full profile reports `2195/2195` passed.

## Findings

1. **HIGH — The result-bearing W2A rerun did not satisfy its prospective
   prerequisite order.** `package.md:108-112` requires criteria 1-6, including
   every critical validation plus dual review/disposition and dual verification,
   to pass before the frozen contrast reruns. The retained freeze records
   `2026-08-02T19:09:57Z`, while `target/nextest/quick/junit.xml` completed at
   13:19 local and `target/nextest/full/junit.xml` started at 14:06 local; both
   were after the 12:09 local freeze. Reviews/verifications also remain pending
   at `package.md:146`. This is additionally inconsistent with the phase order
   at `package.md:90-94`, which schedules the rerun before review. The frozen
   cells, models, operators, thresholds, artifact hashes, and binary hashes do
   match W2A, so the defect is sequencing rather than rule mutation. Treat the
   existing run as prerequisite-ineligible and perform/retain an exact frozen
   rerun only after all stated prerequisites pass; do not weaken criterion 7
   after seeing the result.

2. **HIGH — Repository-facing disposition is prematurely `complete`.** The
   package is still `active / executing` (`package.md:3`) with review,
   verification, and disposition unfinished (`package.md:146`,
   `artifacts/disposition.md:3-7`, `artifacts/review-disposition.md:3-7`), and
   documentation lint is explicitly still pending
   (`artifacts/gate-results.md:21-25`). Nevertheless `docs/ROADMAP.md:34`,
   `docs/planning/snow-surface-energy-balance-roadmap.md:157`, and
   `docs/work-packages/README.md:5080-5087` publish the package as complete and
   advance EB-04X. Keep those surfaces in executing/review-pending state until
   all current-scope gates and finding dispositions actually pass.

3. **MEDIUM — The critical shared fail-closed guard and tolerance edges are not
   contract-derived regression tests.** The public-partition vectors at
   `tests/integration/snow_surface_eb04w_accumulation_melt_diagnostics_contract.rs:122-156`
   prove successful warm snow, mixed, and all-rain behavior, but only assert a
   reconstructed residual on successful outputs. They never force the shared
   validator at `runoff_reconciliation.rs:436-458` to reject a material or
   non-finite residual. The runner-only helper test at
   `snowbench_coe_melt.rs:1183-1189` covers `+1.1e-9` rejection and `-1.0e-9`
   acceptance, not the shared boundary. It also omits the strict activation
   edge implemented at `runoff_reconciliation.rs:298-305` (exactly `1e-12 m`
   snowfall depth versus the next representable value above it). Add direct
   negative shared-boundary coverage and exact/just-over activation and closure
   threshold cases. Also clarify `SC-SNOWFREEZE-001.md:333`: it cites
   `TOL-SNOWFREEZE-013`, whose definition at line 1255 is a group of diagnostic
   residual tolerances, not an explicitly named snowfall-presence threshold.

4. **MEDIUM — Terminal gate evidence is too lossy for the claims it makes.** The
   table at `artifacts/gate-results.md:7-19` records counts but omits exact argv,
   working directory, source/diff identity, exit status, and log/receipt paths
   for the focused tests, owning-crate run, targeted warnings-denied Clippy,
   assurance validation, formatting, and most workspace profiles. The full
   `2195`-test JUnit is present and passing, but the artifact does not meet the
   exact-command/result requirement in `docs/work-packages/AGENTS.md:322-329`.
   The freeze's `source_dirty_diff_sha256` is also no longer the current tracked
   diff hash (`cc25fc...` retained versus `0ae3dd...` observed), with later
   roadmap/assurance edits not reconciled in a terminal exact-diff artifact.
   Record exact commands and identities, finish documentation lint after the
   review artifacts land, explicitly disposition `cargo deny check` as
   not-applicable because no manifest/lock/resolution input changed, and perform
   final exact-diff reconciliation before closure.

5. **LOW — Line-count governance suppresses the required warning.** The checklist
   calls the 2,531-line `runoff_reconciliation.rs` below a “3,000-line review
   trigger” and marks the gate passed
   (`artifacts/line-count-governance-checklist.md:3-12`). Repository policy makes
   every 2,000+ line Rust file `WARN` and requires both a decomposition rationale
   and follow-up split intent (`crates/AGENTS.md:57-60`). Record the warning and
   follow-up intent even if this bounded change does not justify an in-package
   extraction.

## Non-blocking debt / residual risks

- The production edit is localized and readable; static inspection found no
  altered phase equation, selector, coefficient, default, or melt formula.
- The shared and snowbench reconstructions use independent typed/source snowfall
  operands rather than reported `accumulation_m`; warm snow, mixed, and all-rain
  nominal coverage is present.
- The exact W2A cells/models/operators/thresholds compare equal, all eight receipt
  return codes are zero, artifact hashes close, and both retained release-binary
  hashes match. The current source-diff drift appears documentation/assurance
  related, but it still needs explicit terminal reconciliation.

QA disposition: **HOLD** pending the findings above. No production-formula
correctness defect was identified in this review.

---

## Terminal Re-review — Corrected Diff and Proposed Cross-Domain HOLD

Status: **HOLD disposition accepted / documentation and evidence findings
remain**

Evidence mode: **Static + Ran (read-only integrity checks)**

Static: independently re-reviewed the corrected production paths, typed error
surfaces, boundary and real-consumer tests, amended contracts, assurance
adoption chain, rerun source-manifest changes, roadmap/catalog disposition, and
the EROD16 coverage instrument. Ran: `git diff --check` passed; inspected the
retained quick and isolated Nextest JUnit receipts. I did not rerun Rust suites.

### Findings

1. **HIGH — B2 is not fully corrected: the campaign narrative still publishes
   the withdrawn scientific result and advances EB-04X.** The milestone table
   correctly records `HOLD_CROSS_DOMAIN_CORRECTNESS_GATE`, the
   prerequisite-ineligible rerun, W2C authorization, and EB-04X hold
   (`docs/planning/snow-surface-energy-balance-roadmap.md:157-159`). However,
   the narrative at lines 182-185 still says the unchanged rerun found no
   material albedo response and that EB-04X follows. That directly contradicts
   the table, `scientific-synthesis.md`, and the corrected B1 disposition. It
   also makes `review-disposition.md:14` ("all repository-facing surfaces")
   false. Replace that narrative with the cross-domain HOLD, state that the
   first rerun is inadmissible, and keep EB-04X held behind an authorized W2C
   diagnosis.

2. **MEDIUM — B4 is improved but remains incomplete; the `37/227` causal
   reversal is not reproducible from retained evidence.**
   `artifacts/gate-results.md:33-40` now records exact argv and exit results for
   the corrected focused, owning-crate, frost, Clippy, assurance, and quick
   gates, and lines 46-48 correctly withhold full/rerun and disposition
   `cargo deny check` as N/A. Lines 41-42 are descriptions rather than exact
   argv, though, and retain neither the temporary old-trigger patch/diff
   identity nor its machine-readable output. The current default JUnit has been
   overwritten by a later corrected-trigger run, so it proves a second
   `61/231` failure but cannot substantiate `37/227`. The quick JUnit and
   corrected isolated JUnit are sufficient for the HOLD; the stronger claim
   that changing only the old trigger restores `37/227` must either be labeled
   an unretained diagnostic observation or backed by the exact command,
   temporary source identity/patch digest, and retained log/receipt.

3. **LOW — Package-local evidence indexes have minor terminal-state drift.**
   `artifacts/owned-file-manifest.md:19` says "one transaction," while the
   final assurance chain contains both `9ad8f170...` and `d18e6602...`; line 20
   calls the roadmap update an EB-04X handoff despite the hold.
   `artifacts/contract-test-evidence.md:3-10` also remains a pre-implementation
   red-only status even though the corrected target is retained as `6/6`
   elsewhere. Reconcile these summaries so future readers do not have to infer
   terminal state across conflicting artifacts.

### B1-B5 terminal disposition

| Finding | Re-review result |
|---|---|
| B1 prospective rerun order | **Closed for HOLD.** The first rerun is explicitly prerequisite-ineligible and withdrawn; no replacement rerun was attempted after the renewed quick failure. |
| B2 premature completion/roadmap | **Open.** Root roadmap, package catalog, and milestone table are corrected, but the campaign narrative at lines 182-185 is stale and materially contradictory. |
| B3 guard/tolerance tests | **Closed.** Shared-boundary tests cover exact tolerance, both next-representable signs, and non-finite rejection; integration coverage binds exact/just-over activation to named `TOL-SNOWFREEZE-014`. |
| B4 exact terminal evidence | **Partially closed.** Corrected primary gate argv/results and `cargo deny` disposition are present; exact source/diff identity and a reproducible old-trigger reversal receipt are not. |
| B5 line-count governance | **Closed.** Both 2,000+ Rust files are WARN with a bounded-change rationale and explicit follow-up split intent. |

### Non-blocking debt / follow-ups

- The corrected implementation is cohesive and readable: provider availability
  is separated from phase/activation authority, structured kernel errors retain
  their source, and consumer-only closure failure has a distinct typed variant.
  No equation, coefficient, selector, melt formulation, or silent numerical
  fallback was introduced.
- The new direct-production test exercises the real warm-mean/zero-pack path,
  while shared-boundary and snowbench tests cover positive, negative,
  next-representable, non-finite, and noncanonical-density cases. This closes
  the substantive test-quality concern from B3.
- The rerun wrapper's prospective source manifest is materially improved: it
  hashes all W2B result-affecting touched source/contract/test files (including
  the untracked direct-consumer test), a sorted manifest digest, the tracked
  dirty diff, and the wrapper. No new rerun used it, which is correct while the
  prerequisite gate is red.
- The assurance adoption is internally consistent despite the stale owned-file
  summary: the two checked `scientific-full` transactions form
  `4d83e2a9... -> a26a0352... -> f2b8a335...`; the identity lock binds the
  current `SC-SNOWFREEZE-001` hash `4eccdd17...` and snow-report review-lock
  hash `002e8c5e...`, with no invalidated authority reported.
- The EROD16 failure is a genuine hard correctness/coverage failure, not an
  optional metric: the pre-existing instrument requires refusals `<=20%` of
  real storm days. Retained quick and isolated corrected-trigger receipts both
  report `61/231` with `170` clean, conserving/depositing solves. The production
  fixture completing does not waive the failed coverage assertion.
- Withholding the terminal full profile and frozen W2A rerun is correct after a
  deterministic required quick-gate failure. Erosion solver mechanics and any
  prospective amendment of the `<=20%` authority are outside this package and
  require the bounded W2C authorization described in the corrected milestone
  table.

QA disposition: **PASS for `HOLD_CROSS_DOMAIN_CORRECTNESS_GATE` only.** Do not
claim package completion, a terminal albedo result, or EB-04X admission. Correct
the stale roadmap narrative and qualify or retain the old-trigger diagnostic
evidence before treating this re-review ledger as fully reconciled.

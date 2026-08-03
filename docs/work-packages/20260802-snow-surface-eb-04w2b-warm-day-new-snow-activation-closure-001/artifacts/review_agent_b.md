# Independent QA Review B — Resumed Terminal Closure

Latest status: **PASS — terminal-v2 corrections accepted; terminal verification pending**

Evidence mode: **Static + Ran**

Review point: current worktree at HEAD
`a74af48b8e98f91b5d5acdebc0e2da0bf988ba36`. This resumed review replaces
Agent B's operative historical text. The original review and re-review HOLD
dispositions remain relevant as commit history and are summarized below; they
are not rewritten as passes.

## Findings

### HIGH — RB-01: the terminal rerun reused the rejected first run's release binary

The terminal freeze records `target/release/openwepp-snowbench` SHA-256
`79a8a00a...f258`, exactly the same binary hash recorded by the
prerequisite-ineligible first freeze. The binary mtime is
`2026-08-02 12:09:43 -0700`, immediately before the first freeze at
`2026-08-02T19:09:57Z`; the resumed freeze is almost twelve hours later at
`2026-08-03T07:00:33Z`. No exact release build command, post-build metadata, or
new binary identity appears in `artifacts/gate-results.md` before the resumed
run.

`tools/run_frozen_w2a_rerun.py:65-89` hashes current source and the existing
binary but never builds the executable. The explicit source manifest reproduces
against the current W2B files, but it does not prove that those sources produced
the executable that actually ran. This violates the release-binary provenance
rule in `docs/work-packages/AGENTS.md` and makes the new JSON/CSV result
ineligible for terminal scientific adjudication.

Required correction: build the exact release binary target, retain command,
path, mtime/size and hash, then execute a newly named immutable rerun set. Do not
overwrite either existing rerun generation.

### HIGH — RB-02: the purported immutable rerun overwrote historical result-bearing artifacts

The wrapper gives freeze, receipt, results, summary, output directory, and
adjudication new terminal names, but it leaves `w2a.ARTIFACTS` and
`w2a.FIGURES` pointed at the original W2B artifact locations
(`tools/run_frozen_w2a_rerun.py:52-64`). The imported analyzer writes four SVGs
to that shared figure directory and rewrites `artifacts/scientific-synthesis.md`.

The current diff proves all four historical SVGs were overwritten. The
historical synthesis headed `Scientific Synthesis — Prerequisite-Ineligible
Screen`, including its withdrawn-status warning, was replaced by an unlabeled
terminal synthesis. This contradicts the wrapper comment, `package.md:171-174`,
and `artifacts/disposition.md:20-24`, all of which claim the rejected first-run
evidence was preserved without overwrite. The original JSON/CSV chain remains,
but the complete result-bearing record does not.

Required correction: restore or separately retain the historical
prerequisite-ineligible synthesis and figures, route the next terminal
generation to terminal-specific figure/synthesis paths, and make every output
fail closed if its destination already exists.

### HIGH — RB-03: repository-facing completion and EB-04X admission are premature

`package.md:3` and `artifacts/disposition.md:3-28` acknowledge that resumed
dual review, finding disposition, and dual terminal verification are pending.
This review is a HOLD, and RB-01/RB-02 invalidate the asserted terminal rerun.
Nevertheless `docs/ROADMAP.md:34`, the campaign roadmap at lines 157, 159 and
189-191, and `docs/work-packages/README.md:5097-5105` already publish W2B as
complete and admit EB-04X.

These surfaces must remain terminal-review HOLD and keep EB-04X held until a
valid newly built immutable rerun, accepted dual review disposition, dual
terminal verification, and final exact-diff reconciliation all pass.

### MEDIUM — RB-04: terminal provenance is not reconciled to the resumed tree

The resumed section of `artifacts/gate-results.md:77-96` gives outcomes for
W2C reuse, three W2B focused rows, formatting, lint, and the rerun, but the
focused rows omit exact argv, timing and retained log paths. The package's
`artifacts/terminal-diff-reconciliation.md` remains the historical cross-domain
HOLD record rather than an exact reconciliation of the resumed W2C source,
new terminal artifacts, overwritten figures/synthesis, wrapper change,
roadmaps, catalog, prompt state, and final review artifacts. The freeze's dirty
diff hash is necessarily pre-analysis and no longer identifies the terminal
tree.

The terminal JSON hash chain itself is internally consistent, and every W2B
source-file hash in the freeze currently reproduces. Those narrower facts do
not close the package-wide exact-diff requirement.

### MEDIUM — RB-05: W2C's prerequisite release retains contradictory terminal wording

The W2C disposition and verification disposition say formal completion and
dual terminal verification PASS, and the current real fixture confirms the
technical release. However W2C's `package.md` outcome still says W2B may not
resume until revision-60 review/reverification completes, and
`artifacts/review-disposition.md:45-48` still says terminal reverification is
pending. Reconcile these stale statements to the retained completed history so
the prerequisite has one unambiguous lifecycle state.

### LOW — RB-06: line-count governance omits a touched 2,000-line warning

`artifacts/line-count-governance-checklist.md` correctly warns on the 2,598-line
`runoff_reconciliation.rs` and 2,891-line runner test aggregate, but omits the
W2B-touched
`direct_publication/day_input_and_helpers/00c_day_input_builder_impl.rs`, which
is 2,450 lines and therefore also requires a `WARN`, decomposition rationale,
and follow-up split intent. No touched W2B Rust file reaches the 3,000-line hard
boundary.

## Retained Historical Disposition

The original Agent B review held the first W2A rerun for prospective-order,
premature-completion, guard-test, evidence-provenance, and line-count defects.
The subsequent re-review accepted the corrected W2B production/test behavior
for `HOLD_CROSS_DOMAIN_CORRECTNESS_GATE` while requiring W2C. Those historical
HOLD conclusions remain valid evidence for why the first rerun is ineligible;
they do not qualify the resumed result as terminal.

## Non-blocking Debt / Follow-ups

- Test adequacy is acceptable on the current source. Independently ran the two
  orchestrator W2B tests (`2/2`), three runner consumer tests (`3/3`), six
  contract/integration tests (`6/6`), and the real EROD16 fixture (`1/1`,
  `4/231` explicit refusals and 227 clean/depositing solves); all passed.
- `cargo fmt --all -- --check` and `git diff --check` passed independently.
- The frozen cells, models, operators, thresholds, observation role, and
  promotion prohibition compare exactly between the first and resumed freezes.
  Scientific result content also compares exactly after removing provenance:
  all four albedo-materiality flags remain false, maximum mass closure is
  `2.220e-15 m`, and maximum energy closure is `6.094e-08 J m^-2`. This supports
  rule integrity but cannot cure RB-01/RB-02.
- Security/dependency disposition is acceptable: no manifest, lockfile,
  dependency-resolution, nextest-policy, unsafe, or production unwrap/expect
  change was found. `cargo deny check` is therefore correctly not applicable.

QA disposition: **HOLD**. The current W2B implementation and W2C technical
prerequisite pass focused QA, but terminal rerun provenance, non-overwrite,
exact-diff, and completion-truthfulness obligations do not.

---

## Fresh Terminal-V2 Re-review

Status: **HOLD — RB-03 remains partially open**

Evidence mode: **Static + Ran**

This section is the operative Agent B disposition for terminal-v2. The initial
resumed HOLD above remains retained as the finding record.

### Finding

#### HIGH — RB-V2-01: two lifecycle narratives still admit EB-04X during terminal-review HOLD

The primary lifecycle surfaces are corrected: `artifacts/disposition.md` is
`HOLD_TERMINAL_REVIEW`, `docs/ROADMAP.md` keeps W2B as the next review-closure
item, the campaign milestone table keeps EB-04X held, and the catalog records
terminal-v2 review pending. However:

- `package.md:227` still concludes `EB-04X may advance`; and
- `docs/planning/snow-surface-energy-balance-roadmap.md:191` still says
  `EB-04X may now advance`.

Those statements contradict the same package's pending dual review,
verification, and exact-diff gates. RB-03 is therefore only partially
corrected. Replace both with the terminal-review HOLD and keep EB-04X blocked
until accepted dual review disposition and dual terminal verification complete.

### RB-01 Through RB-06 Recheck

| Prior finding | Terminal-v2 result |
|---|---|
| `RB-01` stale release binary | **Closed.** `release-build-receipt.json` records `cargo build --release -p openwepp-runner --bin openwepp-snowbench`, HEAD `a74af48b...`, dirty-source identity `890e4ab1...07aa`, path, size, mtime, and new binary SHA-256 `d6b2e824...9a54`. The freeze, execution receipt, and live binary all carry that same hash. |
| `RB-02` overwritten historical outputs | **Closed.** The wrapper routes every generated surface beneath `artifacts/terminal-v2/`, refuses pre-existing artifact or result directories before build/execution, and the shared synthesis/figure paths have no diff from HEAD. |
| `RB-03` premature lifecycle | **Open only as `RB-V2-01`.** Primary status/table/catalog surfaces are held, but the two narratives above remain contradictory. |
| `RB-04` exact provenance/reconciliation | **Closed for re-review.** The terminal-v2 reconciliation identifies the pre-review tracked/untracked/package tree, wrapper, source manifest, rebuilt binary, and freeze/receipt hashes; it explicitly reserves the self-referential final inventory for terminal verification. |
| `RB-05` W2C lifecycle contradiction | **Closed.** W2C's package outcome and review disposition now state revision-60 review and dual terminal reverification passed. |
| `RB-06` omitted line-count warning | **Closed.** The 2,450-line day-input builder is WARNed with decomposition rationale and follow-up intent; all other 2,000-line warnings remain and no touched nonexempt Rust file reaches 3,000 lines. |

### Independent Integrity Checks

- Recomputed terminal-v2 SHA-256 values: freeze
  `943561dc...d44dbb`, receipt `02461508...a0ed`, and results
  `65b308db...d4cb`; all exactly match `adjudication.json`.
- The adjudication/result chain retains eight successful executions, maximum
  mass closure `2.220e-15 m`, energy closure `6.094e-08 J m^-2`, and no
  promotion. Every explicit W2B source hash in the freeze matches the current
  file, and the scientific projection is unchanged from the frozen historical
  screen after removing provenance fields.
- Executed the wrapper against the existing terminal-v2 tree: it failed before
  build or execution with `terminal-v2 evidence or result-bearing output already
  exists`, confirming the non-overwrite guard.
- `git diff --exit-code` passes for the shared historical synthesis and figure
  tree. `git diff --check` also passes.

### Non-blocking Debt / Follow-up

- The release-build receipt does not store a separate numeric exit field or
  duration, but it is written only after `subprocess.run(..., check=True)`
  succeeds; `gate-results.md` records exit `0` and the 66-second duration. This
  is adequate provenance, though an explicit receipt field would improve
  ergonomics.
- Final exact-diff inventory is correctly deferred only to the package-required
  terminal verifiers because review artifacts change the self-referential
  package manifest. Result-affecting source and binary identities must remain
  unchanged through that verification.

Terminal-v2 QA disposition: **HOLD** solely for `RB-V2-01`. The technical
terminal-v2 evidence is acceptable; correct the two stale EB-04X admissions,
disposition this finding, and obtain dual terminal verification before package
completion.

### RB-03 Narrow Final Recheck

Status: **PASS — RB-V2-01 corrected; no remaining QA finding**

Evidence mode: **Static**

- `package.md:227-228` now keeps EB-04X held until terminal-v2 review,
  verification, and exact-diff closure pass.
- `docs/planning/snow-surface-energy-balance-roadmap.md:191-192` now states the
  same hold. No remaining `EB-04X may advance`, `may now advance`, or equivalent
  admission appears in either corrected surface.
- These statements agree with the already reviewed package disposition, root
  roadmap, campaign milestone table, and catalog hold posture.

`RB-03` and `RB-V2-01` are closed. All `RB-01` through `RB-06` corrections now
pass fresh Agent B re-review. **QA PASS** for terminal-v2 review; package
completion remains pending its separately required finding disposition, dual
terminal verification, and final exact-diff reconciliation.

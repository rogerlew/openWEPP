# Independent QA review B

Static: reviewed role/governance, package/protocol/handoff, report, gate and
line-count dispositions, source/build manifest, complete seven-file P-to-Q patch,
collector and both analyzers, isolated scope mappings, prior F treatment result,
and retained gate logs. Ran: read-only independent JSON arithmetic/grouping,
SHA-256 checks, source line counts, and matched P/Q diagnostic comparison. No
builds, tests, simulations, or source edits performed by this reviewer.
Role: `/root/v34_qa`; requested QA effort medium; effective runtime UNOBSERVED.
Reviewed cut: final `series-05`, `analysis-final-cut3.json`,
`recompute-trace-cut3`, and manifest-bound patch `a254e07c...50f4`.

## Findings

1. **Medium — QA-ATTR-01 — CLOSED as a disposition defect; required gate remains FAIL.**
   `package.md` froze warnings-denied scoped Clippy for every isolated Rust edit.
   `artifacts/gate-results.md` instead proposes an after-implementation amendment
   to matching baseline error headings. The common package non-deferral rule
   permits prospective removal or terminal HOLD, not retrospective self-waiver.
   Independently confirmed all **1,017** P/Q error headings match, and strengthened
   this check to matching diagnostic file and emitted source-line snippets with
   **zero multiset differences**. That is useful no-new-diagnostic evidence, not
   green Clippy. Required correction: retain Clippy FAIL and package executed-HOLD
   with the blocked requirement named, while separately retaining completed
   characterization/no-new-prototype as the experiment result. Do not repair
   unrelated baseline code, suppress diagnostics, or weaken assertions here.
   Focused rereview: current package, gate results, report and handoff explicitly
   retain executed HOLD and Clippy FAIL. The retrospective replacement is
   rejected. This resolves the finding, not the failed acceptance gate.

2. **Medium — QA-ATTR-02 — CLOSED after focused rereview: command/dependency recovery.**
   `artifacts/source-and-build-manifest.md` and retained gate logs do not provide
   exact terminal build/test/Clippy argv, cwd and exit receipts. The final series
   protocol supplies authentic runner argv/cwd and binds environment SHA-256
   `463cd045...4a84`, but the package does not locate that environment input.
   The detailed capture command is similarly not recorded in the manifest.
   Required correction: add bounded exact command/dependency records from retained
   execution evidence, including environment location/hash and external inputs.
   No rebuild is necessary if those records exist; unobserved fields must remain
   explicitly unobserved, not inferred. Explain the retained prior series/cuts so
   the final series cannot be mistaken for a best-of-many timing selection.
   Focused rereview: new `command-and-dependency-receipt.md` supplies terminal
   command/cwd/exit records, final collector invocation, trace posture and custody
   locations; it explicitly distinguishes shell-record reconstruction from argv
   printed in logs. Independently read the now-located external environment and
   verified exact SHA-256 `463cd045...4a84`. Main recovery gap is resolved.
   Final focused rereview: the receipt now identifies the series-01 admission
   error and source-correction/custody chronology superseding series-02/03/04.
   Independently checked all three abbreviated binary hashes against their
   protocol receipts. Final05 remains the sole accepted exact-source series;
   retained development measurements are not pooled or selected for favorable
   timing. Command provenance remains reconstructed from execution records where
   the original logs did not print argv, as explicitly disclosed.

3. **Low — QA-ATTR-03 — CLOSED after focused rereview: stale integrity percentage.**
   `artifacts/attribution-and-recomputation.md` correctly reports **4.20%** in the
   table but says **4.24%** in the decision. Independent final-series median is
   **4.20007455%**. Correct the latter without changing the lower-bound caveat:
   the named bucket is not total integrity/audit cost.
   Current decision correctly says 4.20%. It also correctly distinguishes real
   imported validation/reseal/acceptance from common experimental mechanism-audit
   counters; the latter remain unmeasured in both timing arms. Off/on isolates
   only incremental compact timing, not total observation cost.

## Independent evidence and scope assessment

- All **16** final receipts are valid with exit **0**; independently hashed all
  16 retained raw logs against their receipts. Each of six measured on profiles
  reconciles `runner_ns - sum(exclusive_ns) = 0`. Recomputed principal shares:
  Stage3 parent **16.13554912%**, physical evidence **14.30289024%**, completion
  **12.51349744%**, imported frozen runtime **9.25756077%**, outside named scopes
  **7.44454042%**, snow-free parent **6.51854093%**. Main table values agree.
- Off/on median runner seconds **5.033492 / 5.0364295**; paired wall perturbation
  median **−0.03367914%**, range **−0.32282335–+0.72197451%**; CPU median
  **−0.09832125%**, range **−0.35555689–+0.76340280%**. These are descriptive
  observer perturbations, not treatment speedup. No overhead subtraction occurs.
- Independently verified manifest hashes for the patch, admission identity,
  final protocol/cost analysis, carrier trace, and recomputation analysis.
  Collector disables detailed trace during timing; records fixed fresh-process
  warmups/pairs, CPU0 and exact binary; protects output/control/closure/work
  identity using the declared predecessor helper and rejects F-specific work
  exceptions. This review checked receipt integrity, not a new scientific rerun.
- Raw carrier lifecycle is **400 Provider / 400 Carrier / 200 Evaluator / 72
  Outer**, no errors or dropped records. Independent grouping reproduces **204
  request keys**, **132 repeated groups**, **328 grouped executions**, **196
  nominal repeats**, and **8 multi-output keys**. All 200 evaluator groups have
  two distinct recorded requests with equal recorded outputs. Prior F evidence
  separately records its 400→200 Provider/Carrier reduction. Thus the new report
  appropriately rejects recorded-key equality as effective-input authority and
  does not recast already-established F as a newly proven mechanism.
- No >=5% *distinct interchangeable removable* operation is established; stopping
  without a prototype follows the prospective conditional gate. Large mixed
  phase shares do not establish reusable work. Absence of proof is not proof that
  no future opportunity exists. Independent roles, errors, ownership and mutation
  cannot be collapsed from matching result bytes.
- Complete patch is bounded to seven declared observer files: fixed enum/label
  mappings and two lexical scopes, without changed physical calls, branches,
  validation, solver cadence, output semantics, unsafe code or dependencies.
  Existing shared-clock lifecycle behavior is retained. Three retained focused
  tests report PASS; exact-source format log is distinct from mistaken primary
  worktree logs. Neither inherited tests nor validators are disabled.
- Direct counts **1188, 688, 2935, 2893, 2834, 2355, 1340** match line-count
  governance. Four inherited large files warrant WARN; no >=3000 file appears.
  Small observer mappings do not justify a broad architectural split in this
  package. No primary repository Rust edits are included in the declared diff.

## Non-blocking limits and follow-ups

Custody is not a standalone rebuildable environment. Full-workspace green,
production CLI/scale behavior, arbitrary invalid-state error equivalence and
memory admission are not claimed. The bounded negative result does not waive
the frozen QA gate or terminal dual verification. Any future optimization needs
separate effective-input/role proof and authority; no cache or reuse prototype
is approved here. Unassigned and parent-exclusive work remain honestly unresolved.

## Verdict

**QA PASS for truthful executed-HOLD disposition and the bounded characterization;
frozen Clippy requirement remains FAIL.** QA-ATTR-01/02/03 are resolved as
documentation/disposition findings. The measured characterization and
no-new-prototype decision are supported; package completion is not approved.
Production remains HOLD. Terminal independent verification remains separate.
Only this assigned review artifact was edited.

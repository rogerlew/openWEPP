# Independent correctness and authority review A

Static: read package governance and reviewer role, acceptance/protocol, handoff,
primary report, gate and source/build records, complete seven-file patch,
collector/analyzers, affected scope boundaries, underlying experiment-audit and
provider/evaluator source, and prior F implementation evidence. Ran: read-only
receipt/log identity checks, exact analysis reconstruction, trace grouping and
lifecycle checks, bundle hashing, in-memory exact patch application, and matched
Clippy diagnostic comparison. No builds, tests, simulations, or source edits by
this reviewer. Retained execution logs are inspected evidence, not my executions.

Role/session: `/root/v34_correctness`; requested correctness effort high;
effective runtime UNOBSERVED. Reviewed executable `3e684ac3...ce53`, patch
`a254e07c...50f4`, final `series-05`/`analysis-final-cut3.json` and
`recompute-trace-cut3`; custody manifest `de78b15b...91e5`.

## Findings, ordered by severity

1. **High — ATTR-A01 — corrected disposition; required gate remains FAIL.**
   `package.md:101` requires warnings-denied scoped Clippy. The original
   `artifacts/gate-results.md` retrospectively substituted matched P/Q error
   headings after both commands failed. This cannot discharge an already-frozen
   requirement. Independently reproduced equality of all 1,017 diagnostic
   heading/file/emitted-source-line multisets, stronger than heading counts but
   still not green Clippy. Current gate/report correction explicitly retains
   Clippy FAIL and executed HOLD, without unrelated source repair or suppression.
   That resolves the self-waiver finding, not the failed gate. Bounded
   characterization/no-new-prototype is a separate supported result; package
   completion is not approved.

2. **Medium — ATTR-A02 — FIXED and independently rechecked.**
   The earlier `artifacts/protocol.md:43` classified mechanism-audit work as required runtime
   integrity and calls only the new compact observer experiment-only. Actual
   `stage3_mechanism_experiment_audit.rs` says no session is active in ordinary
   production. The authentic harness starts existing Compact mechanism-audit
   sessions in both timing arms; detailed formatting/hashing occurs only in the
   separate trace. Therefore off/on estimates incremental compact timing
   perturbation, not total experimental observation overhead. Keep existing
   common-counter overhead UNMEASURED in parent/remainder buckets.
   Conversely the named 4.20% `RequiredIntegrity` bucket maps actual imported
   validators, resealing and frozen acceptance in `v9_real_consumer_shadow.rs`:
   do not relabel that whole bucket experiment-only. Focused re-review of the
   corrected protocol and report confirms this distinction is now explicit:
   4.20% is the named runtime-validator subset, common Compact experiment
   counters remain unmeasured, and off/on estimates only incremental timing
   instrumentation. The transient overcorrection labeling the entire 4.20%
   experiment-only was removed. The former 4.24% tail is also corrected;
   independently reconstructed current median is 4.20007455%. No measurement,
   source, accounting, threshold, or authority changed for this correction.

## Independently checked evidence

- All 16 final receipts hash-match their retained logs; each contains exactly one
  authentic record equal to its receipt. Reapplied admission normalization and
  exact protected common identity/work counts to every record. Rechecked off/on
  posture, family partition and population validity. All six measured on runs
  reconcile their 32 exclusive buckets to runner time at exactly zero ns.
- Reconstructed every saved analysis field, distribution and receipt hash using
  the retained analyzer without executing its file-writing main. All agree.
  Principal per-run median shares are Stage3 parent 16.13555%, carrier physical
  evidence 14.30289%, completion construction 12.51350%, imported frozen runtime
  9.25756%, outside named 7.44454%, and snow-free remainder 6.51854%. Independently
  summed the six `Carrier*` buckets plus `TerminalCarrier` within each run:
  median 34.70612%, range 34.57627–34.75617%. This union is not removable cost.
- Trace lifecycle reconciles 72 Outer, 200 Evaluator, 400 Provider and 400 Carrier
  records, all successful, with nonempty recorded operands, unique ordinal per
  kind, no dropped records, and every Provider evaluator identity present. Raw
  grouping independently yields 204 request keys, 132 repeated groups covering
  328 executions, 196 nominal duplicates, eight multi-output groups, and 200
  two-call evaluator groups with distinct requests/equal recorded outputs.
- Read the actual evaluator feedback loop: each invocation begins without a
  hint, issues a provider request, builds the physical preview and then compares
  a subsequent result. Source and prior F evidence, not output hashes alone,
  identify this as the already-established feed-forward F boundary. The trace
  output fingerprint covers the transition, not every retained physical/evidence
  object. Nothing here newly proves full-result interchangeability or requalifies
  F. Across-invocation matching hashes omit sufficient dynamic custody/state;
  even the 124 single-output repeated groups are not new reuse authority.
- The conditional >=5% new-prototype gate needs both complete effective-input
  equivalence and removable exclusive cost. Neither is established for a new
  operation. Not implementing a prototype is the lawful conditional outcome,
  not evasion or proof that future opportunities cannot exist. No convergence
  oracle, tolerance relaxation or hash cache is justified by these observations.
- Checked all 17 bundle entries' sizes/content hashes and each of seven captured
  sources against Q. Independently applied the complete P-to-Q patch in memory,
  requiring exact old lines and zero offsets/fuzz: all seven results equal Q.
  This establishes declared source composition/custody, not standalone rebuild
  availability. No whole-source/toolchain closure is asserted.
- The patch changes observer enum/label routing and two lexical scope guards;
  the shared exclusive-clock arithmetic/lifecycle is retained from P. Children
  preempt parents rather than being added twice. No physical arithmetic, domain
  guard precedence, typed error, canonical serialization, solver cadence, or
  physical invocation is changed. No substantial parallel physics logic is
  introduced. The new snow-free scope remains inside the existing else branch;
  day finalization begins after the unchanged parent loop.
- Inspected exact-Q retained test log: three compact lifecycle/accounting tests
  PASS, including reversal and synthetic error handling. That is not an actual
  invalid-state solver campaign. Authentic accepted-run on/off identity provides
  separate real-consumer evidence. Rustfmt PASS is distinct from mistakenly run
  primary-worktree checks. Four inherited 2,000–2,999-line files retain explicit
  WARN/split rationale; no file reaches 3,000 lines.

## Residual risks and missing evidence

The frozen Clippy requirement remains unsatisfied; no completion waiver is
granted. Command/dependency recovery is independently assigned to reviewer B and
must retain exact observed commands or label unavailable execution metadata.
This is one audit-capable one-OFE executable/host/day, not production CLI, scale,
memory, full-workspace or arbitrary error-path qualification. Scope timing is
descriptive; the near-zero paired perturbation is not proof of zero overhead or
a treatment speedup. Final median shares cannot be added as a synthetic run.
No new optimization or reuse authority is granted. Terminal verification is
separate from this review and is not presumed complete.

## Verdict

**PASS for the bounded characterization and truthful executed-HOLD disposition.**
Both review findings are dispositioned with focused re-review; no remaining
correctness-review blocker to publishing that disposition. **Clippy remains FAIL
and the package remains HOLD**, not complete. Production remains HOLD. No
prototype or new implementation authority is granted. Only this assigned review
artifact was edited.

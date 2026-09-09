# Independent review A

Static: read authorization, package/protocol/source map, complete 921-line combined
patch, collector/analyzer and their reused identity routines, manifest, raw
analysis and current interpretation. Ran: read-only Python reconstruction and
malformed-accounting checks, raw log/receipt comparison, source hashing and line
counts. No builds, Rust tests, profiling runs or source edits by this reviewer.
Role: independent correctness reviewer `/root/v34_correctness`; requested high
effort; effective runtime setting UNOBSERVED. Writes limited to this artifact.

## Findings and disposition

1. **COST-A01 — medium, corrected and independently CLOSED.**
   `cost-breakdown.md` originally called the 31.85% OtherPhysical bucket pure
   covered-carrier evaluation and assigned its whole share to a reuse experiment.
   The actual source maps NativeVegetationEt, terminal-provider carrier, covered
   setup/evidence/completion and imported physical/frozen scopes into this bucket;
   these contain construction and validation as well as physics. Current report
   names the mixed envelopes and measured-child exclusion explicitly, disclaims
   demonstrated same-state duplication, leaves reusable cost UNMEASURED and
   removes unsupported 2x/5x reuse savings. First prove an actual duplicate request
   before implementing reuse; existing physical-only/once-per-batch paths are not
   newly discovered optimization opportunities.
2. **COST-A02 — medium, corrected and independently CLOSED.**
   `cost-breakdown.md`'s 1.268% perfect ceiling is valid for assembly plus linear
   work only, not every possible effect of coordinate compaction. A reduced
   system might also avoid the separately measured inactive-coordinate FD probes.
   Requested explicit distinction between the assembly/linear-only envelope and
   the larger disjoint union, without claiming all costs removable. Independently
   computed per-run union Assembly+Linear+InactiveLeafProbe: median 2.579401%,
   range 2.572182–2.584828%. These named envelopes remain below the owner's 5%
   priority rule; other residual/mapping/guard effects are not measured. This
   correction need not reverse deprioritization and needs no new measurement.
   Current report explicitly distinguishes the 1.268% assembly/linear-only
   envelope from the 2.579% three-bucket union and leaves other effects unmeasured.
3. **COST-A03 — medium, corrected and independently CLOSED.**
   Newly supplied `raw/gates/actual-error-test.log` executes
   `compact_counts_and_errors_without_events`; its source manually creates
   audit scopes and `Err(7)`. `authentic-map-test.log` executes
   `potential_and_final_solves_join_one_authentic_map`; its source manually
   constructs map/solve/sweep audit events. Neither calls a physical solver or
   activates the new compact-cost session. These are legitimate pre-existing
   lifecycle unit tests, not actual rejected-solve or authentic Potential/Final
   solver evidence. Correct their labels and identify the existing source-real
   solver/corpus checks named by the protocol, or explicit legitimate evidence
   reuse with its limits. Do not claim synthetic scope tests executed a solver.
   Focused correction below supplies accurate labels and source-real regression
   evidence without claiming observer-on rejected-solve coverage.

## Independent source and arithmetic checks

The complete patch leaves physical arithmetic, solver branch/error order,
row adjustment, pivoting and publication operations intact. New timing guards
wrap existing operations. Factorization result matching moves outside its guard
without changing the call, arguments or error mapping. Classifying zero-input
leaf coordinates adds no physical reevaluation. Existing identity-anchor
shortcuts remain distinct from FD. The population mask is descriptive, not new
mathematical elimination authority.

The fixed-size same-thread stack charges the previous bucket before each switch
and restores it on guard drop; no parent/child addition is used. Generation
binding prevents stale guards touching a newer session. Non-LIFO/live scopes,
checked-counter overflow and reversed clocks invalidate evidence. Runner start
and end derive from the unchanged runner elapsed interval; setup, post-run
validation and teardown are outside it. Observer failure does not supply a
scientific pass. Off posture retains inactive hook calls/TLS checks and all
pre-existing audits, but no new inner clock reads; it is not an instrumentation-
free production CLI. Timing overhead remains embedded in measured scopes, with
only the aggregate off/on perturbation reported, not subtracted.

Ran read-only reconstruction against all 16 retained numbered receipts:

- Every raw log SHA matches, and its complete emitted mechanism record exactly
  equals the receipt record.
- All 16 collector accounting/denominator checks pass. Every normalized protected
  common identity and carrier/LSE count equals the retained A admission; no F
  frame-counter exception is used.
- Six measured on partitions and paired statistics reproduce every saved analysis
  value exactly. Negative elapsed values, zero denominator and inconsistent
  accounting-difference mutants all reject (three checks).
- The accounting difference is zero per run, not obtained by normalizing medians.
  Individual medians in the display are descriptive, not one synthetic run.
- All ten current modified source hashes match the manifest. Complete patch was
  read; no J derivative or reduced-system treatment is included in P.

Assembly/linear cost is explicitly separate from probe time. Trial scopes also
include their surrounding prospective/backtracking bookkeeping; remaining/result
work is merged rather than presented as a pure physics cost. Some preparation
time precedes family tagging, so family drilldowns are subsets, not forced-equal
partitions. Same-thread wall timing does not attribute worker-thread internals.

## Remaining evidence and limits

Requested retained command/exit/log evidence for focused Rust tests, formatting,
scoped lint, A/P admission and full source composition/patch validation; the
current manifest binds ten modified files and the predecessor baseline kit, but
gate prose alone is not direct execution evidence. Protocol explicitly names
existing exact solver/corpus checks; their executed or legitimate reused evidence
must be identified. The described error test is synthetic `Err` equality, not an
actual rejected solver comparison, and must not be described as the latter.

Line-count check: terminal execution 2932, carrier phase 2892, imported consumer
2837 and runner execution 2355 lines are WARN; other touched files are below
2000 and none reaches 3000. Record the bounded instrumentation rationale and
future split intent; no broad refactor is requested. No production/science
authority, external-suite posture or required-case binding change was found;
the inherited full-workspace failure campaign is not current profiler acceptance.
Required focused validation and dual verification cannot be deferred while this
characterization is marked complete.

Custody supplement: inspected final bundle manifest
`4106705cde42448aa89c5971d39d76861f43c7003a95f98501cb92c2ce0bab7a`.
It retains the ten changed source files and exact runner/sidecar, referring to
the external retained A 10,154-member kit rather than claiming a standalone
P rebuild. Raw series are retained separately in this package. Assigned
verifiers own independent blob/source-composition recovery checks. Supplied
format/lint/test logs show completion but need exact argv/cwd/executable/exit
binding in compact receipts or manifest entries; the Nix banner's target differs
from the recorded build target, so test identity cannot be guessed from names.

Initial verdict before the focused correction below: HOLD for COST-A03 and the identified
execution/custody evidence reconciliation, not for the unwanted ranking or
ordinary memory variation. No numerical/observer integrity defect found in the
reviewed source or actual measured profiles. Production remains HOLD and no next
optimization is authorized by this characterization review.

## Final focused correction and review verdict

Static: reread corrected gate/finding records and both actual test bodies.
Inspected the newly retained source-real test logs, each 1/1 PASS. The Covered
failure test executes two actual `solve_covered_column` calls, requires rejected
outcomes and compares frozen singular/backtracking diagnostics. The Potential/
Final nonmutation test executes actual **open-surface** solves with and without
a cap and verifies accepted outcomes and unchanged beginning state. It is not
a Covered/V3-specific test. Both focused tests leave compact observation inactive;
the report now says so. Actual observer-on/off accepted Covered/V3 participation
and protected-result equality come from the separately reviewed 16-process
runner series. No observer-on rejected-solver claim is made.

COST-A03 is CLOSED. The gate record now identifies synthetic/manual scope tests
accurately and supplies exact focused-test cwd, executable, arguments, exit zero,
Nix setup and explicit target-override interpretation. Independently rehashed
the existing final LSE test executable as
`f57e0205b3722f1c1b05fc97b1975ccae22ea154b02b3bb5eaacb6336adfe724`
and runner as
`c2276fa901e5c0c77a574f390f1c399915122b6c6c87085c18e138b3d0d0c858`,
matching the frozen manifest and series. No source change or remeasurement was
needed for these label/receipt corrections. Line-count WARN rationale is now
recorded; the four large inherited files remain below the mandatory split limit.

Residual limits remain explicit: same-thread observer, one authenticated OFE day,
raw profiled costs with embedded overhead, mixed 31.85% envelope with no proven
reusable fraction, and 49.24% unresolved work after denied sampling. The 1.268%
assembly/linear envelope and 2.579% disjoint three-bucket union are optimistic
cost envelopes, not predictions or universal compaction ceilings. Prior J and
full-workspace failures are neither repaired nor waived by this characterization.

Verdict: **PASS for independent substantive review of this bounded cost
characterization and corrected prioritization report.** COST-A01/A02/A03 are
closed; no remaining correctness finding blocks these measured claims. Required
dual terminal verification and final custody/publication reconciliation remain
separate current obligations before package completion; this review is not a
claim they already passed. Production remains HOLD. No authorization to implement
the recommended next experiment, push, or start a broader campaign follows.

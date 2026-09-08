# Independent terminal review B

Static: read root and work-package instructions, `role-review.md`, package,
worker handoff, frozen experiment protocol and kickoff, treatment/results/gate
records, rearchitecture handoff, final disposition, collector source/tests, and
the exact current package diff. Inspected all six final timing/memory/teardown
raw `results.jsonl` series under `/tmp/openwepp-stage3-resume-20260908`.

Ran: `/workdir/openWEPP/.venv/bin/python -m unittest test_collectors.py` from
the reproduction directory: **FAIL**, 8/8 tests error in setup because the
historical A fixture sidecar no longer binds its executable/checkout. Ran an
independent read-only Python reduction of the final A05/F06 and A04/R05 timing,
memory, and teardown JSONL: every retained final row reports `valid=true`; row
counts are 28/28 for each timing series, 12/12 for each memory series, and 6/6
for each teardown series. Independently observed ten-run post-drop iteration
9-minus-0 RSS deltas (KiB): A/F comparison A `[16976, 4428, -3048]`, F
`[5552, -2396, -10916]`; A/R comparison A `[16336, -188, -6252]`, R
`[-1972, 5824, 2116]`.

Role/session: terminal reviewer B; configured/requested effort: high; effective
runtime setting: UNOBSERVED (no session metadata).

Reviewed identity: current dirty substantive cut, including
`run_series.py` SHA-256 `f1ea797124f4a4e1cb4e02da049a403ac6db6d8780f839b0b334197b4f348052`,
`results.jsonl` SHA-256 `4d67ffc3e8d9abd05ba858900312648d8b311956e9769e7669e0be29f22d51a0`,
and treatment-results SHA-256
`198f8738ba6ddb433b45072836fc08228f1047a015ac8e41c9019f9bf9886ea1`.
Assigned scope: QA, statistics, lifecycle memory/teardown, harness corrections,
and mandatory-obligation legitimacy.

## Findings

### ST3-B-001 — HIGH — corrected collector has a wholly red owning test suite

Location: `artifacts/reproduction/test_collectors.py:15-22`,
`artifacts/reproduction/admit_identity.py:25`, and `artifacts/gate-results.md`
resumed-campaign section.

Evidence: all eight collector tests fail before reaching their assertions
because setup derives identity from a stale `A-admission-01` fixture whose
sidecar no longer binds the referenced executable/checkout. The current
explanation that the external binary was replaced does not satisfy the required
source-quality/negative-control obligation for a changed validation collector.
It also means the prespecified-statistics, failure-retention, memory-schema, and
identity poison controls are not currently exercised.

Required correction: give the tests an immutable self-consistent fixture (or a
current explicitly frozen one), rerun them green, retain the command/result,
and independently re-review the correction. This is a current-scope closure
obligation and may not be deferred or explained away as harmless fixture drift.

Disposition: open; blocks terminal acceptance.

### ST3-B-002 — HIGH — mandatory retained measurement/reporting evidence is incomplete

Location: `artifacts/results.jsonl`, `artifacts/treatment-results.md:6-10`, and
the raw final series presently only under `/tmp/openwepp-stage3-resume-20260908`.

Evidence: the package `results.jsonl` contains only three summary rows: two
timing summaries and scale admission. It does not retain the final sample rows
or durable package-relative bindings for timing, memory, or teardown. The
protocol requires every sample, all paired B-A seconds and B/A ratios, both
bootstrap intervals (improvement and seconds saved), lifecycle sources/units,
paired memory deltas, post-drop trends and their ten-iteration horizon, and the
applicable workload-scaled engineering ceilings including baseline failures.
The current prose reports only selected timing aggregates and qualitative
memory conclusions. In particular, independent reconstruction found sizable
positive post-drop deltas in one A process in each teardown comparison, so “no
consistent post-drop growth” is plausible but not independently auditable from
the delivered artifact and needs the complete per-process trend table/horizon.

Required correction: copy or bind immutable complete final series into the
package, retain hashes and exact identities, generate the prospectively
specified paired/statistical and lifecycle summaries (including ceiling
comparisons), and reconcile treatment/gate/final records to that evidence.
Do not infer heap ownership or long-run boundedness from RSS.

Disposition: open; blocks terminal acceptance and verification.

### ST3-B-003 — MEDIUM — architecture handoff contains stale measurement status

Location: `artifacts/rearchitecture-handoff.md:3-4` and final paragraph.

Evidence: the document is labeled owner-ready and makes final F/R decisions,
but its opening still says “F/R measurements pending” and its closing says final
classifications, absolute savings, memory limits, and reuse decisions remain
pending. Those statements contradict the newly added selected-prototype section
and treatment results.

Required correction: reconcile the status text to the exact measured cut and
retain only genuinely unresolved production/release obligations.

Disposition: open; documentation consistency defect.

## Mandatory-obligation legitimacy

The decision not to run R scale is legitimate under the frozen competitive
predicate because R's interval establishes a regression. Stopping F scale after
the authenticated 10-OFE fixture produced zero provider/carrier work is also a
concrete applicability rejection rather than a silent waiver of a valid scale
comparison. Preserving optimized full-workspace, broad Clippy, and release-golden
failures as FAIL and retaining production HOLD is truthful. However, those
legitimate stop/HOLD decisions do not waive the collector's red owning suite or
the protocol's mandatory evidence-retention and reporting requirements.

Uncertainty: I did not rerun the costly controlled processes or independently
reconstruct scientific outputs. `/tmp` raw evidence is mutable/non-durable and
was inspected only at the above cut. Timing bootstrap values in package summaries
were not accepted as terminally reproducible because the complete algorithm
outputs and paired rows are not delivered in-package.

Verdict: **FAIL** for terminal closure. The F useful-building-block and R reject
directions are supported at a provisional decision level, and EXECUTED HOLD is
the correct disposition, but independent verification must wait for ST3-B-001
and ST3-B-002 to be fixed and re-reviewed; ST3-B-003 must also be reconciled.

## Focused re-review of accepted fixes — 2026-09-08 PDT

Static: inspected only the accepted corrections to `test_collectors.py`, the
six newly retained package-local result series, both timing freeze records,
`finding-disposition.md`, `treatment-results.md`, `gate-results.md`,
`source-and-build-manifest.json`, and `rearchitecture-handoff.md`.

Ran: `/workdir/openWEPP/.venv/bin/python -m unittest test_collectors.py` from
`artifacts/reproduction`: **PASS 8/8**. Confirmed package-local durable result
row counts 28, 12, 6, 12, 6, and 28 (92 total), and SHA-256 values:

- A05/F06 timing `313d3ceb...e83e91c`
- A/F memory `208eb2f6...664b2c`
- A/F teardown `c3341d4d...6e2d4ced`
- A/R memory `5e5e7465...de4453d0`
- A/R teardown `fa43fad4...e946e129`
- A/R timing `9aee044a...884a582`

The timing freeze hashes are `bfc7b0ee...6ca11f` and
`dd6d640a...e2e4`. Each memory/teardown result row also embeds its full frozen
series record, so lack of a separate duplicate freeze file does not lose that
binding.

### Re-review dispositions

- **ST3-B-001: FIXED and independently rechecked.** The tests now consume the
  current durable A05 admission/identity and all eight identity, poison,
  lifecycle, stencil, and statistics controls pass. However,
  `gate-results.md` still says the collector tests “currently FAIL” and calls
  the issue open. That stale gate narrative must be reconciled before terminal
  record closure; it does not invalidate the directly observed PASS.
- **ST3-B-002: PARTIALLY FIXED; remains HIGH/open.** Durable custody of all six
  controlling series is now present. The reporting half of the finding is not
  fixed: `treatment-results.md` remains the same compact table and still omits
  all paired B-A seconds and B/A ratios, the bootstrap interval for median
  seconds saved, quantitative six-pair lifecycle deltas, per-process ten-run
  post-drop trends and explicit observation horizon, and the required
  workload-scaled engineering ceilings including baseline failures.
  `results.jsonl` still contains only three compact rows. Raw availability is
  not a substitute for the protocol's mandatory report.
- **ST3-B-003: FIXED and independently rechecked.** The rearchitecture header
  and closing scope now bind the completed controlled campaigns and current
  treatment result instead of claiming F/R measurements remain pending.

Additional freshness limit: `source-and-build-manifest.json` is not reconciled
to the measured cut. It still identifies A-source-04/F-source-05/R-source-05,
states A admission is in progress and R is not run, while the controlling
timing uses A05/F06 and R05. This prevents that file from serving as a terminal
source/build/evidence manifest, although the raw rows and embedded freezes now
provide durable series-level custody.

Focused re-review verdict: **FAIL / PARTIAL FIX**. Accepted fixes close
ST3-B-001 substantively and ST3-B-003. Terminal verification remains blocked
by the uncorrected mandatory reporting portion of ST3-B-002 and inconsistent
terminal manifest/gate narratives. Production HOLD and the bounded one-OFE F
signal/R rejection remain legitimate; no scale or release qualification is
created by these fixes.

## Second focused re-review — 2026-09-08 PDT

Static: inspected the terminal source/build manifest, expanded treatment
report, corrected collector gate statement, finding disposition, final
disposition, six durable result series, and embedded/timing freeze bindings.
Ran: independently reran `analyze_series.summarize` over both durable timing
series and recalculated paired lifetime-peak/post-drop ranges from both durable
memory series. The reported medians, improvement intervals, seconds-saved
intervals, CPU ratios, lifecycle ranges, ten-run horizon, and one-OFE engineering
ceiling dispositions agree with the raw records to the stated rounding.

The accepted corrections now close ST3-B-001, ST3-B-002, and ST3-B-003. The
source/build manifest binds A05/F06/R05 source and executable identities; all
92 controlling rows are package-local, and each lifecycle result embeds its
freeze. `treatment-results.md` names the durable files, provides the missing
seconds intervals and pair/lifecycle ranges, states the observation horizon,
and reports both applicable engineering ceilings including the inherited
post-return failure. The collector owning suite remains PASS 8/8 and the gate
record no longer describes it as currently failing.

Minor editorial note, non-blocking: the F peak-delta range is printed
`-1.1 to -2.6 MiB`; conventional ascending order would be `-2.6 to -1.1 MiB`.
The underlying values and conclusion are correct.

The final `EXECUTED HOLD` is truthful under the repository non-deferral rule:
it explicitly names the unmet competitive 10/19-OFE scale obligation, the
observed P=0 fixture blocker, a future authorized package as owner, and the
authentic-native-path trigger. It does not label the scale gate PASS and bounds
F to a one-OFE useful building block. Full-workspace, broad Clippy, release
golden, and post-return ceiling failures also remain visible; production is not
qualified.

Second focused re-review verdict: **PASS for the accepted corrections and
truthful HOLD disposition**. No reviewer-B blocker remains for terminal
verification of this executed-HOLD cut. This is not a COMPLETE, production,
standalone-F, or scale-qualified verdict; the named scale obligation remains
open exactly as recorded in final disposition.

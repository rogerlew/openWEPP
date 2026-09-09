Ran: isolated cut3 local-cost/one-OFE series admitted; memory priority FAIL.

## Local results

The release local-cost test completed successfully (1 passed, 86.34 s).
Attempt 0 was valid: 4096 repetitions, two warmup batches per arm and 30
balanced AB/BA pairs. All 90 raw calibration/warmup/measured rows remain in
`/tmp/openwepp-hotpath-jacobian-jmvOZh/j-cut3-local-cost.log`; no samples excluded.
Minimum measured column batches were A 1.756497949 s and J 0.184797670 s,
both above the prospectively fixed 100 ms requirement.

| Matched obligation | Median A batch s | Median J batch s | Median paired acceleration | Median paired savings (95% CI) |
| --- | ---: | ---: | ---: | --- |
| Charged complete selected columns | 1.768698 | 0.187300 | 9.444662x | 89.4120% (89.3673–89.4540%) |
| Fresh base plus selected columns | 2.058004 | 0.457265 | 4.504620x | 77.8006% (77.6516–77.8940%) |

Paired median batch time saved was respectively 1.580875059 s
(95% CI 1.576421746–1.587245446 s) and 1.596820339 s
(1.594371659–1.608462534 s). Intervals reuse the existing paired bootstrap
conventions: 10,000 resamples, seed 20260906, percentile intervals of medians.
The column span includes allocation, complete selected columns, full-matrix
checksums, unchanged row adjustment and deallocation; it is not exclusive FD
construction time. The equal-record 12-case workload is not runtime weighting.

Every retained row passed exact work-count checks. Per repetition A constructs
48 FD columns with 88 complete probes and 400 inclusive leaf calls; J constructs
48 direct columns with zero probes and 48 inclusive leaf calls. J has 16 direct
columns in each of Covered Potential, GenericV3 Potential and GenericV3
FixedFinal; Covered FixedFinal remains zero. Probe calls are eliminated, not
an infinite measured acceleration. Inclusive leaf-call reduction is 8.333333x.
Untimed exact complete consumed matrix/RHS preflight passed before protocol
emission; consumed checksums agree in every measured pair. Checksums remain
anti-elision evidence, not substitutes for exact preflight or reference admission.

Decision: local numerical/cost admission PASS, with a useful measured gain
above the 2x local expansion-priority target. No whole-run fraction is measured
by this harness; no Amdahl extrapolation or complete-run claim follows.

Custody: log SHA-256
`0ef2ca8e4ff75dc3103b46e1607feefed11bda479d385b1ed71d27110a6a2793`;
J source identity
`7b5133c93de4c827d54945e265b94235834bffef4eb21ec62942b3c377930d57`;
measured LSE executable SHA-256
`69b87a51a6c87ca49fe09a81ebe901cfddc38b635a467928f66c6bcad9a1a093`.
The separately admitted J runner executable is
`8493e547df4829e28a1cd784d93a51660347d98ac7256796b1e6544f55f9a145`;
it does not execute this cfg(test)-only local benchmark. Both local arms run in
the same retained LSE executable. Compact analysis and the small one-off
extractor are retained alongside the raw log as `j-cut3-local-cost-analysis.json`
and `analyze_local_cost_once.py`; the extractor imports the unchanged prior
package's paired bootstrap implementation.

## Actual runner

The original one-OFE runner series completed12 balanced fresh-process pairs,
two warmups per executable:28/28 processes valid,24 measured plus4 warmups.
Median A/J complete-run wall times were4.9819585/4.970289 seconds. Median
paired wall savings were0.422922% (bootstrap95% CI0.034531–1.246556%).
Process CPU ratio J/A was0.995961; runner CPU ratio0.994982. Report paired
statistics, not the ratio of marginal medians. All raw samples are retained in
j-cut3-one-ofe-series/results.jsonl; existing analyze_series.py produced
j-cut3-one-ofe-analysis.json with10,000 paired resamples,seed20260906.

All14 J records (12 measured plus2 warmups) independently retained exact integer
direct_columns:{CoveredPotential8000,CoveredFixedFinal0,V3Potential916,
V3FixedFinal928}. Existing collector also admitted exact common input/output/
control identity and frozen per-arm carrier/LSE counts for all28 processes.
Its historical standalone5% predicate is NOT this package's acceptance rule.

Initial engineering interpretation: strong local gain, small complete-run gain,
no observed material CPU regression; useful but coverage-limited, not a full
qualification or throughput solution. Whole-run modifiable fraction f remains
UNMEASURED, so the30% priority target is not invoked. No24-pair extension:
the present uncertainty does not change this bounded interpretation, and no
sample exclusion or favorable rerun is justified. Memory and scale obligations
remain current for a candidate still worth considering. No adjacent block was
prospectively named; the positive-area ground alternative is not silently
recast as an expansion of this successful primary experiment.

## Memory and bounded scale disposition

Six matched single-run memory pairs:12/12 child processes valid, both peak
classes within the frozen max(4MiB,5% of matched A) allowance. Worst lifetime
delta was980KiB; worst sampled-active delta3936KiB, each against4096KiB.
Three matched fresh-process pairs with10 complete runs and teardown per process:
60/60 runs valid. Independent QA checked all18 memory child rows, log hashes,
72 run/iteration/five-phase joins and recomputed active samples; every one of
36 J runs retained9844 direct columns. No live-allocation evidence was collected.

Repeated-run memory priority is FAIL. Pair0 sampled-active process maximum was
A83788KiB/J88276KiB:delta4488KiB exceeds allowance4189.4KiB by298.6KiB.
All three process-lifetime peak comparisons pass, but that does not erase the
active-peak failure. Per-iteration sampled comparisons also exceed their bounds:
pair0 iteration9 delta5412 vs4143.2KiB; pair2 iteration7 delta5180 vs4096KiB.
No favorable sample selection, replacement series or post-hoc threshold change.

Post-drop vectors are nonmonotonic in both arms. First-to-last changes for the
three A processes were11900,-18104,-4560KiB; J14012,16276,-10952KiB. These
do not prove a leak, persistent monotonic growth, lifetime boundedness or a
general no-growth guarantee. Full vectors and phase lifetimes are retained in
j-cut3-teardown-analysis.json and raw results. Outer teardown collector exit is
UNOBSERVED because the delegated runner hit its service usage limit; all six
child processes independently report exit0/valid, and parent observed the
completed rows and collector termination without restarting samples.

Both10-OFE admissions pass with exact A/J common identity. J actually assembles
2924 columns:CoveredPotential1088,V3Potential916,V3FixedFinal920,FixedFinal
Covered0. Authenticated topology has only the downstream OFE vegetated; this
is not proportional participation on all10 OFEs or a new component-energy audit.
Logs/identities are a-cut3-ten-ofe-admission and j-cut3-ten-ofe-admission (and
their receipts/identity JSON). Single admissions are not scale timing samples.

Formal engineering disposition: REJECTED_IMPLEMENTATION under authorization
section11 because the evaluated cut fails the frozen memory-priority criterion.
The independently admitted affine derivative and useful local result remain
valid evidence; rejection is not a claim the affine math failed. With this
memory failure and the small complete-run gain, the candidate is not a
competitive admitted scale case. Independent review confirms the conditional
three-pair10/19-OFE timing obligation is not triggered:scale NOT QUALIFIED,
not passed/deferred. No refinement or alternative is needed after actual
measured rejection; the stop-tuning rule applies, not failed-oracle alternatives.
Broad correctness gates remain separately FAIL and production/package HOLD.

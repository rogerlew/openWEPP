# EXP-STAGE3-20260906 protocol, original draft 1 plus prospective reviewed amendments

Static: prospective protocol with independently reviewed amendments below.
No comparative samples have run at this amendment; runtime admission remains
separate from protocol/design review.

## Identities and isolation

A is current checkpoint plus common observation-only hooks/harness, frozen before
F/R. F and R are independent detached source cuts from A; patches include every
added/deleted path and build input. Retained production receives neither treatment.
Source identity includes tracked executable inputs and explicitly added sources,
Cargo.lock/manifests, flake inputs, compiler and flags. It excludes documentary
results. Persist file/path hashes, complete patches and base commit; verify apply
against base. Build test executable through Cargo JSON artifacts; record SHA256.
Any executable/harness/environment change starts a new series, preserving samples.

Use Linux, one permitted logical CPU (0 if permitted, otherwise lowest allowed
performance core with topology recorded). Environment: RUST_MIN_STACK=67108864,
CARGO_PROFILE_RELEASE_LTO=false, one libtest thread, same allocator and output
posture. Build with nix develop --offline outside measurement windows; invoke
frozen executable directly in captured Nix environment. No network fetching.

## Workload and intervals

Common separately named ignored release test runs existing authenticated
CompleteOwner fixture and execute_hillslope_run_with_runtime_policy. Default
one OFE, 100 m2, one day. Secondary 10/19 OFEs use authenticated existing scale
fixture construction with deterministic areas, frozen identically per comparison.
Do not change forcing, topology validators or publication semantics.

Runner `run_wall_us` is Instant elapsed around the entire production consumer
call, including actual publication. The final harness separately records safe
rustix CLOCK_MONOTONIC nanoseconds immediately before Instant start and after
elapsed capture; these share the parent's monotonic clock for active RSS sample
joins. Process CPU reads /proc/self/stat utime+stime immediately before the
start-clock capture and after the end-clock capture; divide tick delta by
recorded CLK_TCK and report its resolution. Thus CPU and absolute clock scopes
slightly enclose the runner-wall scope; they are not claimed bit-identical
intervals. See controlled_mechanism_experiments.rs:181-195, common-review-a.md
clock correction and collector-review-b.md active-interval validation. Fixture
authoring, validation, detailed trace oracle, digesting and teardown are outside
this interval. Required bounded counters remain enabled in both arms. Authentic
counts A establishes replace no history; expected historical 48 parents, 56
publications, 20 direct trials, 32 split children, four microsteps are checked
and any source-evolution difference recorded. Results/controls must be exact
except deliberate lower-level work elimination.

## Admission

Two independent authority/protocol reviews precede behavior changes and comparisons.
F proves provider multiplicity at actual invocation boundary, stable ordered
outer identity and role/support/mode/attempt/beginning/forcing/topology bindings,
one-call versus both original same-invocation results, all existing guard/error,
ownership/reuse/rollback/restart and full consumer/output obligations.
R proves independent direct graph and stencil enumeration, nonzero real replay,
all actual starts/completions/errors and lifecycle/drop reconciliation, forced
complete node/residual/Jacobian/full-solve/boundary/error/output parity. No
all-centered existential gate applies to this new experiment; no historical
scientific guard is waived. Detailed traces are correctness-only; compact counters
are measured. Both arms match optional audit posture.

## Timing and decision

For each viable independent comparison: two fresh-process warmups per executable,
then 12 pairs ordered AB BA BA AB AB BA BA AB AB BA BA AB. A means baseline;
B means F or R. Run F comparison first. Each process has timeout 600 seconds
for one/10/19 OFEs, 1800 seconds for ten teardown iterations. Exit failure,
timeout or correctness mismatch stops that candidate admission, retains output,
and is never reported as a speedup. An environmental invalidity requires actual
affinity mismatch, missing required fields, identity change, observer failure or
documented competing heavy process; slow/high-RSS successful runs remain valid.
One infrastructure-only rerun permitted with the failed attempt retained.

Report all paired B-A seconds and B/A ratios, median absolute seconds saved and
median improvement 1-B/A. Deterministic 10,000 paired bootstrap resamples, seed
20260906, percentile 2.5/97.5 descriptive interval on median improvement and
median seconds saved. Standalone salvage default: >=5% median wall gain, lower
improvement interval >0, and <=2% median process-CPU regression (tick resolution
reported). If the interval overlaps the priority threshold and plausibly changes
the decision, one extension to 24 TOTAL pairs using the same balanced order;
analyze the combined dataset. Otherwise no extension. Remaining ambiguity is
INCONCLUSIVE_WITH_BOUND. At most one planned performance refinement per mechanism;
correctness repairs precede all admission. Do not add F and R gains.

## Separate memory and lifetime series

Six matched fresh-process pairs, order AB BA BA AB AB BA, per viable candidate.
Parent samples /proc/child/status every 100 ms; readiness verifies executable
identity. Record monotonic sample times, PID lifetime, requested/actual gaps and
sampled active maximum. Child emits explicit pre-fixture, pre-run, end-run,
post-validation and post-drop VmRSS/VmHWM plus selected smaps_rollup boundary
private/anonymous/file evidence outside runner interval. Parent wait4 rusage
ru_maxrss supplies child lifetime HWM, in Linux KiB, separate from sampled maximum.
Original late rss_kib remains labeled endpoint. Observer settings match both arms.

Explicitly drop report, snapshot/audit, telemetry, parsed output and JSON buffers
before post-drop. Then remove fixture directory; directory deletion alone is not
allocation teardown. Memory records themselves remain small bounded observer
storage. In three fresh processes per arm run ten complete one-day iterations
with full teardown, recording lifecycle each time; this is allocation behavior,
not scientific ten-day continuity or proof of long-term boundedness.

Competitive means the standalone timing predicate above passes and the primary
memory series shows no unresolved meaningful increase; a measured useful building
block may also receive these bounded checks if the explicit recommendation depends
on scaling (record that decision before scale runs). For competitive candidates,
three pairs each at 10/19 OFEs with same lifecycle
observer and inputs. Existing active engineering ceiling 128MiB+16MiB*OFE and
return allowance 8MiB+1MiB*OFE are reported separately, including A violations.
Historical 64MiB point ceiling does not admit/reject this experiment. Assess
paired deltas, post-drop trends and bounded working set. Meaningful unresolved
increase makes recommendation provisional; at most one targeted decision-changing
allocation/lifetime experiment, no global allocator/stack/ASLR/host changes.
Mapping fields retain their kernel names: Private_Clean/Private_Dirty are private
mapped resident pages; Anonymous is anonymous RSS; Pss_Anon/Pss_File are proportional
shares, not file-backed RSS. Parent status RssFile is sampled file-backed RSS.

## Evidence and terminal claims

Per-field units/source/operand map precedes implementation. Independently reconstruct
WAT5 area-normalized source and HBP outlet plus storage/clamp closure; compare
deterministic output data exactly, volatile fields only through enumerated allowlist.
Full critical-cut correctness, applicable A0/A1/A3, typed guards, runner/restart,
format/Clippy/anti-evasion/line-count and independent dual terminal verification
remain required. Preserve inherited/expected-red failures without calling them PASS.
Every arm receives a supported decision; unexecuted is not a negative measurement.
Architecture handoff records actual counts, exclusive costs versus nested bounds,
absolute savings, target feasibility gap, <=3 ranked directions, and one decisive
prototype with measurable kill criterion. No production qualification or promotion.

## Prospective F construction-work qualification

Before comparative timing, independent reviews in F-implementation-review-a.md
and F-day-frame-review-b.md approve the exact F-day-frame-attribution.md cut
7536ede62b8b16ca7a9723a0c4deddef73a4465ac7a88595af936ee0c29cdb33.
This qualifies only manifest leaf
`/direct_runtime_counters/day_frame_constructions`; it is mechanism-dependent
work telemetry, NOT volatile provenance. Raw manifests and raw D/P/N remain
retained unchanged. Every other counter/scientific leaf keeps exact comparison.

Let D be that raw counter, P completed native provider calls (equal completed
carrier calls), and N authenticated hydrology owner lane count. The reviewed
native physical-only carrier performs two complete-frame adapter constructions,
each seeding N lanes. Compare the qualified leaf as
`{qualification: F-two-full-lane-seeds, noncarrier_constructions: D-2*N*P}`.
The collector implementation is reproduction/run_series.py frame_work_rule,
frame_work_comparison and the single-leaf normalized_manifest dispatch.

Actual primary admission operands: A D=1205/P=400/N=1; F D=805/P=200/N=1.
The construction reduction400 equals2*N*(400-200), and both noncarrier residues
are405. These exact primary pins remain required. P counts must be balanced
started=completed with errors0, Provider=Carrier, no dropped records, checked
nonnegative integer arithmetic and exact raw-leaf binding. The 405 residue is
not accepted-day count or a guessed assignment to another physical process.

For authenticated N=10/19, the same source-derived equation requires fresh
paired admissions: D_A-D_F=2*N*(P_A-P_F) and equal noncarrier residues. Do not
extrapolate primary P/D or impose405. Bootstrap/complete-owner/snapshot/HBP
cardinality joins, matched native branch, independent resolved/batch/canonical
populations, actual invocation/support/custody parity and physical/output gates
must hold. A new branch, failed call, batch change or mismatched residue rejects
qualification; no broader counter exception is admitted. Sampling, timing,
memory and decision criteria above are unchanged.

## Actual reconstruction and local environment clarification

Source-kit correction includes15 compiler-read JSON documents; see
authority-input-reproduction.md and its exact-base composition manifest.
This corrects source reproducibility, not physical behavior. Reconstruct a FULL
exact-base checkout plus the arm's runtime kit and canonical adjunct; a narrow
source archive alone does not establish a runnable root integration workspace.
F/R runtime patches apply to A commit2b56e6ebc, not directly to e89befa46.

Actual isolated F and R `.venv` entries are explicit symlinks to
`/workdir/openWEPP/.venv` (readlink verified). They share local Python tooling,
not scientific state. Preserve/recreate the required Python environment when
reproducing source-reading/schema gates; the untracked environment is not a
committed source input or a portable dependency bundle. Nix/compiler/stack/LTO
and measured executable identities remain separately frozen as above.

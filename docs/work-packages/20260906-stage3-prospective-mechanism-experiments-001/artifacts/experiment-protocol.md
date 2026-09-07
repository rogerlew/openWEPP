# EXP-STAGE3-20260906 protocol, draft 1

Static: prospective, not yet independently admitted. No comparative samples.

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

Runner wall is monotonic Instant around the entire production consumer call,
including actual publication. Process CPU uses /proc/self/stat utime+stime and
recorded CLK_TCK at those same boundaries (tick resolution reported). Fixture
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

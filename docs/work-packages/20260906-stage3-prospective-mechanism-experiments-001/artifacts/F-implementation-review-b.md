# F implementation review B

Later authority clarification: `F-role-coverage-review-b.md` supersedes the
all-five-role existential requirement stated below, not the all-five helper
tests or every-real-invocation full-carrier oracle. A concrete replacement
retains independent primary A/F multiset evidence and focused-oracle review;
the failed Root-presence assertion remains FAIL in its original log.

Static: independent QA/source review of isolated F against common A
`2b56e6ebc9c8e8073fb68b89626d4d4e2b4602e3`, 2026-09-06. All fourteen changed/new
Rust paths were inspected, including actual attachment, shared carrier arithmetic
and test-only oracle. Review A was not consulted before first findings.
Ran: `git diff --check`, PASS. No build, Rust test, lint, simulation or measurement
was run by this reviewer.

## Findings

| ID / severity | Path / line at inspected cut | Finding and disposition |
| --- | --- | --- |
| FIB-1 / P2, resolved statically | `v11_covered/open_snow_tail_tests.rs:259`; `v11_covered/carrier_phase.rs:2837` | The original guards still named only the now test-only legacy entrypoint. After declaring the additional exact test path, both guards now also cover the actual `execute_covered_carrier_feed_forward_phase_v1`; old obligations remain. Independently inspected the two-line corrective source delta and exact hashes below. No physical behavior changed. |
| FIB-2 / Low, resolved | retained `F-implementation-evidence.md:15` | The expected-red cut is now explicitly historical and superseded by the implemented cut, removing the contradictory current-state reading. |
| FIB-3 / P2, resolved statically | `snow_stage3_v11_adaptive_production_tests.rs:22`; `v9_real_consumer_shadow_wb14_tests.rs:240` | The corrected native fixture initially omitted the existing accepted fixture's 1800-second native-owner migration setting. That getter is consumed for native migration even with short-support mode disabled, so its default 60 seconds was not equivalent to the 1800-second actual parent. The explicit setting is now present before oracle execution, matching the accepted native-prefix setup. All role, mode and closure assertions remain intact. |

Rust paths use `crates/openwepp-hillslope-orchestrator/src/` in isolated F.
No physical correctness blocker was found in the inspected implementation.

## Source and authority assessment

The real attachment now closes over a typed physical request, enters the actual
typed carrier, and selects the feed-forward support API before evaluation.
The request has no hint or coupling ordinal. The observation projection constructs
the old first-call shape only inside detail closures or enabled evidence capture;
the non-test physical carrier does not consume that projection. The retained old
carrier entrypoint is test-only. Batch and canonical-final builders switch their
leader request type without changing their physical inputs or call count.

`evaluate_terminal_carrier_invocation` makes exactly one typed provider call,
performs the original support/beginning/role/attempt join, then executes the
extracted surface/flux arithmetic and canonical terminal-transition guards.
The extracted multiplication/division and operand order match A. The feedback
variant retains its original up-to-32 call loop, hint progression, four-component
live-convergence gate, post-loop check and evidence callbacks. There is no
error-dependent interface switch, fallback, cache, fabricated convergence event
or new scientific tolerance.

The resolved-domain branch still independently calls its selected provider once.
Outer support/mode/state validation, adaptive trial/exact cadence, child/forcing/
topology custody, provider error mapping and result retention remain at their
existing seams. F removes the duplicate same-invocation result construction and
map replacement, not an independent equal-payload invocation. Existing result maps
still retain only the selected candidate through the same ownership interfaces.

The full oracle is included and called only under `cfg(test)` and immediately
returns unless the dedicated same-thread test explicitly enables it. The runner
dependency/non-test build cannot enable it. The oracle executes the canonical
physical body twice from the same immutable beginning inputs; its legacy wrapper
discards only fields that A's physical body did not read. This is supported by
the source diff, not an independent historical solver implementation. Candidate
and both references compare complete phase/custody fields and canonical owner
bytes; the private candidate comparator is exhaustive. The real test requires
all five roles and both discovery/exact modes. Those requirements are assertions
to execute, not evidence of coverage already achieved.

## Required executable admission

Parent/comparator still must execute focused one-call/two-call, typed-interface,
guard/poison and full actual-carrier reference tests; all five authentic roles and
both modes must pass without weakening absent-fixture requirements. Retained
feedback tests must pass on the extracted loop. Actual A/F invocation keys,
independent calls, unchanged batch/canonical cadence, real output/control identity,
complete-owner rollback/restart and protected closure remain package admission
obligations. The synthetic helper and test-only oracle do not alone qualify F.

Critical exact-cut correctness and touched authority gates, full workspace
correctness where required, formatting, warnings-denied Clippy, dependency deny,
anti-evasion and terminal dual verification remain pending their actual logs.
No current expected-red or inherited failure may be relabeled PASS.

## Non-blocking maintainability notes

The shared extraction is cohesive and avoids duplicating scientific arithmetic.
Most collateral formatting is mechanical. Existing WARN files remain close to
the 3000-line boundary: carrier phase 2943, Stage-3 solver 2954 and terminal
execution 2942 at this cut. Keep further test helpers in their dedicated modules,
and reconcile exact final line counts/decomposition disposition after repairs.
Debug comparisons remain supplemental to typed/canonical equality on validated
finite data; do not reuse them as production custody or a nonfinite bit oracle.

## Reviewed source hashes

Paths below are relative to orchestrator `src/` in
`/tmp/openwepp-controlled-mechanisms-Hb6uS2/F`.

| Path | SHA-256 |
| --- | --- |
| `hydrology/03_kernel_support_00_support_helpers.rs` | `4d3f75ca712722b5717036aa83712da84dd2e6753d47ce372dc568928e87aef9` |
| `hydrology/support_helpers_mod/mod.rs` | `606f4d2c157d133b5f9f2d03041a8f01ac89e3d7d853e580213483941494f9e8` |
| `hydrology/support_helpers_mod/runoff_reconciliation.rs` | `f03d6f7e5426661970b0396d8b9f69e80be60878e9d8d4d154b0d9405db72687` |
| `hydrology/support_helpers_mod/runoff_reconciliation/terminal_carrier_provider.rs` | `dcacb51d7ffc5e3b304152935b11f2ace70beb589cc813ca86e614cbcf99e0c8` |
| `hydrology/support_helpers_mod/runoff_reconciliation/stage3_solver.rs` | `b01db67b2226edf2be07a8db615f3ff80139acffc6d284d68cba6e8b863f37ff` |
| `hydrology/support_helpers_mod/runoff_reconciliation/stage3_solver/support.rs` | `27208c28f121273cf301fea7d54774b6769c1eb79f8c2d32ade4688a2771e3f8` |
| `hydrology/support_helpers_mod/runoff_reconciliation/stage3_solver/evaluation.rs` | `be4a88ec959216a72ded7d86f27532887c9719fe3c16f5c3494666b166ef2944` |
| `hydrology/support_helpers_mod/runoff_reconciliation/stage3_solver/terminal_carrier_evaluation.rs` | `c7037c8b72d76db48ee92eebb142cb5512efbd81a3e3ae24ddf93823d7593efd` |
| `hydrology/support_helpers_mod/runoff_reconciliation/stage3_solver/terminal_feed_forward_tests.rs` | `1f78f061ef5093f99edc05d96d1f5241445f0991f0bccb300d43e06a9cf6d3c6` |
| `snow_stage3_v11_terminal_execution.rs` | `ed090ee54d61a1a74e7ca392459466843a9828d9dabd64f182082e02ac972834` |
| `snow_stage3_v11_terminal_feed_forward_tests.rs` | `4fbd55a350afab3ebb994fc6872cde0f68dc415236352031de52c9053cc7cef3` |
| `snow_stage3_v11_adaptive_production_tests.rs` | `6a1cc7a02caf0d6eb1a889695cac93522f8de5bbaa1d7200e9bac098a0446954` |
| `v11_covered/carrier_phase.rs` | `d45dca53b9188e266446218a7f6dcfab40e8dc34f074522857f86823ca0abfc5` |
| `v11_covered/carrier_phase/snow_boundary.rs` | `a2a06ec9fe039850958ecfc7f6fd0f7b832362c2151c40c898012eae78c6c09c` |

## Corrective cut

FIB-1 adds one further test-only path, making fifteen Rust paths total. The
remaining thirteen original source hashes above are unchanged at this correction.

| Corrected path | SHA-256 |
| --- | --- |
| `v11_covered/open_snow_tail_tests.rs` | `9d3758bfc6fc65911904e0e4e0c498dcaae3e5fd7de7d67fe8a513a285928206` |
| `v11_covered/carrier_phase.rs` | `0c0747455e2ea4f26eb2a38da573b8d443643b211accb883f63b27941c6225e1` |

## Postcompile source correction

Static re-review: the remaining snow-boundary projection helper now accepts the
typed physical request, matching its caller. Legacy re-exports, generic feedback
support API, provider variant, loop and coupling-only hooks/functions are
`cfg(test)`; source search finds only the test convenience API calling generic
feedback support. Existing feedback tests still exercise the same loop and
guards. The normal-build path has no feedback implementation or error fallback.
The shared observation request keeps its derived Debug formatting; no custom
fingerprint encoding or broad lint suppression was introduced.

The actual oracle now uses the already present NativeMixedPhase, 0.0006 m,
60-second hard-boundary fixture with the full 1800-second native-owner setting.
This changes the dedicated test fixture, not scientific acceptance assertions
or measured workload. Its reported prior compile/closure failures remain actual
failures; this static review does not classify the corrected fixture as passed.
Ran: `git diff --check`, PASS, before the final one-line duration correction.

| Corrected path | SHA-256 |
| --- | --- |
| `hydrology/03_kernel_support_00_support_helpers.rs` | `3aaa0a4fb3fe4760791b44b2e6466e319edcc012a5b4409c70b4451360c6ceb0` |
| `hydrology/support_helpers_mod/mod.rs` | `64a1fb52579f805700a12ddd2f2edc46807e22ab169c5b4ec26c0411f0ba974a` |
| `hydrology/support_helpers_mod/runoff_reconciliation.rs` | `0308d84ddcf6a625e1f976e3af2282bd71b834ec45272f015ae51eae3eeb1edb` |
| `hydrology/support_helpers_mod/runoff_reconciliation/terminal_carrier_provider.rs` | `e241828f0225db2c56cd2ff53a4a5b29311645e7d67278405ef7a474a7908971` |
| `hydrology/support_helpers_mod/runoff_reconciliation/stage3_solver/support.rs` | `18922a54f913e5915b14c0fe4ad9c79a6f22076a168ec28639adf37173ebfd09` |
| `hydrology/support_helpers_mod/runoff_reconciliation/stage3_solver/terminal_carrier_evaluation.rs` | `fd4c790255c0b76b2b599399c0e0b0e215fc56799b1929b314d928e0a2aa4bc1` |
| `v11_covered/carrier_phase/snow_boundary.rs` | `fcb3f6ccd78dd78a6f3ec3674f8abf4c55e1e052f5951e7e3b32d233f45112c2` |
| `snow_stage3_v11_adaptive_production_tests.rs` | `fb2f4af760cd8cf62663c6927db16067588b2f055c69ca27af9a5fcc4ef8527d` |

QA decision: GO for static implementation QA and serial executable verification.
No open source-review blocker remains at this corrected cut. Runtime/measurement
admission still requires the unexecuted science and correctness gates above.
No performance, scientific qualification or promotion claim.

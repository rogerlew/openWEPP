# Gate results (active)

Ran: offline toolchain preflight PASS, rustc1.95.0. Comparator role available.

| Command/surface | Result | Evidence |
|---|---|---|
| runner cargo check --tests initial | FAIL: JSON macro recursion | raw/common-check.log |
| check after record split | FAIL: test-support feature missing | raw/common-check-02.log |
| check after dev feature edge | PASS | raw/common-check-03.log |
| prospective F contract test | PASS 1 | raw/authority-F-test.log |
| prospective R contract test | PASS 1 | raw/authority-R-test.log |
| LSE observer tests initial | PASS | raw/authority-LSE-observer.log |
| carrier observer tests initial | PASS | raw/authority-orchestrator-observer.log |
| clock/source correction check | FAIL: forbidden unsafe FFI | raw/common-check-04.log |

Corrective response: safe existing rustix time API via test-only dependency;
no production unsafe exception. Corrected check05 PASS, LSE six observer tests,
carrier observer tests, and runner clock test PASS (matching *05 logs).
Early raw duration_ms fields are invalid wrapper metadata, explicitly excluded.
Release A build PASS and Cargo artifact discovery binds SHA
3d0eead3f68cb22c2ea4af8bf587bf297b100c39738c215918fb3ccfa04f0bc3.
Initial full-build log was overwritten by a cache-hit discovery invocation;
original build duration is unavailable, not the cache-hit duration. Compilation
success and binary identity remain directly evidenced, not fresh-build cost.

Exploratory real A consumer runs PASS; A-admission01 contains three processes,
first two missing requested memory variables. A-trace01 contains an initial
non-trace process and a corrected trace process. They remain exploratory logs,
not comparative samples or unambiguous single-process memory custody. The
exclusive-log admission wrapper supersedes this collection defect prospectively.
Detailed A trace has400map/800solve/2400iteration/2000sweep/106800probe,
78800Complete/28000IdentityAnchor/zeroReplay and no dropped events. Independent
binary64 stencil reconstruction PASS. Stricter lifecycle oracle correction also
passes actual A; independent negative mutation verification is pending.

Full critical A correctness is RUNNING via comparator (no fail suppression).
One known failure class is inherited stale assurance dependencies, further
changed by additive authority; see assurance-impact.md. Neither inherited failures
nor new expected-red predecessor seams will be relabeled PASS. F/R runtime
admission and all comparative measurement, full-cut validation, final review and
terminal verification remain pending. No production qualification is claimed.

## Comparator service failure and authorized local fallback

The required comparator_suite_runner role returned a genuine service failure:
GPT-5.3-Codex-Spark usage limit reached, next availability Sep8 13:08. No retry
or replacement-label fiction. Per kickoff section3, parent now executes the
remaining frozen serial gates/measurements locally using run_gate.py,
run_admission.py and run_series.py. Independent reviews/verifications remain
delegated; parent execution is not independent review. Existing logs retained.

A full03 actually ended after external1200s timeout:4202started,4021passed,
181failed(including final SIGTERM),75skipped; incomplete canonical completion.
It lacked required64MiB test-stack environment. Corrected full run must use
run_gate.py fixed environment and nextest's own limits, no external1200s kill.

## Current corrections and reproducibility

Ran: A optimized full01 compiled and entered canonical full test execution;
result pending. It uses the dual-reviewed optimized variant and64MiB stack.
Reviewer B confirmed one common-A extraction-induced source-guard defect: the
four provenance reads span the old qualification and new controlled helper,
but the guard reads only the former. Correction is prospectively mapped and
will preserve four plus actual helper connectivity after this run completes.
Other failures require individual classification, not blanket stack attribution.

Ran: F-source-01 reconstructs from A with patch1742440c... and source identity
8013488350fa713fb3730f30b3804c2990816a4fef10e7ab21716fbd1b005d2e;
R-PC1-source-01 reconstructs with patch4b3bf426... and source identity
d38f49c47df112928de9ed03a79a418d189a241ded3b276cb5a3579e3b2aedec.
Both source-reconstruction01 exclusive logs and metadata report exit0. These
are source reconstruction, not compilation/runtime or binary reproducibility.
Seven offline collector tests PASS after adding mixed sun/shade beta vectors;
real PC1 replay admission remains pending.

Ran: A-full-release-01 completed exit100, not PASS:4,202 tests run,
4,022passed(2slow),180failed,75skipped; test interval513.757s,
build+run wrapper1,066.303186214s. Complete log SHA
5f4c279602e193c305048e7613337c688728daf7dc87124abbe44083c892917e.
Full release/debug inventories both contain4,277 cases across251 binaries;
all case names, ignored flags and filter-match decisions compare exactly.
No test is removed by the optimized variant. Execution modes remain distinct.
The initial comparison printed equality then an optional first-empty-suite
inspection raised StopIteration; equality is independently rerun below before
terminal use. Raw inventory01 logs/metadata preserve both named lists.

Ran: F-focused-04 PASS5/5, including101 actual full-carrier two-reference
comparisons (Full36, Retry24, Half1 24, Half2 17, Root0), Discovery/Exact and
physical closure. Full-reference test39.086s; whole command59.599373874s;
log SHA286b289794dd7b31e18364a5a415459271c2762fc501b03d778f6b896b082ca4.
This does not replace the independent primary A/F invocation multiset gate.
F-release-build-01 is active; no timing trial or R-PC1 runtime pass claimed.

The common provenance source guard now reads both actual helper files in all
three trees, retains four canonical reads and every schema check, and verifies
original baseline/helper include connectivity. Scoped rustfmt check found only
formatting in the new read expression; format correction and focused execution
remain pending. No physics/runtime source changed in main after A freeze.

## Additional completed named gates

Ran: recorded commands/results from the exact raw logs and adjacent `.log.json`
receipts below; this section supersedes earlier running/pending descriptions
only for these named runs. Parent adds later corrections and disposition.

| Raw log | Actual command after `nix develop --offline -c` | Outcome |
|---|---|---|
| A-LSE-debug-01.log | cargo nextest run -p openwepp-land-surface-energy --profile full --success-output final | PASS:146/146,0 skipped; test1.628s, wrapper6.647942640s, exit0. |
| F-full-release-01.log | env CARGO_PROFILE_RELEASE_LTO=false cargo nextest run --release --workspace --profile full | FAIL:4207 run,4005 passed(1 slow),202 failed,75 skipped; test149.392s, wrapper707.666715935s, exit100. No blanket inherited-failure exemption. |
| R-PC1-SG1-focused-02.log | cargo nextest run -p openwepp-land-surface-energy --profile full --success-output final | PASS:150/150,0 skipped; test3.613s, wrapper11.998203543s, exit0. |
| R-runner-check-02.log | cargo check -p openwepp-runner --tests | PASS compilation/check, exit0, wrapper7.593106390s; warning remains in log. Not runtime execution. |
| R-release-build-01.log | env CARGO_PROFILE_RELEASE_LTO=false cargo test --release --offline -p openwepp-runner --lib --no-run --message-format=json | PASS build, exit0, wrapper398.803667020s; no tests executed. |

All five receipts declare RUST_MIN_STACK=67108864 and CARGO_NET_OFFLINE=true.
All are based at commit2b56e6ebc with their arm-specific source deltas; this base
commit alone does not identify mutable treatment contents.

| Log | SHA-256 |
|---|---|
| A-LSE-debug-01.log | 92e922fff2d93e816ef1b9e5ea724a8f7c393c69ed44772d0210fc52827ea064 |
| F-full-release-01.log | 13c1f3d6ebed0a36e38debc33cc4da2dfbb41376fcd985dc7aa7bf596bf6fb6e |
| R-PC1-SG1-focused-02.log | f0a203a7f23dc4c99d8ffb75cd00e468515b01ebbddc4703fb2009fdde13e413 |
| R-runner-check-02.log | f651707888c33cf740b89da28614d3dfaaa65dfbffd58e6c21f3f3cb0fffc215 |
| R-release-build-01.log | e9dbaf5e1876bf545ce50ed123fa58b9b189b9b24721379e1d063d67ec7ca182 |

R release Cargo artifact identifies the runner executable as
`/workdir/.cache/openwepp/targets/R-344ae65295ff/release/deps/openwepp_runner-682944f41aa5eb67`.
Its later measurement hash must be bound separately; successful build is not
R real-consumer, forced-complete parity, full-workspace or comparative admission.

## Resumed controlled campaign, 2026-09-08 PDT

Ran: comparator_suite_runner became unavailable through a recorded service quota;
the authorized serial local fallback executed the frozen scripts. A/F final
timing PASS 12/12 pairs plus four warmups, F memory PASS 6/6 pairs, and teardown
PASS 3/3 pairs of ten runs. A/R timing PASS execution 12/12 pairs plus warmups,
memory PASS 6/6 pairs, teardown PASS 3/3 pairs of ten runs. PASS here means
valid execution/parity, not favorable performance or release qualification.

The first A/F warmup series FAILed collector validity on a shared-cwd provenance
binding and produced no measured pairs. `run_series.py` was corrected to accept
separate frozen arm cwd values; collector syntax passed. Historical collector
unit tests initially FAILed because their external A-runner fixture was replaced
by the new explicitly identified binary cut. The fixture now consumes the durable
current admission record/identity; focused rerun PASS 8/8.

The 10-OFE A/F admissions initially FAILed a common assertion multiplying the
48 temporal supports by lane count. Corrected A05/F06 builds and one-OFE
admissions PASS. Corrected 10-OFE executions PASS, but mechanism qualification
FAILS applicability: provider=carrier=0 and both arms D=5330. The reviewed
scale relation requires a matched native branch with positive provider work.
19 OFE is therefore NOT RUN. R scale is NOT RUN under the frozen competitive rule.

Full-workspace and broad Clippy failures remain FAIL exactly as previously
recorded. Release golden remains FAIL. No production PASS is claimed.

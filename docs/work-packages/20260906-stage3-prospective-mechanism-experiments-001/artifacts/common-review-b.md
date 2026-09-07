# Independent common-baseline implementation review B

Static: common A observation/harness QA review, 2026-09-06. Ran only source,
diff and identity inspection; no builds, Rust tests, workloads or measurements.
Authority approval in `review-b.md` is unchanged. This review was independently
formed without consulting common review A. Source is awaiting compile/corrective
changes; the reviewed hashes below delimit these findings.

## Findings

| ID / severity | Path and line at reviewed cut | Finding and required correction |
| --- | --- | --- |
| CB-1 / P2 | `crates/openwepp-runner/src/hillslope/tests03/controlled_mechanism_experiments.rs:33`; `artifacts/reproduction/run_series.py:71` | Child phases use nanoseconds relative to a process-local `Instant` origin; parent samples use absolute `time.monotonic_ns()`. Phases are parsed after exit, with no join between the two clocks. Samples therefore cannot be assigned to active production-run intervals, particularly across ten teardown iterations. Use one shared monotonic clock domain or live parent phase-receipt times with explicit uncertainty, and verify active-interval sample selection. A lifetime sampled maximum must not be called an active-run maximum. |
| CB-2 / P2 | `crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/runoff_reconciliation/stage3_solver/evaluation.rs:945` | Resolved `Evaluator` completion and scope drop precede the support/beginning/role join at lines 951-964. A provider returning a malformed transition is recorded as a successfully completed evaluator although evaluation returns a typed error. Complete the evaluator after its boundary validation, or give a provider-only observation a distinct meaning. Add a focused malformed-transition counter test that preserves the existing typed error. |

Both findings block admission of the affected observation claim; they are
in-scope corrections, not a reason to end the package or reject a mechanism.

## Accepted and statically verified corrections

| ID | Correction verified |
| --- | --- |
| CB-3 / P2, resolved | Initial `run_series.py` recomputed the expected binary hash for every process, allowing a between-sample replacement in one series. Current lines 120-125 freeze both arm hashes, environment digest and source manifests once; lines 31-33 reject a changed executable before another process. |
| CB-4 / P2, resolved | Initial deterministic log names opened with `wb`, overwriting previous attempts while appending their metadata. Current line 48 uses `xb`; the series freeze uses exclusive creation at line 124. Prior raw attempts cannot be silently replaced. |
| PB-1 / Low, resolved | `experiment-protocol.md:95` defines competitive scale-check admission and requires a recorded decision when building-block recommendations depend on scaling. |
| PB-2 / Low, resolved | `experiment-protocol.md:106` distinguishes private mapped resident fields, anonymous RSS, PSS shares and parent `RssFile`. Child mapping output preserves kernel field names. |

## Observation and QA assessment

The production-call diff introduces scopes and equivalent extracted evaluator
bodies. Static inspection found no altered physical arithmetic, finite-difference
sequence, provider retry, tolerance, branch choice or scientific error mapping.
The retained historical one-OFE test delegates to the extracted authenticated
CompleteOwner workload. The complete consumer call, including publication, remains
inside runner wall/CPU boundaries. Fixture construction and post-run output hashing
remain outside those boundaries.

Compact carrier mode does not call input/output detail closures. Compact LSE mode
does not construct `SweepBase.base_bits` or event payloads; both observers use
checked counts and report record loss. Parent/evaluator/provider ordinals identify
real entries rather than deduplicated payloads. The LSE probe counter is separate
from complete evaluator calls and leaf calls. No replay is claimed for A.
`MapToken` equality deliberately excludes observation identity from physical-phase
equality; its join checks session generation. Actual output/control parity is still
required because these static facts do not prove the common harness harmless.

The post-drop phase occurs after the run function returns, its report/audits/output
parsers are dropped, and the returned JSON record is printed and dropped. Repeated
RSS can therefore describe allocator retention after teardown; it cannot identify
live requested heap bytes. `wait4` accounts for the exec'd test process lifetime,
while `/proc/self/stat` ticks bracket the consumer call. Those scopes are correctly
kept distinct. Detailed audit is disabled by the series script.

Output evidence retains all nonmanifest output file hashes plus the complete raw
manifest; no broad normalization has been added. The exact volatile-field allowlist,
input/control/result comparison and independent closure operand map remain known
precomparison admission dependencies. Freeze them from authentic A output before
accepting comparative samples; `run_series.py`'s current `valid` flag establishes
process/record completeness only, not cross-arm scientific parity.

## Non-blocking debt and follow-ups

- Keep the extension linked to the original frozen identity. The new exclusive
  series-freeze file intentionally rejects reopening the same series; any 24-pair
  extension needs an explicit append/linked-extension route that preserves the
  original freeze, balance, sample identities and combined analysis.
- Add session-generation/old-scope isolation tests when completing observer QA;
  current code has the guards, while tests mainly exercise nesting, overflow,
  bounded detail, successful completion and early exit.
- Existing WARN files remain below 3000 lines and their intake artifacts contain
  decomposition intent. New observation modules isolate the implementation instead
  of adding an unbounded observer subsystem to solver bodies. Recheck exact final
  line counts and warnings-denied Clippy after fixes.

## Reviewed source identity

Base: `e89befa4678eadec039b3e7f7fe0a176af8e9dc5`. Paths below use H =
`crates/openwepp-hillslope-orchestrator/src/`, L =
`crates/openwepp-land-surface-energy/src/`, and U =
`crates/openwepp-runner/src/hillslope/`.

| Path | SHA-256 |
| --- | --- |
| H `stage3_mechanism_experiment_audit.rs` | `26eb5e768ce52987427b9e23f8b0c6339a5cd3f2f5a01ba4035c7612e902cf89` |
| H `lib.rs` | `5e8489fafea131cdf6a45afbdd461eb2700a8a487201b0820ca7e8bb49dbc09a` |
| H `snow_stage3_v11_terminal_execution.rs` | `69d2ccbe61598ebaefb7fc867becb5b29ed9e50240d28a6067950f636619c31d` |
| H `hydrology/support_helpers_mod/runoff_reconciliation/stage3_solver/evaluation.rs` | `707ce03392dcff5bad30312c53df381a551135bde6ca5ffd9cd4c29aead9bee1` |
| H `hydrology/support_helpers_mod/runoff_reconciliation/stage3_solver/support.rs` | `f26cbfb7c7fc9964882744d6984611bbc5d6a7b385de7d78a13a5d0006b47d63` |
| L `solver_mechanism_audit.rs` | `1e7a075c6010ec36246dea3c4232c0c338bc8b2c6863f669133e766c2e06706a` |
| L `solver_covered_solve.rs` | `bf13c0fee7fc2183d7c55a1f9619bd800551beaebbbe7730abc9097f6931eb7c` |
| L `solver_covered_evaluation.rs` | `f3b2db2402a665b8758d7933336836eac6b6bd02bf3bbd5d3cf732357f3f1366` |
| L `transaction.rs` | `6a4a8687c4536f263101290eb57c1dd624ed8caf2ca0ac973760028e3b92cf41` |
| L `transaction_v3_bridge.rs` | `1371563d6d14e2802d882fa20c2ce1399eca6c40d73a0b804241f96eaf01954b` |
| L `lib.rs` | `91d83b22a17b197a12fdf4204cbe12541c98aa119691a4406c64aa56b3577924` |
| U `03_tests.rs` | `9fe774795e8dee6aa92e58aa22d154f879a990af0fd53d828da83d0a3fda080d` |
| U `tests03/stage3_runner_qualification.rs` | `a6eb6c59d38324d58ce6dff4cd954bd919aaf9fab8cc41cdb30d610fd0198319` |
| U `tests03/controlled_mechanism_experiments.rs` | `b8a4b35d3a0f8cf64045d7f0cd50ff1e54e36f9268022ba42201425cccf0d292` |
| package `artifacts/reproduction/run_series.py` | `d4f2b0a422baae1f3d121d25c08037498f07ce988fe2e8b57ce04095865f1526` |

QA disposition: HOLD common-observation measurement admission for CB-1/CB-2 at
this cut. Continue the in-scope fixes and executable verification. No full common-A
QA pass, scientific pass, package completion or production qualification is claimed.

Static: independent common-A source/harness review A. No Rust tests, builds,
real-consumer runs, or measurements executed by this reviewer. The parent is
serializing compilation and corrective edits; this is an implementation review,
not terminal package verification.

## Findings

| ID | Severity | Path/line at first review | Finding and required correction | Status |
| --- | --- | --- | --- | --- |
| CA2 | HIGH | `artifacts/reproduction/run_series.py:91` (initial cut) | Validity checks only process success/cardinality and binary stability inside that process. Changed binaries between processes, wrong scientific/control results and input/output differences could be accepted in one series. Freeze arm identities before collection and enforce each admitted input/output/control signature, including manifest comparison under a frozen explicit volatile-field allowlist. Persist failure then stop. | Parent accepted; between-process binary freeze and exclusive log/series creation verified in revised script. Admitted parity predicates remain pending. |
| CA1 | MEDIUM | `crates/openwepp-runner/src/hillslope/tests03/controlled_mechanism_experiments.rs:33`; `artifacts/reproduction/run_series.py:69` | Child phase timestamps are elapsed from a private OnceLock origin; parent samples use absolute monotonic timestamps. No retained clock join can identify which samples belong to the active runner interval, so sampled active maximum is not reconstructable. Use a common monotonic clock or a retained bounded clock anchor; record actual phase boundaries separately from lifetime HWM. | Parent accepted; correction pending. |
| CA3 | MEDIUM | `artifacts/reproduction/run_series.py:78` (revised script) | Marker JSON is parsed before the accounting row is appended. A timeout during the large JSON record or malformed observation raises JSONDecodeError and loses PID/timeout/exit/wait4/sample metadata from results.jsonl. Retain parse errors and raw log hash in an invalid accounting row, then stop. | Submitted to parent; correction pending. |

HOLD for comparative collection until these bounded collector defects are
resolved and verified. No change to scientific arithmetic or guard precedence
was found in the reviewed common Rust delta. Source implementation and its
already-required tests may continue while measurement remains unadmitted.

## Observation and workload assessment

The original one-OFE ignored test delegates to the extracted common runner
with one OFE and unchanged 100 m2 area, CompleteOwner seed, explicit runfile,
runtime policy, forcing/topology fixture constructor and closure validator.
The 10/19-OFE cases extend the existing authenticated scale constructor.
Fixture/input hashing, validation, detailed trace serialization and teardown
remain outside the complete production consumer's monotonic wall interval.
Existing publication remains inside it. The inherited process-CPU parser
correctly splits after the final parenthesized command field and reads Linux
user/system tick positions with recorded CLK_TCK.

Carrier scopes are inserted around actual outer/evaluator/provider/carrier and
batch boundaries. They preserve every original call, `?`, error mapping and
result selection. Carrier success is recorded before the original wrapper
maps its error; early returns/unwind leave scopes unsuccessful. Detailed
fingerprints are lazy and execute outside the RefCell borrow. Compact mode
does not format/hash or allocate event payloads. Counter overflow and open
scopes reject observer evidence at session finish, without entering science
error precedence. Canonical transition seals are attribution evidence only;
they do not replace complete-result or output parity.

LSE wrappers return the original Result and retain original node arithmetic,
finite-difference perturbation/branch/order, solver sequence and error handling.
The added calls observe leaf work, solve/iteration/sweep/probe/evaluation
lifecycle. Stencil validation is additionally inspected only for detailed
traces. A map token originates at the actual potential phase and rejoins its
final phase within the same observation generation; the synthetic V3 bridge
does not invent one. Metadata equality is explicitly nonsemantic. Numerical
`Ok(Rejected(...))` remains a completed call, distinct from an evaluator Err.
The separately owned collectors have different nesting and result models;
their bounded duplicated bookkeeping does not duplicate physics and is
acceptable here without introducing a new shared framework.

Post-run output evidence retains every nonmanifest file hash and the complete
raw manifest, plus independent WAT5/area/HBP/PASS operands. Existing closure
checks reject the factor-1000 unit alias and compare source, outlet, storage,
clamp and peak surfaces. Actual report/snapshot/audit/telemetry/output locals
and the returned JSON record drop before the caller's post-drop observation.
Detailed vectors are absent when the series driver clears the trace variable;
merely clearing a detailed vector would not otherwise free its capacity.
No inner process closure or forced-reference parity is claimed by outer
output equality alone.

Protocol PB1/PB2 corrections verified: competitiveness is prospectively defined,
and kernel mapping fields distinguish private/anonymous RSS from Pss_File and
sampled RssFile. Protocol hash at this review is
`1bf7803ff9aff236119aae143d62f356d391fd11f2d33e85f732b230ea87feb5`.

## Evidence identity and residual validation

Base is `e89befa4678eadec039b3e7f7fe0a176af8e9dc5`. Exact reviewed file hashes
below identify the common source before pending clock/compile corrections.
Prefixes H=`crates/openwepp-hillslope-orchestrator/src/`,
L=`crates/openwepp-land-surface-energy/src/`,
R=`crates/openwepp-runner/src/hillslope/`.

| File | SHA-256 |
| --- | --- |
| H `stage3_mechanism_experiment_audit.rs` | `26eb5e768ce52987427b9e23f8b0c6339a5cd3f2f5a01ba4035c7612e902cf89` |
| H `snow_stage3_v11_terminal_execution.rs` | `69d2ccbe61598ebaefb7fc867becb5b29ed9e50240d28a6067950f636619c31d` |
| H `lib.rs` | `5e8489fafea131cdf6a45afbdd461eb2700a8a487201b0820ca7e8bb49dbc09a` |
| H `hydrology/support_helpers_mod/runoff_reconciliation/stage3_solver/evaluation.rs` | `707ce03392dcff5bad30312c53df381a551135bde6ca5ffd9cd4c29aead9bee1` |
| H `hydrology/support_helpers_mod/runoff_reconciliation/stage3_solver/support.rs` | `f26cbfb7c7fc9964882744d6984611bbc5d6a7b385de7d78a13a5d0006b47d63` |
| L `solver_mechanism_audit.rs` | `1e7a075c6010ec36246dea3c4232c0c338bc8b2c6863f669133e766c2e06706a` |
| L `solver_covered_solve.rs` | `bf13c0fee7fc2183d7c55a1f9619bd800551beaebbbe7730abc9097f6931eb7c` |
| L `solver_covered_evaluation.rs` | `f3b2db2402a665b8758d7933336836eac6b6bd02bf3bbd5d3cf732357f3f1366` |
| L `transaction.rs` | `6a4a8687c4536f263101290eb57c1dd624ed8caf2ca0ac973760028e3b92cf41` |
| L `transaction_v3_bridge.rs` | `1371563d6d14e2802d882fa20c2ce1399eca6c40d73a0b804241f96eaf01954b` |
| L `lib.rs` | `91d83b22a17b197a12fdf4204cbe12541c98aa119691a4406c64aa56b3577924` |
| R `03_tests.rs` | `9fe774795e8dee6aa92e58aa22d154f879a990af0fd53d828da83d0a3fda080d` |
| R `tests03/controlled_mechanism_experiments.rs` | `b8a4b35d3a0f8cf64045d7f0cd50ff1e54e36f9268022ba42201425cccf0d292` |
| R `tests03/stage3_runner_qualification.rs` | `a6eb6c59d38324d58ce6dff4cd954bd919aaf9fab8cc41cdb30d610fd0198319` |
| `artifacts/reproduction/run_series.py` | `dab941ebef50770b33664e8ac38c9ee52369a29dc11ff6515520b798ca27e3c8` |

Ran: instruction discovery, static diff/source inspection, SHA-256 enumeration,
line counts, and `git diff --check` (PASS). No workflow execution by reviewer.
New observer unit tests cover basic lifecycle/errors, bounded traces, overflow
and open sessions; the parent must run them and the real workload. Stencil
oracle, source-real error/rollback/forced-complete tests, actual original/A
fidelity and full critical-cut correctness remain required admission evidence.
Artifact presence or a compile PASS cannot close these requirements.

WARN files: terminal execution 2910; scalar evaluation 2448; covered evaluator
2651; transaction 2952; runner qualification 2654 lines. Accept the bounded
observer/extraction rationale and documented future decomposition ownership in
the reading maps; no reviewed file reaches 3000. Terminal reconciliation must
recheck after fixes. No selected current-scope gate is waived or deferred.

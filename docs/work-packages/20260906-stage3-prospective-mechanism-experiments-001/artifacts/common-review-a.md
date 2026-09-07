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

## Corrective review A: common source approved

Static plus reviewer-run read-only helper checks. This addendum supersedes the
initial collector HOLD for the exact corrected source below; no comparison has
been admitted or measured by this reviewer.

| Finding | Corrective verification | Disposition |
| --- | --- | --- |
| CA1 | The harness now obtains the same Linux CLOCK_MONOTONIC clock as Python through the safe `rustix::time::clock_gettime(ClockId::Monotonic)` API. Explicit timestamps bracket the actual consumer; each parent status read retains start/end timestamps and enters the active maximum only when its entire bracket is inside that interval. Phase/mapping reads remain separate. The rejected unsafe FFI attempt is absent. | RESOLVED at source review. |
| CA2 | Series startup freezes each executable, source manifest, environment and admitted sidecar. The A/B common signatures must be exactly equal before launch; every emitted record must match that shared scientific/input/output/control/normalized-manifest signature and its own arm's carrier/LSE signature. Binary identity is rechecked across and within processes. Log and freeze files use exclusive creation. | RESOLVED at source review; actual admitted sidecars/rules remain required before comparison. |
| CA3 | JSONDecodeError is retained as parse_errors; missing or malformed science/manifest fields, including indexed-list IndexError, become parity_errors. Active-window schema failures are also caught. The accounting row, raw log hash, wait4 data and invalid status are appended before the series raises its failure. | RESOLVED at source review. |

The safe clock adds `rustix = =1.1.4` with the `time` feature under runner
dev-dependencies, reusing the already locked package version. The lockfile
delta adds its direct runner dependency edge. The runner also enables the
existing LSE `test-support` feature through its dev-dependency. These exact
Cargo paths are recorded in the prospective reading map. Keep applicable
dependency/feature correctness and `cargo deny check` in the selected validation;
the fixes do not justify narrowing existing critical-cut requirements.

Corrected identities (other common Rust identities above unchanged by these
collector corrections):

| Path | SHA-256 |
| --- | --- |
| R `tests03/controlled_mechanism_experiments.rs` | `b81f0bc5a26237bb16c9f0968f12ba0f2e55265054177dd6a631edde427d2bfe` |
| `artifacts/reproduction/run_series.py` | `fa82aebffa65d9b5881feab5984eb65a96b32b774a0ad10214df7d82e55368cd` |
| `crates/openwepp-runner/Cargo.toml` | `389c3203e91ed6d39baf5f7ef757f50e767f4776a96501e284b6914cb8a81c3b` |
| `Cargo.lock` | `5787d315c0ed47a38fb43819688ee451caf3fc8f4392567c9a739cce4eb16c7c` |

Ran by reviewer: a `.venv/bin/python -B` read-only import of `run_series.py`
and explicit helper assertions confirmed that changing only enumerated run
paths/timestamps preserves normalized identity; changing a scientific scalar
does not; positive/negative zero remain distinct; non-path `run_path`
exceptions reject; and input manifests are unmodified. Result PASS. This was
not a process-series, memory or scientific-consumer run. Scoped source/hash
inspection and `git diff --check` also PASS.

Observed external execution evidence: `raw/authority-F-test.log` and
`raw/authority-R-test.log` each report the new named prospective authority test
passed (one test, exit 0). Their hashes are respectively
`701c514a4aa2661d48e77fdf6086cbf4b848a0143b289a5a925e18d4f1c542c6`
and `cb838deb21781b1a0f21f44ecd58a7bf012f400aea276ee07ccea5da8b76ee0a`.
The four authority/test source hashes remain exactly the previously reviewed
cut. Those logs' `duration_ms` annotations are invalid and must not support
duration claims: F is negative and R is incompatible with the visible build/
test durations. The parent explicitly preserves/disregards this metadata; it
does not invalidate the named nextest PASS/exit evidence or require a rerun.

GO for the corrected common-source/protocol design, with no unresolved CA1-CA3
source findings. Actual A-derived output rules and admitted identities,
original/A workload fidelity, observer/clock tests, real-consumer and complete
applicable correctness/lint/dependency gates remain pending measurement
admission. No output allowlist or signature may be inferred from this review,
and no missing workflow is marked PASS. This is not terminal package approval.

## Compile-time documentary-input source-kit repair: GO

Static: reviewed the complete corrected collector and verifier. Exact hashes:
`reproduction/freeze_source.py`
`34290b901f1c22892233eaaa03b49ba33e301ffb08a54eb6635f9a9854005c10`;
`reproduction/verify_source_kit.py`
`fe5d8f80f5082d297552b9c17d8e7f364cb1c4189404f28480f121284dd11a7d`;
explicit catalogue `compiler-doc-list-review-b.json`
`7a611070588b99d5252222543555f40a98eef4b63e096379e0e5a79f74a3418e`.

The collector requires sorted unique canonical docs paths and existing files,
includes them in content identity and exact tracked/untracked delta custody,
and records the explicit catalogue. The verifier requires catalogue paths in
the content identity, archives their exact base paths, applies the tracked
patch, and checks reconstructed bytes/hash/mode/link metadata. New archive
members must be present manifest rows in the declared input scope. No missing
compiler dependency is filled silently from the live repository.

Ran: read-only independent checks of A-source-03 and F-source-03 establish
that all 15 catalogue inputs are present manifest rows. Their reconstructed
scratch bytes each match the corresponding recorded SHA-256/length and retained
source bytes. Read parent execution logs: A reconstruction PASS, 10146 present
rows; F PASS, 10150 present rows; no recorded failures and no builds executed.
Exact identities are A
`0c12a724516f973eb13f9de72afc4d767922a4440f897ecea545966936493038`
and F `b455bc706775e39b6ad7d112dbde75349ecaf9374107097609df33f1daea22fd`.
Reviewer A did not reconstruct or compile either tree.

Scope is the declared crate-executable/compiler inputs, including two actual
production vegetation JSON inputs and thirteen crate-test JSON inputs. It is
not a self-contained root integration/full-workspace documentary dependency
inventory: those tests contain additional docs includes and require the full
base plus separately retained current authority/documentary adjunct. The older
kit's declared-code reconstruction PASS must not be retroactively described as
complete binary reproducibility. This bounded repair is GO for the named
current kits and compiler-negative workflow; actual successful compilation and
full-gate authority reconstruction remain separate evidence.

## Concrete F construction-telemetry qualifier: GO

Static: reviewed the implemented exact-leaf comparison and admission/series
bindings before comparative timing. Frozen hashes:

| File under `reproduction/` | SHA-256 |
| --- | --- |
| `run_series.py` | `0f29393a6e0817173454619be609de1b8455f443aebc807493c9bf60446558e1` |
| `admit_identity.py` | `5484f4e318356cd0df1f5774c280224e4d92678a0c3302071f5836dfe06cbe61` |
| `test_collectors.py` | `e2690394fb4d95920f643fd825e94c89770430420ab5cee0cb278751e575510d` |

The implementation accepts only the reviewed qualification identifier and exact
rule keys. N/P/D are bounded nonnegative integers (not booleans/floats), N is
1/10/19, provider count is positive, and `2*N*P <= D`. N=1 additionally requires
one of the two observed raw D/P pairs. Actual Provider and Carrier rows must
each be precisely balanced successful counts with no extra fields; dropped
records must be integer zero. The frozen rule is compared against a newly
derived rule from every actual raw record, including its N, completed providers
and manifest D. Only `/direct_runtime_counters/day_frame_constructions` becomes
the reviewed qualification/residual object. Normalization deep-copies the raw
manifest; it does not alter retained observations or qualify the whole object.

Admission freezes the raw operand rule, and series startup binds the admitted
source/binary identities. Every sample compares full common identity plus exact
per-arm carrier/LSE audit values against admission; invalid samples remain
recorded. Equal normalized residuals do not replace independent native-branch,
invocation/role/batch/custody and output/closure admission. In particular,
synthetic arithmetic cases at N=10/19 are not scale fixture admission or proof
of their actual residuals.

Ran: all eight supplied offline collector tests PASS. An additional 24 read-only
negative controls PASS for boolean/float/negative/overflow/zero operands,
unsupported N, extra or unbalanced counter fields, malformed/drop records,
foreign-arm operand rules and changes to unrelated direct-runtime counters.
Also independently verified A/F `identity-1-v2` common identity equality, exact
primary D/P pins and residual405, and preservation of the raw record after
normalization. No fresh consumer, compiler or measurement was run by reviewer A.

Decision: GO for this concrete collector implementation on the separately
admitted exact cuts. All remaining actual arm/scaling scientific, full-correctness
and reproduction gates remain mandatory; this is not blanket timing admission.

## Runner-only Clippy scope: prospective GO

Static: reviewed `crates/openwepp-runner/src/hillslope/tests03/controlled_mechanism_experiments.rs` and the original e89 baseline qualification function. Ran: read-only parsing of completed direct runner Clippy logs. A raw SHA256 `0fec212cbd403703d2710a1b5ba56979fa44378e36527bfaf61c5501c2596a85` and F `eda454d541b85eb3a15e50c7fbd4781f54c84cad462ae697d805a1b9ff782c24` contain the same five common-helper diagnostics: two constant assertions, long run function, non-Drop guard drop, and infallible OFE conversion. No builds by reviewer A.

No correctness blocker to the bounded proposed fixes. The entry release assertion is common-added; the run release assertion and its exact message are relocated unchanged from baseline `stage3_runner_qualification.rs:837–841`. The original baseline function already spans 270 physical lines; the common helper has added measurement/scale/evidence logic and is now reported as 353 Clippy-counted lines. Thus the long-function condition has inherited structure plus common growth, not an entirely new condition or an unchanged whole function.

GO for function-scoped `assertions_on_constants` expectations on only the entry and run functions, retaining both exact release guards; a run-only `too_many_lines` expectation explicitly justified by keeping this authenticated fixture/real consumer/wall-clock/custody/drop observation sequence together; and replacing only `usize::try_from(hbp.nofe).expect(...)` with infallible `usize::from(hbp.nofe)` for the existing u16 field. No broader allowances, timing-boundary movement, test-condition removal, fixture/physics change, or lifetime refactor is approved.

The `drop_non_drop` finding is inherited: the exact drop exists at e89 baseline qualification line892. `ReleaseQualificationTelemetryGuardV1` at profile line204 contains only `PhantomData<Rc<()>>` and has no Drop implementation. Preserve this existing statement and disposition it as inherited; neither delete it under a cleanup pretext nor describe it as an actual owner teardown. The R-only two wrapper functions are unchanged and currently contain no independent constant assertions. Any distinct later emitted diagnostic must be reviewed on its own evidence. Final source hashes/diff and actual current-cut runner gate remain pending.

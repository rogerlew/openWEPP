# Carrier observation intake

Static: observation-only common A instrumentation; no physical or custody policy change.

Prospective exact writes (parent assigned):
- `crates/openwepp-hillslope-orchestrator/src/stage3_mechanism_experiment_audit.rs` (new session/observer/tests)
- `crates/openwepp-hillslope-orchestrator/src/lib.rs` (module export)
- `crates/openwepp-hillslope-orchestrator/src/snow_stage3_v11_terminal_execution.rs` (actual outer/provider/carrier scopes)
- `crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/runoff_reconciliation/stage3_solver/evaluation.rs` (actual scalar evaluator scopes)
- `crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/runoff_reconciliation/stage3_solver/support.rs` (actual batch provider scope; parent authorized before edits)
- This artifact.

Ran: `tools/agents/find-agents --for` resolved root `AGENTS.md` plus `crates/AGENTS.md` for every Rust path; root plus `docs/work-packages/AGENTS.md` for this artifact. Instructions read completely: root 160 lines / 12599 bytes; crates 75 / 5436; science-contracts 77 / 5942; work-packages 388 / 26781 (initial output truncation completed with explicit range reads). Package `package.md` read completely. Kickoff read supplied text and file; no historical recovery attempted.

Source read ranges before edits: `lib.rs` 1–60 (file 309 lines / 20908 bytes); terminal execution 406–435,505–515,610–795 (file 2885 / 119524); evaluation 180–280,340–390,445–495,630–715,885–935,1330–1445 (file 2424 / 126230); support 1–140 (file 1189 / 48241). Read-only type discovery: runoff_reconciliation.rs 1150–1185,1600–1638; v11_covered/carrier_phase.rs 580–630. Source ranges are targeted context, not claims of whole-file reading.

Static source parity rationale: scopes do not replace, elide, reorder or retry any provider or carrier call. Early return/`?` remains at each original boundary. Scope drop records unsuccessful exit; successful completion is explicit. No audit failure enters scientific error precedence: observer overflow/lifecycle faults are reported only when the harness consumes its session. Detailed request/result digests are lazy and bounded, disabled in compact timing posture. Each evaluator/outer gets an authentic monotonic ordinal even for equal request bytes. Existing 2000+ source files remain WARN, additions stay below 3000; observation module is independently decomposed. Broader source decomposition belongs to architecture follow-on, not this observation delta.

Returned-transition attribution digest uses an explicit ordered preimage: support start/end nanoseconds (big endian), all eight boundary floating operands (`to_bits`, big endian), beginning Stage-3 state seal, provisional/final identity discriminator and each identity seal, canonical beginning/ending joint seals, canonical probe-child seal, optional snow-soil receipt discriminator/seal. These existing canonical custody seals bind their own fields/owner bytes. This is transition attribution, **not** full carrier-phase or scientific/output parity proof. Request Debug digest is same-source attribution only. Batch detail currently attributes its returned carrier joint; the batch lifecycle count is directly observed. Full phase and produced-output parity remain separate package admission gates.

Carrier success/error is observed on the actual `execute_covered_carrier_phase_v1` result **before** provider maps owner failure to its existing typed terminal-custody error. Scope errors also cover earlier `?` returns and unwind; no observer error changes existing error precedence.

Ran: Nix-shell rustfmt on new module and edited source (plain rustfmt absent in ambient shell). Incidental import reformatting in lib.rs was removed, retaining its one-line module export only. `git diff --check` passed. Terminal execution is 2910 lines and evaluation 2448 lines, both existing WARN files under the mandatory 3000-line refactor threshold; separate observation module 386 lines.

NOT RUN: build, observer tests and full workload evidence delegated to parent serial executor after instrumentation integration. Source rationale is not execution parity evidence.

Review B correction: resolved evaluator scope now spans provider execution, boundary/support/joint validation and the remaining physical substep/closure tail. Success is recorded only at the completed substep boundary. Provider/carrier successes remain independent actual-call results. Added a failed-tail observer regression with a successful provider followed by a failing join, plus source ordering binding through the actual boundary and energy-closure guards. This regression tests observation semantics and source placement, not an injected full physical fixture. Numerical statements, guards and evaluation order are unchanged. Parent reports preceding authority/observer tests passed; this correction requires its new serial execution before claiming PASS.

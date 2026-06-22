# R7C Production Direct Executor Path

Status: complete.

Package type: Array-native runtime implementation work package.

Objective: add an explicit opt-in production direct executor path that routes
parsed typed direct frame construction into `DirectFrameExecutor` for the
run/lane/day loop without calling compatibility climate-day execution for that
mode.

Rationale: R7B added typed constructor APIs, but runner execution still entered
the compatibility climate-day scheduler for every output-producing mode. R7C
must prove that a distinct production direct mode can construct direct frames,
execute canonical R5 phase spans, commit day frames, and publish direct runtime
counters without hot compatibility scheduler/request/writeback edges.

Included scope:

- Add a runtime selection distinct from compatibility, direct skeleton, shadow,
  and direct-publication cutover modes.
- Build a runner-owned direct production frame from parsed inputs, static
  runtime setup, lane geometry, and per-day typed direct inputs.
- Route the opt-in production direct mode through `DirectFrameExecutor` for the
  full run/lane/day loop.
- Record manifest-visible direct runtime counters for production direct mode.
- Preserve compatibility, shadow, skeleton, and R6J cutover behavior.
- Add focused tests proving no compatibility climate-day call, nonzero direct
  phase execution, day frame construction/commit, publication production, and
  zero compatibility-edge invocations.
- Record timing/RSS evidence when the fixture harness available in this repo
  can produce it inside the package execution envelope.

Excluded scope:

- Default activation; compatibility remains the default runner mode.
- R7D publication-producer authority completion.
- Deleting compatibility scheduler/runtime types.
- Broad H2637 performance closure beyond recording current opt-in evidence.

Intended write set:

- `crates/openwepp-runner/src/api.rs`
- `crates/openwepp-runner/src/bin/openwepp-cli-hill.rs`
- `crates/openwepp-runner/src/hillslope/00_runner_intake_and_lane_setup.rs`
- `crates/openwepp-runner/src/hillslope/03_tests.rs`
- `crates/openwepp-runner/src/hillslope/04_direct_publication.rs`
- `crates/openwepp-runner/src/hillslope/05_runner_execution_and_outputs.rs`
- `docs/architecture/array-native-runtime-specification.md`
- `docs/work-packages/README.md`
- `docs/work-packages/20260622-r7c-production-direct-executor-path-001/**`

Dependencies:

- R7A architecture state reconciliation.
- R7B parsed-input typed frame constructors.
- R6J direct publication cutover writers and manifest counter provenance.

Phase plan:

1. Scaffold package, catalog entry, prompt, and evidence placeholders.
2. Add explicit production direct runtime selection and CLI flag.
3. Build direct production frame input assembly from parsed/static runner state.
4. Route production direct mode through `DirectFrameExecutor` without
   `execute_hillslope_climate_days`.
5. Extend direct publication artifact/manifest handling only enough to expose
   direct executor output evidence; do not claim R7D producer-authority
   completion.
6. Add focused runner tests and static scans.
7. Run package gates, dual local reviews, verification, line-count governance,
   and final disposition.

Acceptance gates:

- Direct production mode does not call `execute_hillslope_climate_days`,
  `execute_with_kernel*`, or construct `HillslopeKernelRequest` inside the
  direct production execution branch.
- Opt-in direct production fixture executes all canonical R5 phases and records
  nonzero direct phase/counter evidence.
- Direct production manifest records nonzero run frame construction, day frame
  construction, day frame commit, publication production, phase span runs,
  direct phase entries, direct compute, direct state mutation, downstream
  operand production, and shadow projection counters.
- Direct production manifest records zero compatibility-edge invocations.
- Default compatibility fixture remains unchanged with zero direct runtime
  counters.
- R6J direct-publication cutover behavior and manifest counter locality remain
  unchanged.
- Static scans prove the direct production branch does not call
  `execute_hillslope_climate_days`, `execute_with_kernel*`, or construct
  `HillslopeKernelRequest`.
- Rust closure gates pass: `cargo fmt --check`,
  `cargo clippy --workspace --all-targets -- -D warnings`,
  `cargo test --workspace`, and `cargo deny check`.
- Scoped Markdown lint and `git diff --check` pass.

Security-impact gate:

- No secrets, tokens, network credentials, or local absolute fixture paths are
  introduced.
- The new mode remains explicit opt-in and fail-closed; default compatibility
  behavior remains unchanged.

Review requirements:

- Dual local reviews with explicit finding disposition.
- Dual verification pass with `Static:` and `Ran:` evidence.
- `.rs` line-count governance: production files at or above `2000` lines are
  `WARN`; non-exempt `3000+` line files block closure.

Final disposition: `COMPLETE-R7C-PRODUCTION-DIRECT-EXECUTOR-PATH`.

Closure summary:

- Added explicit `DirectProductionExecutor` runtime selection and
  `--direct-production-executor` CLI flag.
- Routed that opt-in mode through `DirectFrameExecutor` without calling
  `execute_hillslope_climate_days`, `execute_with_kernel*`, or constructing
  `HillslopeKernelRequest` inside the direct production execution branch.
- Skipped compatibility symbol-registry and indexed-shadow diagnostics for the
  production direct selection.
- Published manifest-visible run-local direct runtime counters for direct
  production execution.
- Preserved default compatibility mode, R2A default inactivity, and R6J direct
  publication cutover behavior.
- Recorded same-binary H2637 default and direct-production timing/RSS. Direct
  production is not performance-ready and output parity is not complete; those
  remain R7G and R7D/R7E-R7H scope.

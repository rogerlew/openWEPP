# Implementation Intent

Status: `EXECUTED HOLD / PRODUCTION CLOSURE GATES OPEN`

The implementation target is a default-off Child 2C candidate invoked by the
ordinary `DirectFrameExecutor` publication loop through an explicit opt-in
method. The normal selector, default mode, CoE R4G snow owner, publication
schema, and existing direct path remain unchanged. The candidate stages a
shared-carrier receipt, integer-tick terminal event, V11/LSE/snow/liquid/
hydrology/soil-thermal/BGC owner envelope, and a snow-free continuation before
one commit point; a failed attempt leaves the beginning owner bytes and cursor
unchanged.

The implementation must consume the five released Child 2C contracts directly,
use typed receipts and fail-closed errors, and stage every owner before one
complete-owner commit. The source/test write set is frozen to:

- `crates/openwepp-hillslope-orchestrator/src/snow_stage3_terminal_handoff.rs`
- `crates/openwepp-hillslope-orchestrator/src/lib.rs`
- `crates/openwepp-hillslope-orchestrator/src/direct_runtime/03_executor.rs`
- `crates/openwepp-coupled-time/src/error.rs`
- `crates/openwepp-persisted-restart-v1/src/lib.rs`
- `crates/openwepp-persisted-restart-v1/src/snow_stage3_handoff.rs`
- `tests/integration/snow_stage3_shared_carrier_terminal_handoff_implementation.rs`
- root `Cargo.toml` for one explicit integration-test registration only

Package artifacts and `package.md` are also in scope. No selector, CoE,
calibration, seasonal efficacy, deployment, or publication authority files are
in scope. If the live owner or restart path cannot be proved by implementation
tests, final disposition must be `HOLD`; no surrogate path may be relabeled as
closure.

Follow-on amendment: the user explicitly authorized concrete owner wiring. The
bounded follow-on surfaces are the terminal-handoff module, direct scheduler
seam, V11 consumer adapter, existing typed real-consumer stack, and package
tests. The follow-on must replace the opaque ending-owner handoff with a typed
owner-execution receipt and must invoke the existing
`DirectV11RealConsumerStack` before committing the terminal cursor and
publication frame.

Hold-lift amendment: the user explicitly authorized completion of the remaining
HOLD work. The source write set is expanded to the normal hillslope runner,
typed direct-runtime owner/publication/restart surfaces, the persisted typed
transaction, and package-owned tests. The implementation must prove a real
consumer path and typed owner custody; it must not convert the existing shadow
state into a production claim or introduce proxy physics.

Disposition: the bounded hold-lift work is executed. The ordinary-runner
receiver remains held at the typed owner-input boundary because the current
runner inputs do not contain authoritative V11/LSE/BGC/soil-thermal state,
half-hour provider/GSI cursor state, or a contract-authorized terminal-liquid
receiver. No shadow candidate is promoted by this package.

# Independent correctness review — snow-free recorder preparation

**Reviewer:** `/root/wrapper_correctness`, primary correctness review, independent of the source writer and QA reviewer

**Scope:** affected ten-file recorder candidate only; no renewed route attribution, raw-corpus inspection, native-reader acceptance, cadence repair, or scientific qualification

**Final candidate identity:** tree SHA-256 `a3fbdfe52d21721b28a4cd5cf2eea790fff291831af483c23b54a8a8356fd31f`; patch SHA-256 `353c3c639c45781acfd6a017561dd84153b4a211e8696fb5a7962cba1ed20bba`, over `/workdir/openwepp-experiments/b01-wb14-cadence/current-context-capture-20260913`, as preserved in `final-source.json`

**Evidence class:** Static: inspected the final candidate, affected source delta, producer callsites, recorder payload construction, and focused tests. Inspected supplied execution evidence: `corrected-rustfmt.json` reports exit 0; `candidate-orchestrator-clippy-final.json` reports exit 101 with source unchanged during the command; `final-candidate-diagnostics.json` preserves the compiler diagnostics. The reviewer ran no Rust build, test, runtime, reader, or raw-corpus command.

## Findings

### HIGH — the final candidate does not compile, so the authorized producer-seam acceptance is unavailable

`crates/openwepp-hillslope-orchestrator/src/snow_stage3_v11_current_context_capture.rs:1562` and `:2045` place an `Arc<DirectRunConstructorInputs>` in `serde_json::json!`, but that `Arc` does not implement `Serialize` under the selected build. The same file calls the `#[cfg(test)]` helper `capture_test_active_snow_raw` from a function that is not gated by `#[cfg(test)]` at `:1920`, producing E0425 in the library target. The final selected Clippy command therefore exits 101. These are affected-source compiler errors, distinct from the pre-existing Clippy baseline.

Because compilation stops, none of the focused Rust tests execute. There is no executed evidence that the covered and snow-free production call paths record phase-correct operands, that disabled/enabled/forced-failure modes preserve model results and custody, that the normal non-target selector emits nothing, or that wrong-phase and required-field omissions fail visibly. The authorization makes those actual-producer checks mandatory. Failure 24/24 is the adopted hard stop, so this review cannot request or accept another correction in this continuation.

### MEDIUM — dynamic hydrology preservation lacks an independent mutation-sensitive oracle

`crates/openwepp-hillslope-orchestrator/src/snow_stage3_v11_current_context_capture.rs:112` reconstructs the captured dynamic hydrology state and requires equality with the live frame before emitting a row. That is useful exact round-trip protection for each exercised frame. The independent producer snapshot now compares the immutable constructor inputs, but the focused test does not independently construct or compare `native_hydrology_frame_dynamic_constructor_operands`.

This leaves a bounded residual risk: a future or omitted frame field that equals its constructor default in both fixtures could escape the manual static/dynamic partition. The candidate must not claim exhaustive future-field coverage from the current oracle. This gap is not the cause of the HOLD because a successful actual producer row would still prove exact internal reconstruction for the frame exercised by that run.

## Fix verification

- **Static, corrected:** The earlier HIGH oracle-independence finding is addressed in the final source. Both real producers construct a separate `PreChildProducerSnapshotInputsV1` before constructing `PreChildContext`, at `snow_stage3_v11_real_parent_execution.rs:128` and `snow_stage3_v11_terminal_execution.rs:2353`. Test-only prepared-support and deferred-soil projections no longer call the recorder's corresponding projection helpers. The compared snapshot covers the parent checkpoint, clock and consumer owners, support, provisional reduction/ledger/receipt/binding, prepared support, configurations, provider cursor, native constructor operands, Stage3/material owners, terminal parcels, ordered history, ending snow owner, and deferred-soil reconstructible operands.
- **Static, corrected:** Surface-liquid configuration capture uses the existing canonical-byte method; the candidate does not add `Serialize` to the production configuration or change a production restart/wire schema.
- **Static:** The snow-free call is after the real reduction, ledger, provisional receipt, and binding are constructed and before physical segment execution. It carries `v11-real-consumer`, the transformed snow-free support, actual ordered `owner_joins` and `event_groups`, prefix-validation inputs/result, ending snow-owner bytes, and deferred-soil custody operands. The covered call retains `v11-snow-covered-real-consumer` and covered projection semantics.
- **Static:** New source plumbing is borrowed and diagnostic-only. The binding hoist is value-equivalent to the prior inline construction, capture errors are converted to separate observation rows, and the non-test producer-audit hook is absent through `#[cfg(test)]`. I found no changed arithmetic, clamp or guard precedence, unit conversion, physical branch, admission rule, owner installation, proof consumption, dependency, or production serialization schema in the bounded delta.
- **Static:** The known coordinate disagreement is preserved without normalization: the recorder selector remains outer day 4 / interval 22 / `[385920000000000,385980000000000)`, while nested authority/cursor values are captured as supplied. This recorder preparation does not authenticate the complete historical prefix or resolve the disagreement.
- **Static:** The substantial parallel payload construction is confined to test-only independent-oracle code and is intentionally retained so the recorder cannot define its own expected values. No parallel production physics or custody algorithm was added.

All corrected items above remain unverified by compilation or execution in the final cut.

## Residual risk and missing tests

- The final candidate needs a successful affected compile/lint comparison and execution of the focused actual-producer test before recorder preparation can be accepted.
- No focused test, test listing, runtime capture, fresh-process import, native-reader run, authentic reacquisition, conservation/science comparison, or full-prefix authentication completed for this final cut.
- The dynamic hydrology oracle limitation described above remains even after compilation is repaired.
- The prior outer 4/22 versus nested 0/0 disagreement remains material and unresolved by design.

## Verdict

**HOLD / NO APPROVAL.** The final source preserves a statically plausible observational design and fixes the earlier independent-oracle structure, but it does not compile and none of its mandatory actual-producer acceptance tests ran. Recorder preparation, the current-context checkpoint, native restoration, scientific qualification, and cadence repair remain incomplete.

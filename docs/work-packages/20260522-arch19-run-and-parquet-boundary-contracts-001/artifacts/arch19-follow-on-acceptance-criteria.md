# ARCH19 Follow-On Acceptance Criteria

Static: follow-on criteria derived from ARCH19 boundary hold register.
Ran: none.
Status: `complete`.

## Follow-On Work Items

| follow_on_id | target objective | minimum acceptance criteria | required evidence |
|---|---|---|---|
| `ARCH19-F01` | Canonical `.run` spec + parser contract | 1. Author `docs/specifications/wepp-input-files/specs/run-file.spec.md`.<br>2. Author `SC-INFILE-RUN-001` with full required section set.<br>3. Register `.run` surface in `input-surface-registry.md` with explicit disposition. | static contract/spec diffs + dual review + dual verification artifacts |
| `ARCH19-F02` | `.run` parser implementation in openWEPP | 1. Add `openwepp-input-contract` parser module for `.run`.<br>2. Typed error taxonomy, no unwrap/expect in production path.<br>3. Integration tests covering strict + compat policy branches. | `cargo test` evidence + parser fixtures + runtime seam tests |
| `ARCH19-F03` | Run-model to parser/orchestrator closure | 1. Typed run model fields map to parser invocation context (`run_context`, sidecar policy, file surfaces).<br>2. Orchestrator rejects ambiguous/missing run boundary prerequisites. | runtime integration tests + ownership contract artifact update |
| `ARCH19-F04` | Parquet boundary conformance gate | 1. Add automated check for required schema metadata keys and stable field presence for each supported parquet family.<br>2. Gate failure is hard-fail on drift. | executable validation command output (`Ran`) + gate docs |
| `ARCH19-F05` | HBP->parquet carry-forward closure | 1. Verify schema1/schema2 differentiation and required warning semantics remain observable in parquet-facing outputs.<br>2. Comparator-tier interpretation documented in acceptance results. | targeted fixture tests + documented comparator interpretation |

## Package-Level Hold Lift Criteria

ARCH19 may move from `HOLD` only when:

1. `RUN-HOLD-001..003` are closed or explicitly risk-accepted.
2. `PRQ-HOLD-001..003` are closed or explicitly risk-accepted.
3. Follow-on package disposition artifacts include explicit closure evidence for
   all accepted findings.

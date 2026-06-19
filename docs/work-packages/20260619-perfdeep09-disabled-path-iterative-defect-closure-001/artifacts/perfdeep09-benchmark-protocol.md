# PERFDEEP09 Benchmark Protocol

Status: queued.
Evidence mode: not run.

Required protocol:

1. Build `target/release/openwepp-cli-hill` and record binary SHA.
2. Run with all PERFDEEP opt-ins unset:
   `OPENWEPP_PERFDEEP02_FRAME_ISLAND`,
   `OPENWEPP_PERFDEEP03_LANE_DENSE_STATE`,
   `OPENWEPP_PERFDEEP02_FRAME_ROUNDTRIP_PATH`,
   `OPENWEPP_INDEXED_SHADOW_REPORT_PATH`,
   `OPENWEPP_SYMBOL_REGISTRY_AUDIT_PATH`, and
   `OPENWEPP_HPHYS0245_TRACE_PATH`.
3. Use H2637 no-UI runfile and run-dir consistent with PERFDEEP07/08 unless
   execution records a replacement path and rationale.
4. Record command, run dir, output dir, seconds, RSS, manifest path, and
   protected output checksums.
5. Use single-run screening for speculative candidates, but require a three-run
   final gate before `READY-FOR-R2`.

Pass threshold: final default-disabled H2637 three-run median `<= 676.67 s`.

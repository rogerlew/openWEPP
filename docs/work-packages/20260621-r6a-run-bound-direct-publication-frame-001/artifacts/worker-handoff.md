# Worker Handoff

Status: complete.
Evidence mode: Static + Ran.

R6A is complete. Use this handoff to resume R6 direct publication writer
cutover.

Available direct publication assets:

- `DirectRunPublicationFrame` in
  `crates/openwepp-hillslope-orchestrator/src/direct_runtime.rs`;
- `DirectFrameExecutor::run_publication_capture`;
- `HillslopeRuntimeSelection::DirectPublicationFrameShadow`;
- direct consumers in
  `crates/openwepp-runner/src/hillslope/02_output_and_climate_helpers.rs`:
  HBP, WAT, PASS, loss, and manifest helpers.

Required next R6 steps:

1. Switch production HBP/WAT/PASS/loss/manifest writers to read the direct frame
   under an explicit R6 cutover mode.
2. Prove byte/Arrow identity and metadata/checksum parity against existing
   protected outputs.
3. Keep compatibility writer path as rollback/comparison until direct identity
   gates pass.
4. Add full anti-alias and independent reconstruction fixtures for erosion,
   profile/frost, loss, and manifest fields before claiming public cutover.

Do not reintroduce compatibility WB13 rows, runtime surfaces, writeback payloads,
or skeleton/counter-only evidence as direct publication closure.

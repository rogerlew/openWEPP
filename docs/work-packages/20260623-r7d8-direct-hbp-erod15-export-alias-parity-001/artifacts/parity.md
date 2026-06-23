# Parity Evidence

Status: complete.

## H2637

- Ran: fresh current-binary H2637 5-day fixture in
  `/tmp/r7d8ad-h2637-5day`.
- Ran: default command exited `0`:
  `target/release/openwepp-cli-hill --run-dir /tmp/r7d4-h2637-5day/run --run-file /tmp/r7d8ad-h2637-5day/runfiles/default.run --output-dir /tmp/r7d8ad-h2637-5day/default --policy compat --manifest-path /tmp/r7d8ad-h2637-5day/manifests/r7d8ad-default-current.json`.
- Ran: direct-production command exited `0`:
  `target/release/openwepp-cli-hill --run-dir /tmp/r7d4-h2637-5day/run --run-file /tmp/r7d8ad-h2637-5day/runfiles/direct.run --output-dir /tmp/r7d8ad-h2637-5day/direct --policy compat --direct-production-executor --manifest-path /tmp/r7d8ad-h2637-5day/manifests/r7d8ad-direct-current.json`.
- Ran: byte comparisons all returned `cmp=0`:
  - `H2637.hbp`:
    `9c297168a3fd4f5498930691f1b7b1b2166e12acebdde2d87f0b1f7e59e5b8f6`.
  - `H2637.loss.json`:
    `2467f269e64096e6678831094ac8c52748c940b73d54619db3c3c7d6b1b216f8`.
  - `H2637.pass.parquet`:
    `46d5ec947fe351c927c15dafd34d3487c036ac1bb3c1fc2848e3d22747003e89`.
  - `H2637.plot.parquet`:
    `b48cf4d50ef85ad164a69e0d880cdd96ef288d57ddbb732a0c43659d54b6e31a`.
  - `H2637.wat.parquet`:
    `1b23e8cd8a53f7bd37f95f3c9dde60cec95f421e4b5498b5b2232e6c483a6a80`.
- Ran: direct manifest provenance:
  `scheduler_kernel_executed = false`,
  `publication_source = direct-publication-frame`,
  `compatibility_edge_invocations = 0`,
  `direct_phase_entries = 4791`,
  `direct_compute_operations = 2623`,
  `direct_state_mutations = 2731`,
  `downstream_operand_productions = 2617`,
  `shadow_projections = 2509`.
- Ran: HBP parser latest-event payloads are field-identical:
  - `sim_year_index = 1`, `calendar_year = 2004`, `julian_day = 6`.
  - `duration_seconds = 0.0`.
  - `peak_runoff_m3_s = 0.0000000363`.
  - `total_detachment_kg = 0.60000000000000009`.
  - `total_deposition_kg = 0.0`.
  - `sediment_concentration_kg_m3 = [6.684785959735235]`.
  - `particle_flow_fraction = [1.0]`.

## Residuals

- PASS: none for the current H2637 5-day gate. PASS Parquet bytes match.
- WAT: none for the current H2637 5-day gate. WAT Parquet bytes match.
- HBP: none for the current H2637 5-day gate. HBP bytes and parsed
  latest-event fields match.
- Manifest: direct manifest reports direct publication provenance and
  `compatibility_edge_invocations = 0`.

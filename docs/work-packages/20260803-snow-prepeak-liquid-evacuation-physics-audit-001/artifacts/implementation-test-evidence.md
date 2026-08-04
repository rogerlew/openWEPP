# Implementation And Test Evidence

Status: `executed / characterization only`

Evidence mode: `Static + Ran`

## Ran

- `cargo nextest run -p openwepp-hillslope-orchestrator simimpl29_melt_hour_covers_zero_wind_rain_and_cap_paths`
  — `1 passed`; independently invoked by the CoE investigator.
- `cargo build --release -p openwepp-runner --bin openwepp-cli-hill` — PASS;
  accepted binary `8fb77e17...c673` built from source HEAD `073dafe3`.
- `tools/run_audit.py --execute` — 16 real-consumer direct-production cells,
  all return codes zero: a same-binary baseline plus three frozen operators by
  four sites. Exact
  argv, environment, input hashes, output hashes, and trace hashes are in
  `target/snow_prepeak_liquid_evacuation_physics_audit_v3/execution-receipt.json`.
- Independent streaming reconstruction over four accepted reference traces —
  61,364 daily and 1,472,736 hourly rows; maximum all-row daily mass residual
  `1.00e-12 m`, CoE term residual `2.02e-17 m`, routed alias residual
  `1.56e-17 m`, and Stage-3 energy residual `1.87e-8 J m^-2`.

## Static

- Current CoE equations and unit conversions match pinned legacy commit
  `dac3c950` at the term level.
- Runner/runtime consumers add routed melt exactly once and publish the carried
  authoritative SWE.
- Stage 3 executes after CoE state loss and is snow-neutral by implementation
  and contract.
- Density wet-compaction input duplicate-aliases pack loss; physical driver
  correctness remains an authority gap.

No production implementation, contract, test, fixture, or observation file was
edited. Terminal syntax, hash, protected-path, overwrite, JSON, Markdown, and
diff checks remain recorded in `gate-results.md`.

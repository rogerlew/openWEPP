# Contract Implementation Evidence

Status: corrected

Evidence mode: executed

Purpose: record canonical `SC-*` amendments or explicit no-change authority
findings before any production correction.

Static:

- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
  advanced to `contract_version: 146`.
- `SC-WATBAL-001` now maps canonical `I` to
  `hillslope_wat.Interception` / `hillslope_wat.Interception:mm` and states
  that WB13/WAT parquet publication exposes the daily interception flux in
  `mm`.
- Boundary/unit governance now contains required `Interception` aliases and an
  output registry entry for `hillslope_wat.Interception`.
- WAT schema docs and runner contracts now list `Interception` separately from
  `InterceptionStorage`.
- No process-physics equation, snow magnitude, ET partition, percolation, or
  runoff branch contract was changed.

Ran:

- Static contract conformance tests were run through targeted and workspace
  cargo gates; see `contract-test-implementation-evidence.md` and
  `gate-results.md`.

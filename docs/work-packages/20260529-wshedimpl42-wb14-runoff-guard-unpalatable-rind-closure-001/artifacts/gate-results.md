# WSHEDIMPL42 Gate Results

Status: completed  
Evidence mode: Ran

## Required Validation Gates
- `cargo fmt --check` -> pass
- `cargo clippy --workspace --all-targets -- -D warnings` -> pass
- `cargo test --workspace` -> pass
- `cargo deny check` -> pass (warnings only; no failing advisories/bans/licenses/sources)

## Runtime Closure Gates
- Hillslope cohort rerun (`/tmp/wshedimpl42_unpalatable_20260529T143937Z`) ->
  pass (`39/39`).
- Watershed rerun using `pw0.imp` -> fail:
  - `CLIWAT-E-010`
  - `IMP-E-004` (`jpond=0`, expected `>= 1`)
- Watershed rerun with minimal valid impoundment fixture -> fail:
  - `CLIWAT-E-017`
  - `HBP-E-002: bad magic` for `H1.hbp`

## Gate Verdict
- WB14 closure gate: pass
- Watershed parquet closure gate: fail (follow-on required)

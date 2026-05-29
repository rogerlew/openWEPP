# WSHEDIMPL42 Implementation and Test Evidence

Status: completed  
Evidence mode: Static + Ran

## Production Implementation
- File: `crates/openwepp-runner/src/hillslope/mod.rs`
- Change:
  - In `seed_wb11_runtime_surface_inputs`, breakpoint mode now prefers
    `nbrkpt` when present (authoritative current-day breakpoint cardinality),
    rather than stale `ninten`.

## Regression Test
- Added:
  - `wshedimpl42_breakpoint_seed_uses_current_nbrkpt_not_stale_ninten`
    (`crates/openwepp-runner/src/hillslope/mod.rs`)
- Ran:
  - `cargo test -p openwepp-runner wshedimpl42_breakpoint_seed_uses_current_nbrkpt_not_stale_ninten`
- Result:
  - pass.

## Runtime Closure Execution (Unpalatable-Rind)
- Pre-fix reference evidence:
  - `/tmp/wshed_parity_probe_20260529T044701Z/hillslope_batch_status.tsv`
  - All hillslopes showed `HKERNEL-WB14-RUNOFF-E-003`.
- Post-fix batch rerun:
  - Root: `/tmp/wshedimpl42_unpalatable_20260529T143937Z`
  - `39/39` hillslopes pass (`hillslope_batch_status.tsv` shows all `rc=0`).
  - Output set includes `H1..H39` pass/loss files.

## Watershed Closure Attempts
1. Initial watershed rerun:
   - Command class:
     - `cargo run -p openwepp-runner --bin openwepp-cli-watershed ...`
   - Result:
     - fail: `CLIWAT-E-010` (`IMP-E-004`, `jpond=0` invalid in `pw0.imp`).

2. Retry with contract-valid minimal impoundment file:
   - Rebound `pw0_imp` to `pw0_openwepp.imp` (from
     `tests/fixtures/infile/watershed_impoundment/strict_valid_minimal.imp`).
   - Result:
     - fail: `CLIWAT-E-017` / `HBP-E-002: bad magic` while parsing
       `H1.hbp`.

3. Format inspection of hillslope pass artifact:
   - `file /tmp/wshedimpl42_unpalatable_20260529T143937Z/hillslope_output/H1.hbp`
   - Result:
     - `ASCII text` (daily pass text), not binary HBP shard.

## Outcome
- WB14 blocker closed.
- Full watershed parquet closure condition remains blocked by follow-on gaps:
  impoundment `jpond=0` acceptance path and hillslope pass-file format mismatch.

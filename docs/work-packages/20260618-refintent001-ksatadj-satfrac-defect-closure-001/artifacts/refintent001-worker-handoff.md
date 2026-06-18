# REFINTENT001 Worker Handoff

Evidence class: Static + Ran

Status: complete.

## What changed

- WB14 `ksatadj` now loads the source-intent operands `por`, `cpm`, `thetfc`,
  and `thetdr` for the top two tillage layers.
- `sat_frac` now uses `avsat/(avpor*avcpm)` with the two source-intent caps.
- The old `theta_sum/ul_sum` surrogate and FC/WP reconstruction fallback are
  removed from the active path.
- `cpm` is registered as a hot state root for scheduler surface handling.
- Non-aliased unit tests and the WB14 integration oracle cover the new formula.

## Evidence to trust

- Focused WB14 tests passed.
- Full workspace tests passed.
- `cargo fmt --check`, clippy `-D warnings`, release build, and `cargo deny`
  passed.
- H2637 without UI and with UI exited 0.
- OFE1-OFE5 ladder exited 0.

## Notes for the next worker

- Do not reopen the FARPOINT01 71% flag solely because the H2637 percentage did
  not change. The package closes it by `INV-SUBHYD-032` conformance and closure.
- The legacy 55.5% result remains a comparator flag, not an implementation
  target.
- The initial OFE ladder setup failure was a relative-path invocation mistake,
  not a kernel failure. The absolute-path rerun passed.
- No contract amendment was made or required.

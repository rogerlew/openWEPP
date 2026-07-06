# Terminal-Bin Correction

Status: **EXECUTED-COMPLETE**.

Evidence mode: Static + Ran.

## Authority

Static:

- `SC-OFEROUTE-001` rev 26 authorizes conservative per-bin outflow handoff,
  non-negative published outlet bins, and exact-total preservation.
- The seam source-rate sampler consumes exactly 24 hourly source bins and
  returns zero outside the day window.
- The existing shadow runtime already intended a `+6 h` drain tail after the
  last active source hour; the defect was the one-day cap that erased that tail
  when the last active source was hour 24.

## Correction

Implemented in `crates/openwepp-runner/src/hillslope/laned_shadow.rs`:

- Replaced the one-day routing-window cap with a source-window plus drain-tail
  policy:
  - source window remains `24 h`;
  - routing window is `min(active_end, 24 h) + 6 h`;
  - source and rainfall-intensity rates remain zero after 24 h through
    `seam_rate_at`.
- Added the focused regression
  `routing_window_keeps_drain_tail_for_hour_24_source`.

Ran:

```sh
cargo nextest run -p openwepp-runner routing_window_keeps_drain_tail_for_hour_24_source --no-capture
```

Result: **PASS**, `1` passed / `117` skipped.

Ran:

```sh
cargo nextest run --test laned_shadow_h2637 --run-ignored ignored-only --no-capture
```

Result after correction: **PASS**, `1` passed / `1` skipped, `484.578 s`.
This run includes the real native-patched H2637 shadow path that previously
failed at day 88 with `NegativeOutletBin`.

Line-count governance: `crates/openwepp-runner/src/hillslope/laned_shadow.rs`
is `704` lines after the edit, below the 2000-line WARN threshold.

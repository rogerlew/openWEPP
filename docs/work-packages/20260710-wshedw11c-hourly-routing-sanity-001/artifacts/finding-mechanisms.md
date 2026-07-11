# Finding Mechanisms

Status: `EXECUTED-CURRENT`

Evidence mode: `Static + Ran`

## W11C-F001: Negative routed storage

Ran: early pulse cases produce material negative published storage for
`ipeak=3` and `ipeak=4` at both tested timesteps.

Static: `kernel/hourly.rs` computes channel storage as the unrestricted daily
flux difference `channel_inflow_m3 - channel_outflow_m3`. Its per-interval
ledger uses `(qin + qlat - q1) * dtchr`. `ws11_grid_end_disposition` checks only
finiteness and accepts negative totals. The negative storage therefore directly
accounts for terminal output exceeding external input; it is not an independent
rounding residual.

The fresh first-day fixture has zero initial channel storage, zero baseflow,
zero transmission loss, one external source, and no impoundment. A negative
end store therefore cannot be a physical stored-water magnitude; observed
minima of `-65.192021 m3` (KW) and `-210.400475 m3` (static MC) are material,
not roundoff. `SC-ROUTE-001` does not yet state a dedicated interval-storage
lower-bound guard; W11D must confirm/amend that authority against pinned
`sinit`/`sfnl`/`chvol` lineage before production correction.

## W11C-F002: Peak amplification and timestep sensitivity

Ran: the corrected sidecar changes normalized routing grids from 24 intervals
at 3,600 seconds to 144 intervals at 600 seconds. Static MC reaches peak/input
ratios `1.126831` and `1.152433`; variable MC reaches `1.535760` and
`1.418897`. Variable-MC early-spike peak changes from `1.185839` to
`3.071519 m3/s` across the two grids.

Static: there is one nonzero external contributor, zero baseflow, no
impoundment, and no initial pulse storage in the fixture. These results are
defect-shaped passive-routing anomalies, but W11C does not adjudicate or alter
the pinned KW/MC recurrence.

## W11C-F003: Legacy event publication is not terminal

Ran: `ipeak=2` publishes 14,400 m3 for a 7,200 m3 pulse, `element_id=1`, and
rate-like tiny sediment values.

Static: `network_frame.rs` selects every dispatched channel when no interval
publication is active, sums every channel's `runoff_volume_m3` and
`sediment_yield_kg`, and uses the first channel ID. For serial channels this is
throughflow summation, not terminal-outlet publication.

## W11C-F004: Prior timestep evidence aliased through compatibility

Ran: parsing the old W11B three-line `nchnum=0` sidecar returned normalized
`dtchr=60`, not the written 600. The corrected W11C four-line `nchnum=2`
sidecar parses and executes 3,600/24 and 600/144 grids distinctly.

Static: `chaninp.rs` discards empty lines, requires four nonempty records, and
compatibility-defaults parse/count failures. An empty channel-ID record cannot
survive that collector, so the old three-line fixture necessarily defaulted.

These mechanisms are sufficient to shape a follow-up defect-closure package;
they are not production corrections.

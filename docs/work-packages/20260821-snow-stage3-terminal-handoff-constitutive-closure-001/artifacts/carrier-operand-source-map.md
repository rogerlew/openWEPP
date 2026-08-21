# Carrier operand source map

Status: `AUTHORITY PRESENT / LIVE-CONSUMER DERIVATION BLOCKED`.

`Static:` Child 2C's typed equations and validation live in
`snow_stage3_terminal_handoff.rs::evaluate_shared_carrier`. It requires the
sealed exposure, actual reference/canopy/snow surfaces, sorted participant
support receipts, reciprocal longwave, and independent heat/vapor closure.

`Static:` The new attachment does not yet derive those operands from its
committed V11/LSE and Stage-3 owners, nor does it call
`evaluate_shared_carrier`. Its prepared support carries a sealed
`DirectV9ShadowIntervalInput` for the released consumer boundary, which is
insufficient to authorize a covered carrier. The old day-frame carrier
derivation remains a historical/test-only compatibility path and cannot close
this package.

`Static:` The first lift must add a typed covered executor whose carrier
operands are derived from the accepted forcing and actual staged owners; no
configured live temperature, humidity, conductance, snow mass, or ledger may
be promoted to authority.

# Consumer Path Evidence

Status: `EXECUTED-PASS-FOCUSED`

Evidence mode: `Static + Ran` direct consumer and CLI integration tests.

## Real producer-to-consumer path

1. HBP minor-1 hourly water and sediment arrays enter
   `WatershedNetworkFrame` as typed hillslope contributions.
2. `ws11_project_hourly_totals` projects each 24-slot integrated array onto the
   exact covering `dtchr/ntchr` grid using interval/hour overlap.
3. `ws11_route_interval_water` publishes `qin`, whole-reach `qlat`, routed
   `q1`, and interval storage from the pinned segmented KW/MC recurrence. It
   applies the `mofapp=1` lateral average, branch-specific reference flow,
   signed interior MC state, and the shared outlet epsilon boundary. It routes
   zero-input tails and seeds the next day from prior terminal `q1`.
   `ws11_route_interval_sediment` publishes the same-index class ledger and
   carried geometry.
4. For a dependency channel, water intake is its typed `q1_m3_s` vector and
   sediment intake is each upstream ledger's `egress_kg` vector at the identical
   interval. Missing, mixed-authority, grid-mismatched, or class-mismatched
   dependencies fail closed.
5. `RoutedChannelState` carries both interval states through dispatch.
   `WatershedNetworkFrame::build_watershed_publication` selects terminal channel
   IDs for active interval publication. The runner then writes the normal EBE
   parquet fields from that publication.
6. For multi-class channels, the CLI reads required `pw0.sol`, derives the
   surface `prtcmp` composition through the existing migrated implementation,
   and binds channel-indexed `crfrac` before dispatch. This is pinned
   `convrt.for` authority, not an inlet-sediment composition fallback.

## Negative old-path proof

`run_direct_channel_node` selects the interval lane immediately after reading
the channel context. An activated call returns from
`run_direct_interval_channel_node` before scalar hydrology, event peak
partition, and the legacy scalar WS20 call. The active lane passes explicit
interval hydraulic operands and `t_exp=t_norm=dtchr`; it passes a deliberately
hourly-resolved dummy peak partition whose event scalar fields are all zero.
The active public sediment yield is the sum of interval egress kilograms, not
the legacy `qsed_kg_s` field. Non-activated branches still enter the old event
path and publish no interval state.

## Ran consumer evidence

`wshedw11b_two_channel_direct_consumer_reads_same_grid_class_egress` passed and
asserted vector equality for downstream `qin == upstream q1` and, for every
interval, downstream class inlet `==` upstream class egress. The dynamically
built production runner fixture contains two serial channels and two HBP files;
`cargo nextest run -p openwepp-runner --test
mt3_hbp_hourly_consumer_contract` passed both the protected one-channel case and
the W11B two-channel same-total/different-shape case. Release-binary identity and
numeric output values are recorded separately after the delegated release run.

The protected P102 five-class CLI test passed after consuming soil-derived
`crfrac`; its routed yield differs from the raw detachment-minus-deposition
alias and is identical across `--jobs 1/4`.

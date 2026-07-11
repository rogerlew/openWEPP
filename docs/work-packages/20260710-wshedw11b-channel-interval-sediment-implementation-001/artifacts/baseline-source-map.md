# Baseline Source Map

Status: `EXECUTED`

Evidence mode: `Static` source mapping against pinned baseline SHA
`dac3c950d8b16cc73774bf5ce2e7e11f80baac70`; prior W11 revision/hash commands
are incorporated as historical `Ran` evidence, not relabeled as current runs.

| Mechanism | Pinned source | W11B implementation binding |
|---|---|---|
| Normalized water grid | `wshinp.for:463-495`, `pmxchr.inc` | `ntchr * dtchr = 86400 s`; `q1(1..ntchr)` is the routed outlet grid. |
| HBP projection precursor | `chrqin.for:82-170` plus ADR-0036 D2 | Replace scalar reconstruction with exact overlap of hour-integrated `V_h/S_h` onto the same grid. |
| Dependency handoff | `wshchr.for:231-252` | Add upstream `q1(it)` and class egress directly to downstream same-index intake. |
| Kinematic water routing | `wshchr.for:395-469` | `ipeak = 3`, stateful interval routing. |
| Muskingum-Cunge water routing | `wshchr.for:473-615` | `ipeak >= 4`; only `ipeak = 5` refreshes dynamic coefficients. |
| Channel bed composition | `convrt.for:84-88`, `prtcmp.for` | Project required watershed `pw0.sol` surface texture through the existing Rust `prtcmp` port and bind channel-indexed fractions as `crfrac`. |
| Hydraulic profile | `chnrt.for:230-242` | Total `qlat` partitions effective length; per-length solve operand is `qlat_eff = qe/leff`. |
| Event sediment core | `chnrt.for:293-885`, `detach.for`, `case12.for`, `case34.for` | Reuse the complete migrated WS18-WS26 spatial solve once per active interval with interval operands. |
| Incision/contact clock | `dcap.for:154-215` | `di`, `timpot`, `timex`; low boundary shear with remaining depth re-enters incision at `timsh=timpot`. |
| Capped widening geometry | `dcap.for:225-261` | On cap, reconstruct `eros=dct*t_norm*wflow/rho_soil`, then derive new width/depth from that erosion. |
| Geometry carry/reseed | `wshdrv.for:1097-1114,1179-1189` | Carry geometry monotonically; only run start and primary tillage for `ishape=3` reseed. |

Static conclusion: baseline supplies the water grid, dependency topology, shared
segment solve, geometry transitions, and both GAP-014 corrections. V53 supplies
the explicitly labeled intervalization and closure rules where baseline
sediment remains event-scalar.

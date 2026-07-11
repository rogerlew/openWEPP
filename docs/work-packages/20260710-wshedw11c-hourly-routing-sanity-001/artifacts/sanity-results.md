# Sanity Results

Status: `SANITY-FAIL`

Evidence mode: `Ran`

## Debug Matrix

Ran:

```text
cargo nextest run -p openwepp-runner \
  --test mt3_hbp_hourly_consumer_contract \
  wshedw11c_hourly_routing_sanity_matrix --no-capture
```

Final debug run ID: `f695f3db-0627-4c28-8d97-8e5c5d023158`; one test passed
after 15.908 seconds and emitted all 35 case rows plus 12 timestep comparisons.

All wave cases used exactly 7,200 m3 and 240 kg except the zero controls. The
test reparsed both HBP files from disk for source totals. Routed-ledger
residuals were at roundoff scale, sediment yield was 240 kg to roundoff, and
all zero controls were exact zero. The water identity is algebraic because
production derives storage from routed inflow/outflow; it is not a physical
pass when its storage operand is negative.

### Representative wave results

| `ipeak` | `dtchr` | Scenario | Outlet m3 | Storage m3 | Peak m3/s | Peak/input |
|---:|---:|---|---:|---:|---:|---:|
| 3 | 3600 | early spike | 7265.192021 | -65.192021 | 0.999952 | 0.499976 |
| 3 | 600 | early spike | 7261.723300 | -61.723300 | 1.999994 | 0.999997 |
| 3 | 3600 | late spike | 3572.784836 | 3627.215164 | 0.992440 | 0.496220 |
| 3 | 600 | late spike | 6490.734902 | 709.265098 | 1.999994 | 0.999997 |
| 4 | 3600 | early spike | 7410.400475 | -210.400475 | 1.939039 | 0.969520 |
| 4 | 600 | early spike | 7271.305388 | -71.305388 | 2.253662 | 1.126831 |
| 4 | 3600 | early spread | 7207.734082 | -7.734082 | 0.519821 | 1.039642 |
| 4 | 600 | early spread | 7227.628853 | -27.628853 | 0.576217 | 1.152433 |
| 5 | 3600 | early spike | 7178.763628 | 21.236372 | 1.185839 | 0.592919 |
| 5 | 600 | early spike | 6838.490642 | 361.509358 | 3.071519 | 1.535760 |
| 5 | 3600 | early spread | 7136.576167 | 63.423833 | 0.774940 | 1.549880 |
| 5 | 600 | early spread | 7071.631220 | 128.368780 | 0.709448 | 1.418897 |

Uniform forcing produced the input peak (`0.0833333333 m3/s`) and near-zero
storage for all branches/timesteps. Late pulses retained more end-of-grid
storage than early pulses. Broader pulses had lower peaks than spikes within
each branch/timestep.

### Legacy CREAMS control

`ipeak=2` executed, and shifted early/late pulses were identical as expected
for event-scalar routing. It did not publish a terminal network surface:

- 7,200 m3 external input published as 14,400 m3, the sum of serial channel
  throughflows;
- `element_id=1`, not outlet channel 2;
- 240 kg input published as `0.133333 kg` for spike, `0.033333 kg` for spread,
  and `0.005556 kg` for uniform forcing.

Therefore legacy daily/event-scalar execution is enabled, but this two-channel
publication cannot be treated as terminal volume or sediment yield.

## Findings

| ID | Severity | Finding | Evidence |
|---|---|---|---|
| W11C-F001 | High | KW and static MC generate material negative ending storage and terminal volume greater than external input for early pulses. | minima: `-65.192021 m3` KW and `-210.400475 m3` static MC |
| W11C-F002 | High | Static and variable MC can amplify passive-route peak above the only external input peak, with large timestep sensitivity. | maximum ratio `1.549880`; variable-MC spike changes `1.185839 -> 3.071519 m3/s` from 3600 to 600 s |
| W11C-F003 | High | Legacy CREAMS multi-channel publication sums serial throughflow and does not publish terminal sediment. | 7,200 m3 -> 14,400 m3, element 1, spike sediment `0.133333 kg` |
| W11C-F004 | High test-evidence defect | The prior W11B `nchnum=0` three-line sidecar collapsed to compatibility `dtchr=60`; its claimed written `600 s` was not the executed timestep. | W11C parser assertion initially observed `dtchr_norm_s=60`; corrected fixture proves 3600/600 separately |

Classification: `SANITY-FAIL`. Executability, exact zero behavior, algebraic
water closure, and wave sediment closure pass. Material nonnegative-storage,
passive peak behavior, and legacy terminal-publication requirements do not.

## Release Reproduction

Ran: exact final-tree release binary matrix run ID
`29024159-9f78-4506-9918-09c7f007af0d` passed `1/1` in 2.836 seconds. It emitted
35 result, 33 finding, and 12 timestep rows. Finding counts were:

- material negative storage: 8;
- terminal outflow greater than external input: 12 (eight wave cases plus four
  legacy serial-throughflow publications);
- legacy first-channel publication: 5;
- legacy serial-throughflow volume: 4;
- sediment publication delta: 4 legacy cases.

Release results reproduce the debug values and `SANITY-FAIL` classification.

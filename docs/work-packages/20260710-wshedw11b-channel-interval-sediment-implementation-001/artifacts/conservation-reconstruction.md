# Conservation Reconstruction

Status: `EXECUTED-PASS-FOCUSED`

Evidence mode: `Static + Ran` independent test-side reconstruction.

## Water reconstruction

For each routed channel, the direct-consumer test reads the published typed
vectors and independently computes
`sum_i((qin_i + qlat_i - q1_i) * dtchr)`. It compares that operand reconstruction
with the sum of published per-interval storage and separately checks
`channel_inflow_m3 - channel_outflow_m3 == channel_storage_m3`. This does not
call the producer's closure validator.

The projector vector independently checks `1.25 + 2.75 = 4.0 m3` over a
quarter-hour grid: the first four slots each contain `0.3125 m3` and the next
four each contain `0.6875 m3`. Thus the proof covers magnitudes, not only a
dimensionless or one-sided bound.

The production two-channel CLI fixture independently reads HBP, EBE, and
channel-water-balance parquet outputs. Both equal-total timing shapes use
`7,200 m3` external runoff, require terminal EBE outflow not to exceed that
source, require nonnegative first-day residual storage, and reconstruct
`external = terminal egress + network storage` within `1e-9` relative. The
published balance column must be zero, while spike and spread terminal peaks
remain distinct.

Current focused CLI values are:

| Shape | External (m3) | Terminal egress (m3) | Storage (m3) | Peak (m3/s) | Sediment (kg) |
|---|---:|---:|---:|---:|---:|
| spike | 7,200 | 7,088.171478291323 | 111.82852170867682 | 2.1122146208271415 | 240 |
| spread | 7,200 | 7,160.979461604386 | 39.020538395613585 | 0.5002525682549819 | 240 |

The reconstruction residual is below `1e-12 m3` in both rows.

## Sediment reconstruction

For every channel, interval, and class, the consumer test independently reads
the five published operands and checks
`inlet + lateral + detached == egress + deposited`. It then verifies that the
public daily yield equals the sum of published daily class egress.

For the two-channel network it excludes the internal upstream egress from the
external-input total and reconstructs
`upstream lateral + downstream lateral + all detached == all deposited +
terminal downstream egress`. The test also requires positive terminal egress.
This is the real downstream consumer path: downstream inlet vectors are first
proven equal to upstream same-index egress, then network closure is rebuilt from
the typed channel states.

The producer's detachment operand is no longer a signed closure residual. For
Case III, IV, and II-transition spans it integrates the DCAP class rates over
the relevant triangular/trapezoidal segment extent. Deposition is then solved
as `incoming + lateral + constructive detachment - outgoing`; a materially
negative result is a typed failure. The production geometry-mutation vector
requires positive published constructive detachment, while the two corrected
DCAP terminal tests reconstruct the class-summed rate independently from the
resulting depth/width geometry and the pinned `96 lbm/ft3` density.

The two-channel wave-routing vector independently uses the sole external
`864 m3` baseflow and checks `external = terminal outflow + storage_upstream +
storage_downstream`; downstream same-grid `qlat` is exactly zero because the
upstream baseflow is already present in dependency `q1`.

## Boundary dispositions

- Zero routed flow deposits every incoming class mass, emits zero egress and
  detachment, and leaves geometry unchanged.
- Grid-end water storage remains a water ledger; suspended sediment carry is
  explicitly empty.
- Daily projection closure compares the sum of interval lateral class masses to
  the independently retained 24-hour class source totals.
- Geometry mass is independently reconstructed in vector 10(b)/(c) from depth,
  width, length and density operands; the capped terminal is not accepted from
  width self-consistency alone.

All focused reconstruction assertions passed in the current 105-test
orchestrator run and 2-test runner suite.
The release CLI magnitude and shape audit is recorded after the delegated
release run.

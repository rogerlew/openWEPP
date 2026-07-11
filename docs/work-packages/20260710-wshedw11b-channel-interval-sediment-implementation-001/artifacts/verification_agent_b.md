# Verification Agent B

Status: `EXECUTED-PASS`

Evidence mode: `Static + Ran` on the frozen 2026-07-10 tree.

## Verdict

`PASS`. The real two-channel release CLI has physically bounded water
magnitude, positive residual storage, exact external water closure, terminal
sediment mass closure, and timing sensitivity. Typed-state tests independently
close every channel/interval/class ledger and prove same-index dependency
handoff. Pinned `wshchr` spatial routing, soil-to-`prtcmp`-to-`crfrac` lineage,
the protected P102 path, and negative old-path use all pass review. No
Verification B science, consumer, conservation, or protected-boundary blocker
remains.

## Independent release-output reconstruction

The delegated final build was
`cargo build --release -p openwepp-runner --bins`. The exact production binary
used below is `target/release/openwepp-cli-watershed`, SHA-256
`0e0ce234d1807dc64f01ac7a579541e72f0c3c08d2c416c39e54df021b1b8357`,
size 9,336,832 bytes, mtime `2026-07-10 19:46:55 -0700`. Verification B read
the resulting parquet files independently with PyArrow; it did not invoke the
producer's closure validator.

| Case | Fresh release output | External HBP water | Terminal water | Network storage | External residual | Peak | Terminal sediment |
|---|---|---:|---:|---:|---:|---:|---:|
| spike | `/tmp/w11b_cli_spike_1783738025745325877/out_release` | 7,200 m3 | 7,088.171478291323 m3 | 111.828521708677 m3 | 0.0 m3 | 2.112214620827 m3/s | 240.0 kg |
| spread | `/tmp/w11b_cli_spread_1783738026376507618/out_release` | 7,200 m3 | 7,160.979461604386 m3 | 39.020538395614 m3 | 0.0 m3 | 0.500252568255 m3/s | 240.0 kg |

The two-sided magnitude audit rejects the former amplification defect. Terminal
water is 98.4468% and 99.4580% of external input; positive stored water is
1.5532% and 0.5420%, respectively. Thus `terminal + storage = 7,200 m3`
exactly in both cases. The independently reconstructed aggregate channel
residuals, `Inflow - Outflow - Storage`, are `-9.095e-13 m3` and
`+9.095e-13 m3`; published balances are the same. Aggregate channel inflow and
outflow contain the internal channel-to-channel transfer and are therefore not
mistaken for external network operands.

Both outputs identify terminal channel/element 2. Their peak difference is
`1.611962052572 m3/s`, proving same-total/different-shape water sensitivity.
For sediment, the external HBP total and terminal EBE/`sed_del` are exactly
240.0 kg in both cases; the independent residual is 0.0 kg.
`totalwatsed3` reports `tdet = 240.00000000000003 kg`, `tdep = 0.0 kg`, and
`sed_del = 240.0 kg`, aligned with this non-eroding fixture.

## Same-grid sediment and mass closure

Static producer-to-consumer lineage is direct:

1. `hourly.rs:1008-1117` projects each local 24-hour class source to the
   normalized grid and copies every upstream interval's `egress_kg` into the
   downstream inlet at the same index.
2. `hourly.rs:815-959` invokes the complete segment core once per active water
   interval with explicit interval water operands and `t_exp = t_norm =
   dtchr`, then publishes inlet, lateral, constructive detachment, deposition,
   and egress by class.
3. `hourly_tests.rs:261-418` proves downstream `qin == upstream q1`, downstream
   class inlet equals upstream class egress for every index, independently
   reconstructs every class ledger as `inlet + lateral + detached = egress +
   deposited`, and closes the two-channel external network without counting
   internal egress twice.

The final orchestrator suite passed 105/105, the W11B/ENDDET selector passed
23/23, and the typed integration suite passed 18/18. The M-T3 timing test also
requires equal daily publication for equal sediment totals while the interval
egress vectors differ. The exact zero-source continuity correction is bounded:
all segment branches first guarantee finite/nonnegative outgoing state, and
only an exact zero inlet, zero lateral, zero detachment triple is forced to
zero egress; nonzero negative-deposition residuals remain typed failures.

## Pinned `wshchr` fidelity

Static comparison used pinned baseline commit
`dac3c950d8b16cc73774bf5ce2e7e11f80baac70` and
`wshchr.for` SHA-256
`608dbb02e1759248ab15abec9a6827abd0756db7f29fcbe8c8077bebc0c702a3`.
The final Rust route matches the relevant pinned sequence:

- KW uses `qref = qtmax`; static and variable MC use
  `qref = 0.5 * (qtmin + qtmax)`, with zero-initialized `qtmin` represented by
  `0.5 * qtmax` (`wshchr.for:326-328`; `hourly.rs:325-331`).
- Celerity determines a 1..101 segment grid; prior spatial state is initialized
  by linear inlet-to-outlet interpolation (`wshchr.for:369-389`;
  `hourly.rs:349-374`).
- Lateral flow uses the adjacent-time average and reach-length normalization;
  KW, static MC, and MVPMC3 consume current-upstream, prior-upstream, and
  prior-local spatial states in pinned order (`wshchr.for:395-571`;
  `hourly.rs:393-491`).
- Finite signed interior MC `qs` remains intact. The `1e-8 m3/s` floor is
  applied only to published KW/MC outlet `q1`, and the pinned `qmaxi/qlavg`
  zero-update gate resets the spatial state when prior inlet, prior published
  outlet, current inlet, and averaged lateral flow are all zero. The
  source-level anti-evasion test binds all four operands, zero publication,
  spatial reset, and the pre-segment `continue`
  (`hourly_tests.rs:497-541`).
- Prior-day terminal `q1/qin/qlat` is finite/nonnegative validated and seeds
  the next covering grid.

These checks close the earlier reference-flow and epsilon-placement findings;
they do not rely on comparator agreement alone.

## Soil composition lineage

For multi-class production input, the runner parses the required watershed
soil, selects each channel-indexed surface layer, converts sand/clay/organic
matter percentages to fractions, derives the five canonical particle classes
with the Rust port of pinned `prtcmp.for`, and assigns their fractions to the
channel control (`openwepp-cli-watershed.rs:473-479,536-600`). This matches
`convrt.for:84-88`, which maps channel-indexed `frac(k,ichan)` directly to
`crfrac(k,ich(ichan))`. More than five classes, missing OFEs/layers, parse
errors, invalid texture, or missing requested class authority fail explicitly.
A one-class system uses the unique normalized composition `[1]`; it is not a
multi-class inference. The protected P102 five-class production fixture passed
on the frozen tree (nextest run `5df3b651-f7d5-4028-9ccc-26c8a3f8bd73`, 1/1).

## Negative old-path and protected-boundary proof

`direct.rs:174-202` returns from `run_direct_interval_channel_node` immediately
when the activation predicate is true, before scalar hydrology, event peak
partition, and the event-scalar WS20 call. The interval caller supplies
explicit hydraulic operands and a deliberately zeroed hourly-resolved dummy
peak partition. Active daily yield is the sum of interval class egress.

The public CLI old-runtime anti-evasion test passed and requires the typed
network/dispatch/publication path while rejecting legacy writeback/symbol-map
markers. Non-activated/minor-0 typed contributors still take the event lane and
publish no interval state; the final typed suite covers `ipeak` branches 1..5.
Mixed, malformed, partial dependency, and impoundment-ineligible authority
remain fail-closed. The protected P102 jobs-1/jobs-4 identity and nonzero
five-class sediment assertions pass.

## Comparator posture

Confidence-tier disposition is conservative:

- High confidence: equation/order/source comparison against the pinned
  `wshchr`, `chrqin`, `chnrt`, `dcap`, `detach`, `case12`, `case34`, and
  `enddet` files; their hashes are recorded in
  `artifacts/logs/legacy-source-provenance.log`.
- High confidence: independent release-output magnitude and conservation
  reconstruction above, plus exact contract-derived vectors.
- Not treated as like-for-like numeric authority: the W11B HBP minor-1
  same-grid interval sediment sequence is a canonical v53 refinement not
  represented by the pinned event-scalar executable input/output surface.
  Its expected timing divergence cannot be graded as a legacy regression.

No comparator agreement is used as an acceptance target, and no observed
delta contradicts pinned source or canonical v53 authority.

## Gates and eligibility

Ran on the frozen tree:

| Gate | Result |
|---|---|
| `cargo fmt --check` | PASS |
| `cargo clippy --workspace --all-targets -- -D warnings` | PASS |
| `cargo nextest run --workspace --profile full` | PASS, 1,677/1,677; 4 slow; 3 skipped; 591.594 s |
| `cargo nextest run --workspace --profile erosion` | PASS, 312/312; 3 slow; 1,368 filtered/skipped |
| `cargo deny check` | PASS |
| `markdown-doc lint --path docs/work-packages/20260710-wshedw11b-channel-interval-sediment-implementation-001 --format json` | PASS, 32 files; 0 errors; 0 warnings |
| `git diff --check` | PASS |

No touched/new Rust file reaches 3,000 lines. The pre-existing 2,000+ runner
CLI, runner behavior-test, and `direct.rs` owners have explicit WARN/no-block
dispositions and bounded W11B changes.

The release provenance, comparator disposition, and gate summary artifacts are
now populated from the current logs. Verification B finds no failed, blocked,
or unjustifiably not-run current-scope gate and considers the package eligible
for `EXECUTED-COMPLETE` disposition.

# Reproduction and Attribution

Status: `EXECUTED`

Evidence mode: `Static + Ran`

## Fresh Staging and Reproduction

The package restaged H2637 from the canonical wepp-forest WB05A source at
`/home/workdir/wepp-forest/docs/ablation/20260430_uncapped-spectacular_h2637_hillslope_closure-spike/artifacts/repro/source_wepp1/runs/`.
The source management file was byte-identical to the committed two-year H2637
fixture before applying the existing Tier-1 native-management patch.

Key staged hashes:

| Input | SHA-256 |
| --- | --- |
| `p2637.cli` | `f5e6f06c13988761cc823e0ad7a96e9ad1bf4953be5146b845be086627def9dc` |
| patched `p2637.man` | `dd25fd510cb3a77238fc2ad9b5f9abb254296caca8695caab75540e78a1a3cb0` |
| `p2637.slp` | `4017740811aa3ac3215383150e94ac08aa7b9ddba08764a177d62a94cb0aa815` |
| `p2637.sol` | `9508cb02263457c5b82cd5168b5efbc95dcda257537019accddb4e17dfcdd3d7` |
| `pmetpara.txt` | `f6d58adf7df64c9b1ef6cfc61f4617bc57cbe6150f7654b48d3943be261f9aba` |

The exact release target was rebuilt with
`cargo build --release -p openwepp-runner --bin openwepp-cli-hill`.
The pre-correction binary was `target/release/openwepp-cli-hill`, size
`10687664` bytes, SHA-256
`2fd65b10c60c2e43354e5675452247b092a7e031ae9771bea53bd11a2e195630`.
That hash matches the discovery-audit binary because intervening commits did
not relink the hillslope target.

Ran from `/home/workdir/openWEPP`:

    env -u OPENWEPP_LANED_ACTIVE -u OPENWEPP_LANED_ACTIVE_DISABLE \
      -u OPENWEPP_LANED_SHADOW -u OPENWEPP_LANED_ACTIVE_TRACE \
      -u OPENWEPP_LANED_ACTIVE_TRACE_DETAIL \
      -u OPENWEPP_LANED_ACTIVE_STEP_TRACE \
      taskset -c 4 /usr/bin/time -v \
      target/release/openwepp-cli-hill \
      --run-dir /tmp/openwepp_laned_nob_001_pre_9fa0a294/runs \
      --run-file p2637.native.false.run.toml \
      --output-dir /tmp/openwepp_laned_nob_001_pre_9fa0a294/output \
      --policy compat --legacy-sidecar-discovery

Result: exit `1`, user `48.35 s`, wall `48.39 s`, max RSS `47348 KiB`:

    CLIHILL-E-011 ... direct runtime day execution failed at lane 8 day 2621:
    ... lane 8 day 2621 routing failed: NegativeOutletBin

This independently reproduces `LANED-NOB-001` on current `main@9fa0a294`.

## Localized State Capture

The existing row-scoped active trace selectors were used with temporary,
package-local error-path printing at the already-retained
`NegativeOutletBin` guard. The diagnostic source block was removed immediately
after the run; no diagnostic instrumentation remains in the production diff.
Diagnostic binary SHA-256:
`9e896c08626f6f9af8c44a2a47658f42728b61b85102c04c8b7a40b275d955b0`.

Selector:

    OPENWEPP_LANED_ACTIVE=1
    OPENWEPP_LANED_ACTIVE_TRACE=1
    OPENWEPP_LANED_ACTIVE_TRACE_DETAIL=2621:8
    OPENWEPP_LANED_ACTIVE_STEP_TRACE=1

At lane 8/day 2621, the failing OFE has:

| Operand | Captured value |
| --- | ---: |
| local lateral source | `0.0 m²` |
| upstream inflow | `2.3671499562305767e-3 m²` |
| booked outlet outflow | `-3.2103056801436887e-5 m²` |
| storage change | `2.3992530130320148e-3 m²` |
| positivity-clamp injection | `0.0 m²` |
| terminal outlet-bin deficit | `-3.2103056801436893e-5 m²` |
| nonnegative outlet-bin sum after forward redistribution | `0.0 m²` |

The step trace shows eight consecutive negative predictor outlet faces over
`t = 105600..108000 s`; the corrector face remains zero or small positive.
Representative terminal rows:

| Step | `pred_out_face` (m²/s) | `corr_out_face` (m²/s) | booked step outflow (m²) | predictor limiter reductions |
| ---: | ---: | ---: | ---: | ---: |
| 352 | `-1.6296614167408843e-13` | `0.0` | `-2.4444921251113263e-11` | `0` |
| 356 | `-1.1047181701272030e-8` | `1.3335287997910260e-13` | `-1.6570572522588077e-6` | `0` |
| 359 | `-1.0522068720127724e-7` | `2.4912887562743840e-9` | `-1.5409409766750426e-5` | `0` |

For step 359, the pre-step committed outlet discharge is the prior record's
`q[n-1] = 2.5439635736195367e-10 m²/s`. The predictor boundary formula
`2 q[n-1] - q[n-2]` and the captured face imply
`q[n-2] = 1.0572947991600115e-7 m²/s`, about `415.61` times the outlet-cell
discharge. The donor extrapolation therefore generates the negative face even
though every physical forcing and committed discharge is nonnegative.

## Named Mechanism and Ownership

Named mechanism: **unbounded negative predictor outlet-face extrapolation on a
source-quiet dry-front/recession span**.

`KinematicWaveSolver::step` constructs the predictor outlet face as
`2 q[n-1] - q[n-2]`. Rev-41's conservative stage limiter enforces only the
upper available-water bound. It does not enforce the physical lower bound of
zero, so the negative face is admitted, booked as negative outflow, and
artificially increases mesh storage. `BinRecorder` correctly refuses to
publish it: forward deficit carry has no later positive bin to absorb the
terminal deficit.

Ownership is `crates/openwepp-hillslope-orchestrator/src/ofe_routing/kinematic_wave.rs`
inside the declared envelope. The finite nonnegative upstream handoff and zero
local source rule out snow/winter production and seam booking. The typed guard
is correct and must remain; the correction belongs in the conservative stage
flux construction so the invalid negative boundary flux is never created.

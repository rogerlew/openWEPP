# Implementation and Test Evidence

Status: `EXECUTED-PASS-FOCUSED`

Evidence mode: `Static + Ran` on 2026-07-10.

## Before/after contract evidence

Before production, `cargo nextest run -p openwepp-watershed-orchestrator
hourly_tests` exited `101` with 29 missing interval types/methods. After the
implementation and accepted review/verification fixes, the combined focused
selector passed 23 tests:
all 11 named W11A obligations plus production-path pinned-core, distinct-water,
two-channel baseflow, geometry carry/tillage, partial-dependency, consumer, and
`ipeak=3,4,5,6` gates.

## Production implementation

Static:

- `kernel/hourly.rs` owns activation, exact hourly overlap projection, pinned
  segmented `wshchr` KW/MC routing, `mofapp=1`, branch-specific `qref`,
  outlet-only epsilon handling, pinned `qe/qt/qlat/leff` assembly, interval
  WS20 sediment calls, geometry carry, zero-flow deposition, per-class ledgers,
  and daily closure.
- The watershed CLI projects multi-class channel `crfrac` from the required
  `pw0.sol` through pinned `convrt.for` indexing and the existing Rust `prtcmp`
  implementation. Missing or invalid soil authority still fails closed.
- `network_frame.rs` exposes typed water, per-class sediment, and geometry
  interval state. Downstream channels consume upstream `q1` and class egress at
  the same index. Active network publication uses terminal channel state, so
  internal routed mass is not counted twice.
- `02_ws20_segment_routing.rs` integrates gross class detachment from the DCAP
  upper/lower rate construction over the applicable segment span. It then
  derives deposition independently from continuity using ingress, the
  constructive detachment operand, and egress; a negative physical deposition
  residual fails closed. The interval caller uses `t_exp=t_norm=dtchr` and
  explicit hydraulic operands. The event caller retains its former event clock
  and operand path. An exact zero-source class cannot synthesize egress;
  nonzero negative-deposition cases remain typed failures.
- `01_ws22_ws23_ws26_detachment.rs` corrects both `GAP-ROUTE-014` terminals:
  capped widening reconstructs geometry from capped erosion, and the
  post-contact low-boundary-shear route re-enters incision and decrements
  remaining mid-layer depth.

## Ran focused gates

| Command | Result |
|---|---|
| `cargo nextest run -p openwepp-watershed-orchestrator hourly_tests wshedw11b_enddet` | PASS, 23/23 |
| `cargo nextest run -p openwepp-watershed-orchestrator` | PASS, 105/105 before final heavy rerun |
| `cargo nextest run -p openwepp-runner --test mt3_hbp_hourly_consumer_contract` | PASS, 2/2 |
| `cargo nextest run --test wshedw5_typed_watershed_runtime_contract` | PASS, 18/18 |
| protected P102 five-class production CLI test | PASS, 1/1 |
| focused `dcap` expanding/low-shear terminal tests | PASS |
| focused orchestrator `cargo clippy --all-targets -- -D warnings` | PASS |
| `git diff --check` | PASS |

The package-authorized heavy runner owns the release binary, full workspace,
erosion-profile, deny, documentation, and comparator results recorded later in
`gate-results.md`.

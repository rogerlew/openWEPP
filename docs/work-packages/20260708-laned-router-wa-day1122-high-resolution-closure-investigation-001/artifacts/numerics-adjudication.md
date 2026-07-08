# Numerics Adjudication

Status: EXECUTED-HOLD-ACTIVE-ROUTER-CLAMP-NUMERICS
Evidence mode: Ran.

## Decision

Hold the WA/Tier-2 target-`dx` path and do not certify WA active-router
fidelity from the current evidence. The day-1122 fine-rung hard failures are
bounded to diagnostic high-resolution rungs, but the underlying signature is a
mesh-sensitive positivity-clamp amplification inside the active router, not an
upstream hydrology/crop/source-producer change.

The retained active mesh default (`10 cells/OFE`) does not fail the day-1122
closure guard, and this package made no code or production-policy change.
However, `baseline_fixed10` itself shows a material day-1418 clamp-injection
event, so it would be incorrect to call the issue only a harmless
high-resolution reference-rung limitation.

## Findings

1. The current package-local release rerun reproduced the parent failure on
   clean current code provenance: release binary Git HEAD
   `03429ce41d439ff3ab2425bf93a8e00d5c39fa42`, binary SHA256
   `6dcd6275d5d8891a23258fb84a5d143c57b1b0f251f709c8f27711ebc2551308`.
2. `baseline_fixed10`, `dx20`, `dx10`, and `dx5` completed with trace output.
   `dx2p5` and `dx1p25` failed the same guard,
   `laned_active_day_cascade_residual`, at day 1122.
3. The day-1122 residuals are small in absolute terms:
   `dx2p5` residual `-0.0001100301742553711 m3` (`0.110030174255 L`) and
   `dx1p25` residual `0.000011086463928222656 m3`
   (`0.0110864639282 L`). They fail because the relative guard is scaled to
   injected source (`4889.3212269567 m3`) while the clamp/storage operands have
   grown to `190M-858M m3`.
4. The largest completed-rung magnitude is not day 1122. It localizes to day
   1418, lane 5. The clamp for that lane/day is `145554.778351 m3` at
   `baseline_fixed10`/`dx20`, `457540698.111 m3` at `dx10`, and
   `27708994361.1 m3` at `dx5`.
5. `H1.wat.parquet` active hydrology rows are unchanged across completed
   rungs for inspected days 1122, 1167, and 1418. The amplification is therefore
   router-internal, not a crop growth, runoff, climate, or management-source
   producer change.
6. `dx20` is identical to `baseline_fixed10` for this member because all OFEs
   are `108.34 m`, so the `10` cell floor controls (`ceil(108.34 / 20) = 6`,
   floored to `10`). The finer rungs use `11`, `22`, `44`, and `87` cells per
   OFE for `dx10`, `dx5`, `dx2p5`, and `dx1p25`.

## Classification

Classification: active-router numerics hold, centered on positivity-clamp
amplification and cancellation in the active TVD-MacCormack path.

Not supported:
- A production mesh-policy promotion.
- A claim that the WA blocker is only a high-resolution reference-adequacy
  issue.
- A tolerance relaxation in this package.
- Any route-coefficient or crop/source-producer fix.

Supported:
- Default/off behavior is untouched by this package.
- The current active fixed `10 cells/OFE` WA rung passes closure guards but has
  a material clamp-magnitude fidelity risk.
- Target-`dx` candidate evidence is non-promotable until the active router's
  positivity-clamp behavior is understood and bounded or corrected.

## First Follow-On

Scaffold a focused hold-lift package:
`20260708-laned-router-wa-positivity-clamp-numerics-hold-lift-001`.

Required first actions:
- Add or extract a minimal WA day-1418/day-1122 active-router reproducer that
  preserves the source series, geometry, friction operands, upstream handoff,
  and mesh counts.
- Instrument per-step or per-OFE positivity clamp, CFL, depth/discharge extrema,
  and upstream handoff mass on the reproducer.
- Decide contract-first whether large positivity-clamp ratios require a hard
  fidelity guard, a solver fix, or a ratified bounded residual class.
- Test any solver fix against D10B conservation/oracle fixtures and the WA
  reproducer before reopening target-`dx` promotion.

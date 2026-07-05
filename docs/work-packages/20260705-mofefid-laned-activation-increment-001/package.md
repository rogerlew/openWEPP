# MOFEFID LANE D — ACTIVATION INCREMENT (runtime seam shadow + real-H2637 vector)

Status: `EXECUTED — AWAITING CODEX REVIEW` (Claude-executed; operator:
"scaffold and execute the activation increment", 2026-07-05). Branch:
`laned-activation-increment`. SC-OFEROUTE-001 rev 15 + the rev-16
terminology disambiguation (Codex framing review): the runtime SHADOW
(landed, fixture-exercised) and the ACTIVATION wiring (outstanding:
routing owning the water path, runtime closure hard-fail, DC01-disable,
the enumerated flip preconditions) are now distinct claims at every
contract site — the acceptance verb this increment closes is "runtime
wiring exists and is fixture-exercised," NOT activation.

## What landed

1. **Publication-side seam forcing from LIVE surfaces**: the day row
   gains an in-memory `dc01_surface_hourly_weights` field — the lane's
   OWN M2 distribution recomputed at publication over
   `wb14_hourly_excess` + the `ui_SCrunf`-lineage carry. (The first cut
   read `lane.transfer.surface_hourly_weights` — the DOWNSTREAM INFLOW
   distribution — and the H2637 run caught it immediately: supply
   reconstruction 1.0 → ≤1e-15 after the own-weights fix. The shadow
   validated its own wiring.)
2. **`hillslope::laned_shadow`** (runner): opt-in
   (`OPENWEPP_LANED_SHADOW=1`) collector — per lane-day depth series
   (`weights × qofe`, the ADR-0036 weights-times-total authority),
   event-day cascade over the real `ofe_routing` machinery (bare-cell
   `k_o = 500` labeled first cut → `GAP-OFEROUTE-007`), day-window
   clipped to the active span, diagnostics-only manifest block.
3. **The REAL-H2637 executed vector**
   (`tests/fixtures/laned_shadow_h2637/`, staged from the wepp-forest
   ablation record, cli truncated to 2 years; test
   `laned_shadow_h2637`): 19 OFEs, 731 days, 610 routed. HARD: byte
   identity of HBP + pass parquet with the shadow on/off; no manifest
   shadow keys when off; supply reconstruction ≤1e-9 (measured
   ~5e-16). DIAGNOSTIC: aggregate router conservation <15% (measured
   6.0%); toe delivery >0.9 (measured 0.990); the lump-only day class
   present-and-small (measured 10/731).

## Findings (the shadow earned its keep)

- **Melt-limb seam coverage**: 10/731 runoff days carry NO hourly shape
  from the two GAP-006 D1 limbs (uniform DC01 fallback) — snowmelt-
  sourced runoff is a THIRD supply limb the seam design must
  disposition before activation. Labeled in INV-OFEROUTE-012 rev 15.
- **GAP-OFEROUTE-005 real-hillslope reproduction**: the cascade's
  run-level conservation aggregate is resolution-sensitive and
  dt-non-monotone on the steep 19-OFE regime — (sample_dt, max_dt)
  sweep (900,300)→6.0%, (900,120)→10.0%, (120,300)→22.1%; the
  inter-OFE sampled handoff is implicated. The shock-numerics package
  now has a production-shaped case alongside Iwagaki.

## Erosion hourly-shape touchpoint (dispositioned)

Under SHADOW, erosion keeps consuming the DC01 weights unchanged (byte
identity proves it). At ACTIVATION the erosion hourly substrate
(ADR-0036) should consume the ROUTED hydrograph — recorded as a flip
precondition alongside INV-011, GAP-005 numerics, the melt limb, and
GAP-007 operand sourcing. Not designed here.

## What this increment does NOT claim

No production activation (INV-OFEROUTE-011 open; the rev-15 status
enumerates the full flip-precondition set). No runtime closure
hard-fail wiring (that engages when routing OWNS water — under shadow,
DC01's kernel closure gates remain the authority). No friction
fidelity (GAP-007).

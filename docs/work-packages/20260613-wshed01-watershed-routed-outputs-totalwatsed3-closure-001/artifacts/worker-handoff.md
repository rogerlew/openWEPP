# Worker Handoff

Status: W-D executed-hold; W-D-REDO ready

Evidence mode: Ran + Static

## Current State

W-A characterized the watershed CLI and scoped the remaining work. W-B cleared
the no-impoundment parser seam. W-C cleared the WS10 channel guard seam and
published WAT-backed watershed outputs. W-D ran the totalwatsed3 audit and
fixed confirmed publication defects, but the package is not complete;
implementation continues with W-D-REDO.

The W-B blocker is resolved:

- `pw0.imp` declares no impoundments with `jpond=0`.
- Legacy treats no-impoundment watersheds as valid.
- openWEPP now accepts `jpond=0` only when structural count is zero.
- The real arboreal-dendrite run now reaches
  `CLIWAT-E-020` / `WKERNEL-WS10-CHANNEL-E-003`.

The W-C blocker is resolved:

- zero-sediment HBP contributor payloads with zero fractions are accepted when
  mass and concentration support are zero;
- `nchnum=0` is treated as channel detail output disabled, not as a routing
  domain violation;
- configured and legacy-discovery arboreal-dendrite CLI runs emit all `14`
  watershed parquet outputs;
- `totalwatsed3.parquet` has `2192` daily rows with WAT-backed fields.

The W-D publication defects are resolved:

- exact totalwatsed3 hydrology columns now emit `m^3` volumes while depth
  aliases remain mm;
- MOFE `latqcc` uses only the outlet OFE per WAT file/day/`wepp_id`;
- optional profile and interception WAT fields now publish into
  `totalwatsed3`;
- configured and legacy-discovery totalwatsed3 audits now report zero profile
  violations and `interception_reported_total_mm=551.502748`.

The W-D blocker remains:

- independent closure still fails:
  `closure_reconstructed_with_storage_total_mm=2950.498418`;
- current `runvol` is still filled from WAT `Q`, so runoff consistency is
  source self-consistency, not independent PASS runoff closure.

## Next Dispatch

```text
Execute increment W-D-REDO of docs/work-packages/20260613-wshed01-watershed-routed-outputs-totalwatsed3-closure-001/artifacts/watershed-staged-increment-plan.md end-to-end.
```

## W-D-REDO Requirements

- Expose or reconstruct canonical daily PASS runoff volume from HBP/PASS
  publication authority.
- Bind that independent PASS value into `totalwatsed3.runvol` and `Runoff`.
- Rerun the configured and legacy-discovery totalwatsed3 audits.
- Preserve the W-B no-pond parser contract.
- Preserve W-C anti-placeholder publication and multi-row output.
- Preserve W-D volume/depth, outlet-lateral, profile, and interception fixes.
- Gate on totalwatsed3 water-balance conservation with independent operands.
- Record residuals and any cross-repo consumer mismatch without editing wepppy
  production code unless explicitly scoped.

## Watchpoints

- `openwepp-cli-watershed.rs` is `2072` lines. Keep W-D-REDO out of this file
  unless daily PASS runoff binding truly belongs in the CLI.
- `crates/openwepp-watershed-output/src/writers.rs` is `2043` lines and over
  the `2000`-line warning threshold. W-D-REDO should avoid growth or split
  before adding more writer logic.
- W-D-REDO acceptance is totalwatsed3 closure with independent operands, not
  legacy magnitude matching.

## Claude review (2026-06-14) — W-D hold endorsed; W-D-REDO scope is broader than the runoff operand

Evidence mode: Ran (read `watershed_wat.rs` + `writers.rs`).

W-D's hold is correct (no hollow closure claimed) and the runvol finding is
confirmed: `build_watershed_daily_rows_from_wat` (`watershed_wat.rs:216`)
builds watershed rows **entirely from hillslope WAT** — the pass file is used
only to locate the sibling `.wat.parquet` (`:232`); runoff is the area-weighted
sum of WAT `runoff_mm` (`:151`). So runvol is WAT-self-consistent, not the
independent channel-routed PASS lineage. Right call.

**But the structural gap is deeper than the runoff operand — two co-equal
issues, recorded so W-D-REDO is not scoped too narrowly:**

1. **The watershed output is a hillslope-WAT sum, not a channel-routed
   balance.** There is no `from_pass` / channel-routed-runoff path; the PASS
   data is not used for the water balance at all.
2. **Channel water-balance terms are hardcoded zero.** `channel_loss_m3 = 0.0`
   and `channel_storage_m3 = 0.0` (`writers.rs:163,165`), never populated. So
   channel transmission loss and channel storage are absent from the balance.

The genuine watershed identity is:
`Σ hillslope (P − ET − Perc − ΔS_hillslope) = water delivered to channels`,
then `water delivered − outlet runoff(PASS) − channel loss − ΔS_channel = 0`.
Swapping runvol to the independent PASS outlet runoff (Codex's W-D-REDO) is
**necessary but likely not sufficient**: PASS outlet runoff < Σ hillslope
runoff by the channel loss/storage, so substituting it while channel terms
stay zero will **move the residual, not close it**. The 2950 mm is plausibly
that channel-routing gap (or, for a small watershed like arboreal-dendrite,
partly a units/area-weighting artifact — see below).

**W-D-REDO scope (refined):**
- Source runvol/Runoff from the independent PASS outlet-runoff lineage
  (Codex's point). ✓
- **Populate the channel water-balance terms** (`channel_loss_m3`,
  `channel_storage_m3`) from real channel routing, so the watershed balance is
  *complete* — don't leave them zero while changing runoff.
- **Measure the 2950 mm attribution** before fixing: runoff-operand vs
  channel-terms vs units. The m³/mm split this increment touched is a classic
  unit-bug surface (agent memory `comparator-surface-artifacts`: dimensional
  mismatches, no harness dimensional guard) — rule out an area-weighting /
  m³-vs-mm contribution to the 2950 before attributing it to physics.
- Acceptance: the totalwatsed3 identity closes at noise on a **complete,
  independent-operand** watershed balance (outlet runoff from PASS, channel
  terms real, hillslope contributions from WAT — genuinely different sources),
  not 0==0 self-consistency.

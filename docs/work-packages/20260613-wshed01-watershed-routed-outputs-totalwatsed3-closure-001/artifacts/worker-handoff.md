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

## Claude review CORRECTION (2026-06-14, operator-directed) — totalwatsed3 is HILLSLOPE-ONLY; retract the channel-terms scope

Evidence mode: Ran (read the authoritative producer
`wepppy/wepp/interchange/totalwatsed3.py` + `tools/totalwatsed3_daily_closure_audit.py`;
wepppyo3 `wepp_interchange/src/hill_wat.rs`).

**My prior W-D review (channel loss/storage terms) was wrong** — operator
correction: totalwatsed3 is a **hillslope-only, area-weighted aggregation**,
not a channel-routed watershed balance. The authoritative producer confirms:

- `_aggregate_pass` (`totalwatsed3.py:583`): the closure **`Runoff`** is
  `SUM(runvol)` from the **hillslope PASS file** (`H.pass.parquet`) — schema
  `:127` "Daily runoff depth from PASS runoff volume". Codex's PASS-runoff
  instinct was correct; it is the hillslope pass runoff, NOT channel-routed.
- `_aggregate_wat` (`:613`): area-weighted, **MOFE-aware per-OFE collapse** —
  **latqcc outlet-OFE-only** (`OFE == _max_ofe_id`, explicitly "to avoid
  counting internal" routing, `:641-644`), QOFE summed × Area (`:660`).
- **No channel terms anywhere.** There is no channel loss/storage in
  totalwatsed3. **Retract** the prior "populate `channel_loss_m3` /
  `channel_storage_m3`" requirement — those belong to the separate
  `chanwb`/`chnwb` channel outputs, not totalwatsed3.

**Corrected W-D-REDO scope** — align openWEPP's totalwatsed3 to the
authoritative wepppy/wepppyo3 hillslope-only semantics:

1. `Runoff` from the **hillslope PASS `runvol`** (area-weighted sum), not WAT
   `Q`. This is the independent operand that closes the audit.
2. Per-OFE collapse exactly per `_aggregate_wat`: **latqcc outlet-OFE-only**,
   QOFE summed, area-weighted (MOFE01 made WAT per-OFE; naive cross-OFE sums
   double-count — likely a large part of the 2950 mm).
3. **No channel terms** in totalwatsed3.
4. **Architectural question for the operator/W-D-REDO:** the authoritative
   totalwatsed3 producer is wepppy/wepppyo3 (it consumes `H.pass.parquet` +
   `H.wat.parquet`). Should openWEPP *produce* totalwatsed3 at all (W-C's
   `build_watershed_daily_rows_from_wat`), or just emit correct hillslope
   `H.pass`/`H.wat` and let the wepppy/wepppyo3 producer + audit own
   totalwatsed3? The cleaner design is the latter (single authoritative
   producer; openWEPP's job is correct hillslope interchange inputs). If
   openWEPP does produce a native totalwatsed3, it must match the wepppy
   semantics exactly (hillslope-only, PASS runoff, per-OFE collapse) — a
   parallel re-implementation that can diverge. Decide before W-D-REDO codes.

The 2950 mm is therefore a hillslope-aggregation issue (WAT-Q-instead-of-PASS-
runvol + per-OFE collapse + possible m³/mm units), **not** channel routing.

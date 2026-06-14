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

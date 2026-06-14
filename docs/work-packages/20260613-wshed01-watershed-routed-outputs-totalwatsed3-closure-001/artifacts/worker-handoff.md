# Worker Handoff

Status: W-B executed-hold; W-C ready

Evidence mode: Ran + Static

## Current State

W-A characterized the watershed CLI and scoped the remaining work. W-B cleared
the no-impoundment parser seam. The package is not complete; implementation
continues with W-C.

The W-B blocker is resolved:

- `pw0.imp` declares no impoundments with `jpond=0`.
- Legacy treats no-impoundment watersheds as valid.
- openWEPP now accepts `jpond=0` only when structural count is zero.
- The real arboreal-dendrite run now reaches
  `CLIWAT-E-020` / `WKERNEL-WS10-CHANNEL-E-003`.

## Next Dispatch

```text
Execute increment W-C of docs/work-packages/20260613-wshed01-watershed-routed-outputs-totalwatsed3-closure-001/artifacts/watershed-staged-increment-plan.md end-to-end.
```

## W-C Requirements

- Start from the observed W-B hard stop:
  `CLIWAT-E-020` / `WKERNEL-WS10-CHANNEL-E-003`.
- Route the hillslope shards over the channel network to watershed-level routed
  parquet outputs.
- Preserve the W-B no-pond parser contract.
- Reject placeholder/default-zero watershed publication as closure evidence.
- Gate on watershed water-balance conservation with independent operands.

## Watchpoints

- `openwepp-cli-watershed.rs` is already 2031 lines. If W-C must edit it
  substantially, keep the change narrow or plan a split.
- W-C must not accept one-row/default-zero parquet output as closure.
- W-D acceptance is totalwatsed3 closure with independent operands, not legacy
  magnitude matching.

# Worker Handoff

Status: W-A executed; W-B ready

Evidence mode: Ran + Static

## Current State

W-A characterized the watershed CLI and scoped the remaining work. The package
is not complete; implementation starts with W-B.

The current blocker is localized:

- `pw0.imp` declares no impoundments with `jpond=0`.
- Legacy treats no-impoundment watersheds as valid.
- openWEPP rejects `jpond=0` before structural-count reconciliation.

## Next Dispatch

```text
Execute increment W-B of docs/work-packages/20260613-wshed01-watershed-routed-outputs-totalwatsed3-closure-001/artifacts/watershed-staged-increment-plan.md end-to-end.
```

## W-B Requirements

- Read `docs/specifications/science-contracts/AGENTS.md` before production
  edits.
- Add red tests for zero impoundments aligned with zero structural count.
- Implement typed empty impoundment set semantics for `jpond=0`.
- Preserve typed failures for malformed/negative/mismatched counts.
- Rerun focused parser/CLI tests.
- Rerun the arboreal-dendrite watershed CLI enough to prove it proceeds past
  `CLIWAT-E-010`; record the next hard stop or the first routed output.

## Watchpoints

- `openwepp-cli-watershed.rs` is already 2031 lines. If W-B or W-C must edit it
  substantially, keep the change narrow or plan a split.
- W-C must not accept one-row/default-zero parquet output as closure.
- W-D acceptance is totalwatsed3 closure with independent operands, not legacy
  magnitude matching.

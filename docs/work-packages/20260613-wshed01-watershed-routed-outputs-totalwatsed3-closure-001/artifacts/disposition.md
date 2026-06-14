# Disposition

Status: W-C executed-hold; package active; W-D queued

Evidence mode: Ran + Static

W-A, W-B, and W-C are complete for their scoped seams. The package remains
active because W-D totalwatsed3 closure is the acceptance surface.

Disposition:

- W-A baseline: fail-closed before watershed output writing at
  `CLIWAT-E-010`/`IMP-E-004`.
- `jpond=0`: fixed as a parser defect on valid no-impoundment input.
- Runfile contract: schema v1 still requires `inputs.pw0_imp`; an explicit
  supported `.imp` file with `jpond=0` is accepted only when `pw0_str` declares
  zero impoundments.
- W-B arboreal-dendrite behavior: proceeds past `CLIWAT-E-010`; next observed
  hard stop is `CLIWAT-E-020` / `WKERNEL-WS10-CHANNEL-E-003`; output file
  count remains `0`.
- W-C hard-stop classification: valid zero-sediment HBP payload and `nchnum=0`
  output-disabled channel state were rejected by over-strict WS10 guards.
- Routing/output result: arboreal-dendrite configured and legacy-discovery
  runs exit `0`, emit all `14` watershed parquet outputs, and produce `2192`
  `totalwatsed3.parquet` rows with non-placeholder WAT fields.
- totalwatsed3 contract: documented from openWEPP and wepppy sources.

Next required increment:

```text
Execute increment W-D of docs/work-packages/20260613-wshed01-watershed-routed-outputs-totalwatsed3-closure-001/artifacts/watershed-staged-increment-plan.md end-to-end.
```

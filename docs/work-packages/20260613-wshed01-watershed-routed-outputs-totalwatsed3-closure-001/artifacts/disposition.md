# Disposition

Status: W-B executed-hold; package active; W-C queued

Evidence mode: Ran + Static

W-A is complete. W-B is complete for its scoped parser/CLI seam and is held
only because package-level watershed routing/totalwatsed3 closure remains for
W-C/W-D.

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
- Routing/output scope: mapped; channel-routing hard stop is next.
- totalwatsed3 contract: documented from openWEPP and wepppy sources.

Next required increment:

```text
Execute increment W-C of docs/work-packages/20260613-wshed01-watershed-routed-outputs-totalwatsed3-closure-001/artifacts/watershed-staged-increment-plan.md end-to-end.
```

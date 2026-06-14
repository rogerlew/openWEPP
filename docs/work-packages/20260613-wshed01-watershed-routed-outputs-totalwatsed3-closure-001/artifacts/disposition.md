# Disposition

Status: W-A executed; package active; W-B queued

Evidence mode: Ran + Static

W-A is complete. It met its characterization gates and made no production
edits.

Disposition:

- Current CLI behavior: fail-closed before watershed output writing at
  `CLIWAT-E-010`/`IMP-E-004`.
- `jpond=0`: parser defect on valid no-impoundment input.
- Routing/output scope: mapped; post-parser routing behavior not yet observed.
- totalwatsed3 contract: documented from openWEPP and wepppy sources.

Next required increment:

```text
Execute increment W-B of docs/work-packages/20260613-wshed01-watershed-routed-outputs-totalwatsed3-closure-001/artifacts/watershed-staged-increment-plan.md end-to-end.
```

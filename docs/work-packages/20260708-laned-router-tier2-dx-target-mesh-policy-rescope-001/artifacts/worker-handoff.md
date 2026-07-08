# Worker Handoff

Status: EXECUTED-COMPLETE
Evidence mode: Static.

## Next Package Candidate

`20260708-laned-router-wa-day1122-high-resolution-closure-investigation-001`

Objective:
- Diagnose why `wa_cascades_forest_h1` day 1122 fails the active day cascade
  residual at `dx2p5` and `dx1p25`.
- Attribute the huge clamp/storage/outlet magnitudes seen at `dx10/dx5`.
- Decide whether this is a bounded diagnostic-regime limitation, a solver
  numerics defect, or a route-coefficient/geometry stressor.

Starting evidence:
- `artifacts/mesh-ladder-runs/wa_cascades_forest_h1/dx2p5/time.log`
- `artifacts/mesh-ladder-runs/wa_cascades_forest_h1/dx1p25/time.log`
- `artifacts/mesh-ladder-summary.json`
- `artifacts/mesh-fidelity-adjudication.md`

Reproduction command:

```text
.venv/bin/python docs/work-packages/20260708-laned-router-tier2-dx-target-mesh-policy-rescope-001/artifacts/run_mesh_ladder.py --skip-build --members wa_cascades_forest_h1 --rungs dx2p5 dx1p25
```

Package boundary:
- Do not change production mesh policy in the investigation package.
- Keep active production default fixed `10 cells/OFE`.
- Treat H2637 only as synthetic stress evidence.

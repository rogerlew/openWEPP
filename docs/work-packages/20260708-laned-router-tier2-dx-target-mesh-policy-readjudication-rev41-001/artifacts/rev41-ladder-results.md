# Rev-41 Ladder Results

Status: `EXECUTED-COMPLETE`
Evidence mode: Ran.

Primary evidence:

- `artifacts/mesh-ladder-summary.md`
- `artifacts/mesh-ladder-summary.json`
- Generated package-local run trees under `artifacts/mesh-ladder-runs/`
  (intentionally ignored for commit; the summary JSON/Markdown preserve the
  adjudication metrics, timings, hashes, and provenance)

Command:

```text
.venv/bin/python docs/work-packages/20260708-laned-router-tier2-dx-target-mesh-policy-readjudication-rev41-001/artifacts/run_mesh_ladder.py
```

Release binary:

- Build: `cargo build --release -p openwepp-runner --bins`
- Path: `target/release/openwepp-cli-hill`
- SHA256:
  `8427529a166a880699fd06a6a39ed6f6bb23ca039a62dc670cd784ebce11e6f6`
- Git HEAD: `abc69bdda5458dd5389902e61a7626213aaf54cb`

All 24 rungs passed runtime execution. No selected-cohort or H2637 rung failed
active closure, the rev-40 clamp-source guard, or the rev-41 positivity
correction's roundoff-only clamp posture.

## Real-Cohort Timing

| Rung | Aggregate real-cohort user time |
|------|--------------------------------:|
| `baseline_fixed10` | 17.46 s |
| `dx20` | 18.07 s |
| `dx10` | 23.43 s |
| `dx5` | 84.70 s |
| `dx2p5` | 364.25 s |
| `dx1p25` | 1648.41 s |

`dx5` is the first tested target-`dx` rung whose provisional
candidate-vs-`dx2p5` deltas clear the real-cohort candidate tolerance surface,
but that comparison is not promotional while fine-reference adequacy remains
unclosed. It is about `4.85x` the current fixed10 aggregate user time on the
real cohort.

## WA Closure Lift

`wa_cascades_forest_h1` now completes both fine-reference rungs:

| Rung | Wall | User | Solver steps | Outlet m3 | End storage m3 | Pass `tdet` sum |
|------|------|-----:|-------------:|----------:|---------------:|----------------:|
| `dx2p5` | `4:20.62` | 260.53 | 8548214 | 860494.4847690226 | 4906.224738371967 | 35151.290618501305 |
| `dx1p25` | `19:14.96` | 1154.78 | 17051717 | 860480.0378746358 | 4920.671632758908 | 35153.69157664854 |

The pre-rev41 WA day-1122 high-resolution closure blocker is therefore lifted
for this ladder.

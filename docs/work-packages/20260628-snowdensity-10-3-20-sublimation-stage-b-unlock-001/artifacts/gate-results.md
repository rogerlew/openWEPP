# Gate Results

Evidence class: Static + Ran.

Primary real run:

```bash
.venv/bin/python tools/snowfreeze_observed/sublimation_stage_b_unlock.py
```

Result:

- Current default: `15` robust fails / `179` robust score.
- Partition + Stage A sublimation: `19` robust fails / `168` robust score.
- Stage B surface-layer candidate: `15` robust fails / `178` robust score.
- Stage B primary gate: `FAIL`.
- Stage B bidirectional guardrail: `FAIL` (`3` worse robust cells).
- Stage B conservation: `PASS`.
- Activation authorized: `false`.

Trace/conservation:

- Stage B selector trace: `PASS`.
- Stage B sublimation vapor closure max residual:
  `4.440892098500626e-16 m`.
- Stage B partition closure max residual: `5.551115123125783e-17 m`.
- Stage B total aggregate sublimation: `7.980802132555099 m`.
- Stage A composition total aggregate sublimation: `17.521858543414357 m`.

Disposition: `NON-PROMOTION-GATE-NOT-MET`.

# Hold Legitimacy Audit

Status: `EXECUTED-HOLD-MN-CORN-H4-SHAPE-NONCONVERGED`
Evidence mode: Ran.

## Exact Blocker

`mn_corn_h4` routed hourly shape does not satisfy the strict one-third
fine-reference adequacy rule after adding the `dx0p625` rung.

Required threshold:

- production shape tolerance: max L1 `<= 0.05`
- fine-reference adequacy threshold: max L1 `<= 0.0166667`

Observed:

- prior `dx2p5` vs `dx1p25`: max L1 `0.02018051100943346`
- new `dx1p25` vs `dx0p625`: max L1 `0.02094494047849004`

## Evidence

Executed command:

```text
.venv/bin/python docs/work-packages/20260708-laned-router-tier2-dx5-fine-reference-hold-lift-001/artifacts/run_mesh_ladder.py --members mn_corn_h4 --rungs baseline_fixed10 dx20 dx10 dx5 dx2p5 dx1p25 dx0p625
```

All seven `mn_corn_h4` rungs passed active closure. The release binary SHA was
`8427529a166a880699fd06a6a39ed6f6bb23ca039a62dc670cd784ebce11e6f6`.

The first attribution check does not support a counter cliff:

- `uniform_shape_rows`: `16` on `dx1p25` and `dx0p625`.
- `erosion_source_shape_degenerate_rows`: `22` on `dx1p25` and `dx0p625`.
- `positive_shape_rows`: `83` on both rungs.
- `tail_fold_total_m3` moves by `0.015727541710411685 m3` over
  `4510.466965990125 m3` source.
- `end_storage_total_m3` and terminal outlet shift by equal and opposite
  `0.06748291198 m3`, preserving the active closure ledger.

The max offending row is `sim_day_index=792`, `lane_index=1`: no tail fold, no
uniform fallback, no degenerate shape, and only a `9.04e-5 m3` end-storage
difference. The failure is a routed hourly shape convergence surface.

## Why This Is a Legitimate Hold

The package was explicitly barred from amending the adequacy rule at the
margin. The new finer reference did not reduce the shape delta below the
unchanged threshold. Continuing into `SC-OFEROUTE-001` promotion would promote
a production active mesh policy without the package's required adequate
reference basis.

This is not safely closeable inside this package because the next work is not
another promotion action; it is a shape-surface numerics attribution problem.
Changing the shape tolerance, changing the reference rule, or accepting the
miss as "close enough" would be tolerance-fitting against the operator's
instruction.

## First Follow-On

Scaffold a narrow `mn_corn_h4` routed-shape attribution package. First actions:

1. Reproduce `sim_day_index=792`, `lane_index=1` as a single-day/single-lane
   trace fixture across `dx2p5`, `dx1p25`, and `dx0p625`.
2. Attribute whether the shape delta is caused by hourly binning, end-window
   storage partitioning, source-shape sampling, TVD mesh convergence, or D13
   routed-hydrograph normalization.
3. Only after attribution, decide whether the valid lever is a numerics
   correction, a shape-consumer correction, a predeclared tolerance amendment,
   or an explicit no-promotion verdict.

Suggested package name:
`20260708-laned-router-mn-corn-h4-routed-shape-attribution-001`.

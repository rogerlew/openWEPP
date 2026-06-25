# Rubric Profile

Evidence mode: Ran.

Evaluation authority: `SC-SNOWFREEZE-001` v74,
`INV-SNOWFREEZE-050`, `TOL-SNOWFREEZE-011`.

Generated report:

- `target/snowfrost_fidelity_h/three_way_comparison.json`
- `target/snowfrost_fidelity_h/three_way_comparison.md`

Schema: `snotel-density-three-way-comparison-v2`.

Top-level counts:

- Fork routing counts: `STRUCTURAL = 5`.
- Rubric labels across all model/site cells:
  `fail = 107`, `marginal = 81`, `pass = 52`, `strong = 72`,
  `unavailable = 88`.
- Forcing-robust (`R`) labels:
  `fail = 67`, `marginal = 41`, `pass = 40`, `strong = 68`,
  `unavailable = 84`.
- `openwepp_defective_cells = 0`; observation-only failed cells are retained as
  `UNRESOLVED` unless independent correctness authority exists.

The report now includes per-model/per-site/per-cell profiles. It does not reduce
the comparison to a scalar. The density fork labels are retained only as routing
metadata.

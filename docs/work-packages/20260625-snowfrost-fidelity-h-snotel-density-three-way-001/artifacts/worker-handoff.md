# Worker Handoff

Current state: H is executed-held.

Keep:

- `tests/fixtures/snotel_observed/observations/**`
- `tools/snowfreeze_observed/snotel_density_three_way.py`
- `tools/snowfreeze_observed/pysnobal_compare.py` water-year segmentation and
  venv-preserving interpreter handling
- `SC-SNOWFREEZE-001` v73/v74 amendments
- H package artifacts

Do not:

- Tune SSD to residuals.
- Promote PySnobal, legacy WEPP, or observation disagreement into production
  correctness authority.
- Change production snow/frost physics from this package.

Next recommended package:

`SNOWFROST-FIDELITY-H2-PYSNOBAL-CSS-WY2017-REPRO-001`

Objective:

- Build a minimal reproducer for CSS Lab WY2017 PySnobal failure.
- Determine whether the failure is caused by bridge configuration, PySnobal
  model limits, or an extreme forcing/state trajectory.
- If valid, define a site/year exclusion or partial-profile policy; otherwise
  fix the bridge and rerun H to complete the PySnobal overlay.

After H2, rerun H with the existing v74 rubric profile and close either complete
or with a narrower, ratified PySnobal exclusion.

# Worker Handoff

Current state: H is complete-with-disposition.

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

`SNOWFROST-FIDELITY-I-SNOW-DEPTH-STRUCTURAL-REMEDIATION-001`

Objective:

- Resolve the structural snow-depth fork identified by H. All five SNOTEL sites
  route `STRUCTURAL` in the auxiliary density fork, so changing the SSD seed does
  not close the observed depth/SWE/density profile.
- Start from the snow-depth producer/carry/input/settlement scope already queued
  in ROADMAP W.1: snowpack initial state/carry, snowfall depth input,
  density/settlement, rain-on-snow storage, melt depletion, and publication
  lineage.
- Keep H's v74 rubric profile as the observation scoring authority. Legacy WEPP
  and PySnobal remain ADR-0017 flag/profile evidence only.

Optional only: a separate PySnobal-hardening package may strip/retry thin snow
or patch SNOBAL if broader PySnobal coverage becomes useful. It is not required
to proceed with openWEPP snow-depth remediation.

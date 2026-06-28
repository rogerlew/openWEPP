# Disposition

Evidence class: Static + Ran.

Disposition: `ACTIVATED`.

SNOWDENSITY-10.3.19 adopted `harder_pomeroy_hourly` as the no-env
direct-production snow phase default composed with the activated melt and density
bundle. The explicit `legacy_rst` selector remains the rollback/test path.

Primary gate:

- Cross-SNOTEL forcing-robust rubric real run: new default `15` robust fails and
  score `179`; prior activated bundle `17` robust fails and score `172`.

Supporting gates:

- Workspace suite: no regression under the new default.
- Conservation: trace residual maximum `5.551115123125783e-17 m`.
- Release notes: humid-NE depth regression remains a non-representative roadmap
  item; density median signed bias rises to `+23.6 kg m-3` and remains tracked
  separately.
- No fixture, output schema, cap, frost, or `.run` disable-option change.

# Closeout

Evidence mode: Static + Ran.

Disposition: `HOLD-ACTIVATION-EVIDENCE-MISSING`

## Result

The diagnostic consumed committed real direct-production WAT report lineage from
SNOWDENSITY-10.3.8, 10.3.11, and 10.3.12 and paired rows by date across four
states:

- current default;
- `coe_liquid_holding_capacity_v1`;
- `coe_liquid_holding_capacity_v1 + physics_bulk_density_compaction_v1`;
- rejected `physics_bulk_spring_densification_v1`.

The bundle remains better than default (`1147 -> 498`) but is not activation-
ready because Policy-B full-surface no-regression evidence is missing.

## Residual Attribution

- Complete four-model transition rows: `1414`.
- Source bundle paired rows: `1415`.
- Bundle under-persistence failures: `234`.
- Under-persistence induced by the bundle density arm: `177`.
- Under-persistence induced from holding-only pass rows: `150`.
- Under-persistence induced from holding-only over-persistence rows: `27`.
- Under-persistence persisted from holding-only under-persistence: `57`.

This supports the lead hypothesis that the density arm carries a mechanism cost:
it removes a large over-persistence bias but introduces a substantial shallow-
pack tail. The correct next step is mechanism-cost/no-regression adjudication,
not another compaction-rate acceleration.

## March/April Tail

Under the active `522 kg m^-3` cap:

- `CAP_LIMITED_DEPLETION_REQUIRED = 33`
- `PATCHY_MELTOUT_OR_DEPLETION_REQUIRED = 16`
- `COMPACTION_ONLY_FEASIBLE_WITHIN_522_CAP = 20`
- `UNDER_PERSISTENCE_OR_ACCUMULATION_DEFICIT = 128`

The `550 kg m^-3` SNOBAL cap re-anchor remains follow-up only. This package did
not amend the density cap or rerun physics under a different cap.

## Policy-B Status

- Gate-eligible paired-snow improvement: pass.
- Paired surface no-worse guard versus holding-only: pass.
- Full workspace regression/identity with bundle as default: missing.
- Non-snow climate no-regression: missing.
- Erosion and water-balance no-regression: missing.
- Watershed routing no-regression: missing.
- Composite melt-density conservation under bundle activation: missing.

Activation remains blocked by missing Policy-B full-surface evidence. Frost
attribution remains separately blocked by snow-control residuals.

## Boundary Status

- Default activation changed: no.
- Production physics changed: no.
- Density cap changed: no.
- Parser/runfile/user selector added: no.
- Fixture inputs changed: no.
- Public output schema changed: no.
- Qwet/frzftp changed: no.
- Frost attribution authorized: no.

## Follow-Up

Recommended next package:
`SNOWDENSITY-10.3.14-POLICY-B-NO-REGRESSION-AND-CAP-AUTHORITY`.

It should settle the `550 kg m^-3` cap authority question contract-first and
define/run the Policy-B full-surface no-regression suite before any activation
claim.

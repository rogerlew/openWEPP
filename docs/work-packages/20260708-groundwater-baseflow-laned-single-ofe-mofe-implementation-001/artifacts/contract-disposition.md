# Contract Disposition

Status: `EXECUTED-HOLD-GWDSV-CHANNEL-CONSUMER`

Evidence: `Static + ran`

Contract-first disposition before production code:

- `SC-GWBASEFLOW-001` is sufficient for the first production implementation
  pass. It explicitly authorizes the daily linear-reservoir recurrence, Lane D
  MOFE aggregation, active-router non-source boundary, generated baseflow
  export, and fail-closed behavior.
- `SC-INFILE-GWCOEFF-001` parser semantics are sufficient as-is. No parser
  grammar change is needed; the runner must consume `GwcoeffFile` rather than
  reparse with ad hoc logic.
- No `SC-OFEROUTE-001` amendment is required before code because the active
  source builder already excludes any groundwater inputs. The package will add
  implementation evidence that generated `gwbfv`/`gwdsv` remain outside
  `laned_active_lane_source`.
- `SC-SUBHYD-001` and `SC-INFILE-CHANINP-001` are namespace boundaries only for
  this package. No lateral-flow or channel `cbase` behavior will be amended.
- Boundary/output registry entry `hillslope_wat.Base:mm` was added for generated
  groundwater baseflow publication.
- No contract amendment was needed for the implemented recurrence or active
  router negative proof.
- Generated groundwater-reservoir deep seepage (`gwdsv`) and `bftharea`
  watershed/channel threshold consumption remain legitimate hold boundaries;
  no producer-only claim closes them.

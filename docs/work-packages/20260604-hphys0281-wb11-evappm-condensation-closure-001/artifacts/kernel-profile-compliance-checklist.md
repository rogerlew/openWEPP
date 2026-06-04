# Kernel-Profile Compliance Checklist

Status: completed/HOLD
Evidence mode: static + ran

- Static: contract-first sequence followed: canonical `SC-EVAP-001` authority, red tests, pre-implementation red gate, then production code.
- Static: physics authority is pinned baseline `/workdir/wepp-forest_260430_baseline` at `dac3c950d8b16cc73774bf5ce2e7e11f80baac70`.
- Static: no heuristic/proxy equation or guard relaxation was introduced.
- Static: material-negative `pmet.es_m` remains rejected by the WB11/WB17 guard.
- Ran: focused tests, SIMIMPL18 fixtures, workspace clippy, workspace tests, deny, unit registry, docs lint, and diff hygiene passed.
- HOLD: SC-EVAP still has pre-existing HPHYS0279 unit-compliance findings for older `Ep`/`Es`/`Er` rows.

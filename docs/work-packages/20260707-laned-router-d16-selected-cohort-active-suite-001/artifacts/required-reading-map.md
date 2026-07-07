# Required Reading Map

Status: EXECUTED. Evidence mode: Static.

Read before scaffold/execution:

- `AGENTS.md`
- `docs/work-packages/AGENTS.md`
- `docs/specifications/science-contracts/AGENTS.md`
- `docs/standards/AGENTS.md`
- `docs/standards/prompt-wording-guidance.md`
- `tools/owcmp/AGENTS.md`
- `tools/owcmp/specification.md`
- `tests/fixtures/AGENTS.md`
- `docs/contracts/openwepp-management-lanuse-authority-contract.md`
- `docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md`
- `docs/specifications/science-contracts/contracts/SC-OFEROUTE-002.md`
- `docs/work-packages/20260707-laned-router-d16-hybrid-disturbed-route-coeff-source-acquisition-001/package.md`
- `docs/work-packages/20260707-laned-router-d16-hybrid-disturbed-route-coeff-source-acquisition-001/artifacts/final-disposition.md`
- `docs/work-packages/20260707-laned-router-d16-hybrid-disturbed-route-coeff-source-acquisition-001/artifacts/worker-handoff.md`
- `docs/work-packages/20260707-laned-router-d16-hybrid-disturbed-route-coeff-source-acquisition-001/artifacts/active-consumer-proof.md`
- `docs/work-packages/20260707-laned-router-d16-hybrid-disturbed-route-coeff-source-acquisition-001/artifacts/wepppy-implementation.md`
- `docs/work-packages/20260707-laned-router-d16-hybrid-default-promotion-001/artifacts/timing-and-fidelity.md`
- `docs/work-packages/20260707-laned-router-d16-hybrid-default-promotion-001/artifacts/worker-handoff.md`
- `/home/workdir/wepppy/AGENTS.md`
- `/home/workdir/wepppy/wepppy/wepp/management/managements.py`
- `/home/workdir/wepppy/wepppy/nodb/mods/disturbed/route_coefficients.py`
- `/home/workdir/wepppy/wepppy/nodb/mods/disturbed/disturbed.py`

On-demand references used during materialization:

- `tools/owcmp/suites/minnesota-corn-ksflag1.json`
- `tools/owcmp/suites/n-idaho-single-ofe-ksflag0.json`
- `tools/owcmp/suites/wa-cascades-mofe-ksflag0.json`
- `/wc1/runs/*/landuse/landuse.parquet`
- `/wc1/runs/*/disturbed/disturbed_land_soil_lookup.csv`
- `/wc1/runs/*/wepp/runs/p*.{run,man,slp,cli,sol}`

Reading disposition:

- `SC-OFEROUTE-001` confirms active owner, active closure, and parent routing
  surfaces.
- `SC-OFEROUTE-002` confirms hybrid remains experimental and promotion depends
  on production-facing evidence; this package does not alter selector posture.
- The LANUSE authority contract plus the prior source-acquisition package
  authorize WEPPpy Disturbed as the native route-coefficient producer.
- `tools/owcmp` inventory manifests remain preflight declarations; package-local
  run evidence is needed for this hold-lift.

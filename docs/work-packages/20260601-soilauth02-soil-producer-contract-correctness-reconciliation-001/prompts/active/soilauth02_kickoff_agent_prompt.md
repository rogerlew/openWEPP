Scope: local workspace engineering task; flat-file reads/edits only; no
external connectivity.

Execution mode: package-end-to-end (default).

Phase plan: execute all phases in package.md sequentially through disposition.

Required reading (read before edits):
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/work-packages/20260601-soilauth02-soil-producer-contract-correctness-reconciliation-001/package.md`
- `/workdir/openWEPP/docs/work-packages/20260601-soilauth01-soil-producer-contract-conformance-audit-001/artifacts/soilauth01-producer-conformance-matrix.md`
- `/workdir/openWEPP/docs/specifications/wepp-input-files/specs/soil-file.spec.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-SOIL-001.md`
- `/workdir/openWEPP/crates/openwepp-input-contract/src/parsers/soil.rs`
- `/workdir/openWEPP/crates/openwepp-hillslope-orchestrator/src/runtime_inputs/02_soil_slope.rs`
- `/workdir/wepppy/wepppy/weppcloud/routes/usersum/input-file-specifications/soil-file.spec.md`
- `/workdir/wepppy/wepppy/wepp/soils/utils/wepp_soil_util.py`
- `/workdir/wepppy/wepppy/soils/ssurgo/ssurgo.py`
- `/workdir/wepppy/wepppy/nodb/mods/disturbed/disturbed.py`

Files:
- `docs/work-packages/README.md`
- `docs/work-packages/20260601-soilauth02-soil-producer-contract-correctness-reconciliation-001/**`
- `docs/specifications/wepp-input-files/specs/soil-file.spec.md`
- `docs/specifications/science-contracts/contracts/SC-INFILE-SOIL-001.md`
- `docs/specifications/science-contracts/index.md`
- `crates/openwepp-input-contract/src/parsers/soil.rs`
- `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/02_soil_slope.rs`
- `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/08_tests.rs`
- `tests/fixtures/infile/soil/**`
- `tests/integration/soilauth02_soil_producer_reconciliation_contract.rs`

Task: execute SOILAUTH02 end-to-end by reconciling P0/P1 mismatch items from
SOILAUTH01, updating authoritative contract surfaces, parser/fixture evidence,
and producer-side implementation behavior where ownership is in `wepppy`.

Constraints: contract-first sequencing; no silent defaults; every closure must
reference exact provenance from SOILAUTH01 and updated source evidence.

Autonomy: execute package phases end-to-end and update required artifacts
without requesting additional user direction unless hard-blocked.

Outputs: update package artifacts/disposition for all completed phases with
truthfulness labeling (`Static:` vs `Ran:`).

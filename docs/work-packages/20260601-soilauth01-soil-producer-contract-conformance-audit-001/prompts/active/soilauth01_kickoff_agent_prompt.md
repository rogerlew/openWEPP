Scope: local workspace engineering task; flat-file reads/edits only; no
external connectivity.

Execution mode: package-end-to-end (default).

Phase plan: execute all phases in package.md sequentially through disposition.

Required reading (read before edits):
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/work-packages/20260601-soilauth01-soil-producer-contract-conformance-audit-001/package.md`
- `/workdir/openWEPP/docs/specifications/wepp-input-files/README.md`
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
- `docs/work-packages/20260601-soilauth01-soil-producer-contract-conformance-audit-001/**`

Task: execute SOILAUTH01 end-to-end by producing a datver-complete `.sol`
producer conformance matrix (`7778/9002/9003/9005`) and a prioritized mismatch
closure queue for SOILAUTH02.

Constraints: authoritative producer contract is
`docs/specifications/wepp-input-files/specs/soil-file.spec.md`; parser/runtime
authority is `SC-INFILE-SOIL-001`; use explicit provenance for every mismatch;
no speculative rewrites or undocumented fallback policies.

Autonomy: execute package phases end-to-end and update required artifacts
without requesting additional user direction unless hard-blocked.

Outputs: update package artifacts/disposition for all completed phases with
truthfulness labeling (`Static:` vs `Ran:`).

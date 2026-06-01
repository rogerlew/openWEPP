Scope: local workspace engineering task; flat-file reads/edits only; no
external connectivity.

Execution mode: package-end-to-end (default).

Phase plan: execute all phases in package.md sequentially through disposition.

Required reading (read before edits):
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/work-packages/20260601-soilauth03-soil-producer-contract-anti-drift-guards-001/package.md`
- `/workdir/openWEPP/docs/work-packages/20260601-soilauth02-soil-producer-contract-correctness-reconciliation-001/artifacts/soilauth02-reconciliation-gap-ledger.md`
- `/workdir/openWEPP/docs/specifications/wepp-input-files/specs/soil-file.spec.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-SOIL-001.md`
- `/workdir/openWEPP/docs/specifications/external-authority/required-suite-obligations.json`
- `/workdir/openWEPP/docs/specifications/external-authority/registry.yaml`

Files:
- `docs/work-packages/README.md`
- `docs/work-packages/20260601-soilauth03-soil-producer-contract-anti-drift-guards-001/**`
- `docs/specifications/wepp-input-files/specs/soil-file.spec.md`
- `docs/specifications/science-contracts/contracts/SC-INFILE-SOIL-001.md`
- `docs/specifications/science-contracts/index.md`
- `docs/specifications/external-authority/required-suite-obligations.json`
- `docs/specifications/external-authority/registry.yaml`
- `tests/integration/soilauth03_soil_contract_drift_guards_contract.rs`

Task: execute SOILAUTH03 end-to-end by implementing anti-drift guardrails for
`.sol` producer contract obligations (required symbols/order/arity) and fixture
provenance/hash integrity with explicit hard-fail release posture.

Constraints: contract-first sequencing; canonical `.sol` authority surfaces are
`soil-file.spec.md` and `SC-INFILE-SOIL-001`; no silent acceptance of missing
required fields; guard tests must include injected-drift red cases.

Autonomy: execute package phases end-to-end and update required artifacts
without requesting additional user direction unless hard-blocked.

Outputs: update package artifacts/disposition for all completed phases with
truthfulness labeling (`Static:` vs `Ran:`).

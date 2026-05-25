# MOFE04 Kickoff Agent Prompt

Scope: local repository science-contract/kernel publication-closure task;
flat-file reads/edits only; no external connectivity.
Execution mode: package-end-to-end (default).
Phase plan: execute all phases in package.md sequentially through disposition.

Required reading (read before edits):
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/work-packages/20260525-mofe04-output-publication-closure-001/package.md`
- `/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SED-001.md`
- `/workdir/openWEPP/docs/work-packages/20260525-mofe01-multi-ofe-readiness-assessment-and-queue-001/artifacts/mofe-readiness-assessment-report.md`
- `/workdir/openWEPP/docs/work-packages/20260525-mofe01-multi-ofe-readiness-assessment-and-queue-001/artifacts/mofe-readiness-wp-queue.md`
- `/workdir/openWEPP/docs/work-packages/20260525-mofe03-wave2-routing-activation-and-input-synthesis-001/artifacts/mofe03_disposition.md`

Files:
- `docs/work-packages/20260525-mofe04-output-publication-closure-001/**`
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md` (if authority amendments are required)
- `docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md` (if authority amendments are required)
- `crates/openwepp-runner/src/hillslope/mod.rs`
- `crates/openwepp-hillslope-output/src/hillslope_wat.rs` (if required)
- `tests/integration/cli03_runner_contract_derived_tests.rs`
- `tests/integration/cli04_runner_wat_parquet_contract_derived_tests.rs` (if required)

Task: execute MOFE04 objective end-to-end by closing multi-OFE WB13/WAT
publication assumptions with explicit, deterministic, contract-authoritative
output policy and provenance.

Constraints:
- contract-first sequencing is mandatory:
  1. contracts,
  2. contract-derived tests,
  3. pre-implementation contract gate,
  4. production code edits.
- do not edit production runner/output code before completing steps 1-3.
- canonical `SC-*` contracts remain authority; package notes are evidence only.
- no silent defaults/clamping/fallback on domain violations; use typed errors.
- preserve existing non-MOFE behavior unless explicitly required by contract.

Autonomy: execute package phases end-to-end and update required artifacts
without requesting additional user direction unless hard-blocked.

Outputs: updated MOFE04 artifacts, implemented publication-policy closure,
contract-derived integration tests, gate evidence, dual reviews/verifications,
and disposition.

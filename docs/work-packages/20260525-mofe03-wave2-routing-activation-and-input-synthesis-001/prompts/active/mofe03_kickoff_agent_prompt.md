# MOFE03 Kickoff Agent Prompt

Scope: local repository science-contract/kernel intake-activation task;
flat-file reads/edits only; no external connectivity.
Execution mode: package-end-to-end (default).
Phase plan: execute all phases in package.md sequentially through disposition.

Required reading (read before edits):
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/work-packages/20260525-mofe03-wave2-routing-activation-and-input-synthesis-001/package.md`
- `/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-RUNOFFPART-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-HYDRAULICS-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SED-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `/workdir/openWEPP/docs/work-packages/20260525-mofe01-multi-ofe-readiness-assessment-and-queue-001/artifacts/mofe-readiness-assessment-report.md`
- `/workdir/openWEPP/docs/work-packages/20260525-mofe01-multi-ofe-readiness-assessment-and-queue-001/artifacts/mofe-readiness-wp-queue.md`
- `/workdir/openWEPP/docs/work-packages/20260525-mofe02-cross-file-ofe-parity-hard-gate-001/artifacts/mofe02_disposition.md`

Files:
- `docs/work-packages/20260525-mofe03-wave2-routing-activation-and-input-synthesis-001/**`
- `docs/specifications/science-contracts/contracts/SC-SED-001.md` (if authority amendments are required)
- `docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md` (if authority amendments are required)
- `crates/openwepp-runner/src/hillslope/mod.rs`
- `tests/integration/cli03_runner_contract_derived_tests.rs`
- `tests/integration/erod14_contract_authority_closure_contract.rs` (if required)

Task: execute MOFE03 objective end-to-end by making EROD14 Wave-2 executable
from production runfile intake surfaces with explicit activation policy and
runtime symbol synthesis from parsed/runtime inputs.

Constraints:
- contract-first sequencing is mandatory:
  1. contracts,
  2. contract-derived tests,
  3. pre-implementation contract gate,
  4. production code edits.
- do not edit production runner/runtime code before completing steps 1-3.
- canonical `SC-*` contracts remain authority; package notes are evidence only.
- no silent defaults/clamping/fallback on domain violations; use typed errors.
- preserve existing non-MOFE behavior unless explicitly required by contract.

Autonomy: execute package phases end-to-end and update required artifacts
without requesting additional user direction unless hard-blocked.

Outputs: updated MOFE03 artifacts, implemented activation/seeding policy,
contract-derived integration tests, gate evidence, dual reviews/verifications,
and disposition.

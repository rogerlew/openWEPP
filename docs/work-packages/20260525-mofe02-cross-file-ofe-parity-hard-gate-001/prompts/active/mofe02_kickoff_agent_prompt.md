# MOFE02 Kickoff Agent Prompt

Scope: local repository science-contract/kernel intake-validation task;
flat-file reads/edits only; no external connectivity.
Execution mode: package-end-to-end (default).
Phase plan: execute all phases in package.md sequentially through disposition.

Required reading (read before edits):
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/work-packages/20260525-mofe02-cross-file-ofe-parity-hard-gate-001/package.md`
- `/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-SLOPE-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-SOIL-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-MANAGEMENT-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-RUNOFFPART-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SED-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-ROUTE-001.md`
- `/workdir/openWEPP/docs/work-packages/20260525-mofe01-multi-ofe-readiness-assessment-and-queue-001/artifacts/mofe-readiness-assessment-report.md`
- `/workdir/openWEPP/docs/work-packages/20260525-mofe01-multi-ofe-readiness-assessment-and-queue-001/artifacts/mofe-readiness-wp-queue.md`

Files:
- `docs/work-packages/20260525-mofe02-cross-file-ofe-parity-hard-gate-001/**`
- `docs/specifications/science-contracts/contracts/SC-INFILE-{SLOPE,SOIL,MANAGEMENT}-001.md` (if authority amendments are required)
- `crates/openwepp-runner/src/hillslope/mod.rs`
- `crates/openwepp-runner/src/errors.rs` (if required)
- `tests/integration/**` (MOFE02 contract-derived tests)

Task: execute MOFE02 objective end-to-end by implementing a hard cross-file OFE
parity gate for hillslope intake (`slope == management == soil`) with typed
hard-fail behavior before runtime surface merge.

Constraints:
- contract-first sequencing is mandatory:
  1. contracts,
  2. contract-derived tests,
  3. pre-implementation contract gate,
  4. production code edits.
- do not edit production runner/orchestrator code before completing steps 1-3.
- canonical `SC-*` contracts remain authority; package notes are evidence only.
- no silent defaults/clamping/fallback on parity mismatches; use typed errors.
- preserve existing non-parity behavior unless explicitly required by contract.

Autonomy: execute package phases end-to-end and update required artifacts
without requesting additional user direction unless hard-blocked.

Outputs: updated MOFE02 artifacts, implemented parity gate, contract-derived
mismatch tests, gate evidence, dual reviews/verifications, and disposition.

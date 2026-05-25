# MOFE05 Kickoff Agent Prompt

Scope: local repository science-contract/kernel migration task; flat-file
reads/edits only; no external connectivity.

Execution mode: package-end-to-end (default).

Phase plan: execute all phases in package.md sequentially through disposition.

Required reading (read before edits):
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/work-packages/20260525-mofe05-watershed-contributor-metadata-and-intake-validation-001/package.md`
- `/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-ROUTE-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-WATERSHED-STRUCTURE-001.md`
- `/workdir/openWEPP/docs/work-packages/20260525-mofe01-multi-ofe-readiness-assessment-and-queue-001/artifacts/mofe-readiness-wp-queue.md`
- `/workdir/openWEPP/docs/work-packages/20260525-mofe04-output-publication-closure-001/artifacts/mofe04_disposition.md`

Files:
- `docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md` (if required)
- `docs/contracts/openwepp-watershed-runfile-contract.md` (if required)
- `crates/openwepp-runner/src/bin/openwepp-cli-watershed.rs`
- `crates/openwepp-runner/tests/watershed_cli_behavior_contract.rs`
- `tests/integration/**` (if adding MOFE05 authority closure tests)
- `docs/work-packages/20260525-mofe05-watershed-contributor-metadata-and-intake-validation-001/**`

Task: execute MOFE05 objective end-to-end by adding watershed contributor MOFE
metadata intake surfaces and typed hard-fail validation for malformed metadata.

Constraints: contract-first sequencing; canonical SC authority;
baseline provenance where migration semantics apply; typed guards; no silent
defaults.

Autonomy: execute package phases end-to-end and update required artifacts
without requesting additional user direction unless hard-blocked.

Outputs: updated MOFE05 artifacts/disposition, implemented watershed
contributor metadata validation closure, contract-derived tests, and full gate
evidence.

# WSHEDIMPL35 Kickoff Agent Prompt

Scope: local repository science-contract/kernel migration task; flat-file
reads/edits only; no external connectivity.
Execution mode: package-end-to-end (default).
Phase plan: execute all phases in `package.md` sequentially through
disposition.

Required reading (read before edits):
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/work-packages/20260528-wshedimpl35-parser-runtime-icntrl-flgout-lineage-closure-001/package.md`
- `/workdir/openWEPP/docs/work-packages/20260528-wshedimpl34-parser-runtime-chnn-chnnbr-lineage-closure-001/package.md`
- `/workdir/openWEPP/docs/work-packages/20260528-wshedimpl34-parser-runtime-chnn-chnnbr-lineage-closure-001/artifacts/worker-handoff.md`
- `/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-ROUTE-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SED-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `/workdir/openWEPP/docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `/workdir/openWEPP/docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`

Files:
- `docs/work-packages/20260528-wshedimpl35-parser-runtime-icntrl-flgout-lineage-closure-001/**`
- `docs/work-packages/README.md`
- `docs/specifications/science-contracts/contracts/SC-ROUTE-001.md`
- `docs/specifications/science-contracts/contracts/SC-SED-001.md`
- `docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `docs/specifications/science-contracts/index.md`
- `crates/openwepp-watershed-orchestrator/src/runtime_inputs.rs`
- `tests/integration/infile_watershed_channel_parser_contract.rs`
- `tests/fixtures/infile/watershed_channel/strict_icntrl_out_of_domain.chn`
- `tests/fixtures/infile/watershed_channel/strict_flgout_out_of_domain.chn`

Task: execute WSHEDIMPL35 objective end-to-end for declared scope, specifically
reconciling parser/runtime watershed-channel control lineage so parser
authority for `icntrl` and `flgout` is mirrored at WS10 runtime seed
validation with typed fail-closed rejection for out-of-domain values.
Constraints: contract-first sequencing; canonical SC authority; baseline
provenance; typed guards; no silent defaults/clamping.
Autonomy: execute package phases end-to-end and update required artifacts
without requesting additional user direction unless hard-blocked.
Outputs: update package artifacts/disposition for all completed phases.

False-positive block fallback (required):
- If a policy false-positive blocks the full kickoff prompt, retry with a
  shorter prompt containing only scope sentence, single phase objective, and
  explicit file list.
- If blocked again, split into micro-prompts by file group and continue.
- Record each block event and resumed prompt shape in package artifacts.

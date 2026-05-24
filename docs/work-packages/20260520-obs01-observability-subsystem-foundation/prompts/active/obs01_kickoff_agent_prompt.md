# OBS01 Kickoff Agent Prompt

Execution mode: package-end-to-end (default).
Phase plan: execute all phases in package.md sequentially through disposition.
Autonomy: execute package phases end-to-end and update required artifacts without requesting additional user direction unless hard-blocked.
Required reading (read before edits):
- /workdir/openWEPP/AGENTS.md
- /workdir/openWEPP/docs/codex_exec_plans.md
- /workdir/openWEPP/docs/work-packages/README.md
- /workdir/openWEPP/docs/work-packages/20260520-obs01-observability-subsystem-foundation/package.md


You are executing OBS01 for openWEPP observability subsystem architecture.

Objectives:
1. Define first-class observability subsystem boundaries and responsibilities.
2. Define typed debug intent and structured trace/event schema requirements.
3. Define kernel stimulation and replay-window interfaces that avoid full
   end-to-end run dependency.
4. Produce migration/disposition plan away from ad-hoc `wepp_observe*` flags.
5. Promote stable OBS01 outputs into canonical subsystem specification files
   under `docs/specifications/subsystems/observability/`.

Constraints:
- Evidence mode: `Static` unless execution is explicitly run.
- Use `[DIRECT]` and `[INFERENCE]` tags per claim.
- Do not reintroduce parser-sidecar compatibility for `wepp_observe*`.
- Keep correctness over completion; unresolved critical interface ambiguity must
  be flagged as `HOLD`.

Required outputs:
- `artifacts/observability-subsystem-charter.md`
- `artifacts/kernel-stimulation-use-cases.md`
- `artifacts/typed-observability-intent-schema.md`
- `artifacts/trace-event-schema.md`
- `artifacts/replay-window-interface.md`
- `artifacts/legacy-observe-migration-plan.md`
- `docs/specifications/subsystems/observability/observability-subsystem-contract.md`
- `docs/specifications/subsystems/observability/debug-intent-schema.md`
- `docs/specifications/subsystems/observability/trace-event-schema.md`
- `docs/specifications/subsystems/observability/replay-window-interface.md`
- `docs/specifications/subsystems/observability/legacy-observe-migration.md`
- `artifacts/obs01_disposition.md` with artifact -> canonical mapping

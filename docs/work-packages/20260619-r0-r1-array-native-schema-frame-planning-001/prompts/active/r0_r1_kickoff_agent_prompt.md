# R0/R1 Kickoff Agent Prompt

Execute `/workdir/openWEPP/docs/work-packages/20260619-r0-r1-array-native-schema-frame-planning-001`
end-to-end as a planning-only package.

## Scope

Scaffold and execute R0/R1 planning artifacts only. Do not edit Rust, tests,
science contracts, output schemas, or runtime activation paths. Do not claim
runtime readiness while PERFDEEP07 remains in `HOLD`.

## Required Reading

Core:

- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/AGENTS.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/work-packages/20260619-r0-r1-array-native-schema-frame-planning-001/package.md`
- `/workdir/openWEPP/docs/architecture/array-native-runtime-specification.md`
- `/workdir/openWEPP/docs/decisions/0025-array-native-hillslope-day-frame.md`
- `/workdir/openWEPP/docs/work-packages/20260619-perfdeep06-array-native-fast-path-inventory-001/artifacts/perfdeep06-working-set-inventory.md`
- `/workdir/openWEPP/docs/work-packages/20260619-perfdeep06-array-native-fast-path-inventory-001/artifacts/perfdeep06-publication-operand-ledger.md`
- `/workdir/openWEPP/docs/work-packages/20260619-perfdeep06-array-native-fast-path-inventory-001/artifacts/perfdeep06-direct-frame-api-plan.md`
- `/workdir/openWEPP/docs/work-packages/20260619-perfdeep07-zero-cost-disabled-direct-frame-hydrology-001/artifacts/disposition.md`
- `/workdir/openWEPP/docs/work-packages/20260619-perfdeep07-zero-cost-disabled-direct-frame-hydrology-001/artifacts/gate-results.md`

Conditional:

- `/workdir/openWEPP/docs/specifications/science-contracts/AGENTS.md` if a
  contract or guard-authority change is discovered.
- `/workdir/openWEPP/crates/AGENTS.md` before any Rust edit.
- `/workdir/openWEPP/tests/AGENTS.md` before any test edit.

On-demand:

- current runtime source files named in the package manifest when static
  source references are needed.

Required-reading budget: `179056` bytes for the core evidence set, `OK`
(`<=400000` bytes). See `artifacts/required-reading-map.md`.

## Autonomy

Autonomy: execute the planning-only package through disposition, including
artifact updates and scoped docs validation, without asking for next steps
unless blocked.

## Subagent Authorization

Subagent authorization: none.

## Gates

- No Rust/test/contract/output-schema edits.
- R0/R1 planning artifacts complete.
- PERFDEEP07 hold-lift blocker recorded.
- Scoped markdown lint passes.
- `git diff --check` passes.
- Review and verification artifacts check gate legitimacy.

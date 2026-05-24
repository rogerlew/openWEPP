# PL10b Kickoff Agent Prompt

Execution mode: package-end-to-end (default).
Phase plan: execute all phases in package.md sequentially through disposition.
Autonomy: execute package phases end-to-end and update required artifacts without requesting additional user direction unless hard-blocked.
Required reading (read before edits):
- /workdir/openWEPP/AGENTS.md
- /workdir/openWEPP/docs/codex_exec_plans.md
- /workdir/openWEPP/docs/work-packages/README.md
- /workdir/openWEPP/docs/work-packages/20260523-pl10b-contract-blind-authority-and-conformance-001/package.md


You are executing `20260523-pl10b-contract-blind-authority-and-conformance-001`.

Domain context (explicit):
- This package is contract-authority and conformance governance between `PL10`
  and `PL11`, not a production kinetics implementation package.
- `PL10` removed first-slot dispatch coupling; `PL10b` must lock authoritative
  contract intent before `PL11` runtime projection expansion proceeds.
- Legacy provenance/comparator authority defaults to
  `/workdir/wepp-forest_260430_baseline` per ADR-0012.

Objectives:
1. Author PL transition-control/runtime-projection contract authority in a blind
   Phase 1 pass without reading openWEPP implementation code.
2. Amend canonical `SC-PLANT-001` with algorithm details needed for annual
   extension and perennial event/cycle payload semantics.
3. Author contract-derived conformance tests directly from contract assertions,
   invariants, symbols, and guards.
4. Execute contract tests against current implementation and record evidence.
5. Reconcile every gap as `contract defect`, `implementation defect`, or
   `ambiguous authority requiring escalation`, then patch queue dependencies.

Mandatory scope boundaries:
- Include: blind-authoring protocol+attestation, contract amendment evidence,
  contract-test specification, contract-test execution evidence, gap
  reconciliation matrix, and PL11 dependency patching.
- Exclude: implementing new production PL kinetics (`PL12`, `PL13`).
- Exclude: comparator closeout packages (`PL14`, `PL15`) and risk-acceptance
  closure of unresolved blockers without documented authority.

Mandatory constraints:
- During Phase 1, do not read `openWEPP` implementation code.
- Contract authority must satisfy:
  - `docs/specifications/science-contract-authoring-procedure.md`
  - `docs/specifications/science-contracts/kernel-process-contract-profile.md`
- Preserve canonical symbol continuity with legacy/openWEPP contract
  vocabulary; where boundary names differ, provide explicit alias mappings.
- No silent defaults/clamping for invalid runtime projection domains.
- Record evidence mode and source class explicitly (`Static:` / `Ran:`).
- Correctness over completion: unresolved authority-critical findings remain
  unresolved with explicit escalation/disposition, not implied closure.

Required outputs:
- `artifacts/pl10b-blind-authoring-protocol.md`
- `artifacts/pl10b-sc-plant-001-contract-amendment.md`
- `artifacts/pl10b-contract-test-specification.md`
- `artifacts/pl10b-contract-test-execution-evidence.md`
- `artifacts/pl10b-gap-reconciliation-matrix.md`
- `artifacts/pl10b-queue-dependency-patch-summary.md`
- `artifacts/pl10b-kernel-profile-compliance-checklist.md`
- `artifacts/worker-handoff.md`
- `artifacts/owned-file-manifest.md`
- `artifacts/gate-results.md`
- `artifacts/pl10b_disposition.md`
- `artifacts/review_agent_a.md`
- `artifacts/review_agent_b.md`
- `artifacts/verification_agent_a.md`
- `artifacts/verification_agent_b.md`

Required gates:
- Docs/governance checks must confirm artifact-to-artifact consistency:
  blind protocol -> contract amendment -> test spec -> execution evidence ->
  gap reconciliation -> queue patch -> disposition.
- Kernel profile compliance checklist must trace all required assertions to
  `SC-PLANT-001` authority text.
- `PL11` dependency update must be reflected in queue/dependency artifacts.
- If code is changed, run:
  1. `cargo fmt --check`
  2. `cargo clippy --workspace --all-targets -- -D warnings`
  3. `cargo test --workspace`
  4. `cargo deny check`

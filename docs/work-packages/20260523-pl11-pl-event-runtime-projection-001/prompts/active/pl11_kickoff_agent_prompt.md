# PL11 Kickoff Agent Prompt

Execution mode: package-end-to-end (default).
Phase plan: execute all phases in package.md sequentially through disposition.
Autonomy: execute package phases end-to-end and update required artifacts without requesting additional user direction unless hard-blocked.
Required reading (read before edits):
- /workdir/openWEPP/AGENTS.md
- /workdir/openWEPP/docs/codex_exec_plans.md
- /workdir/openWEPP/docs/work-packages/README.md
- /workdir/openWEPP/docs/work-packages/20260523-pl11-pl-event-runtime-projection-001/package.md


You are executing `20260523-pl11-pl-event-runtime-projection-001`.
Start only after `PL10b` disposition confirms blind-authority contract and
contract-test reconciliation closure for PL11 scope.

Objectives:
1. Close `PL09-GAP-004` and `PL09-GAP-005` by projecting annual and perennial
   transition-control payloads into deterministic runtime symbol families.
2. Add typed guard/failure behavior for invalid projection cardinality,
   indexing, bounds, and domain states.
3. Preserve typed-seam non-regression posture (`ARCH15`/`ARCH21`).
4. Treat contract authority as a completion gate: algorithm intent and
   symbol/guard authority must be explicit in `SC-PLANT-001` before disposition.
5. Enforce kernel profile consistency using
   `docs/specifications/science-contracts/kernel-process-contract-profile.md`.
6. Author/update contract-derived conformance tests and run a documented
   pre-implementation contract gate before runtime projection code edits.
7. Close all PL10b contract-conformance failures currently captured in
   `tests/integration/parser_runtime_seam_integration.rs` ignored gates:
   - `pl10b_contract_conformance_requires_annual_extension_projection_symbols`
   - `pl10b_contract_conformance_requires_perennial_cutday_indexed_projection`
   - `pl10b_contract_conformance_requires_perennial_grazing_cycle_payload_projection`
   - `pl10b_contract_conformance_rejects_invalid_grazing_window_domain`
   - `pl10b_contract_conformance_rejects_empty_perennial_grazing_cardinality`

Constraints:
- Do not implement production decomposition kinetics (`PL12`) or growth
  transition kinetics (`PL13`).
- Do not modify production runtime projection code until:
  1. contract authority updates are drafted, and
  2. pre-implementation contract-gate evidence is recorded.
- Do not introduce silent defaults/clamping for invalid payloads.
- Maintain canonical symbol continuity and explicit alias mapping where needed.
- PL11 disposition is not complete until the PL10b ignored contract-gate tests
  above pass when explicitly executed.

Required outputs are listed in `package.md` Deliverables.

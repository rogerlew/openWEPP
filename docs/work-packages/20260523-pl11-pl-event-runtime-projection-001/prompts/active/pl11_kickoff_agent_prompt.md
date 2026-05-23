# PL11 Kickoff Agent Prompt

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
6. Close all PL10b contract-conformance failures currently captured in
   `tests/integration/parser_runtime_seam_integration.rs` ignored gates:
   - `pl10b_contract_conformance_requires_annual_extension_projection_symbols`
   - `pl10b_contract_conformance_requires_perennial_cutday_indexed_projection`
   - `pl10b_contract_conformance_requires_perennial_grazing_cycle_payload_projection`
   - `pl10b_contract_conformance_rejects_invalid_grazing_window_domain`
   - `pl10b_contract_conformance_rejects_empty_perennial_grazing_cardinality`

Constraints:
- Do not implement production decomposition kinetics (`PL12`) or growth
  transition kinetics (`PL13`).
- Do not introduce silent defaults/clamping for invalid payloads.
- Maintain canonical symbol continuity and explicit alias mapping where needed.
- PL11 disposition is not complete until the PL10b ignored contract-gate tests
  above pass when explicitly executed.

Required outputs are listed in `package.md` Deliverables.

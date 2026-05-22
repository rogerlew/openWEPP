# PL02 Kickoff Agent Prompt

You are executing `20260522-pl02-plant-runtime-boundary-contract-001`.

Objectives:
1. Define the typed openWEPP runtime boundary contract for PL surfaces.
2. Define explicit mutable-state ownership for landuse-management schedule,
   growth, and decomposition/residue state families.
3. Define canonical symbol continuity and alias requirements for PL surfaces.
4. Define strict parser-to-runtime seam requirements and typed-failure policy.
5. Publish implementation-ready handoff constraints for PL03+ follow-ons.

Constraints:
- Correctness over completion; unresolved ambiguities remain `HOLD`.
- Preserve variable naming continuity with legacy WEPP symbols and explicit
  alias mappings where openWEPP boundaries differ.
- Do not invent physics or semantics; all claims require direct evidence from
  baseline source/contracts/spec references.
- Use truthfulness posture (`Static:` vs `Ran:`) in every artifact.

Required outputs:
- `artifacts/pl-runtime-boundary-contract.md`
- `artifacts/pl-runtime-state-surface-map.md`
- `artifacts/pl-runtime-ownership-matrix.md`
- `artifacts/pl-runtime-canonical-symbol-alias-requirements.md`
- `artifacts/pl-runtime-seam-requirements.md`
- `artifacts/pl02-follow-on-implementation-handoff.md`
- `artifacts/worker-handoff.md`
- `artifacts/owned-file-manifest.md`
- `artifacts/gate-results.md`
- `artifacts/pl02_disposition.md`
- `artifacts/review_agent_a.md`
- `artifacts/review_agent_b.md`
- `artifacts/verification_agent_a.md`
- `artifacts/verification_agent_b.md`

Required gates:
- Docs-only execution: artifact completeness and consistency checks.
- If code is changed, run:
  1. `cargo fmt --check`
  2. `cargo clippy --workspace --all-targets -- -D warnings`
  3. `cargo test --workspace`
  4. `cargo deny check`

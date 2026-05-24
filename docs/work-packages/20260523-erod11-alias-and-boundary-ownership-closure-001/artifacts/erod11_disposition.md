# EROD11 Disposition

Status: `completed`
Evidence mode: `Static + Ran`

## Disposition

- Package state: `completed`
- Scope outcome: `implemented` (governance/contracts + contract-derived test)
- Production kernel code changes: `none`

## Exit Criteria Check

- [x] EROD10 alias-ambiguity Wave-0 gate is closed for required boundaries.
- [x] Canonical-to-runtime alias ownership is explicit in canonical companion
      `SC-*` contracts.
- [x] Required alias-ambiguity gap rows are dispositioned to `closed`.
- [x] Contract-first sequence evidence is complete:
  1. contract implementation,
  2. contract-derived tests,
  3. pre-implementation contract gate,
  4. no production edits in EROD11 scope.
- [x] Wave-0 alias-ambiguity disposition verdict is explicit.
- [x] Dual review/disposition/verification artifacts are complete.
- [x] Truthfulness labeling is present across artifacts.

## Governance Notes

- EROD11 now fully dispositions alias-ownership ambiguity for required
  boundaries but does not release production erosion-physics implementation
  gates.
- Scaffolded/placeholder physics acceptance remains prohibited and governed by
  downstream package holds.

## Post-Dispatch Review Finding Disposition

| finding_id | source | severity | decision | action_taken | artifact_ref | notes |
|---|---|---|---|---|---|---|
| `EROD11-RVW-001` | `agent_a` + `agent_b` | `high` | `amended` | Replaced substring-based gap assertions with row-scoped table parsing (`assert_gap_status`) and exact promotability checks. | `tests/integration/erod11_alias_boundary_ownership_contract.rs` | Prevents false-pass from unrelated `closed` tokens. |
| `EROD11-RVW-002` | `agent_a` + `agent_b` | `high` | `amended` | Added explicit assertions that all required non-promotable blocker gaps remain `non-promotable`. | `tests/integration/erod11_alias_boundary_ownership_contract.rs` | Preserves erosion-physics `HOLD` enforcement in contract-derived tests. |
| `EROD11-RVW-003` | `agent_a` | `medium` | `amended` | Updated `SC-WATBAL-001` registry note to explicitly state `GAP-WATBAL-002` remains open/non-promotable. | `docs/specifications/science-contracts/index.md` | Removes summary-layer ambiguity about blocking status. |
| `EROD11-RVW-004` | `agent_a` + `agent_b` | `medium/low` | `rejected` | Kept evidence token form ``[DIRECT][Static] + [Ran]`` unchanged. | `SC-SED-001`, `SC-HYDRAULICS-001`, `SC-ROUTE-001`, `SC-WATBAL-001`, `SC-RUNOFFPART-001` | Rejected because this token form is already established in canonical contracts (e.g., `SC-PLANT-001`) and changing only this lane would reduce global consistency. |

# SC-OFEROUTE-002 Verification Agent B Follow-Up

Scope: follow-up verification of the B-L1 delta only, plus any approval-lift
blocker affected by that delta.

Evidence:

- Static: re-read the current `SC-OFEROUTE-002` guard-map row for
  `INV-OFEHYB-006`, the prior `verification_agent_b.md` blocker, and current
  retained test names in `cascade.rs` and `kinematic_wave.rs`.
- Ran: scoped documentation/contract gates listed below.

Verdict: **GO** for approval lift from Agent B scope.

The B-L1 follow-up fix is verified. The `INV-OFEHYB-006` guard-map row no
longer uses the module shorthand "`rev30_deficit_carry_tests` functions"; it
now names the concrete retained vectors directly
(`docs/specifications/science-contracts/contracts/SC-OFEROUTE-002.md:309`):

- `absorb_deficit_exact_total_and_non_negative`
- `dispose_terminal_carry_material_deficit_fails_closed`
- `dispose_terminal_carry_subnoise_absorbs_backward_exactly`
- `dispose_terminal_carry_all_dry_subnoise_drop_is_bounded`
- `bin_recorder_returns_material_terminal_deficit_exactly`

The named tests exist at the current code anchors:

- `absorb_deficit_exact_total_and_non_negative`
  (`crates/openwepp-hillslope-orchestrator/src/ofe_routing/cascade.rs:1073`)
- `dispose_terminal_carry_material_deficit_fails_closed`
  (`crates/openwepp-hillslope-orchestrator/src/ofe_routing/cascade.rs:1099`)
- `dispose_terminal_carry_subnoise_absorbs_backward_exactly`
  (`crates/openwepp-hillslope-orchestrator/src/ofe_routing/cascade.rs:1116`)
- `dispose_terminal_carry_all_dry_subnoise_drop_is_bounded`
  (`crates/openwepp-hillslope-orchestrator/src/ofe_routing/cascade.rs:1150`)
- `bin_recorder_returns_material_terminal_deficit_exactly`
  (`crates/openwepp-hillslope-orchestrator/src/ofe_routing/kinematic_wave.rs:1388`)

No Agent B approval-lift blockers remain after this follow-up. This verdict is
limited to Agent B scope; it does not replace any required Agent A verification
or final disposition action.

## Gates Run

- `markdown-doc lint --path docs/specifications/science-contracts/contracts/SC-OFEROUTE-002.md --path docs/work-packages/20260707-laned-router-hybrid-contract-authority-001/artifacts/science-contracts/SC-OFEROUTE-002/verification_agent_b_followup.md`:
  PASS, 2 files, 0 errors, 0 warnings.
- `python tools/check_sc_binding_exposure.py docs/specifications/science-contracts/contracts/SC-OFEROUTE-002.md`:
  PASS-DEFERRED, 4 binding exposure rows, 4 science-review-follow-on rows.
- `bash tools/release/check_sc_unit_compliance.sh --path docs/specifications/science-contracts/contracts/SC-OFEROUTE-002.md`:
  PASS.
- `git diff --check`: PASS.
- Static retained-test lookup with `rg`: PASS, all five named tests found.

No production code was modified.

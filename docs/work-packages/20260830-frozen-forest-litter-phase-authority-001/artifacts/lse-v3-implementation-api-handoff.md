# LSE V3 public API handoff

Status: `READY FOR SURFACE-OWNER INTEGRATION`

Evidence mode: `Static + Ran`

- `migrate_v2_configuration_to_v3` and `migrate_v2_state_to_v3` perform the
  one-way immutable identity transition.
- `execute_litter_phase_v3(&LitterPhaseTransactionInput)` is the only complete
  vapor/phase candidate constructor. It takes exact support nanoseconds,
  transaction/OFE/tile/owner identities and owner-state digests, selected
  litter constants, immutable beginning liquid/ice/energy/temperature, the
  accepted phase-free atmospheric operands, and separately finalized signed
  liquid/ice rates.
- `AcceptedLitterPhaseCandidate` returns ending liquid/ice/energy/temperature
  plus a sealed `LitterPhaseReceipt`; it does not mutate either owner.
- `validate_litter_phase_receipt`, `litter_phase_receipt_json`, and
  `litter_phase_receipt_from_json` provide exact replay and fail-closed restart
  validation.
- `build_v3_ending_state` creates the non-mutating LSE candidate from accepted
  ending energy/temperature tile updates.

The integration owner must still cross-check receipt owner digests against the
surface-owner envelope, preserve phase-before-current-ingress/WB14 chronology,
exclude ice from WB14 and soil `frozwt`, and commit or roll back the complete
envelope atomically.

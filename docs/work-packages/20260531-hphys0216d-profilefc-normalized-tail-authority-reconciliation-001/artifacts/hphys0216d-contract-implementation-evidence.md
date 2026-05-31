# HPHYS0216D Contract Implementation Evidence

Status: completed
Evidence mode: Static

## Canonical authority intake
Read:
- `docs/specifications/science-contract-authoring-procedure.md`
- `docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `docs/specifications/science-contracts/contracts/SC-SOIL-001.md`
- `docs/specifications/science-contracts/contracts/SC-PERC-001.md`
- `docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`

## Contract amendments landed
1. `SC-WATBAL-001`
   - added `HPHYS0216D ProfileFC Layer+Tail Authority Reconciliation`.
   - authority now requires
     `ProfileFCStore = Σ(thetfc_i*dg_i)*1000 + wb13_profile_fc_tail_mm`.
2. `SC-SOIL-001`
   - added `HPHYS0216D ProfileFC Normalized-Tail Contribution Addendum`.
   - requires `wb13_profile_fc_tail_mm` publication and reconciliation.
3. `SC-PERC-001`
   - WB13 coupling requirements now include FC tail symbol guard posture.
4. `SC-SYSTEM-001`
   - added `HPHYS0216D ProfileFC Layer+Tail Boundary Authority Addendum`.

## Contract closure statement
- FC publication authority no longer depends on direct seed publication.
- Tail contribution is explicit, typed, and fail-closed at publication boundary.

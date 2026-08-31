# WGHL-FULL-001J native-V2 carrier joint custody

## Diagnosis and correction

Static: terminal reconstruction retains the selected unpublished V2 soil trial.
The prior carrier guard treated that trial ending as the typed owner for every
joint posture. Native V2 beginning joints instead contain the resident active
owner's canonical custody bytes; these are intentionally different payloads
(the diagnostic run observed 2,309 resident bytes versus 1,382/1,383 trial
ending bytes). Substituting the trial ending while validating a beginning joint
therefore produced `covered carrier typed/joint beginning`.

Static: `carrier_phase.rs` now represents the two lawful typed postures
explicitly. `ResidentBeginning` derives all six non-snow owners from
`DirectV10RealConsumerShadow::canonical_owner_state_bytes`. `CandidateEnding`
replaces only `soil_thermal` with the selected unpublished V2 trial ending.
Selection requires exactly one complete six-owner profile to match the sealed
seven-owner joint, except that byte-identical V1/no-candidate profiles are one
unambiguous identity. The V2 trial remains retained for exact carry and is not
projected, cached, installed, accepted, or receipted.

Static: no support, transaction, snow, LSE, surface-liquid, hydrology, BGC,
vegetation, receipt, or terminal-event field changed. Exact non-snow cardinality
is checked, stale V2 carry and substituted non-soil owner bytes fail closed, and
`owner_finalization.rs`, terminal execution, and open-snow sources were not
edited by this follow-up.

## Tests and validation

Ran the focused V2 custody and receipt-free carrier set:

```text
nix develop -c cargo nextest run -p openwepp-hillslope-orchestrator \
  -E 'test(native_v2_selected_joint_binds_resident_beginning_and_trial_ending_exactly) | \
      test(native_v2_selected_joint_rejects_stale_carry_and_substituted_owner) | \
      test(v2_carrier_composition_is_trial_only_and_receipt_free) | \
      test(phase_has_no_stage3_evaluation_or_publication_surface)' \
  --no-capture --no-fail-fast
```

- run: `cff1a59b-5b1e-44dd-919d-dec4c37be35a`
- result: `PASS`, 4/4; 1,186 skipped
- log: `/tmp/wghl-001j-carrier-v2-focused-final.log`
- SHA-256: `247a52e2414c7fd68885f614d8b85f142f6161370c9f940ea55d4980f841fc3e`

Ran the complete carrier-phase unit shard:

```text
nix develop -c cargo nextest run -p openwepp-hillslope-orchestrator \
  -E 'test(/covered_carrier_phase_tests/)' --no-capture --no-fail-fast
```

- run: `f2661b53-3c15-4cbf-81be-7c1cfaef136d`
- result: `PASS`, 7/7; 1,183 skipped
- log: `/tmp/wghl-001j-carrier-v2-shard.log`
- SHA-256: `674c4d3371869a39d2cf43a560845c00f947a18fb4f27b76dae661acf5ee3d41`

Ran `nix develop -c cargo check -p openwepp-hillslope-orchestrator --lib`:
`PASS`. Log `/tmp/wghl-001j-carrier-v2-check.log`, SHA-256
`c482532e7bfb65b8bde73fda6e12ccaac7bed163c1e877bc90279ba14b5c64b5`.

Ran individual Rust formatting and owned-path `git diff --check`: `PASS`.
`carrier_phase.rs` is 1,834 lines, below the 2,000-line warning threshold.
No public API or production diagnostic was added.

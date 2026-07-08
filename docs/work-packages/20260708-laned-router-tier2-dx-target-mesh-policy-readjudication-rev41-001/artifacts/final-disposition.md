# Final Disposition

Status: `EXECUTED-HOLD-DX5-PROMOTION-UNRATIFIED`
Evidence mode: Static + Ran.

## Outcome

The package re-adjudicated Tier-2 target-`dx` mesh policy on the
`SC-OFEROUTE-001` rev-41 solver and closes held.

What changed:

- WA high-resolution rungs now complete; the prior day-1122 closure blocker is
  lifted.
- `SC-OFEROUTE-001` rev 42 records the updated evidence posture.
- No production mesh-policy flip was made.

Why held:

- Fine-reference adequacy is still not fully closed:
  `mn_corn_h4` shape max L1 is `0.02018051100943346`, above the one-third
  threshold `0.0166667`.
- `dx5` is only provisional candidate evidence because the reference basis is
  not fully adequate.
- `dx5` costs `84.70 s` aggregate real-cohort user time versus `17.46 s` for
  fixed10.

Operational posture:

- Active production remains fixed `10 cells/OFE`.
- This package does not ratify fixed10 as fidelity-adequate.
- H2637 remains synthetic stress only.

## Gates

Final gates passed except the intentional hold gate:

- Full ladder: 24/24 rungs completed.
- Focused Case-4/Lane D tests passed.
- `cargo fmt --check`, clippy, full nextest, and deny passed.
- Contract/profile/unit checks passed.
- Markdown lint and `git diff --check` passed.

The only failed gate is the required fine-reference adequacy gate, which is the
hold condition.


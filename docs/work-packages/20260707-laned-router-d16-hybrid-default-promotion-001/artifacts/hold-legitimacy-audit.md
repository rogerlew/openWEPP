# Hold Legitimacy Audit

Status: EXECUTED-HOLD-FIDELITY-TOLERANCE. Evidence mode: Static + Ran.

## Hold Condition

`SC-OFEROUTE-002#INV-OFEHYB-008` cannot be closed for active-path default
promotion at the current 10-cell/OFE mesh because the package lacks a
contract-authorized fidelity tolerance for the observed H2637 active
plain-vs-hybrid publication deltas.

This is not a timing blocker:

- Active plain: `39.73 s` user / `0:39.75` wall.
- Active explicit hybrid: `33.45 s` user / `0:33.47` wall.
- Hybrid is `6.28 s` user faster (`15.8 %`).

This is not a Case-4 blocker:

- Ran `cargo nextest run -p openwepp-hillslope-orchestrator
  'ofe_routing::d10b_reconciliation_tests::case4_hybrid_manning_ladder_meets_iwagaki_oracle'
  --profile quick`.
- Result: 1 test passed in `144.949 s`.

## Evidence Proving The Blocker

Contract authority:

- `SC-OFEROUTE-002` lines 23-35: hybrid subsystem remains
  experimental/unpromoted and promotion is gated by the acceptance posture.
- `SC-OFEROUTE-002` lines 286-310: current selector is
  `OPENWEPP_LANED_ACTIVE_IMPLICIT=1`; unset means plain rev-27 active behavior.
- `SC-OFEROUTE-002` lines 394-398: promotion requires the Case-4 ladder plus
  named ratified fidelity/timing tolerances.
- `SC-OFEROUTE-002` lines 410-419: GAP-OFEHYB-002 ratifies only
  exact-evaluator numeric dust and explicitly does not change default or
  tolerance posture.

Fresh D16 evidence:

- Release binary SHA256:
  `57a5ffb0df6040d166d4d768439861dc1d4d138dfbb24af709bb785444cf62c8`.
- Active plain vs active explicit hybrid manifest delta:
  `total_routed_outlet_m3` moves by `-1646.0279772533 m3`
  (`-0.4395701615 %`).
- `H2637.hbp` changes hash:
  `efd8c4255fbe976ecafb2bc89defb7bebd4e2054c9e65c89cd5353c4c31c3790`
  -> `bfb2b002f8b67cd3c4b42504ae9cbc02189c13651f658b0c035c51cd23f50621`.
- `H2637.pass.parquet` changes hash:
  `21c54bf2b045c3fb2f79f39ca174e36a4d188b39f7064f2a75f1170be6bb1656`
  -> `44e3da28ed5a2c4b310507d8d2f03e65c3a902e2f01e59f08e11e732d80e1f34`.
- Pass parquet sums move by `-1.8883 %` for `tdet` and `-6.4742 %` for
  `sedcon_1..5`.
- These deltas are sparse but material; they are not the
  GAP-OFEHYB-002 branch-equilibrium dust ratified at `<= 3.84e-10` relative.

## Why Not Close In This Package

Closing promotion would require inventing or ratifying a production tolerance
for current-mesh H2637 publication deltas after seeing the deltas. That would
be a reverse-fit tolerance without a broader fidelity basis.

The safe in-envelope options were considered:

- **Flip default anyway:** rejected. It would violate
  `SC-OFEROUTE-002#INV-OFEHYB-008`, which says promotion holds until named
  fidelity/timing tolerances pass.
- **Ratify the observed H2637 deltas ad hoc:** rejected. The deltas include
  publication surfaces (`H2637.hbp`, pass sediment outputs), and no current
  contract authority says `0.44 %` routed outlet and `6.47 %` sediment movement
  are acceptable default-production changes.
- **Optimize or change the hybrid scheme:** out of scope for D16 promotion
  adjudication; that would be a new numerics/authority package.

## First Actionable Follow-On

Scaffold a fidelity-tolerance/default-promotion hold-lift package before any
selector flip. The first action should be:

1. Define production-facing hybrid promotion tolerances before new code:
   hydrograph/outlet timing and magnitude, HBP/output publication surfaces,
   and sediment-pass impacts.
2. Run a fixture cohort, not only H2637, at the current 10-cell/OFE mesh:
   at minimum H2637 plus contrasting dry, low-runoff, high-runoff, steep,
   and multi-event hillslopes.
3. If the cohort supports tolerances, amend `SC-OFEROUTE-002` and rerun D16.
   If it does not, keep hybrid explicit opt-in and move to Tier-2/Tier-1 or
   another scheme-level fidelity improvement instead.

No partial production flip was made.

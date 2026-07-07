# Promotion Readiness Audit

Status: EXECUTED-HOLD-HYBRID-VIABILITY. Evidence mode: Static + Ran.

## `INV-OFEHYB-008`

`SC-OFEROUTE-002#INV-OFEHYB-008` requires:

- Case-4 oracle subgate closure.
- Ratified fidelity tolerances.
- Ratified timing tolerances before any promotion beyond evidence-gathering.

The Case-4 subgate and the H2637 exact bare-skin solve-cost blocker are already
resolved by prior packages. The selected-cohort evidence now adds two current
promotion blockers.

## Blocker 1: Fidelity Tolerance

H2637 active plain-vs-hybrid deltas remain material and are not ratified for
default production:

- `total_routed_outlet_m3`: `-0.43957 %`.
- `tdet` pass sum: `-1.8883 %`.
- `sedcon_1..5` pass sums: `-6.4742 %`.
- `H2637.hbp` and `H2637.pass.parquet` hashes differ.

The GAP-OFEHYB-002 tolerance only ratified exact bare-skin branch-equilibrium
dust in the already-hybrid path. It does not authorize the broader active
plain-vs-hybrid production deltas.

## Blocker 2: Timing No-Harm

The selected cohort does not support default promotion as a general active-path
speedup:

- Aggregate selected user time is `57.34 s` plain vs `59.95 s` hybrid
  (`+4.55 %` hybrid).
- WA Cascades forest is `15.65 s` plain vs `24.50 s` hybrid (`+56.55 %`).
- N Idaho forest is `0.96 s` plain vs `1.23 s` hybrid (`+28.13 %`).

H2637 remains a strong positive signal, but current default promotion would make
some selected active workloads materially slower.

## Verdict

Current hybrid selector is not default-promotable at current mesh.

Promotion would need either:

- a selector that chooses hybrid only when predicted to be faster and within
  ratified fidelity bounds, or
- a solver improvement that makes generic non-bare implicit solves cheap enough
  for WA/N Idaho/MN style cases, plus fidelity/tolerance ratification.

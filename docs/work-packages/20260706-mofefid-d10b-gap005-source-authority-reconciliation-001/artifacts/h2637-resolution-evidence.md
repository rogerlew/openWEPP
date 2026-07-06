# H2637 Resolution Evidence (D10B)

Status: executed
Evidence mode: Ran (class fixture) + Static (real-fixture pathway note)

## Real-H2637 pathway status

The D10-era executed-vector shadow test no longer exists (D11 rev-20
fail-closed gate: the legacy H2637 fixture lacks native
`routing_coefficients`; the surviving test is
`h2637_legacy_shadow_fails_closed_without_routing_coefficients`). The
GAP-OFEROUTE-005 resolution-sensitivity class is therefore adjudicated on
the H2637-SHAPED class fixture (19 OFEs, steep 0.25-0.61 gradients, 20 m
OFEs, 10 cells/OFE — the shadow's working resolution — event-day pulse,
shadow window-clip rule), through the same `run_cascade` path the shadow
calls.

## Pre-correction reproduction (S0)

`logs/s0-seam-ledger-decomposed.json`: residuals 9.0% / 7.0% / 20.2% /
17.8% / 35.9% / 53.7% across the sweep grid — dt-non-monotone and
ANTI-CONVERGENT, reproducing and exceeding the shadow-recorded
6.0%/10.0%/22.1% class. Decomposed exactly into outflow-ghost
over-discharge + unbooked TVD boundary leak + inflow booking + handoff
quadratures (`seam-conservation-ledger.md`).

## Post-correction (S4)

`logs/s4-seam-ledger-final.json`: the cascade conservation residual is
ZERO to machine epsilon at EVERY sweep point:

| (sample_dt, max_dt) | pre-correction residual | post-correction residual |
|---|---:|---:|
| (900, 300) | 9.0% | 0.0 (gap 1.4e-14) |
| (900, 120) | 7.0% | 0.0 (gap 3.5e-15) |
| (120, 300) | 20.2% | 0.0 (gap 1.3e-14) |
| (120, 120) | 17.8% | 0.0 (gap 1.0e-14) |
| (60, 30) | 35.9% | 0.0 (gap 9.5e-15) |
| (15, 5) | 53.7% | 0.0 (gap 3.4e-14) |

Also enforced as a contract-derived test
(`nineteen_ofe_conservation_is_resolution_convergent`, residual < 1e-9 at
every point; measured <= 3.4e-14).

## Verdict

ACCEPTANCE-GRADE for the numerical-method class: the `GAP-OFEROUTE-005`
resolution-sensitivity class is eliminated at its mechanism, not merely
bounded. The shadow's `laned_shadow.rs` resolution note and the
`(900, 300)` operating constants remain valid (any operating point now
conserves identically); a real-H2637 shadow re-execution requires a
native `routing_coefficients` fixture (D11 gate) and belongs to the D15
rerun preflight, not this package.

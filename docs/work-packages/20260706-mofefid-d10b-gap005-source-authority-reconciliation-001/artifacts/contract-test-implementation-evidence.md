# Contract-Test Implementation Evidence (D10B S3)

Status: executed
Evidence mode: Ran

Contract-derived tests authored against rev 24, recorded FAILING 5/5
against the pre-correction scheme (`pre-implementation-contract-gate.md`),
and passing 5/5 after the S4 corrections:

| Test (`ofe_routing::d10b_reconciliation_tests`) | Contract binding |
|---|---|
| `case4_manning_solver_converges_to_iwagaki_oracle` | INV-OFEROUTE-011 rev 24/25 (oracle acceptance, ratified tolerances, Richardson reference, non-divergence) |
| `case4_manning_tvd_dissipation_is_mass_neutral_and_tv_diminishing` | INV-OFEROUTE-006(b) exact face-form mass-neutrality + rev-25 TV-transient bound |
| `solver_ledger_books_scheme_actual_boundary_fluxes` | INV-OFEROUTE-006(a) booked-equals-actual (Algorithm item 5) |
| `handoff_injection_is_flux_integral_conservative` | Algorithm item 6 (conservative handoff) |
| `nineteen_ofe_conservation_is_resolution_convergent` | INV-OFEROUTE-006(c) (19-OFE class fixture; exactness achieved) |

Plus oracle self-evidence tests (4) in `ofe_routing::iwagaki_oracle` and
the dispositioned behavior-pin updates (`behavior-pinned-test-audit.md`).

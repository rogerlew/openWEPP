# Final Disposition

Status: EXECUTED-COMPLETE-ADR0037-REMOVAL. Evidence mode: Static + Ran.

ADR-0037 has been executed end to end.

Completed:

- Final working hybrid state archived on
  `abandoned/hybrid-implicit-stepping` at
  `b1d5fd4410b700012d857ef4056000163e6aa6a0`.
- `SC-OFEROUTE-002` deleted from main and registry posture changed to
  `withdrawn`.
- `SC-OFEROUTE-001` rev 37 records removal of live hybrid rows while
  preserving historical revs 28-35.
- Hybrid implicit code, selector, manifest fields, implicit profile
  counters, and hybrid tests removed from main.
- `OPENWEPP_LANED_ACTIVE_IMPLICIT` now fails closed with an ADR-0037
  startup error when present.
- Z-shaped rating and selector-determinism knowledge extracted into
  `docs/numerics/`.
- Four-member active-plain HBP/pass-parquet identity passed exactly.
- Dual review / verification artifacts are complete, and all findings were
  dispositioned.
- Final canonical gates passed; see `artifacts/gate-results.md`.

No holds remain.

Next action: scaffold the separate Tier-2 Δx-target mesh-policy re-scope
package described in `artifacts/worker-handoff.md`.

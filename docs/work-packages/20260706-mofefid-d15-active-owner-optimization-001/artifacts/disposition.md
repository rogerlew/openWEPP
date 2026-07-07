# Disposition (D15A)

Status: **EXECUTED** (final call in `final-disposition.md` after dual review +
verification).

Evidence mode: Ran + Static per artifact.

## What this package did, in order

1. **S0-S2** — reproduced the blocker-package regression exactly (median
   `92.41 s` user shadow-on vs `2.45 s` off; counters identical to the
   recorded state), and attributed it with the persistent slots + `perf`:
   ~33 % libm `pow`, the CFL slot 3.65x D14 (the rev-26 perturbed celerity
   evaluation), steps ×1.64 (the corrected CFL trajectory).
2. **S3-S4** — landed five bit-identical optimizations (OPT-5..9): fixed
   point returns its own `(alpha, q)`; TV(q) diagnostic reuses cached/committed
   discharges; loop-invariant `slope.sqrt()`; lazy Froude; vegetation
   early-out; precomputed material-break flags. `92.4 → 78.8 s` user with
   protected outputs, the shadow manifest block, and all trajectory counters
   bit-identical; 67/67 focused tests green.
3. **S5** — adjudicated the remaining regression as contract-mandated (the
   decomposition closes to ~1 %); the D14 budget's basis (the pre-rev-24
   scheme) is superseded. Flagged for operator ratification.
4. **P0-P1** — operand-lineage table + architecture + readiness audit BEFORE
   production edits; `SC-OFEROUTE-001` rev 27 amended contract-first
   (selector, day window/reset, erosion tail-fold + full-mesh-hold
   degeneracy, activation tolerances, clamp booking).
5. **P2-P3** — the opt-in ACTIVE owner: two-phase active day loop,
   DC01-surface-disable + live INV-009 double-feed guard (lateral unchanged),
   live rev-27 day-closure hard-fails, D13 routed-erosion producer flip,
   rev-21 operand consumption, manifest `laned_active` evidence block, and
   the consumer-path proofs with negative old-path evidence. The closure
   hard-fails caught two real seam defects during implementation (mesh-basis
   aliasing; wet-gate/dry-floor inconsistency) — both fixed contract-first.
6. **P4** — executed active H2637: `37.4 s` user endpoint ×3, 610/731 days
   routed, machine-exact day closures (7.3e-16 / 2.5e-13 / 2.4e-13 maxima),
   1 counted degenerate lane-day; default/off byte-identical at three package
   checkpoints; full gate table in `gate-results.md` (fmt/clippy/full
   suite 1410/1410/deny/anti-evasion all green).

## Package-boundary honesty

- The activation claim covers the hillslope production runtime's water
  ownership + the erosion shape + closure + manifest evidence surfaces. The
  watershed-facing HBP outlet and per-lane WB publication stay
  SC-RUNOFFPART-owned lane-local products (named rev-27 follow-on), and
  active-mode erosion water MAGNITUDE is a named follow-on adjudication.
  Neither is a silent partial flip: DC01 surface runon is fully disabled on
  active lanes and nothing double-feeds.
- The first execution of the new ignored active integration test failed with
  its panic message lost to log truncation; the isolated rerun and the final
  gate rerun are the recorded resolution (`gate-results.md`); the code review
  assessed causes (inherited env — since neutralized — or a host transient).
- Post-review fix batch: QA-H2 seam repair (breakpoints + independent
  cross-ledger check, which caught a real 0.11 % booking error), CR test
  hygiene, latqcc all-days coverage + 1-ulp `sbrunv` reconstruction, contract
  reconciliation (QA-M1..M4) — all re-verified (`gate-results.md`).

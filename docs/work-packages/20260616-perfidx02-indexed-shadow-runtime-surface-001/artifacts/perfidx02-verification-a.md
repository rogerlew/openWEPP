# PERFIDX02 Verification A

Status: PASS 2026-06-16
Evidence mode: **Ran**

Verification focus: runtime evidence.

- H2637 without UI shadow report: `mismatch_count = 0`,
  `clone_sparse_speedup = 69.88178900746499`.
- H2637 with UI shadow report: `mismatch_count = 0`,
  `clone_sparse_speedup = 54.09607714529098`.
- OFE1-OFE5 shadow reports: every case `mismatch_count = 0` and sparse clone
  speedup is above `54x`.
- H2637 both UI variants plus OFE1-OFE5 completeness audits:
  `unknown_symbol_count = 0`.
- Anchor comparison: `ANCHOR_MISMATCHES=0`.
- Determinism comparison: `DETERMINISM_MISMATCHES=0`.

Verification result: PASS.

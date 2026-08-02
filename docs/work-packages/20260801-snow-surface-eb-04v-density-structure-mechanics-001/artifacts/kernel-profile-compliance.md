# Kernel-Profile Compliance

Status: `PASS`.

Evidence mode: `[Static] + [Ran]`.

- Canonical authority: `SC-SNOWFREEZE-001` revision 120.
- Implementation status: current Anderson/SNOBAL density process remains
  `IMPLEMENTED`; future process amendment remains `NOT_IMPLEMENTED`.
- Calibration status: `NOT_APPLICABLE`; no parameter was fitted.
- Identifiability: `PARTIALLY_IDENTIFIABLE`; exact process tendencies are now
  observable, but existing observations are diagnostic-only and drivers
  covary.
- Units and closure: all ledger contributions use `kg m^-3`; daily independent
  closure passed at `3.411e-13 kg m^-3` against the `1e-9 kg m^-3` tolerance.
- Consumer: the real direct-production JSONL path reads and publishes the typed
  ledger. This is not producer-only or shadow evidence.
- Guards: no bounded canonicalization, fallback masking, new default, unsafe
  block, or protected authority-suite binding was introduced.

The package therefore complies as behavior-neutral diagnostic observability.
It supplies no authority for calibration or promotion.

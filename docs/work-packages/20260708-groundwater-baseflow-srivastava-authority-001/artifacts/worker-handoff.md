# Worker Handoff

Status: M-T2B-ready authority handoff.

Canonical target:
`docs/specifications/science-contracts/contracts/SC-GWBASEFLOW-001.md`.

M-T2B should implement the contract-authorized linear groundwater-reservoir
baseflow process. First actionable sequence:

1. Add contract-derived tests for `TV-GWBASEFLOW-001` through
   `TV-GWBASEFLOW-008` before production implementation.
2. Consume `SC-INFILE-GWCOEFF-001` parser state exactly:
   missing sidecar disables the reservoir process; present malformed or
   out-of-domain sidecar fails closed; no coefficient defaults.
3. Implement one-hillslope daily recurrence from `SC-GWBASEFLOW-001` with
   explicit storage carry and domain guards.
4. Add MOFE/Lane D aggregation accounting so deep-percolation recharge,
   generated groundwater baseflow, deep seepage, `latqcc`, and
   `ui_SCrunf`/return-flow exfiltration remain separate.
5. Treat recharge, generated baseflow, and deep seepage pass fields as daily
   timestep volumes in `m^3`; convert to `m^3 s^-1` only at channel consumers.
6. Add boundary-symbol registry entries or record a blocking implementation
   hold for groundwater storage/recharge/baseflow/deep-seepage surfaces.
7. Prove the real pass/HBP or watershed consumer reads generated `gwbfv` and
   `gwdsv` before claiming export or publication closure.
8. Add publication metadata that distinguishes generated zero, disabled process,
   missing authority, and legacy-carried generated baseflow.

Protected boundaries:

- Do not implement nonlinear Srivastava et al. (2017) baseflow formulas under
  this authority.
- Do not collapse `latqcc`, generated groundwater-reservoir baseflow, and
  `chan.inp` `cbase`.
- Do not feed generated groundwater baseflow into Lane D's active surface-router
  source series.
- Do not claim consumer-path or publication closure with producer-only evidence.

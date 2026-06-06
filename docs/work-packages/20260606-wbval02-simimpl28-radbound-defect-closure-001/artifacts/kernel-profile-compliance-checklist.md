# Kernel-Profile Compliance Checklist

Status: complete

Evidence mode: mixed `Static:` and `Ran:`

- [x] Contract-first sequence followed.
  - Static: `SC-CLIMATE-001` version `23` was amended before production code.
- [x] Canonical `SC-*` authority cited for every kernel-affecting behavior.
  - Static: `INV-CLIMATE-006`, `INV-CLIMATE-013`, and pinned baseline
    `sunmap.for`/`radcur.for`/`hr_tmp.for` lineage are cited.
- [x] No silent defaults, clipping, or canonicalize-and-proceed behavior.
  - Static: invalid daily radiation now fails closed at `radly`.
- [x] Typed fail-closed behavior preserved or contract-amended before change.
  - Static: `RuntimeContextSymbolOutOfRange` remains the runtime error class;
    hourly HPHYS0277 guard remains in place.
- [x] Truthfulness labels present in evidence artifacts.
  - Static: artifacts use `Static:` and `Ran:` labels.
- [x] DC-ExecPlan conversion rule evaluated.
  - Static: seven-gate analysis identified source invalidity, not an
    in-envelope valid-radiation physics defect.
- [x] `HOLD` legitimacy, envelope adequacy, and protected-boundary integrity
      reviewed.
  - Static: package closes as validated invalid upstream input, not `HOLD`;
    upstream source boundary is defect-shaped in `worker-handoff.md`.
- [x] Validation commands run and recorded.
  - Ran: package tests, targeted tests, release build, six-wrapper validation,
    clippy, deny, and the workspace-test residual are recorded in
    `gate-results.md`.

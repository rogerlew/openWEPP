# HPHYS0240 Kernel Profile Compliance Checklist

Status: completed
Evidence mode: Static + Ran

Static: contract-first sequencing was followed:

- Contracts amended first: `SC-WATBAL-001`, `SC-RUNOFFPART-001`,
  `SC-SUBHYD-001`.
- Contract-derived tests added before production code.
- Pre-implementation contract gate recorded before production code.
- Production code changed only after the pre-code gate.

Static: canonical authority and no-proxy posture:

- Same-pass carryover authority is encoded in canonical `SC-*` contracts.
- No surrogate or heuristic process-physics formula was introduced.
- Carryover equation remains Chapter-5/Chapter-4 closure form with resolved
  `runoff_carryover` replacing stale-only `wb12_runon_input`.
- Malformed present carryover fluxes fail typed guards instead of being
  silently defaulted or replaced.

Static: error-handling posture:

- Present non-finite carryover flux maps to `HKERNEL-WB14-RUNOFF-E-002`.
- Present negative/out-of-range carryover flux maps through flux domain guards
  to `HKERNEL-WB14-RUNOFF-E-003`.
- State fallback validation applies only when carryover flux is absent.

Ran: required validation gates passed:

- `cargo fmt --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo test --workspace`
- `cargo deny check`

Static: source-level anti-evasion guards were not required for HPHYS0240 because
the package did not edit external-authority suite posture, cohort fixtures, or
required-case bindings.

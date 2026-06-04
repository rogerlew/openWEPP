# Kernel Profile Compliance Checklist

Status: completed/HOLD
Evidence mode: mixed static-and-ran

Static: HPHYS0277 is kernel-affecting because runtime projection controls
kernel-boundary hourly winter forcing symbols.

Ran: contract/test/gate evidence was generated locally; workspace-wide tests
remain HOLD for unrelated SIMIMPL18/WB11 ET failures.

## Checklist

- Contract-first sequencing: satisfied.
- Canonical `SC-*` authority before production edits: satisfied in
  `SC-CLIMATE-001` version `17`.
- Baseline provenance for physics constants: satisfied with pinned
  `radcur.for` lineage.
- Contract-derived red/green test: satisfied.
- Typed error for domain violation: satisfied with
  `RuntimeContextSymbolOutOfRange`.
- Silent defaults/clipping/substitution prohibited: satisfied; no clipping or
  compensation was introduced.
- H1/H7/H39 valid-run compatibility: satisfied.
- Full H1..H39 valid-run compatibility: satisfied.
- Dual review: satisfied; Review A reported no findings and Review B reported
  governance finding `B-1`.
- Review finding disposition: `B-1` accepted and dispositioned. Package status
  was moved back to `in_review/HOLD` until both verification artifacts existed,
  then closed to `completed/HOLD` after Verification A and Verification B found
  no blockers.
- Dual verification: satisfied; `verification_agent_a.md` and
  `verification_agent_b.md` are complete.
- Workspace gate: HOLD because `cargo test --workspace` fails in known
  SIMIMPL18/WB11 ET tests outside this package.

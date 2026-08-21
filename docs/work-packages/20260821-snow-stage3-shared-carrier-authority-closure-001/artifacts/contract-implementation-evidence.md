# Contract implementation evidence

Status: complete / authority draft

Evidence mode: Static + Ran

Static: canonical amendments are present in:

- `SC-COUPLEDTIME-001@3`, Child 2C shared-carrier and event-boundary amendment:
  active-participant maximum support, two-sided boundary predicates,
  proposal/accepted receipt fields, tie-breaking, retry, and `ERR-CT-021`.
- `SC-LANDSURFACEENERGY-001@7`, Child 2C successor-support amendment:
  accepted-event receipt join, post-event-only operands, and
  `LseSupportAdmissibilityReceiptV1` at `600000000 ns`.
- `SC-SNOWENERGY-001@14`, Child 2C shared carrier amendment: one shared node,
  complete turbulent residuals, reciprocal longwave, sealed exposure wind,
  typed wrong-regime/scope failures, and runtime `IMPLEMENTATION_MISSING`.
- `SC-VEGETATION-001@26` and `SC-VEGETATIONTRANSACTION-001@15`: V11 carrier
  ownership, exact-once flux lineage, event/support joins, rollback, and
  complete-owner-only commit.

Static: provenance is restricted to existing V8 shared tile-air/neutral-wind
authority, Stage 3 FSM2/libsnobal longwave and 5 m transfer geometry,
coupled-time chronology, and the released Child 2B support receipt. No
attenuation factor, sub-ULP storage rule, or new empirical parameter was
introduced.

Ran: `python3 tools/check_sc_binding_exposure.py --strict` on all five amended
contracts returned `PASS` with 4, 1, 13, 16, and 5 consolidated rows.

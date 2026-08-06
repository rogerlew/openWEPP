# Kernel Profile Compliance

Status: PASS after dual review and independent consumer QA.

Evidence mode: Static plus Ran on 2026-08-06.

- Authority: v128 precedes Rust and admits evaluation only; CoE remains the
  sole production melt owner.
- Units: every carrier component is retained as interval/daily `J m^-2`, vapor
  as `kg m^-2`, ice/melt as `kg m^-2`, support as seconds, and coverage as a
  fraction.
- Guards: unsupported/conflicting requests fail closed; primitive turbulence
  errors retain their typed `MeteorologyError` source and operator/forcing/
  geometry context.
- Conservation: both paired totals reconstruct from independent schema-v5
  operands; sequential energy, coverage, and production state/ledger identity
  are checked.
- Boundaries: internal active/lower conduction is explicitly labeled and no
  snow-ground, persistence, physical-chronology, terminal-recipient, seasonal,
  or cutover claim is made.
- Consumer/publication: evaluator evidence reaches only the enabled internal
  schema-v5 JSONL trace. The authoritative partition alone reaches production,
  and real enabled/disabled WAT/HBP/PASS writer bytes are exact.

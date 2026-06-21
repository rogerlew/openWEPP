# Kernel Profile Compliance

Static: planned and binding for R5C.

| Profile item | R5C disposition |
|---|---|
| Typed inputs | Required for decomposition and residue partition phases. |
| Direct compute | Required; PL17 decomposition scalar update is computed from direct fields. |
| State mutation | Required; direct day frame stores decomposition and residue partition states. |
| Downstream operands | Required; both phases publish typed downstream operand structs. |
| Shadow projection | Required; both phases project lane/day identity plus key operands. |
| Typed errors | Required through `DirectRuntimeError`; invalid domains fail closed. |
| No provisional production physics | Satisfied by limiting R5C to SC-RESIDUE-001 PL17 tracked seed-pool update and typed residue partition projection. |
| Public-output authority | Not changed; R6 owns publication cutover. |
| Compatibility isolation | Direct-runtime source must remain free of boundary symbols, request payloads, writeback surfaces, hot tables, and symbol lookups. |

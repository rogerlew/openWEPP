Evidence: Static

## Findings (Severity-Ranked)

### FDIR-A-001 — High
- File: `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-IRRIGATION-FIXEDDATE-001.md:172`, `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-IRRIGATION-FIXEDDATE-001.md:206`, `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-IRRIGATION-FIXEDDATE-001.md:231`, `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/irrigation-fixeddate-file.spec.md:202`
- Issue: Ordering behavior is not mode-complete. The contract enforces hard ordering closure (`1..itemp` deterministic order) via error guards, while the paired spec and gap register explicitly carry unresolved legacy warning-only ordering behavior. There is no explicit compatibility acceptance/rejection branch for ordering anomalies.
- Why it matters: Parser behavior for ordering faults is non-deterministic across strict vs compatibility implementations, and a known unresolved legacy branch is currently encoded as if fully settled.
- Proposed disposition: amend

### FDIR-A-002 — Medium
- File: `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-IRRIGATION-FIXEDDATE-001.md:177`, `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-IRRIGATION-FIXEDDATE-001.md:183`, `/home/workdir/openWEPP/docs/specifications/wepp-input-file-parser-contract-authoring-procedure.md:184`
- Issue: Boundary export mapping remains abstract (`parser output manifest`, `scheduler boundary`, `diagnostics boundary`) without concrete process-boundary surfaces or field-level boundary contracts.
- Why it matters: The parser-contract procedure requires explicit cross-boundary mapping so downstream interfaces can be implemented and verified consistently.
- Proposed disposition: amend

### FDIR-A-003 — Medium
- File: `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-IRRIGATION-FIXEDDATE-001.md:83`, `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-IRRIGATION-FIXEDDATE-001.md:230`, `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/irrigation-fixeddate-file.spec.md:95`, `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/irrigation-fixeddate-file.spec.md:200`
- Issue: `iryr` semantics are flagged as unresolved (calendar-year vs simulation-relative-year), but the simulation model currently publishes a single concrete `event_year` field with no explicit interpretation discriminator or guard-backed mode.
- Why it matters: Year semantics are execution-critical for event triggering; unresolved semantics need explicit model representation to prevent hidden interpretation drift.
- Proposed disposition: amend

Final recommendation: HOLD

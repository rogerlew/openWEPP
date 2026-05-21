Evidence: Static

## Findings (Severity-Ranked)

### SNOW-A-001 — High
- File: `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-SNOW-001.md:40`, `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-SNOW-001.md:49`, `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-SNOW-001.md:155`, `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/snow.spec.md:44`
- Issue: The contract does not define explicit handling for version/prefix variants (for example a leading version-like line). In compatibility mode, surplus-line acceptance can mask this into a semantically shifted parse.
- Why it matters: Unsupported-format precision is required; otherwise malformed files can parse into physically wrong parameters with only compatibility warnings.
- Proposed disposition: amend

### SNOW-A-002 — Medium
- File: `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-SNOW-001.md:118`, `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-SNOW-001.md:124`, `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-SNOW-001.md:165`
- Issue: `SNOW-E-002` is defined as missing-record closure, but guard mapping also reuses it for strict surplus/trailing-token policy failures.
- Why it matters: Distinct failure classes are conflated, reducing determinism for typed error handling and downstream disposition logic.
- Proposed disposition: amend

### SNOW-A-003 — Medium
- File: `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-SNOW-001.md:79`, `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-SNOW-001.md:123`, `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-SNOW-001.md:142`
- Issue: Compatibility acceptance of trailing tokens (`SNOW-W-002`) has no explicit per-line provenance field; only surplus-record count is modeled.
- Why it matters: Warning-class observability is incomplete for one compatibility branch, weakening auditability of strict-vs-compat parser outcomes.
- Proposed disposition: amend

Final recommendation: HOLD

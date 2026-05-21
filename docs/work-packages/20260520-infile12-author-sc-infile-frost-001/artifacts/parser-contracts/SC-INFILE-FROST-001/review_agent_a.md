Evidence: Static

## Findings (Severity-Ranked)

### FROST-A-001 — High
- File: `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-FROST-001.md:41`, `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-FROST-001.md:118`, `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-FROST-001.md:183`, `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/frost.spec.md:189`
- Issue: Version/prefix variant policy is unresolved (`FROST-GAP-002`) and not represented by explicit taxonomy/guard behavior.
- Why it matters: The contract requires explicit unsupported-form behavior; leaving this unresolved makes parser acceptance/rejection non-deterministic across implementations.
- Proposed disposition: amend

### FROST-A-002 — High
- File: `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-FROST-001.md:89`, `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-FROST-001.md:135`, `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-FROST-001.md:182`, `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/frost.spec.md:187`
- Issue: `kfactor(1..3)` class mapping remains HOLD-level unresolved, but propagation/cross-file text currently presents deterministic class semantics as if settled.
- Why it matters: Class-index mapping drives runtime conductivity controls; unresolved mapping cannot be treated as implementation-ready semantics.
- Proposed disposition: amend

### FROST-A-003 — Medium
- File: `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-FROST-001.md:77`, `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-FROST-001.md:127`, `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-FROST-001.md:169`, `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/frost.spec.md:136`
- Issue: Compatibility clamp/default observability is modeled as a single boolean (`legacy_clamp_applied`) and does not carry field-level clamp provenance despite field-specific warning behavior.
- Why it matters: Field-specific mutation provenance is needed to verify and debug compatibility normalization paths deterministically.
- Proposed disposition: amend

Final recommendation: HOLD

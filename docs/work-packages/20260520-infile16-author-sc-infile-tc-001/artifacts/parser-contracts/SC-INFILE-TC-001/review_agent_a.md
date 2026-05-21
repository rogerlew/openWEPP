Evidence: Static

## Findings (Severity-Ranked)

### TC-A-001 — High
- File: `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-TC-001.md:38`, `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-TC-001.md:65`, `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-TC-001.md:148`
- Issue: Strict-mode matrix/policy requires typed IO failure for sentinel open errors, but `luntc` derivation is defined as `1` on open success and `0` otherwise, collapsing open-fail with missing branch.
- Why it matters: This internal inconsistency can mask strict IO faults and produce divergent parser behavior.
- Proposed disposition: amend

### TC-A-002 — Medium
- File: `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-TC-001.md:118`, `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-TC-001.md:151`, `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-TC-001.md:63`
- Issue: Watershed-only applicability is guard-enforced (`G-TC-006`), but no explicit run-context model field is defined in field/propagation tables to drive that guard deterministically.
- Why it matters: Cross-context enforcement is under-specified and can vary by implementation.
- Proposed disposition: amend

### TC-A-003 — Medium
- File: `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-TC-001.md:39`, `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-TC-001.md:111`, `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-TC-001.md:140`, `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-TC-001.md:147`
- Issue: Sentinel-body content is declared accepted and semantically inert in strict and compatibility modes, but taxonomy/guarding emits compatibility warning semantics (`TC-W-003`) without a strict-mode observability rule.
- Why it matters: Strict-vs-compat behavior for content-insensitive bodies is not precisely specified for executable observability.
- Proposed disposition: amend

Final recommendation: HOLD

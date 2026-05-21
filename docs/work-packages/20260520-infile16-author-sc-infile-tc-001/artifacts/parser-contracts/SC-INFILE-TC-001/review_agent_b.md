# Review Agent B — SC-INFILE-TC-001

Evidence: Static

## Findings (severity-ranked)

### TC-B-001
- Severity: high
- File/line: `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-TC-001.md:38`, `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-TC-001.md:65`, `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-TC-001.md:75`, `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-TC-001.md:105`, `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-TC-001.md:148`
- Issue: Strict policy requires typed non-ENOENT open-failure handling (`TC-E-000`), but the `luntc` derivation/default still encodes open-fail => `0`, and propagation of `luntc` omits the open-error guard (`G-TC-003`).
- Why it matters: This can silently collapse strict IO faults into the missing-file branch, breaking strict/compat separation and making mode activation behavior non-auditable.
- Proposed disposition: amend

### TC-B-002
- Severity: medium
- File/line: `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-TC-001.md:39`, `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-TC-001.md:67`, `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-TC-001.md:111`, `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-TC-001.md:140`, `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-TC-001.md:128-129`
- Issue: The contract declares content-insensitive acceptance in both strict and compatibility modes, yet also emits a compatibility warning (`TC-W-003`) for ignored body content without defining a field-level warning trigger or boundary export distinction for that condition.
- Why it matters: Compatibility behavior must be precise and testable. Ambiguous warning semantics make parser outcomes non-deterministic across implementations and weaken observability contract completeness.
- Proposed disposition: amend

## Final recommendation
HOLD

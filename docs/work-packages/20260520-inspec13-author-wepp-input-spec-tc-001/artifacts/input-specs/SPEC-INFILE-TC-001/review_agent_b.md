# Review Agent B — SPEC-INFILE-TC-001

Evidence: Static

## Findings (severity-ranked)

### TC-B1
- Severity: medium
- File/line: `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/tc.spec.md:138`
- Issue: The gap/conflict register omits explicit row-level provenance tags (`usersum2024`, `legacy-code`, `wepppy`, `wepppyo3`, `literature`) required by the specification authoring procedure.
- Why it matters: Conflict disposition and verifier closure become non-deterministic without normalized provenance tagging.
- Proposed disposition: amend

### TC-B2
- Severity: medium
- File/line: `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/tc.spec.md:38`, `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/tc.spec.md:107`
- Issue: Strict-vs-compat behavior for open-failure conditions is only partially specified (`strict mode` typed error is stated, but compatibility-mode behavior is not explicitly typed in Section 8).
- Why it matters: Sentinel surfaces are execution-gating inputs; missing explicit compatibility taxonomy can produce divergent behavior across parser implementations.
- Proposed disposition: amend

### TC-B3
- Severity: medium
- File/line: `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/tc.spec.md:15`, `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/tc.spec.md:140`
- Issue: Core legacy-behavior authority is cited from a retirement snapshot path, but the spec does not add an explicit authority-resolution note that bounds this to legacy-compat provenance vs active source lineage.
- Why it matters: Source-authority ambiguity increases risk of drift when implementations are validated against current `wepp-forest` source trees.
- Proposed disposition: amend

## Final recommendation
HOLD

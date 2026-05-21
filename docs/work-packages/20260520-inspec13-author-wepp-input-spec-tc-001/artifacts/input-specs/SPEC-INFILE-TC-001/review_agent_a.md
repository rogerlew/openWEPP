# Review Agent A — SPEC-INFILE-TC-001

Evidence: Static

## Findings (severity-ranked)

### TC-A-001
- Severity: medium
- File/line: `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/tc.spec.md:138`
- Issue: Gap/conflict register omits explicit row-level provenance tags (`usersum2024`, `legacy-code`, `wepppy`, `wepppyo3`, `literature`).
- Why it matters: The input-spec authoring procedure requires provenance-tagged conflict rows for auditable authority arbitration and deterministic disposition.
- Proposed disposition: amend

### TC-A-002
- Severity: medium
- File/line: `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/tc.spec.md:38`, `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/tc.spec.md:107`
- Issue: Strict-mode open-failure behavior is identified, but compatibility-mode outcome for the same branch is not explicitly codified in typed expectations.
- Why it matters: Parser behavior can drift if strict-vs-compat policy is only partially specified for a key sentinel branch (`open` failure vs missing).
- Proposed disposition: amend

### TC-A-003
- Severity: low
- File/line: `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/tc.spec.md:143`
- Issue: `TC-GAP-004` is a naming/UX governance concern (omni key aliasing) but is currently modeled as a `HOLD` parser-correctness blocker.
- Why it matters: Mixing governance naming debt with parser correctness blockers reduces gate clarity and can stall promotion for non-correctness reasons.
- Proposed disposition: amend

## Final recommendation
HOLD

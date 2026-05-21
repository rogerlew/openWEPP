# Review Agent B — SPEC-INFILE-CHANINP-001

Evidence: Static

## Findings (severity-ranked)

### CHANINP-B1
- Severity: high
- File/line: `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/chaninp.spec.md:111`, `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/chaninp.spec.md:135`, `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/chaninp.spec.md:147`
- Issue: Legacy default/clamp behavior (`ichout`/`nchnum` normalization and `dtchr` normalization after error branches) is described, but Section 8 does not provide a complete strict-vs-compat typed taxonomy for these branches.
- Why it matters: This is executable parser-contract core behavior; incomplete mode typing can produce materially different routing/output behavior across implementations.
- Proposed disposition: amend

### CHANINP-B2
- Severity: medium
- File/line: `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/chaninp.spec.md:193`
- Issue: Gap/conflict register does not include explicit row-level provenance tags.
- Why it matters: Provenance-tagging is required for deterministic conflict disposition and reviewer/verifier traceability.
- Proposed disposition: amend

### CHANINP-B3
- Severity: medium
- File/line: `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/chaninp.spec.md:10`
- Issue: `last_updated_utc` is not rendered as an explicit UTC timestamp value (date-only form), unlike other specs in this corpus.
- Why it matters: Non-normalized metadata weakens auditability and revision-trace consistency across the specification set.
- Proposed disposition: amend

## Final recommendation
HOLD

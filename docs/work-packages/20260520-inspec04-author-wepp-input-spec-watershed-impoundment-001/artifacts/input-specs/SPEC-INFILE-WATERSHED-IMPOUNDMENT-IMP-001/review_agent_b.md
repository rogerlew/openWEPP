# Review Agent B — SPEC-INFILE-WATERSHED-IMPOUNDMENT-IMP-001

Evidence: Static

## Findings (severity-ranked)

### B1 — High
- File/line: `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/watershed-impoundment-file.spec.md:6`, `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/watershed-impoundment-file.spec.md:294`
- Issue: Spec status is `draft` while unresolved conflicts are explicitly tracked with HOLD conditions in the gap register.
- Why it matters: Authoring procedure requires unresolved source conflicts/gaps to remain in HOLD state; current lifecycle metadata is governance-inconsistent.
- Proposed disposition: `amend` (set status to `draft-HOLD` and align promotion language accordingly).

### B2 — Medium
- File/line: `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/watershed-impoundment-file.spec.md:29`, `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/watershed-impoundment-file.spec.md:40`
- Issue: Datver compatibility matrix permits legacy no-version behavior (`<=10`) but grammar defines only `datver_line jpond_line` preamble.
- Why it matters: Record grammar and version-policy sections are internally inconsistent, reducing parser-contract enforceability.
- Proposed disposition: `amend` (either add explicit legacy grammar variant or remove/support-disallow branch with typed error).

### B3 — Medium
- File/line: `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/watershed-impoundment-file.spec.md:202`
- Issue: Cross-file count constraint captures `npond > jpond` stop behavior but does not define normative treatment of `jpond > npond` (extra impoundment blocks).
- Why it matters: Parser closure behavior for extra records is a correctness-critical ambiguity for deterministic ingestion.
- Proposed disposition: `amend` (add explicit acceptance/rejection rule and typed error/warning behavior for surplus `.imp` records).

## Final Recommendation
`HOLD`

Rationale: One high-severity governance inconsistency and unresolved grammar/closure ambiguities block promotion.

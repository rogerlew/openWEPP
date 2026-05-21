# Review Agent B — SPEC-INFILE-WATERSHED-STRUCTURE-STR-001

Evidence: Static

## Findings (severity-ranked)

### B1 — Medium
- File/line: `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/watershed-structure-file.spec.md:65`, `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/watershed-structure-file.spec.md:74`
- Issue: Grammar/record definition does not provide an explicit normative row-count rule for `line_n` records (termination condition is implied, not stated as a contract rule).
- Why it matters: Draft completeness requires line-by-line format clarity; without an explicit record-count formula, truncated/extra-row detection and parser guard mapping are underspecified.
- Proposed disposition: `amend` (state required record count rule explicitly and tie it to expected topology cardinality inputs).

### B2 — Medium
- File/line: `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/watershed-structure-file.spec.md:55`, `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/watershed-structure-file.spec.md:124`
- Issue: Datver Case B (`ver <= 10`) is acknowledged as legacy-compat mode, but typed error/default section does not define the strict-mode failure/warning contract for this branch.
- Why it matters: Parser-contract procedure requires explicit guard mapping for rules/invariants; current text leaves enforcement behavior ambiguous.
- Proposed disposition: `amend` (add explicit typed outcome when Case B is disallowed and explicit warning/compat behavior when enabled).

### B3 — Low
- File/line: `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/watershed-structure-file.spec.md:170`
- Issue: Gap G1 references stronger usersum constraints than currently enumerated hard checks, but does not enumerate those additional constraints in a machine-actionable list.
- Why it matters: Disposition and later verifier closure are harder when gap scope is not explicit.
- Proposed disposition: `amend` (expand G1 with concrete rule bullets and intended enforcement stage).

## Final Recommendation
`HOLD`

Rationale: Required sections exist, but guard-path precision and topology row-count closure are not yet explicit enough for promotion.

# ASSURE-02 Handoff

Status: accepted and handed to ASSURE-03

## Decision Recorded

The user accepted the proposed scientific communication direction by
instructing execution of ASSURE-03. The accepted scope includes:

- ADR-0038 and the four-record/public-research-object boundary;
- the report standard and groundwater manuscript prototype;
- lifecycle ownership and independent review/impact rules;
- the exact v1 migration and `ASSURE03-REL-001` release conflict; and
- the independently closable ASSURE-03 through ASSURE-05 sequence.

The decision atomically accepts ADR-0038, activates the v2 report standard, and
finalizes v1 format retirement. It authorizes ASSURE-03 but does not declare
release safe, approve the groundwater prototype for publication, reassess
snow/frost, or authorize WEPPcloud vendoring.

## Executed Handoff

Scaffold ASSURE-03 from
`docs/planning/scientific-assurance-v2-migration-plan.md`. Its first technical
gate is `ASSURE03-REL-001`: separate ordinary validation from release assembly,
make release mode fail closed during transition, and add negative tests. Then
retire the v1 public candidate into the exact Git/hash recovery record and prove
the neutral zero-report surface.

The active package is
`docs/work-packages/20260714-assure03-v1-retirement-zero-report-001/`.

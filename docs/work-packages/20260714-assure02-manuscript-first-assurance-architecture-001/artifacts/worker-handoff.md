# ASSURE-02 Handoff

Status: held for user or named scientific-steward acceptance

## Decision Requested

Review the proposed scientific communication direction, especially:

- ADR-0038 and the four-record/public-research-object boundary;
- the report standard and groundwater manuscript prototype;
- lifecycle ownership and independent review/impact rules;
- the exact v1 migration and `ASSURE03-REL-001` release conflict; and
- the independently closable ASSURE-03 through ASSURE-05 sequence.

Acceptance must be explicit. It atomically accepts ADR-0038, activates the v2
report standard, and finalizes v1 format retirement. It authorizes scaffolding
ASSURE-03 but does not declare release safe, approve the groundwater prototype
for publication, reassess snow/frost, or authorize WEPPcloud vendoring.

## If Accepted

Scaffold ASSURE-03 from
`docs/planning/scientific-assurance-v2-migration-plan.md`. Its first technical
gate is `ASSURE03-REL-001`: separate ordinary validation from release assembly,
make release mode fail closed during transition, and add negative tests. Then
retire the v1 public candidate into the exact Git/hash recovery record and prove
the neutral zero-report surface.

## If Revisions Are Requested

Keep ASSURE-02 held, amend the proposed documents and prototype, and repeat the
affected independent reviews and verifications before requesting acceptance
again.

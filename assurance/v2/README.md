# Scientific Assurance V2 Internal Sources

Status: ASSURE-04A source admission; nonpublic

This tree holds canonical scientific-assurance source, not generated reader
documentation. The manuscript and supplement carry the scientific argument in
reader-first prose. Strict YAML records identify
authorship, accountable review roles, agent assistance, claims, methods,
dependencies, results, figures, references, research objects, review state,
and publication state; they do not generate conclusions or substitute
lifecycle labels for evidence. The current architecture fixture discloses its
agent author and blocks review while its human report lead and scientific
approver are unassigned.

The current groundwater report is a positive architecture fixture derived from
the accepted ASSURE-02 manuscript prototype. Its `DRAFT` and `fixture_only`
fields are governance controls, not a reader-facing assessment of the science.
It has no public route, review lock, export permission, release snapshot, or
vendoring authority.

## Layout

```text
assurance/v2/
  catalog.yaml
  schemas/
    catalog.schema.json
    report.schema.json
    result.schema.json
  reports/<report-id>/
    report.yaml
    manuscript.md
    supplement.md
    results/*.json
```

Every local claim-bearing file is a confined, regular, non-symlink repository
path bound by SHA-256. External evidence uses immutable identities. Restricted
evidence must disclose its restriction and review role without exposing a local
protected path or content digest.

## Validate

From the repository root:

```bash
cargo run --quiet -p openwepp-assurance -- validate --all
cargo run --quiet -p openwepp-assurance -- validate \
  --report linear-groundwater-reservoir-recurrence
```

Validation is deterministic and offline. It checks schema and contract
versions, content identities, logical-reference closure, units, unused
declarations, restrictions, and draft lifecycle consistency. It does not plan,
render, approve, publish, or scientifically reevaluate a report.

ASSURE-04B owns dependency planning, ASSURE-04C owns deterministic staging and
assembly, and ASSURE-04D owns review locks and promotion. Public scientific
communication remains under `usersum/` and continues to contain zero assurance
reports until an approved report is promoted.

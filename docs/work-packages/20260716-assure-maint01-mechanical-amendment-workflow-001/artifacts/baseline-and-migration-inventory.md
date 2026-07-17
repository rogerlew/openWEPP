# Baseline And Migration Inventory

Evidence class: Ran and static

Frozen base: `15763d7f6d5d4125333d9b7583424c714f5f5ea4`

## Protected Scientific Content

A normalized semantic comparison loaded each frozen and migrated `report.yaml`,
selected the science projection (`id`, version, trust domain, dependencies,
units, claims, methods, results, value bindings, tables, figures, references,
and research objects), and removed only derived SHA/root fields. Both
projections compared equal:

| Report | Result | Canonical semantic digest |
| --- | --- | --- |
| `linear-groundwater-reservoir-recurrence` | PASS — exact semantic equality | `76d2d4693f449c543a71cc63049a18f7b58ca91340514a0151a45d4fc640b84d` |
| `snow-and-frozen-soil-process-evaluation` | PASS — exact semantic equality | `b28dee0c929147602e89743b91c58810c9d56d88234bde4502e14d8a7229a850` |

The four manuscript/supplement diffs contain only replacement of duplicated
attribution/lifecycle prose with `{{assurance:attribution}}` and
`{{assurance:lifecycle}}` directives plus short introductions to those
generated regions. Claims, methods, values, tables, figures, evidence,
limitations, and conclusions are unchanged.

## Public And Lifecycle Boundaries

- `git diff --exit-code <frozen-base> -- usersum`: PASS; zero diff bytes.
- Production validation after migration: PASS; two internal reports and zero
  public reports.
- Groundwater remains `DRAFT` with no human report lead.
- Snow/frost remains `IN_REVIEW`; Roger Lew remains report lead and material
  producer, and independent approvals remain absent.
- Export, release transfer, vendoring, WEPPcloud, kernel, runtime, comparator,
  and science-contract sources are not mutation targets.

## Authored Versus Generated Ownership

Derived path digests and review roots were removed from authored catalog and
report YAML. The active identity is now owned by `identity.lock.json`; each
report has a generated `review.lock.json`; human-authority inputs are immutable
review events; and each transition is explained by a content-addressed receipt
under `assurance/v2/transactions/`.

# Worker Handoff

Status: complete

Evidence class: Static + Ran

The final maintenance interface is:

```text
openwepp-assurance amend admit-report \
  --report <report-id> \
  --path <repo-relative-report.yaml> \
  [--if-generation <generation-id>] \
  (--check | --apply)
```

`--check` is read-only and validates an invocation-owned candidate. `--apply`
atomically updates the catalog, identity and review locks, and canonical
root-bound receipt. Repeating the request at the installed generation is a
deterministic no-op.

The admitted V2 source catalog contains exactly:

1. `linear-groundwater-reservoir-recurrence`;
2. `snow-and-frozen-soil-process-evaluation`; and
3. `native-forest-canopy-phenology-evaluation`.

The stable disposable preview is:

```text
target/assurance-preview/usersum/assurance/reports/
```

Rebuild it with `openwepp-assurance build --all --staging-root
target/assurance-preview`, then verify it with the corresponding `check --all`.
The canopy report includes eight content-identified retained research SVGs,
sanitized for inline rendering, plus its generated transfer figure.

All three reports remain `DRAFT`. This handoff grants no scientific approval,
release transfer, snapshot authority, public catalog inclusion, or permission
to modify tracked `usersum/**`.

Future maintenance should extract another cohesive unit before either
`v2.rs` or `v2/amendment.rs` reaches 3,000 lines. A separate low-risk fixture
performance package may reduce the cost of full-tree transaction tests.

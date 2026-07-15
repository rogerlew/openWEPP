# ASSURE-03 Search And Link Classification

Status: implemented; review and terminal implementation searches passed

Evidence class: Static + Ran

## Classification

| Occurrence class | Disposition |
| --- | --- |
| Current public navigation/content | V1 IDs, routes, aggregate grades, and worksheet links absent. |
| Model-science narrative | Scientific rationale, datasets, quantitative findings, and limitations retained; premature v1 links/grade removed. |
| Current governance/standard | ADR-0038 and v2 contracts active; v1 standard final-retired; prohibited terms appear only as rules or historical explanation. |
| Migration authority/package | Old paths retained as the exact removal inventory, negative-test inputs, and recovery evidence. |
| Historical packages/ADRs | Preserved as factual evidence; not bulk-rewritten. |
| Code/test guard | Retired paths and candidate concepts appear only as negative inputs that must fail. |

## Active-Surface Search

Ran a scoped search over `usersum`, `assurance`, the workflow, release tools,
the assurance crate, and current governance/standards. No retired v1 stable ID,
public link, `CANDIDATE`, or `INSUFFICIENT_EVIDENCE` occurs in `usersum` or the
active catalog/export. Remaining matches are:

- the migration plan's explicit path inventory;
- the report standard's prohibited-headline example;
- the lifecycle contract's statement that no public candidate state exists;
- compiler/release negative-path lists; and
- package-local preservation and test records.

The exact pre-removal repository-wide hit set is recoverable from the frozen
commit. Historical package hits remain historical and are not current routes.

## Link Boundary

The neutral catalog links only to `../README.md#model-science`, which resolves
inside portable `usersum`. `usersum/README.md` links to the neutral catalog and
snow/frost narrative. The snow/frost narrative contains no link to removed v1
pages. No generated public page links into contributor-only `docs/`.

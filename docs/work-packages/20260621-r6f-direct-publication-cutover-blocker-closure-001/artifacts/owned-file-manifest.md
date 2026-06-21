# Owned File Manifest

Status: scaffolded.

Record every file touched by R6F. Do not edit outside the package write set
without amending `package.md` first.

| File | Edit class | In-scope rationale | Validation tied to edit |
|---|---|---|---|
| `docs/work-packages/20260621-r6f-direct-publication-cutover-blocker-closure-001/**` | Package artifacts | Required package evidence. | `wctl doc-lint --path docs/work-packages` |
| `docs/work-packages/README.md` | Catalog update | Active/held package pointer. | `wctl doc-lint --path docs/work-packages` |
| `docs/ROADMAP.md` | Roadmap update | Forward queue pointer for R6F. | `wctl doc-lint --path docs` if changed. |

## Execution Edits

| File | Edit class | In-scope rationale | Validation tied to edit |
|---|---|---|---|
|  |  |  |  |

## Out-of-Set Edit Requests

If execution discovers a necessary edit outside the declared write set, record
the reason here, update `package.md`, and continue only after the new boundary
is explicit.

| Requested file | Reason | Package amendment | Decision |
|---|---|---|---|
|  |  |  |  |

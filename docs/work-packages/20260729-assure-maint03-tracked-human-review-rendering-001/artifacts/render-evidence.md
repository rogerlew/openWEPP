# Tracked Review Render Evidence

Evidence class: Ran

The tracked review lane is
`usersum/assurance/review-drafts/`. It contains 92 regular files:

- one generated review-only catalog;
- three resolved `index.md` reports;
- three resolved supplements;
- three build manifests;
- 21 SVGs across displayed figures and linked research objects; and
- 61 additional public-safe research objects.

The exact relative-path inventory SHA-256 is
`172c4eb950f30e5fd706c3bc0fc795d38749498e108f3a9f632c5d6860ac584c`.
The ordered relative-path/content-digest stream SHA-256 is
`b7f5ede453605172375152c3206ad793384b3fce7b6d0d4edb1712155a18a9b3`.

The generated catalog links:

- `linear-groundwater-reservoir-recurrence/1.0.0/index.md`;
- `native-forest-canopy-phenology-evaluation/1.0.0/index.md`; and
- `snow-and-frozen-soil-process-evaluation/1.0.0/index.md`.

`render_assurance_review_drafts.py --apply` installed the complete tree.
An immediate independent `--check` rebuilt the real catalog in a new owned
temporary root and reported `PASS (92 files current)`.

All 25 Markdown files parsed through `cmark-gfm`. No unresolved typed
directives or known count-noun duplication remained. All 21 SVG files parsed
as XML and contained one direct title, one direct description, and
`role="img"`.

The first render exposed eight linked raw retained-SVG research objects without
accessibility metadata. The assembler was corrected to sanitize those consumer
copies with their declared figure title and alternative text. The regenerated
tree then passed the all-SVG check.

Independent Review B found two remaining `1 transitions` table cells. The
canonical `transition_count` unit now uses the inclusive symbol
`transition(s)`, and the typed source was readopted. The final render contains
`1 transition(s)` and `0 transition(s)` with no invalid plural form.

Protected-boundary SHA-256 values remained:

| Path | SHA-256 |
| --- | --- |
| `usersum/assurance/README.md` | `65115fe549cbee3107a120f2719b45c00ca2e63b49ebc5c6fc2d7ea350a3cb70` |
| `assurance/catalog.yaml` | `cb9cb601739b64f8cb497c0999ca268859946f2ce7e57532de8c08b9ed30801f` |
| `assurance/generated/wepppy-usersum.yaml` | `08b21cbe20ed059eb61f01f9965d8603779501bde90a728bc7a6c138a69258eb` |

`validate --all` reported three internal `DRAFT` reports and zero public
reports. No `usersum/assurance/reports/` directory exists.

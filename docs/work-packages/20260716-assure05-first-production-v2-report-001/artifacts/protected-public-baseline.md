# ASSURE-05 Protected Public Baseline

Status: FROZEN

Evidence class: Static

Before report-source edits, the tracked public surface contained exactly one
assurance file and zero reports:

| Path | SHA-256 |
| --- | --- |
| `usersum/assurance/README.md` | `65115fe549cbee3107a120f2719b45c00ca2e63b49ebc5c6fc2d7ea350a3cb70` |
| `assurance/templates/catalog.md` | `65115fe549cbee3107a120f2719b45c00ca2e63b49ebc5c6fc2d7ea350a3cb70` |
| `assurance/catalog.yaml` | `cb9cb601739b64f8cb497c0999ca268859946f2ce7e57532de8c08b9ed30801f` |
| `assurance/generated/wepppy-usersum.yaml` | `08b21cbe20ed059eb61f01f9965d8603779501bde90a728bc7a6c138a69258eb` |

The draft production-domain source under `assurance/v2/` is not a public report.
Unless exact named human approval and release-transfer gates pass, all four
hashes and the one-file `usersum/assurance` inventory must remain unchanged at
terminal disposition.

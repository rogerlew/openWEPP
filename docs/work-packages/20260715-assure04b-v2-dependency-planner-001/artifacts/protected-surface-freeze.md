# ASSURE-04B Protected-Surface Freeze

Status: frozen before implementation

Evidence class: Static

Frozen base: `22fb7dfbafdb9e82a42afe0a5356b4c923a45232`

| Path | SHA-256 |
| --- | --- |
| `assurance/catalog.yaml` | `cb9cb601739b64f8cb497c0999ca268859946f2ce7e57532de8c08b9ed30801f` |
| `assurance/templates/catalog.md` | `65115fe549cbee3107a120f2719b45c00ca2e63b49ebc5c6fc2d7ea350a3cb70` |
| `assurance/generated/wepppy-usersum.yaml` | `08b21cbe20ed059eb61f01f9965d8603779501bde90a728bc7a6c138a69258eb` |
| `usersum/assurance/README.md` | `65115fe549cbee3107a120f2719b45c00ca2e63b49ebc5c6fc2d7ea350a3cb70` |

The deterministic aggregate over sorted SHA-256 rows for every regular file
below `usersum/` is
`deb9f2c646aa5eb4ad9e427f8a7cec6ad51e3f9f6b47ffe29d14b4aed4bdcb7a`.
These bytes are outside the write set and must match at closure.

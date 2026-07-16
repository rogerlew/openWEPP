# ASSURE-04C Protected-Surface Freeze

Status: frozen at intake

Evidence class: Ran

Frozen base: `e704f0202278ebb86c6a8c667caf73d599be04ab`

| Surface | Intake SHA-256 |
| --- | --- |
| `assurance/catalog.yaml` | `cb9cb601739b64f8cb497c0999ca268859946f2ce7e57532de8c08b9ed30801f` |
| `assurance/templates/catalog.md` | `65115fe549cbee3107a120f2719b45c00ca2e63b49ebc5c6fc2d7ea350a3cb70` |
| `assurance/generated/wepppy-usersum.yaml` | `08b21cbe20ed059eb61f01f9965d8603779501bde90a728bc7a6c138a69258eb` |
| `usersum/assurance/README.md` | `65115fe549cbee3107a120f2719b45c00ca2e63b49ebc5c6fc2d7ea350a3cb70` |
| aggregate sorted `usersum/**` file-hash stream | `deb9f2c646aa5eb4ad9e427f8a7cec6ad51e3f9f6b47ffe29d14b4aed4bdcb7a` |

Inherited v2/API identities:

| Surface | Intake SHA-256 |
| --- | --- |
| `assurance/v2/catalog.yaml` | `e76d43e9ee337bf5678243a9b09b1f4c19eb5f2e8ea54a6af5ac485ab02324a8` |
| `assurance/v2/schemas/catalog.schema.json` | `7d15b4e56c2d519680ee906d2df1346a721a9dbcd2ec647fc7f3d787d2b6a520` |
| `assurance/v2/schemas/report.schema.json` | `70e09461fb223458c75726a7ce32038e84c62105e7b918bce0ffa68c937c5ba4` |
| `assurance/v2/schemas/result.schema.json` | `417efb4dbf2d9209cff3c41f52eca2637325c667dccc7c3588d14a0e8dc673a4` |
| `crates/openwepp-assurance/src/v2.rs` | `c95831852492bf2811f6c5ab772619af991457a591a7f0047540bfb5b25a343e` |
| `crates/openwepp-assurance/src/v2/planner.rs` | `aa3aa7eb35ec5c6dee4c09ee51c30acc00ca9473ed3b2629be5b9ff0791c0e18` |

The v2 source/schema/API rows are authorized implementation inputs and may
change inside the declared write set. The four public-transition files and
aggregate `usersum` identity are immutable closure gates.

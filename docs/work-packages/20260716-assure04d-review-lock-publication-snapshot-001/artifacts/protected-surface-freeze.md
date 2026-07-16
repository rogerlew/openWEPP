# ASSURE-04D Protected-Surface Freeze

Status: frozen at intake

Evidence class: Ran

Frozen base: `ec396c458a5015c504011a75814ff13e274544a1`

| Protected surface | Intake SHA-256 |
| --- | --- |
| `assurance/catalog.yaml` | `cb9cb601739b64f8cb497c0999ca268859946f2ce7e57532de8c08b9ed30801f` |
| `assurance/templates/catalog.md` | `65115fe549cbee3107a120f2719b45c00ca2e63b49ebc5c6fc2d7ea350a3cb70` |
| `assurance/generated/wepppy-usersum.yaml` | `08b21cbe20ed059eb61f01f9965d8603779501bde90a728bc7a6c138a69258eb` |
| `usersum/assurance/README.md` | `65115fe549cbee3107a120f2719b45c00ca2e63b49ebc5c6fc2d7ea350a3cb70` |
| sorted `usersum/**` file-hash stream | `deb9f2c646aa5eb4ad9e427f8a7cec6ad51e3f9f6b47ffe29d14b4aed4bdcb7a` |

Terminal ASSURE-04C input/API anchors:

| Surface | Intake SHA-256 |
| --- | --- |
| `assurance/v2/catalog.yaml` | `9cbee9307d0d6ae08a1a8c1f74b4e7700943d48bc48a5e15ff4cd00330d4650a` |
| `assurance/v2/schemas/catalog.schema.json` | `6db4343d92aac08083d1ff93053ef5f64daf8320247f9449341eb4cd882b0119` |
| `assurance/v2/schemas/report.schema.json` | `386eb54167a1c5579faa38c278e0336b25aa1078d0e223cf7cd14604ae15b896` |
| `assurance/v2/schemas/result.schema.json` | `417efb4dbf2d9209cff3c41f52eca2637325c667dccc7c3588d14a0e8dc673a4` |
| `crates/openwepp-assurance/src/v2.rs` | `f5c57358d3dd01b29ca4a78a864d7cdf451a3c583912db7e100663bda93d3ad1` |
| `crates/openwepp-assurance/src/v2/assembly.rs` | `a6f9e003b62c0748d084560ba6d59cd4b42a22f9b1e8e202ab815e8f74ffaef6` |
| `crates/openwepp-assurance/src/v2/confined.rs` | `14882e945478a87e05ed7f486fac1a5fd81f39cec6c588a0cd0c792b2da9c3a9` |
| `crates/openwepp-assurance/src/cli.rs` | `3b8c7b2a7ec9d23fe50eb821af79f546cdfcb45368829c8094054791c32d8539` |

The v2 source/schema/API anchors are authorized implementation inputs. The four
public-transition files, every tracked `usersum/**` byte, real release/snapshot
trees, exports, and vendor surfaces are immutable package closure gates.

## Focused Closure Recheck

Ran after implementation and retained-evidence generation:

- `git diff -- usersum assurance/catalog.yaml assurance/templates/catalog.md assurance/generated/wepppy-usersum.yaml`: empty;
- sorted `usersum/**` file-hash stream:
  `deb9f2c646aa5eb4ad9e427f8a7cec6ad51e3f9f6b47ffe29d14b4aed4bdcb7a`;
- all three protected assurance source hashes equal intake; and
- no tracked release, export, vendor, or WEPPcloud surface is in the write set.

Status: PASS at focused closure. Terminal verification must renew this check
after every accepted review remediation.

## Post-Review Remediation Recheck

Ran after the second-review fixes and renewed retained evidence. All four named
hashes and the sorted aggregate `usersum/**` hash stream equal intake. `git
diff` is empty for every protected surface, and no tracked release, export,
vendor, or WEPPcloud path appears in status.

Status: PASS. Terminal verification will renew once more after heavy evidence.

## Post-Heavy-HOLD Remediation Recheck

Ran after the strict-Clippy-only test restructuring and renewed focused
evidence. All four named hashes and the sorted aggregate `usersum/**` hash
stream still equal intake. The protected diff is empty, and all three touched
release scripts pass `bash -n`.

Status: PASS. The restarted independent heavy runner and terminal verifiers
must independently renew the freeze.

## Post-CRAP-HOLD Remediation Recheck

Ran after the production decomposition and focused coverage estimate. All four
named hashes and the sorted aggregate `usersum/**` hash stream still equal
intake. The protected diff is empty, all three release scripts pass `bash -n`,
and `git diff --check` passes.

Status: PASS. Independent review and the restarted heavy runner must renew the
freeze because production Rust changed after the second HOLD.

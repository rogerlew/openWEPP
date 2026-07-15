# ASSURE-04A Implementation And Consumer Evidence

Status: PASS — implementation and terminal gates complete

Evidence class: Static + Ran

## Implemented Source

- `assurance/v2/catalog.yaml` binds three JSON Schema 2020-12 companions and
  one report manifest by SHA-256.
- The report manifest identifies canonical manuscript and supplement prose,
  typed authorship and agent-assistance disclosures, ten dependencies, two
  units, all nine accepted claims, seven methods, two strict JSON result
  objects, two future result-bearing figure declarations, two references, two
  public-safe research-object declarations, one draft review, and one draft
  publication record.
- The groundwater source leads with positive key findings and a scientific
  manuscript structure. `DRAFT` and `fixture_only` remain internal governance
  fields and are not substituted for the scientific argument.
- No source, generated, export, or public file under the protected ASSURE-03
  surface changed.

Current direct source identities:

| Source | SHA-256 |
| --- | --- |
| v2 catalog | `e76d43e9ee337bf5678243a9b09b1f4c19eb5f2e8ea54a6af5ac485ab02324a8` |
| catalog schema | `7d15b4e56c2d519680ee906d2df1346a721a9dbcd2ec647fc7f3d787d2b6a520` |
| report schema | `70e09461fb223458c75726a7ce32038e84c62105e7b918bce0ffa68c937c5ba4` |
| result schema | `417efb4dbf2d9209cff3c41f52eca2637325c667dccc7c3588d14a0e8dc673a4` |
| report manifest | `39a69a4fe723b26842becf719e3df8380985b478022c477fca32b46b58bea3bb` |
| manuscript | `18a270516f1c5e221e1d9721e37bbb83d8aca69431952cf71336e7f35d30db13` |
| supplement | `77a3c8b804a1f6f01c1b1ae9f2ea9cc341f91efa95d827530c14e7f29f92d8fe` |
| two-day result | `41ada54b6ce96cc897bc7125ba737bab8194835488672903f717c2f350c6e483` |
| H2637 result | `5fc3aa1834a41f277bd750373bd50c4223a5cf8503e25f3f16c13e509faed82d` |

## Executable Consumer

`V2Repository::open` admits the catalog and schema registry. Named validation
then traverses only the selected report's complete source set; all-report
validation traverses every catalog report in stable ID order. Both routes use
the same manifest, content, identity, reference, unit, restriction, and
lifecycle validation functions.

The real CLI first validates the separate ASSURE-03 public boundary and then
calls the v2 repository:

```text
validate --all
validate --report linear-groundwater-reservoir-recurrence
```

Both current one-report commands produced repository root
`ac01170fe76ea5f56dd8ec7b75734f09df86589dde8a8ab6f907fc6834504e93`
and report root
`f303e702916c93202e0b79500e4c3aeec3108865acc897c663c6625878c28575`,
with one selected internal `DRAFT` fixture and zero public reports.

`plan --all`, `build --all`, `check --all`, and zero-report snapshots retain
the ASSURE-03 implementation. Report-specific plan/build/check reject the
request and name ASSURE-04B or ASSURE-04C as owner.

## Focused Execution

Ran:

```text
cargo nextest run --profile quick \
  --test assurance_v2_source_contract \
  --test assurance_dossier_build_contract
```

Current post-B-T01-remediation result: PASS, Nextest run
`3971cb34-0b18-451b-b52e-2db7c483888c`, 25/25 after presence-aware nullable
admission. The suite covers real named/all CLI
consumers, unselected-report isolation,
unknown/missing fields, duplicate/unresolved/unused identities, unknown units,
hashes,
versions, required-nullable omission across five record families, real Draft
2020-12 schemas, nested schema and constant drift,
confinement, symlinks, special entries, restricted evidence, authorship and
agent-provenance blockers, lifecycle contradictions, every record family,
protected bytes, and all ASSURE-03 compatibility cases.

Focused `cargo clippy -p openwepp-assurance --all-targets -- -D warnings`,
`cargo fmt --check`, real `validate --all`, real `check --all`, and
`git diff --check` passed. Review remediation added the test-only `jsonschema`
dependency and reconciled `Cargo.lock`. The earlier post-decomposition heavy
sequence passed workspace Clippy, 1,985/1,985 selected full-profile tests,
dependency policy, and fresh adjudicated CRAP with zero actionable rows, but
B-T01 changed `v2.rs`. The complete amended-source sequence then passed
workspace Clippy, 1,986/1,986 selected full-profile tests, dependency policy,
and fresh adjudicated CRAP with zero actionable rows.

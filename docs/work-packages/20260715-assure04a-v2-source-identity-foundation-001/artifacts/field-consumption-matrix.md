# ASSURE-04A Field-Consumption Matrix

Status: implemented and tested

Evidence class: Static + Ran

Every admitted field is deserialized with `deny_unknown_fields` and then either
validated directly, used in a relation/identity, or both. Raw-file hashing is
not counted as semantic field consumption.

| Family | Fields consumed | Negative proof |
| --- | --- | --- |
| Catalog | versions, source state, schema entries, report entries | unknown/missing/version/schema-set/duplicate-ID/path tests |
| Schema companion | dialect, ID, title, object type, unknown-field posture, required fields, properties, definitions, version/lifecycle constants | real Draft 2020-12 validation plus top-level, nested-field, and constant drift mutations with refreshed catalog hashes |
| Report | versions, ID/version/title/owner binding, lifecycle, fixture flag, all record arrays | unknown/missing/version/binding and nonempty-family guards |
| Authorship | identity, draft authors, human lead, scientific approver, accountability state, external-review claim | unassigned-role and false-peer-review contradiction guard |
| Agent assistance | identity, procedure/model, objective, input and exact-output dependencies, disposition, nondeterminism, limits, review, provenance and authorization flags | dependency-family closure and incomplete-provenance review-entry guard |
| Content | identity, path/hash/media type, provenance/procedure, all reference lists | media-type, unsafe path, hash, unknown/duplicate reference tests |
| Dependency | identity, kind, provenance/procedure, access/license, local/external/restricted identity fields | blank license, hash/path/symlink/special, restricted-leak tests |
| Unit | ID/symbol/quantity/definition | blank definition, unknown result unit, unused unit tests |
| Claim | identity, statement/type/scope, method/result/dependency/unit/reference links | blank statement and unresolved/duplicate/unused tests |
| Method | identity, description/procedure, dependency/unit links | blank procedure and reference closure tests |
| Result | identity, path/hash/media, method/dependencies/units, semantics/precision/realization/provenance/procedure | blank precision, schema/ID/unit/nonfinite/hash tests |
| Result value | ID, finite value, unit, precision | strict JSON, duplicate ID, unknown unit, empty precision checks |
| Figure | identity, kind/results, procedure, alternative text, caption | blank alternative text and kind/reference checks |
| Reference | identity, citation/immutable identity/access/license/dependency | blank immutable identity and dependency closure tests |
| Research object | identity, access/license, safe path/hash or restriction fields, relations, instructions | blank instructions and safe/restricted nonleakage checks |
| Review | identity, state/decision/root/approvers/independence | premature approval mutation |
| Publication | identity, state, public/snapshot/release/export/vendor/supersession/withdrawal fields | premature export mutation and full contradiction guard |

Ran: focused Cargo tests passed 24 tests after review remediation, including
real Draft 2020-12 schema execution. Terminal full-profile Nextest passed all
1,985 selected tests. The field-family mutation table lives in
`assurance_v2_source_contract::every_record_family_has_executable_field_consumption`.

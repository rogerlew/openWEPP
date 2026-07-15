# ASSURE-04A Review Finding Disposition

Status: PASS; all findings accepted, remediated, and independently re-reviewed

Evidence class: Static + Ran

No finding was rejected or deferred. Coding-agent review is internal software
and architecture review; it is not scientific approval or external peer
review.

| Finding | Disposition | Remediation and proof |
| --- | --- | --- |
| A1: accepted claim meanings collapsed | Accepted | Restored separate `GW-P01`–`GW-P09` source records, bindings, and supplement rows. Added exact content-identified consumer (`708a5d…`) and router-exclusion (`fed22f…`) dependencies for `GW-P06`, `GW-P07`, `GW-METHOD-CONSUMER`, the supplement, and agent input provenance. |
| A2: DOI citation materially wrong | Accepted | Corrected manuscript, dependency, and reference to Srivastava, Dobre, Wu, Elliot, Bruner, Dun, Brooks, and Miller (2013), *Transactions of the ASABE* 56(2), 603–611, DOI `10.13031/2013.42691`. |
| A3: authorship/accountability/agent provenance absent | Accepted | Added typed authorship and agent-assistance records, visible manuscript disclosure, conclusions, and About metadata. Unassigned human lead/approver and incomplete historical agent configuration mechanically block review entry; no external peer review is claimed. |
| A4: companion schemas permissive/nonexecutable | Accepted | Strengthened versions, IDs, paths, cardinality, uniqueness, and kind/lifecycle conditionals; added test-only `jsonschema` Draft 2020-12 execution for all three real sources plus negative schema vectors. |
| B1: global relationship closure | Accepted | Replaced the global reference lookup with family-specific dependency/unit/claim/method/result/figure/reference/research-object sets and added a wrong-family regression vector. |
| B2: nested schema drift admitted | Accepted | Bound every required nested definition field set and result-value fields to the typed contract; added a refreshed-hash unit `definition`→`meaning` adversarial vector. |
| B3: schema constants not inspected | Accepted | Bound schema, contract, lifecycle, fixture, and source-state constants to executable values and added a refreshed-hash version-constant vector. |
| A5/B4: schema/Rust lexical mismatch | Accepted | Aligned Rust with the companion ID grammar (alphanumeric first byte) and semantic-version grammar (three ASCII-numeric components, no leading zeros); loader-level reconciled-hash vectors reject `-leading-punctuation` and `00.1.0`. |
| B-T01: schema-required nullable omission admitted | Accepted | Replaced plain `Option<T>` with a three-state missing/null/value admission type for all 16 affected fields. Added five-family omission vectors. Independent remediation re-review reproduced the original attack and observed fail-closed rejection; focused Nextest passed 25/25. |

Final post-B-T01 focused Clippy passed with warnings denied. Quick Nextest run
`3971cb34-0b18-451b-b52e-2db7c483888c` passed 25/25. Real all-source validation
passed with repository root
`ac01170fe76ea5f56dd8ec7b75734f09df86589dde8a8ab6f907fc6834504e93`
and report root
`f303e702916c93202e0b79500e4c3aeec3108865acc897c663c6625878c28575`.

Pre-B-T01 coding-agent re-review A passed with no new findings (focused Nextest
run `5012d3f8-7d58-4ea5-9703-4f7e18a81db0`). Terminal coding-agent re-review B
passed with no new findings (focused Nextest run
`e649c8c0-affd-4682-8f6a-a0fddcea7161`). The later CRAP-driven decomposition
changed only validator structure; the
restarted full workspace and fresh CRAP gates proved that intermediate tree.
Verification B then found B-T01, so those heavy results and the earlier
Verification A became stale for final closure. B-T01 remediation re-review,
the fresh amended heavy sequence, and renewed independent terminal Verification
A and B all passed.

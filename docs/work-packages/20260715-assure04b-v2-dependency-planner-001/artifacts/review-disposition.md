# ASSURE-04B Review Disposition

Status: complete; all findings accepted, implemented, and independently
verified

Evidence classes: Static and Ran

| Finding | Disposition | Remediation and evidence |
| --- | --- | --- |
| `ASSURE04B-A01` | accepted | `resolve_node_state` now applies intrinsic-blocked > blocked-prerequisite > intrinsic-stale > changed-prerequisite > current. A graph unit test and real stale-manuscript/missing-result fixture prove the intermediate and report remain blocked. |
| `ASSURE04B-A02` | accepted | Moved confined reads to `v2/confined.rs`: an `openat` no-follow descriptor chain opens each directory and final file, `metadata()` validates the opened descriptor, and `read_to_end` reads that same descriptor. Unix unit tests replace an opened directory and final file path with outside symlinks and prove retained descriptors read only the original bytes while fresh traversal rejects the replacements. The package/write set records the direct locked `libc` dependency. |
| `ASSURE04B-A03` | accepted | Count artifacts updated for 6/6 crate and 35/35 assurance integration tests; line counts refreshed; renewed quick workspace passes 1,916/1,916 with 34 profile skips. |
| `ASSURE04B-B01` | accepted | Duplicate independent confirmation of A01; closed by the same precedence implementation and two-level regression. |
| `ASSURE04B-B02` | accepted | Renamed the authoritative implementation-roadmap heading to one/all plans, named real human/JSON plan consumers, and explicitly assigned report build/check assembly integration to 04C. The current-priority roadmap label now says plans. |
| `ASSURE04B-B03` | accepted | Duplicate evidence-freshness finding; closed with A03. |

No finding is rejected, deferred, follow-up, or undispositioned. Both
independent remediation checks passed with no new finding; heavy closure may
begin.

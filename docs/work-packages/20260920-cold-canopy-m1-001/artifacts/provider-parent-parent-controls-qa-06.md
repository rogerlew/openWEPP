# Provider-parent expected-red controls QA — 06

**Reviewer:** `/root/provider_qa` (secondary QA)  
**Evidence:** Static review of the 760-entry frozen cut, aggregate SHA-256 `075a936eb7b5d963d23709e509af648a97d96459b3b8bb345ac3f3b629407b18`; reviewed control SHA-256 `a7ed819e5ca6d24f216b85a0572a4b41466e1ff10bc9bcaf297b2e7806ff3f74`. No parent body, compilation, control execution, or physical work occurred.

## Findings

No blocking regression in the lineage correction.

The control now distinguishes support-staged owner bytes from the sealed finalized owner set. At every support it decodes the actual staged vegetation owner and proves the M1 revision and every stratum transaction lineage remain at their beginning values. After the consuming commit, it reads actual sealed finalized owner states, installs exactly that set, and requires a distinct finalized vegetation byte representation.

The final assertions require M1 revision and every stratum lineage to increment by exactly one, allow mutation only to vegetation or BGC plus vegetation, preserve snow and the four unaffected owners byte-for-byte, and compare the finalized/staged vegetation state after removing only logical-lineage fields. Parent revision/publication increments, cleared clock/staging, exact slab IDs, and the exact second-commit refusal remain asserted.

## Non-blocking follow-up

This corrects the previously withdrawn test05 staged/finalized equality claim. Raw accessor wiring, compilation, execution, body behavior, and physical validation remain separate missing evidence.

## QA disposition

**PASS — retain bounded test-body release with the corrected staged/finalized lineage contract.** No body or launch clearance follows.

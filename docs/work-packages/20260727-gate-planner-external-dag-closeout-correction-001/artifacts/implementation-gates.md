# Implementation Gates

Status: `FOCUSED PASS / IMPLEMENTATION REVIEW REQUIRED`

Evidence class: `Ran + Static`

Exact worktree base: `9dd2c5a340d8c083d0297facebad767fab988186`

| Gate | Result |
|---|---|
| Gate-planner Nextest | PASS, 225/225; 14 skipped; run `8777103a-75e9-4407-942e-e6525b440619` |
| Gate-planner all-target/all-feature Clippy with warnings denied | PASS |
| CAL Python discovery | PASS, 29/29 |
| Publication focused suite | PASS, 20/20 |
| External-DAG focused suite | PASS, 23/23 |
| `cargo check -p openwepp-gate-planner` | PASS |
| Rust formatting and diff hygiene | PASS |
| Gate-policy JSON parsing | PASS |
| Canonical CAL plan reconstruction | PASS, plan `00128a620fba5f9843b441bc19ece58be185944b0cd3f5abf80cdcea648daf98` |
| Canonical package chain through reviewed scaffold amendment | READY, `dbeb771a5d9df8a1bc09d3379879ebd29ad1de0818402132de507629338ab9fb` |

No heavy gate, CAL population, or Harvard read occurred.

The clean-anchor RED state is retained as static executable-path evidence from
`76fc06a6`: pre-LIGHT and audit each called consuming custody verification;
audit invoked inventory reconstruction twice; audit admission and proof
verification each invoked ledger admission; and audit construction preceded
HEAVY STARTED. The green tests name and measure the corrected mutation and call
counts. This artifact does not misrepresent the anchor source reconstruction as
a run of tests that did not yet exist at that commit.

A real Generation-B `run_external_transition` test now exercises pre-LIGHT
mutation-free verification, LIGHT, audit-only atomic dispatch consumption,
HEAVY proof verification, final independent receipt verification, one
inventory reconstruction, one ledger admission, balanced lifecycle,
same-dispatch rejection, and successful fresh dispatch in the retained custody
root.

The first dual implementation review at `327454b3` returned `HOLD`. Corrections
add the real consumer fixture, dispatch-scoped atomic consumption, attestation
time freshness, actual consumer error-class assertions, interruption-idempotent
publication recovery, dynamic destination-ancestor attachment checks, and
line-count deduplication. Dual re-review is required at the next clean commit.

The second review cycle added immutable historical attestation custody,
authenticated RESTORE progress with final all-entry baseline verification,
future/older-than-24-hour rejection, receipt-bound exact attestation time, and
actual root-replacement `Identity` classification. Focused evidence is green
and awaits final exact-commit review.

The final Rust review required durability of absent-baseline unlink at the
actual retained parent plus direct multi-entry replay and absent-branch
attachment-race evidence. Recovery now fsyncs that parent before success; the
expanded tests pass and `publication.rs` was reduced to preserve line-count
headroom.

Campaign-strength full workspace and anti-evasion gates remain prohibited until
the corrected implementation is committed, receives dual implementation
review, and obtains canonical pre-heavy `READY`.

# Renewed Terminal Verification B

Evidence class: `Static` plus `Ran` documentation, identity, reference, remote-
capability, spelling, and diff checks

Disposition: `HOLD`

I independently verified the exact round-2 closure candidate. I did not rely on
either first-round terminal verification or the round-2 reviewers' final
`PASS` statements. The campaign/increment philosophy is coherent, and most of
the 17 findings are substantively repaired. The candidate is not yet safe to
close because the authenticated-receipt design contains a circular identity,
the selected custom evidence-ref trust boundary is not mechanically protected,
the assurance supersession fold has a dangling-replacement path, and the
implementation handoff omits acceptance cases explicitly required by the
accepted review findings. Package closure status is also premature.

## Exact Candidate Identities

The live files assessed before and after the checks had these SHA-256
identities:

| Subject | SHA-256 |
| --- | --- |
| ADR-0039 | `b31e60ba3860fbbad8b34b723e02efc5d48bf96072924d4ff8aea63da3d92aa6` |
| Testing/gate standard | `4756b0af8283e66a6415c51076ece934330312f95c94dc9e1099cb67eb373917` |
| Implementation handoff | `199f33fe76287a63f72f917c3e94763227a112eaf0b426cb293dea6f268a179d` |
| Round-2 disposition | `deb9412c9f1ef56e0588dd19b01088777c620d70641c6be19eb44fb2db6c0ca4` |
| Package contract | `e60f562f2a969470141d5f77c3c5b943907190139ad83b5b752f9930ac7cf6b3` |
| Claimed final disposition | `0119bf9c03839ccff41301add9f11267cc70b8b54fdd2fb538e166724b3f5092` |

The authority hashes differ from the intermediate hashes recorded during the
reviewers' remediation passes, as expected. This verification applies only to
the identities above.

## Seventeen-Finding Audit

| Finding | Exact-tree result | Verification |
| --- | --- | --- |
| C-001 — bounded test edits and CRAP | `PASS` | Closed coverage-loss reasons, complete known covering-test expansion, and global fallback now preserve affected measurement without making ordinary additive test work global. |
| C-002 — nonblocking authority outcomes | `PARTIAL` | The A0–A6 execution/scientific reduction is now exhaustive, but the handoff does not carry the required all-outcome A1–A6 fixture matrix. |
| C-003 — A0 admission | `PASS` | A0 is explicit, unique, current, non-deferrable, and fail-closed for missing, ambiguous, provisional, or stale authority. |
| C-004 — assurance under-selection | `PARTIAL` | Registry-wide discovery and release-inventory equality are closed; dangling assurance supersession can still remove an open entry without proving its replacement exists. |
| C-005 — certified-head persistence | `PARTIAL` | Source and evidence identities, stable campaign head, immutable subject alias, two phases, and one atomic transaction are defined. The selected custom-ref protection and receipt-attestation identities remain incomplete. |
| C-006 — concurrent increments | `PASS` | Expected-parent admission, current-head terminal replan, conflict/abandonment/supersession, and compare-and-swap loss are explicit. |
| C-007 — cutover scorecard | `PASS` | Population, observation window, zero safety misses, deterministic replay, friction targets, migration order, and rollback triggers are fixed prospectively. |
| C-008 — truthful package closure | `REGRESSED` | The package again says `EXECUTED-COMPLETE` and claims both renewed verifiers passed before both artifacts existed; the artifact index omits every round-2 review, disposition, gate, and verification artifact. |
| D2-001 — authenticated evidence | `PARTIAL` | Trust classes, issuer policy, revocation, target binding, and offline bundles are present, but the receipt/attestation identity cycle below prevents a unique construction. |
| D2-002 — hermetic reuse | `PASS` | Reuse defaults off and content reuse requires observable input confinement, explicit ambient inputs, accepted trust, and independent closure recomputation. |
| D2-003 — certificate storage | `PARTIAL` | The source/certificate storage cycle is closed, but the custom ref's protected-writer mechanism is neither selected nor supported by a cited provider authority. |
| D2-004 — ledger and backstop reduction | `PASS` | Campaign/obligation transitions, compare-and-swap, stale-writer behavior, and exact age/count backstop reduction are closed. |
| D2-005 — gate DAG and predicates | `PARTIAL` | Definition/node IDs, full node hashing, closed predicate algebra, graph validity, and failure precedence are closed. `PASS_WITH_RETRY` has contradictory authority-suite acceptance semantics and required ID-mutation fixtures are absent from the handoff. |
| D2-006 — canonical Git/Cargo inputs | `PASS` | Rename-disabled NUL records, dirty-source selection, rejected unsupported states, isolated locked/offline graphs, declared matrices, and normalized graph binding are defined. |
| D2-007 — assurance target and fold | `PARTIAL` | Target-bound axes, request/currency separation, immutable entries, and order-independent dominance are present; supersession is not atomic with or conditional on a valid replacement. |
| D2-008 — CI aggregation and cutover | `PASS` | Planner and required aggregate contexts are distinct; missing/canceled jobs fail closed; dual-required migration and rollback are explicit. |
| D2-009 — primary references | `PARTIAL` | Added Git/Cargo, JSON, provenance, and attestation sources are authoritative and reachable, but no cited authority supports the normative atomic multi-ref transaction or protection of the chosen GitHub custom-ref namespace. |

## Residual-Finding Audit

| Residual | Exact-tree result | Verification |
| --- | --- | --- |
| Exhaustive authority outcome reduction | `PASS-AUTHORITY` / `PARTIAL-HANDOFF` | The table closes `CONFORMS`, `DIVERGES`, `INCONCLUSIVE`, and `NOT_EVALUATED`; the demanded all-outcome fixture matrix is absent. |
| Campaign-wide compare-and-swap | `PASS-ALGORITHM` / `PARTIAL-TRUST` | One mutable head and an atomically created immutable alias close the lost-update algorithm. Remote atomic capability exists, but protected custom-ref authorization is undefined. |
| Campaign-to-release reuse | `PASS` | Release reuse now conjunctively requires exact inputs, release-accepted `PROTECTED_CI`, `HERMETIC_CONTENT`, and no release-rerun policy. |
| Obligation transitions | `PASS-AUTHORITY` / `PARTIAL-HANDOFF` | `SUPERSEDED`, retry, block, stale, bootstrap, replacement, and accepted-receipt transitions are closed; stale-rerun failure/block and supersession fixtures are not carried forward. |
| Assurance entry reduction | `HOLD` | Closed names and basic dominance are present, but a `SUPERSEDED` entry needs only a named replacement ID, not atomic creation/existence and target/report equivalence. |
| Schema consistency | `PASS-AUTHORITY` / `PARTIAL-HANDOFF` | Rename is delete-plus-add, prerequisites use `node_id`, and full node payload hashing is explicit; the requested node-ID field-mutation fixture is absent. |
| Bootstrap transition | `PASS` | `LEGACY_UNVERIFIED` is bootstrap-only, cannot become pass or gain trust, and can clear only through adopted rerun or atomic named replacement. |

## Actionable Findings

### V2-B-001 — High — Receipt and attestation identity are circular

The standard defines `receipt_id` as the digest of the complete canonical
receipt payload excluding only `receipt_id` itself
(`testing-and-gate-strategy.md:529-535`). The required receipt payload includes
an authenticated attestation identity (`:693-708`), while release eligibility
requires the attestation bundle to bind the executed subjects and artifact
digests (`:730-744`). Under the selected GitHub/SLSA model, an attestation is
produced for an already identified subject. If it attests the receipt, its
identity cannot be known until the receipt digest exists; inserting that
identity changes the receipt digest. If it does not attest the receipt, the
issuer proof is not bound to the exact result record it is supposed to
authenticate.

Define a non-circular two-layer contract: first derive the immutable receipt and
`receipt_id` without an attestation locator/digest; then produce a signed
attestation envelope whose subject is that exact receipt ID/digest plus its
artifacts; finally derive a separate attested-receipt/envelope identity. State
which layer the ledger and certificate consume and reject an envelope whose
subject does not equal the recomputed receipt.

### V2-B-002 — High — The selected protected custom Git ref has no enforceable writer contract

Section 11.1 chooses
`refs/openwepp/evidence/campaigns/<campaign-id>/...`, calls it protected, and
requires that only protected CI advance it (`testing-and-gate-strategy.md:918-951`).
The live remote is GitHub. A dry-run confirmed that its receive-pack advertises
the `atomic` capability and accepts the proposed custom namespace, but neither
the authority nor handoff defines how GitHub prevents ordinary repository
writers from creating, deleting, or rewinding those custom refs. GitHub's
documented repository rulesets protect branches and tags; the cited artifact-
attestation mechanism authenticates content but does not itself enforce ref
write authorization. A writer can therefore at least destroy the campaign-head
availability and compare-and-swap anchor even if they cannot forge a valid CI
attestation.

Select and reference an enforceable store: for example a provider-protected
branch namespace with a named CI app as the only bypass actor, a dedicated
evidence repository with narrower write authority, or another append-only
store. Define remote atomic compare-and-swap mechanics and captured provider-
rule evidence. Add the primary Git `update-ref`/`push --atomic` and selected
provider ruleset references; current Git status/diff/index references do not
support this transaction or authorization claim.

### V2-B-003 — High — Assurance supersession can resolve a dangling replacement

An impact entry may transition to `SUPERSEDED` with a named replacement
(`testing-and-gate-strategy.md:1097-1113`), but the standard does not require the
replacement entry to exist atomically, match the same report/target, or remain
in the fold. The aggregate rule then treats everything other than open,
refresh-required, and refresh-complete entries as resolved/no-material-impact
(`:1134-1148`). A dangling or unrelated replacement can therefore remove the
only open assessment and permit transfer.

Require an atomic supersession event that proves the replacement entry exists,
matches report and target authority, and is included in the same fold. A
dangling, withdrawn, mismatched, or unresolved replacement must dominate as
`IMPACT_PENDING`. Add attempted dangling, cross-report, cross-target, and open-
replacement fixtures.

### V2-B-004 — High — Accepted review fixtures were not preserved in the implementation handoff

The reviews required concrete acceptance cases as part of their remedies, and
the disposition says every finding and residual was corrected without
deferral. The handoff's 32 scenarios omit several required cases:

- every scientific outcome for A1 through A6, especially A1/A3
  `INCONCLUSIVE`/`NOT_EVALUATED` and promoted A4/A5 outcomes;
- stale-obligation rerun `FAIL`/`BLOCKED` and obligation supersession;
- assurance refresh completion that omits one impact, plus supersession and
  withdrawal interactions;
- node-ID mutation under executor, argument, prerequisite, acceptance, and
  artifact-namespace changes;
- Git rename-as-delete/add and rejected invalid-path states; and
- release reuse rejection for `NON_REUSABLE` and `SAME_EXECUTION` receipts.

Add those cases to the handoff or link a complete machine-readable acceptance
matrix. Otherwise the follow-up can satisfy the handoff while failing remedies
that the closure record claims are mandatory.

### V2-B-005 — Medium — `PASS_WITH_RETRY` is not reduced consistently for authority suites

Section 5.1 says the only accepted A1/A3 execution is `PASS` and that non-pass
execution blocks; A2/A4/A5/A6 likewise require `PASS`
(`testing-and-gate-strategy.md:189-218`). The aggregate permits
`PASS_WITH_RETRY`, including infrastructure retries for A0/A1/A3 (`:578-586`),
and receipt reuse accepts a policy-permitted `PASS_WITH_RETRY` (`:771-779`). It
is therefore unclear whether an authority gate that passes after a permitted
infrastructure retry satisfies its execution obligation or blocks as non-pass.

Either prohibit retry satisfaction for named authority classes or define
`PASS_WITH_RETRY` as an accepted execution-integrity result only under a closed
retry policy while retaining mandatory flake debt. Apply that rule consistently
to outcome reduction, aggregate transition, ledger acceptance, and reuse.

### V2-B-006 — High — Closure truthfulness and artifact discovery regressed

`package.md` and `final-disposition.md` declare `EXECUTED-COMPLETE`, and the
work-package catalog claims renewed dual terminal verification, before the
required renewed artifacts both passed. This directly violates the package's
“close only if both verify” gate and reintroduces C-008. The artifact README
also stops at first-round evidence and omits both round-2 reviews, the round-2
disposition/gate record, and renewed verifications.

Reopen the package and catalog during remediation, update the artifact index,
rerun both independent verifiers on the next exact authority bytes, and issue
the final disposition only after both return `PASS`.

## Checks Run

Ran against the identities above:

- `markdown-doc lint` and `markdown-doc validate` over ADR-0039, the canonical
  standard, all three catalogs, and the complete package: `PASS`, 20 package
  files plus the five separately scoped authority/catalog files, zero errors
  and zero warnings;
- `git diff --check`: `PASS`;
- `uk2us` previews for the ADR, standard, package files, handoff, reviews,
  dispositions, and verification inputs: package prose is clean; the large
  work-package catalog has unrelated historical normalization suggestions, but
  none occur in this package's added catalog entry;
- every external URL cited by the standard and research basis: 24 distinct
  URLs returned HTTP 200 on 2026-07-17;
- `git push --dry-run --atomic origin HEAD:refs/heads/main` with packet tracing:
  the GitHub remote advertised `atomic`; no remote mutation occurred;
- `git push --dry-run --atomic origin
  HEAD:refs/openwepp/evidence/verification-probe`: GitHub accepted the custom
  refspec as a dry-run new reference; no remote mutation occurred; and
- exact status/write-set inspection: all candidate changes remain within the
  declared documentation-only write set. No Rust, test, fixture, workflow,
  dependency, assurance realization, or release implementation changed, so the
  package's implementation-gate `N/A` claim is truthful.

The external sources are live and generally authoritative. The reference gap
is semantic rather than link health: the source list does not cite the Git ref-
transaction or provider protection mechanisms on which certification depends.

## Terminal Result

The core campaign-scoped strategy should be retained. The canonical Git/Cargo
impact algorithms, typed DAG, hermetic reuse conditions, ledger/backstop
machine, CI aggregate, and most assurance state mechanics are substantially
stronger after round 2. `HOLD` is required until V2-B-001 through V2-B-006 are
dispositioned and both renewed verifiers assess the resulting exact bytes.

## Remediation Verification

Evidence class: `Static` plus `Ran` documentation, identity, reference,
American-English, external-link, and diff checks

Technical disposition: `PASS`

I independently re-read the remediated authority, ADR, implementation handoff,
terminal finding disposition, reopened package state, catalogs, and artifact
index. The exact remediated identities are:

| Subject | SHA-256 |
| --- | --- |
| ADR-0039 | `c4772959c9915fdad9eed2cbc16dd00e1272ab630a1643487a506687388bb62a` |
| Testing/gate standard | `a0c69780af9f1211b7f779e7f812210e64d75b1373323c52f6a26b13a87850c2` |
| Implementation handoff | `ed0101e25f4f360d43154e03aa4520f8dd4d312d3b70b759cdd3c2b5c3cccb18` |
| Terminal finding disposition | `ae09d4fecb5fffe6c1efdfc0a95f17503b7afd31b9d5fad26620724ecc93a71e` |
| Open package contract | `845b7d90b6b0a77089f7c6976b98d89372ce373a46159b1d3a337ca8838a2437` |

### Finding Reverification

- **V2-B-001 — PASS.** The receipt is now an immutable unsigned payload. Its
  `receipt_id` is derived before an attestation exists. A separate signed
  envelope binds the independently recomputed receipt ID/digest and every
  referenced artifact, receives its own `envelope_id`, and is the object used
  with the receipt by ledgers and certificates. Locators remain outside receipt
  identity; mismatched, recursive, missing, and inconsistent envelopes are
  invalid. The prior identity cycle is removed.
- **V2-B-002 — PASS.** The custom ref namespace is replaced by a mutable
  `refs/heads/openwepp-evidence/<campaign-id>` branch and immutable
  `refs/tags/openwepp-evidence/<campaign-id>/<subject-commit>` tag. Two active
  branch/tag rulesets restrict creation/update/deletion, only a dedicated
  evidence-publisher GitHub App may bypass, and provider configuration/app
  identity are authority-root inputs. Finalization uses one remote atomic push,
  an exact branch lease, absent-tag creation, and fail-closed capability,
  ruleset, app, and attestation checks. Official Git atomic-push, reference-
  transaction, and GitHub ruleset authorities are now cited.
- **V2-B-003 — PASS.** Assurance supersession is valid only through one atomic
  event that creates or proves a same-report, same-target, same-realization,
  same-generation replacement and includes it in the same fold. Dangling,
  cross-report, cross-target, withdrawn, or recursively invalid replacement
  rejects the transition and leaves the original open. A valid replacement
  chain is excluded only when its terminal replacement remains in the fold.
- **V2-B-004 — PASS.** Handoff scenarios 33–38 now preserve the complete A0–A6
  outcome matrix; stale rerun, block, pass, and supersession states; assurance
  omission/supersession/withdrawal cases; full node-ID mutation set; Git
  change/unsupported states; and rejection of nonreusable, same-execution,
  unauthenticated, wrong-subject, and wrong-trust release evidence.
- **V2-B-005 — PASS.** `accepted execution` now consistently means clean
  `PASS`, or `PASS_WITH_RETRY` only for prospectively authorized
  infrastructure-only failures under the same closed policy generation.
  Semantic/scientific failure cannot retry into acceptance; all attempts and
  mandatory debt remain visible. Outcome, boundary, and reuse language use the
  same definition.
- **V2-B-006 — PASS for the open candidate.** The package is
  `IN_PROGRESS-TERMINAL-REMEDIATION`; its README and repository catalog say
  renewed verification remains open; the former final disposition is expressly
  superseded; and the artifact index names every round-2 review, disposition,
  gate, renewed verification, and terminal finding disposition. No completion
  claim remains in the live candidate.

I also rechecked every original C/D finding and residual interaction. The new
receipt envelope does not weaken hermetic-root verification; the evidence
branch/tag transaction retains one campaign-wide compare-and-swap authority;
assurance supersession composes with registry-wide discovery and exact release
inventory equality; retry acceptance cannot convert A1/A3 semantic failure;
and the expanded fixtures preserve the previously accepted state, identity,
reuse, and bootstrap remedies. No actionable technical or governance ambiguity
remains from this verification lane.

### Remediation Checks Run

Ran against the identities above:

- scoped `markdown-doc lint` and `markdown-doc validate` over ADR-0039, the
  standard, all three catalogs, and the complete 22-file package: `PASS`, zero
  errors and zero warnings;
- `git diff --check`: `PASS`;
- `uk2us` previews over all remediated authority and package prose: `PASS`, no
  differences;
- every distinct external reference in the standard and research basis: HTTP
  200, with no failed URL; and
- live hash recheck after all commands: unchanged from the identities recorded
  above.

This is a technical `PASS` on the open remediation candidate, not permission to
backdate closure. After both renewed verifiers pass, closure-only status,
catalog, final-disposition, gate-record, and artifact-index changes must remain
non-normative and receive the promised exact-byte recheck before
`EXECUTED-COMPLETE` is truthful.

## Final Closure-only Recheck

Evidence class: `Static` plus `Ran` scoped documentation, identity,
American-English, and diff checks

Final verifier-B disposition: `PASS`

After both renewed technical verifiers returned `PASS`, I inspected the exact
closure-only bookkeeping transition. The normative authority remains byte-for-
byte identical to the technical candidate:

- ADR-0039 SHA-256:
  `c4772959c9915fdad9eed2cbc16dd00e1272ab630a1643487a506687388bb62a`;
- testing/gate standard SHA-256:
  `a0c69780af9f1211b7f779e7f812210e64d75b1373323c52f6a26b13a87850c2`;
  and
- implementation handoff SHA-256:
  `ed0101e25f4f360d43154e03aa4520f8dd4d312d3b70b759cdd3c2b5c3cccb18`.

The closure records assessed are:

- package contract SHA-256
  `e60f562f2a969470141d5f77c3c5b943907190139ad83b5b752f9930ac7cf6b3`;
- final disposition SHA-256
  `dc4039a5d0472eee945a4251b2263682672bf825ea4ec64383ae187851cc91bf`;
- repository work-package catalog SHA-256
  `a1903719a4eadfa49c5b43553e1a6c86a2815311d070adb2b77d49fc85f9be69`;
- package README SHA-256
  `04f10178a4310bbeccc3897166b15bc7996c250630fd1d54a589df6c3831c4fe`;
- artifact index SHA-256
  `8cadc107c648d177f0e52037c1163cec69a915a3ea1a6803eba979b64a145fff`;
  and
- round-2 gate record SHA-256
  `6421955956bfd1c601bff4e4ef579c1caba0fffaf1d0bcf933c41c058d9cd213`.

The transition changes only truthful lifecycle summaries. `package.md`, the
package README, the catalog, and final disposition consistently say
`EXECUTED-COMPLETE`; they retain the documentation-only boundary, state that
current tooling is not implemented/aligned, and point to the follow-up handoff.
The artifact index contains both review rounds, both finding dispositions, both
gate records, both original and renewed verification artifacts, and the
terminal finding disposition. No closure record converts deferred evidence to
pass or claims current implementation conformance.

I reran `markdown-doc lint` and `markdown-doc validate` over ADR-0039, the
standard, all three catalogs, and the complete 22-file package: `PASS`, zero
errors and zero warnings. `git diff --check` passes. `uk2us` previews for every
closure-only file produce no differences. A terminal hash recheck confirmed
the identities above and that no authority or handoff byte changed.

No actionable closure-only finding remains. This verifier's final exact-byte
result is `PASS`.

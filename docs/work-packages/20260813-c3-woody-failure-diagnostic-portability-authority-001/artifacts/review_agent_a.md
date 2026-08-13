# Independent Review A — Terminal Audit

Evidence: `Static + Ran`

Audit disposition: `HOLD on audited bytes`

Audit base: `53ed7c178c09585e36b1e17dfb998e5bf10e17a2` plus the
then-current worktree. This record describes the exact bytes audited before
remediation of the findings below. Later edits require fresh terminal
verification; they do not retroactively turn this audit into a PASS.

## Scope and Method

The review read the applicable repository, work-package, and science-contract
instructions; the complete package and artifact set; the Version 10 amendment;
the immutable predecessor definitions; the V6 definition, generator, vectors,
and verifier; and the affected authority tests and catalog entry. It also
inspected the exact terminal diff and ran the following gates:

- V6 independent regeneration and authority verification;
- science-contract admission and unit compliance;
- authority-suite anti-evasion and AUTH11 required-suite guards;
- the vegetation boundary authority suite;
- Markdown lint for the package, contract, science-contract index, and package
  catalog; and
- `git diff --check`, digest reconstruction, changed-path inspection, and
  production/runtime exclusion checks.

## Severity-Ranked Findings

### `V6-A-001` — Critical — applicable authority suite failed

Audit-snapshot references:
`tests/integration/vegetation_boundary_authority_contract.rs:186` and
`:621`; package acknowledgment at
`artifacts/gate-results.md:18`; exit criterion at `package.md:80`.

The fresh command
`cargo nextest run --test vegetation_boundary_authority_contract --profile quick`
ran 21 tests and returned 19 PASS / 2 FAIL. Both failures required
`contract_version: 9` after the canonical contract had been promoted to
Version 10. The same test still bound the registry to the V5 identity and the
2026-08-12 review date. The package had already recorded the stale V9
assertions as an expected failure outside one worker's bounded write set.

Scientific/governance impact: the canonical authority and its required
contract-derived regression surface disagreed. Under the gate non-deferral
rule, a known current-scope failure cannot be waived by a worker-local write-set
boundary or by successful admission alone. Version 10 could not be terminally
released on those bytes.

Proposed disposition: `accepted / closure-blocking`. Update the authority test
to bind Version 10, V6, and the current registry identity; add the V6-specific
contract-derived consumer described in `V6-A-002`; rerun the complete 21-test
suite on stable bytes.

### `V6-A-002` — High — V6 committed fixture lacked its claimed Rust consumer

Audit-snapshot references:
`SC-VEGETATION-001.md:1740`; V6 evaluator and fixture generation at
`artifacts/reference_calculator_v6.py:29`, `:160`, and `:190`; authority
verifier at `artifacts/verify_v6_authority.py:66`.

The contract states that ordinary Rust tests consume the committed V6 JSON,
but the audited `vegetation_boundary_authority_contract.rs` contained no V6
definition/vector consumer. The Python verifier executed the fixture correctly,
but it did not satisfy the explicit Rust-consumer claim.

Scientific/governance impact: the fixture was independently generated and
executable, but its repository-native contract binding was incomplete. A later
Rust implementation could drift from the frozen evidence without the promised
ordinary-test gate detecting that drift.

Proposed disposition: `accepted / closure-blocking`. Add a Rust authority test
that consumes the committed V6 definition and vectors, binds the immutable
hashes and V6 section, evaluates the closed positive inventory and every poison,
and checks the exact V5-to-V6 transition.

### `V6-A-003` — High — terminal lifecycle evidence was incomplete

References: `package.md:75`, `package.md:76`,
`artifacts/terminal-diff-reconciliation.md:5`, and
`artifacts/final-disposition.md:3`.

The package still had both terminal progress items unchecked, terminal diff
reconciliation was `PENDING_STABLE_POST_REVIEW_BYTES`, final disposition was
`IN PROGRESS`, and the prompt remained active. Those labels were truthful, but
they prove the package was not terminally complete.

Scientific/governance impact: no stable-byte terminal reconciliation yet bound
the final write set, protected definitions, exact gate snapshot, prompt
identity, and no-production-change claim. Closure or handoff would therefore
overstate the evidence lifecycle.

Proposed disposition: `accepted / closure-blocking`. After all substantive
findings are remediated, reconcile the new exact diff, rerun both independent
terminal verifiers, archive the prompt byte-for-byte, and only then mark the
package complete.

### `V6-A-004` — Medium — package catalog entry was malformed and stale

Audit-snapshot reference: `docs/work-packages/README.md:14` through `:20`.

The V6 bullet had been inserted between the V4 package's headline and its
indented continuation, causing the V4 evidence text to render as part of the V6
entry. It also described V6 as "under dual review" after Version 10 was already
approved/active.

Scientific/governance impact: package discovery misattributed V4 closure
evidence to V6 and contradicted the canonical lifecycle state.

Proposed disposition: `accepted / required documentation correction`. Restore
the V4 bullet as one contiguous entry and give V6 a separate entry matching its
approved/active, terminal-pending state.

### `V6-A-005` — Low — post-review digest block carried a stale label

Reference: `artifacts/gate-results.md:34`.

The digest block following the post-review results was labeled "Current
pre-review identities" even though it represented the stable post-review V6
artifacts.

Scientific/governance impact: hashes themselves matched, but the temporal label
could confuse later evidence reconstruction.

Proposed disposition: `accepted / documentation correction`. Relabel the block
to identify the exact post-review or terminal byte set.

## Substantive Checks That Passed

- The Version 10 amendment is scientifically narrow. The sole eligible value is
  rejected `backtracking_limit.step_norm=max_j(abs(delta_x_j))`; accepted state,
  flux, residual, conservation, authorization, convergence, branch-selection,
  and rollback behavior remain excluded
  (`SC-VEGETATION-001.md:1629`, `:1659`, and `:1707`).
- The selected comparison is rtol-only and symmetric:
  `abs(a-b) <= 3e-7*max(abs(a),abs(b))`. It has an inclusive binary64 boundary,
  no universal absolute tolerance, and only about 4.4 percent headroom above
  the observed cross-runtime delta (`SC-VEGETATION-001.md:1678` and `:1698`).
- Eligibility is fail-closed before numerical comparison: identity, presence,
  failure/category, candidate absence, unit/basis, count/order, branch,
  rollback, finite class, sign class, and zero class compare exactly
  (`reference_calculator_v6.py:14` and `:29`). Negative norms, nonfinite values,
  zero/nonzero changes, and sign changes reject.
- The independent fixture covers the observed pair, exact largest passing
  boundary, adjacent inside/outside values, reversed operands, lower-side
  behavior, signed zero, minimum subnormal, negative norm, nonfinite values,
  and complete-record identity/acceptance/rollback poisons
  (`reference_calculator_v6.py:160` and `:178`). The regenerated bytes matched
  `2e7005f88d788399e914b2034c0193fc6f08d1657532a349ec797b966432356b`.
- The V5-to-V6 fixture preserves canonical non-identity payload bytes and
  derives distinct configuration, state, and diagnostic identities
  (`reference_calculator_v6.py:126`; `verify_v6_authority.py:76`).
- Every protected V1--V5 definition matched its recorded SHA-256. The immutable
  V5 section matched
  `22edf6816d078833029d59bfb263b3c7ccdc8669a3fcccf73d82e601add34a5f`
  (`verify_v6_authority.py:26` and `:56`).
- V6 identities matched: generator `bfa805000a6e29b3c56a666ea97a4e48...`,
  vectors `2e7005f88d788399e914b2034c0193fc6...`, definition and canonical copy
  `a5a5ed77b4672b97b7c50103089067d7...`, verifier
  `a71f0d149a753183d2b97d59d0609c18...`, and contract section
  `fba3486765a3819ab44659e80f9fb1eb...`.
- Dual scientific review had authorized the narrow rule and lifecycle promotion
  was correctly `approved/active`. Fresh admission passed with
  `A0_ADMITTED contracts=45 science_surfaces=0` and authority digest
  `85210adc267bab7ca4f6693bb0684e354620acc9b04505f0cb188840bceb9576`.
- Fresh SC unit compliance, authority anti-evasion, AUTH11 3/3, Markdown lint,
  and `git diff --check` passed.
- The audited diff contained no production Rust or runtime change. It changed
  contract/evidence/test documentation only and did not activate V6.

## Recommendation

`HOLD` on the audited bytes. The portability rule itself is scientifically
acceptable and its frozen Python evidence is strong, but `V6-A-001` through
`V6-A-003` prevent truthful terminal closure. Remediation must be followed by a
fresh exact-byte terminal audit; this record must remain as the retained
history of the failed audit rather than being rewritten to GO.

# Terminal Verification A

Evidence: `Static + Ran`

Verdict: `PASS-WITH-NOTES`

Verification base: `53ed7c178c09585e36b1e17dfb998e5bf10e17a2` plus the
current corrected worktree bytes. This is a separate post-remediation
verification; it does not rewrite either retained HOLD review.

## Finding Closure

| finding_id | verification | evidence | status |
|---|---|---|---|
| `V6-A-001` | The contract-derived authority test now binds Version 10, V6, the 2026-08-13 registry identity, and lifecycle agreement. The package write set includes the test. Fresh isolated execution returned 23/23. | `tests/integration/vegetation_boundary_authority_contract.rs:185`, `:196`, and `:628`; `package.md:52` | `closed` |
| `V6-A-002` | Rust now independently evaluates the exact eligibility fields and rtol-only comparison, then applies it to every named numeric case, including `lower_side_boundary`, and every eligibility/firewall/nonfinite poison. Definition, section, transition, and predecessor hashes are also consumed. | `tests/integration/vegetation_boundary_authority_contract.rs:1877`, `:1884`, `:1947`, and `:2042` | `closed` |
| `V6-A-003` | The package correctly remains nonterminal: promotion, terminal verification, prompt archival, terminal reconciliation, and final disposition are withheld. This is the required state until both verifiers pass and the parent performs post-verification lifecycle work. | `package.md:76`; `artifacts/terminal-diff-reconciliation.md:5`; `artifacts/final-disposition.md:3` | `still-open by lifecycle design` |
| `V6-A-004` | The V4 catalog entry is contiguous and V6 has a separate entry that says remediation is active and promotion withheld. No V4 evidence is attributed to V6. | `docs/work-packages/README.md:14` and `:21` | `closed` |
| `V6-A-005` | The digest block is now labeled `Current reviewed authority identities`; all listed hashes match current bytes. | `artifacts/gate-results.md:44` | `closed` |
| `V6-RB-001` | The declared write set includes the authority test, predecessor assertions remain exact, strict Clippy passes, and the focused suite passes 23/23. | `package.md:52`; `tests/integration/vegetation_boundary_authority_contract.rs:1947` | `closed` |
| `V6-RB-002` | Contract front matter/body and the registry agree on Version 10 `in_review/draft`; catalog and package withhold promotion. Admission fails closed for exactly that lifecycle, as required before dual verification. | `SC-VEGETATION-001.md:4`; `SC-VEGETATION-001.md:23`; `docs/specifications/science-contracts/index.md:70`; `package.md:76` | `closed for pre-promotion verification` |
| `V6-RB-003` | Both retained reviews now preserve stable IDs, severity, references, impact, disposition, and recommendation. The disposition table contains one row per `V6-A-*` and `V6-RB-*` finding with all required fields. | `artifacts/review_agent_a.md`; `artifacts/review_agent_b.md`; `artifacts/review-finding-disposition.md:9` | `closed` |
| `V6-RB-004` | Catalog ownership and current lifecycle wording are correct. | `docs/work-packages/README.md:14` and `:21` | `closed` |
| `V6-RB-005` | No `__pycache__` directory or `.pyc` file remains after both Python checks were run with `PYTHONDONTWRITEBYTECODE=1`. | package-tree filesystem audit | `closed` |

The current bytes match every remediation claim in
`review-finding-disposition.md`. `V6-A-003` is intentionally not claimed closed;
its remaining steps occur only after both verifier records exist.

## Fresh Gate Results

- `PYTHONDONTWRITEBYTECODE=1 .venv/bin/python
  .../reference_calculator_v6.py --verify .../openwepp_c3_woody_v6_vectors.json`:
  PASS; committed bytes regenerate exactly.
- `PYTHONDONTWRITEBYTECODE=1 .venv/bin/python
  .../verify_v6_authority.py`: PASS; executable cases, definition copies,
  contract sections, transition, and immutable predecessors verified.
- `cargo nextest run --test vegetation_boundary_authority_contract --profile
  quick`: PASS, 23/23 on the final retry.
- `cargo clippy --test vegetation_boundary_authority_contract -- -D warnings`:
  PASS after exact `to_bits()` assertions replaced five strict-float
  comparisons. No lint suppression or tolerance weakening was introduced.
- `cargo nextest run --test auth11_required_suite_obligation_guards_contract
  --profile quick`: PASS, 3/3.
- `check_sc_unit_compliance.sh --path SC-VEGETATION-001.md`: PASS.
- `check_authority_suite_antievasion.sh`: PASS.
- `check_science_contract_admission.sh --base-ref 53ed7c178c09585e36b1e17dfb998e5bf10e17a2
  --worktree`: expected fail-closed result because the corrected lifecycle is
  intentionally `in_review/draft`. Active admission must be rerun after both
  verifiers pass and promotion occurs.
- Markdown lint: PASS for the package, contract, science-contract index, and
  package catalog with zero findings before this record; this record was linted
  separately after creation.
- `cargo fmt --all -- --check` and `git diff --check`: PASS.

One concurrent authority-suite attempt observed an unrelated V3 oracle
subprocess return empty stdout. The immediately following isolated run on the
same source bytes passed all 23 tests. Because the failure was in unchanged V3
oracle process execution, while the exact isolated retry and the earlier
corrected run both passed, it is recorded as transient process-contention
evidence rather than a V6 regression.

## Protected Identities

- V1: `003107043e8eb5bda6d9d6476e3ea01690815e3280ac98daf169317ce4d09157`.
- V2: `38e1bb90abd3ff82879f7d9c80b0377bb510a3b97fdd2b6f07c12b7c42b80dc3`.
- V3: `7768657ca3d03603b66f5cd6677f032ee630fdd46d6ffadf214c713065f73852`.
- V4: `8ace38d1148f95261306cd6b0bf6f22e23ac8ead4cb6897dbdb53061b78ee437`.
- V5: `0ee6a50d5f72da0b9344d8bf1b77674e95a66ab196edc068851bb419eb7b36f3`.
- Immutable V5 section:
  `22edf6816d078833029d59bfb263b3c7ccdc8669a3fcccf73d82e601add34a5f`.
- V6 generator:
  `bfa805000a6e29b3c56a666ea97a4e4825f9262a3ef1f0daa5c3cfb5f2dd6532`.
- V6 vectors:
  `2e7005f88d788399e914b2034c0193fc6f08d1657532a349ec797b966432356b`.
- Both V6 definitions:
  `a5a5ed77b4672b97b7c50103089067d70ade03bc1b5aff4e08ba6fdffc05d426`.
- V6 verifier:
  `a71f0d149a753183d2b97d59d0609c184618f993d83e2a8c4abba87bc8671ba1`.
- V6 contract section:
  `fba3486765a3819ab44659e80f9fb1eb304ee5953cd8c41f3046b95442ef0891`.

## Diff and Claim Audit

The exact changed/untracked path inventory is confined to the declared package
tree, the canonical contract and registry, the package catalog, the canonical
V6 definition copy, and the contract-derived integration test. No production
crate source, runtime selector, Cargo manifest/lockfile, deployment,
publication, or consumer-cutover path changed. The Rust edits are test-only.
V6 remains unactivated, and production remains bound to V5.

## Verdict

`PASS-WITH-NOTES`. `V6-A-001`, `V6-A-002`, `V6-A-004`, `V6-A-005`, and every
`V6-RB-*` finding are verified closed on the current corrected bytes.
`V6-A-003` remains open only for the deliberately sequenced post-verification
lifecycle work: obtain Terminal Verification B, promote Version 10, rerun active
admission and affected focused gates, reconcile the final diff, archive the
prompt byte-for-byte, and issue final disposition. This verifier does not
authorize skipping any of those steps.

## Post-promotion addendum

Evidence: `Static + Ran`

Verdict: `PASS`

The exact promotion-only bytes were reconciled after Terminal Verification B.
The authority lifecycle changed from `in_review/draft` to `approved/active` in
the contract front matter, rendered body status, and registry row. Matching
package progress, catalog, final-disposition, and append-only gate records now
state that promotion followed both separate verifier verdicts and that terminal
reconciliation remains active.

Fresh promoted-byte results:

- science-contract admission: PASS,
  `A0_ADMITTED contracts=45 science_surfaces=0`, authority SHA-256
  `7759fe4819ee3741298abcddf86966ad5fa3d68837ac7cf380f614d1f7b76753`;
- V6 regeneration and authority verifier with
  `PYTHONDONTWRITEBYTECODE=1`: PASS;
- strict authority-test Clippy with `-D warnings`: PASS;
- vegetation boundary authority suite: PASS, 23/23;
- SC unit compliance and authority anti-evasion: PASS;
- AUTH11 required-suite guards: PASS, 3/3;
- Rust formatting and `git diff --check`: PASS; and
- Markdown lint: PASS for the 17-file package, 63-file science-contract tree,
  and package catalog, with zero findings.

The generator, vectors, both V6 definitions, verifier, V6 contract section,
immutable V5 section, and protected V1--V5 identities remain exactly those
recorded above. The unchanged V6 section digest is
`fba3486765a3819ab44659e80f9fb1eb304ee5953cd8c41f3046b95442ef0891`.
No scalar rule, eligibility firewall, fixture, identity transition, test
semantics, production source, runtime selector, manifest/lockfile, deployment,
publication, or consumer-cutover path changed during promotion.

`V6-A-003`'s promotion and promoted-byte gate prerequisites are now satisfied.
It can close after the active kickoff prompt is archived byte-for-byte and the
resulting exact terminal diff and final disposition are reconciled. No other
finding has reopened.

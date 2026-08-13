# Independent Science Review B

Evidence: `Static + Ran`

Review target: the exact dirty-worktree bytes independently audited against
base commit `53ed7c178c09585e36b1e17dfb998e5bf10e17a2` on 2026-08-13, before
post-audit remediation. Concurrent changes made after that audit are not
silently incorporated into this recommendation.

Final recommendation: `HOLD`

## Scope and Method

Reviewer B independently read the root, work-package, science-contract,
standards, and test instructions; the package and prompt; the complete V6
contract amendment; both definition copies; the generator, vectors, verifier,
protected-digest ledger, gate record, review/disposition evidence, terminal
reconciliation, and exact changed-path inventory. The review did not rely on
the producer's GO conclusion and made no file changes during the audit.

The review specifically tested whether the proposed tolerance could escape
the rejected diagnostic evidence surface, whether protected predecessor bytes
or section boundaries moved, whether lifecycle promotion was supported by the
required retained evidence, and whether the package's focused and admission
requirements passed on the audited bytes.

## Severity-Ranked Findings

### `V6-RB-001` — Critical — focused contract authority suite fails

References on the audited bytes:

- `tests/integration/vegetation_boundary_authority_contract.rs:186-191`;
- `tests/integration/vegetation_boundary_authority_contract.rs:621-634`;
- `package.md:52-57` and `package.md:78-85`;
- `artifacts/gate-results.md:18` and `artifacts/gate-results.md:23-32`.

The audited authority test still required `contract_version: 9` in two places
after the canonical contract moved to Version 10. Independent execution of
`cargo nextest run --test vegetation_boundary_authority_contract --profile
quick` ran 21 tests and returned 19 passed / 2 failed. The failures were
`canonical_schema_and_registry_entry_are_bound` and
`coupled_c3_model_stack_and_biogeochemistry_boundary_are_admitted`, both on the
stale Version 9 assertion.

The package's intended write set did not include the contract-derived test,
while its exit criteria require applicable focused/workspace gates to pass.
The gate record retained an earlier expected 17/21 failure but did not contain
a terminal passing authority-suite run.

Scientific/governance impact: canonical Version 10 authority is not bound by
the repository's focused contract-obligation suite. Closing despite the known
failure would violate the non-deferral and exact-terminal-gate rules; calling
the failure “outside this agent's bounded test write set” does not make the
current requirement pass.

Proposed disposition: `accepted`. Amend the package write set to include the
contract-derived authority test, reconcile its lifecycle/model assertions to
Version 10/V6 without weakening predecessor assertions, and retain a fresh
21/21 passing run on the final bytes. Until then the package remains `HOLD`.

### `V6-RB-002` — Critical — lifecycle promotion precedes mandatory verification

References:

- `package.md:73-75`;
- `artifacts/gate-results.md:23-24`;
- `artifacts/final-disposition.md:3-8`;
- `docs/specifications/science-contract-authoring-procedure.md:233-260`.

The package promoted `SC-VEGETATION-001@10` and its registry row to
`approved/active` after dual review but before the two separate terminal
verification agents passed. The canonical authoring procedure defines
verification after finding fixes as a separate hard gate and permits promotion
only after both verifiers return `PASS` or `PASS-WITH-NOTES`.

Scientific/governance impact: an active contract lifecycle currently claims a
promotion condition that the retained package evidence explicitly says is
pending. Review and verification are distinct controls; iterative reviewer GO
does not substitute for the mandatory verifier records.

Proposed disposition: `accepted`. Restore or otherwise retain fail-closed
review lifecycle until compliant review/disposition evidence and both
independent verifications pass, then promote and rerun admission on the exact
promoted bytes. Do not use this HOLD review as promotion evidence.

### `V6-RB-003` — High — retained review and disposition records are incomplete

References on the audited bytes:

- `artifacts/review_agent_a.md:1-15`;
- the prior compressed `artifacts/review_agent_b.md:1-14` replaced by this
  retained review;
- `artifacts/review-finding-disposition.md:5-18`;
- `docs/specifications/science-contract-authoring-procedure.md:205-231`.

Both retained reviewer files were compressed final summaries. They did not
preserve severity-ranked stable finding IDs, exact file/line references,
scientific/governance impacts, or proposed dispositions. The disposition table
described findings but omitted the procedure's required stable `finding_id`
and `artifact_ref` fields and did not preserve an explicit rationale field per
row.

Scientific/governance impact: a reader cannot reconstruct which exact reviewed
bytes supported each finding, independently verify closure, or prove that all
accepted findings were mapped without relying on unretained agent dialogue.
This is especially material because the retained GO summaries claimed no open
finding while the focused authority suite still failed.

Proposed disposition: `accepted`. Preserve complete independent Reviewer A and
Reviewer B records, then rebuild the disposition with stable IDs, sources,
severity, decisions, actions, artifact references, rationale, and closure
status for every finding. Run separate verification only after those fixes.

### `V6-RB-004` — Medium — package catalog insertion corrupts entry ownership

Reference on the audited bytes: `docs/work-packages/README.md:14-20`.

The V6 bullet was inserted between the V4 package headline and its indented
`DUAL REVIEW GO / HEAVY 6/6 PASS / TERMINAL A+B PASS` continuation. Those V4
closure claims therefore appeared to belong to V6. The V6 bullet also said
“under dual review” after both reviews had reportedly completed.

Scientific/governance impact: the execution catalog falsely attributes heavy
and terminal evidence to the new package and gives a stale lifecycle status.
Markdown lint cannot detect that semantic ownership error.

Proposed disposition: `accepted`. Restore the V4 continuation to its V4 entry
and give V6 a separate, current, narrowly scoped status that makes no heavy,
terminal, runtime, or activation claim.

### `V6-RB-005` — Low — generated Python cache remains in the package tree

Reference:

- `artifacts/__pycache__/reference_calculator_v6.cpython-312.pyc`.

The audited package tree contained a generated bytecode cache. It was ignored
by Git and therefore absent from `git diff --name-only`, but remained part of
the filesystem bytes inspected for terminal hygiene.

Scientific/governance impact: none to the comparison result, but retaining
generated interpreter state makes package-size and exact-tree statements less
clean and can confuse later artifact inventories.

Proposed disposition: `accepted`. Remove the cache before closure and execute
future generator/verifier checks with bytecode generation disabled.

## Portability and Anti-Laundering Assessment

No material defect was found in the selected scalar rule itself.

- The canonical contract restricts eligibility to the same rejected trajectory
  and requires exact V6 model/configuration, transaction, occupancy, capped
  pass, hydraulic solve, field, typed failure, candidate absence, presence,
  iteration/backtracking counts, cardinality/order, active bounds and water
  caps, complementarity branches, stored unit/basis, and rollback identity at
  `SC-VEGETATION-001.md:1641-1676`.
- The only eligible scalar is nonnegative
  `backtracking_limit.step_norm=max_j(abs(delta_x_j))` over the rejected
  unscaled six-variable correction. Accepted values, residuals, pivots, matrix
  norms, authorization, conservation, and branch-selection values are excluded.
- The rule is exactly
  `abs(a-b) <= 3e-7*max(abs(a),abs(b))`, inclusive, symmetric, and binary64, at
  `SC-VEGETATION-001.md:1678-1705`. No absolute tolerance, hidden ULP allowance,
  decimal pre-rounding, retry widening, or platform exception is admitted.
- Negative values, nonfinite values, sign-class changes, and zero/nonzero
  changes reject before tolerance evaluation. `+0.0` and `-0.0` are the sole
  shared zero class.
- The acceptance firewall at `SC-VEGETATION-001.md:1707-1725` prevents the
  evidence comparator from accepting a solve, publishing a candidate, changing
  a convergence threshold, mutating a transaction, or normalizing physical or
  conservation values.
- `reference_calculator_v6.py:13-57` implements those exact preconditions and
  the rtol-only operation order. Its complete-record poisons at lines 190-209
  cover identity, presence, failure, candidate, units/basis, counts/order,
  branches, accepted-value posture, and rollback. Lines 164-188 cover the
  observed pair, inclusive boundary, immediate outside value, reversed/lower
  direction, signed zero, subnormal zero mismatch, sign, negative norm, NaN,
  and both infinity signs.
- The V5-to-V6 fixture preserves canonical non-identity scientific payload bytes
  while deriving distinct V5/V6 configuration, state, and diagnostic identities.
- The verifier binds both V6 definition copies, the V6 contract-section digest,
  immutable V5 section digest, generator/vector identities, executable cases,
  and every canonical predecessor definition copy.

The observed allowance is approximately `0.0011777563267315204` versus an
observed absolute delta of `0.0011254859045948251`, so the measured case uses
about 95.6 percent of the envelope. The narrow margin and complete exact
preconditions support the selected `3e-7` ceiling without tolerance laundering.

## Protected Bytes, Diff, and Claim Boundary

Independent SHA-256 reconstruction matched all protected definitions:

- V1: `003107043e8eb5bda6d9d6476e3ea01690815e3280ac98daf169317ce4d09157`;
- V2: `38e1bb90abd3ff82879f7d9c80b0377bb510a3b97fdd2b6f07c12b7c42b80dc3`;
- V3: `7768657ca3d03603b66f5cd6677f032ee630fdd46d6ffadf214c713065f73852`;
- V4: `8ace38d1148f95261306cd6b0bf6f22e23ac8ead4cb6897dbdb53061b78ee437`;
- V5: `0ee6a50d5f72da0b9344d8bf1b77674e95a66ab196edc068851bb419eb7b36f3`.

Both V6 definition copies matched
`a5a5ed77b4672b97b7c50103089067d70ade03bc1b5aff4e08ba6fdffc05d426`.
The generator, vectors, verifier, V6 section, and protected V5 section matched
their bound identities. No production crate, Cargo manifest/lockfile, runtime
selector, deployment, publication, consumer-cutover, or activation path was in
the audited diff. Production remained bound to V5, consistent with the
package's implementation-authority-only claim.

## Commands Run

- `.venv/bin/python .../reference_calculator_v6.py --verify
  .../openwepp_c3_woody_v6_vectors.json`: PASS; byte-identical regeneration.
- `.venv/bin/python .../verify_v6_authority.py`: PASS.
- `check_science_contract_admission.sh --base-ref HEAD --worktree`: PASS;
  `A0_ADMITTED contracts=45 science_surfaces=0`, authority SHA-256
  `85210adc267bab7ca4f6693bb0684e354620acc9b04505f0cb188840bceb9576`.
- `check_sc_unit_compliance.sh --path SC-VEGETATION-001.md`: PASS.
- `check_authority_suite_antievasion.sh`: PASS.
- `cargo nextest run --test auth11_required_suite_obligation_guards_contract
  --profile quick`: PASS, 3/3.
- `cargo nextest run --test vegetation_boundary_authority_contract --profile
  quick`: FAIL, 19/21; see `V6-RB-001`.
- `markdown-doc lint` for the package, contract, contract index, and package
  catalog: PASS with zero findings.
- `cargo fmt --all -- --check` and `git diff --check`: PASS.
- Exact changed/untracked-path audit, definition-copy comparison, predecessor
  digest reconstruction, package-size check, and prompt-state inspection: PASS
  subject to the findings above.

## Terminal and Prompt Posture

The active kickoff prompt was correctly still present and absent from the
archive on the audited bytes. `artifacts/terminal-diff-reconciliation.md:5-8`
and `artifacts/final-disposition.md:3-8` truthfully remained pending. Because
this review returns `HOLD`, the prompt must remain active and those artifacts
must not be promoted to complete until all accepted findings are corrected,
both independent verifiers pass on the corrected exact bytes, the prompt is
archived byte-for-byte, and the final diff is reconciled again.

## Recommendation

`HOLD`. The portable comparison itself is narrow, digest-bound, and resistant
to tolerance laundering, and no production/runtime activation claim was found.
The package cannot promote or close on the audited bytes because its focused
authority suite fails and its review, disposition, and promotion lifecycle do
not satisfy canonical retained-evidence requirements. Resolve
`V6-RB-001` through `V6-RB-005`, rerun the affected gates, and request fresh
independent verification on the resulting exact bytes.

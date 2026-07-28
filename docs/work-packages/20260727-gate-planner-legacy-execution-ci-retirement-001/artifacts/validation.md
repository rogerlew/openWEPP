# Validation

Evidence class: Ran.

## Focused Results

The following direct requirements pass at reviewed implementation subject
`f8c4502ada673e93734d391d098961c3e8cf1e58`:

- science-contract admission against scaffold commit `3f9d2c15`:
  `A0_ADMITTED`, 39 contracts, zero changed science surfaces;
- focused Nextest: 23 of 23 tests passed across
  `advisory_linter_authority_contract`,
  `quality_observatory_workflow_contract`,
  `quality_observatory_merged_coverage_contract`, and
  `auth11_required_suite_obligation_guards_contract`;
- `check_authority_suite_antievasion.sh`: passed;
- `quality_observatory_workflow.py self-test`: passed, including the expected
  deferred-priority case;
- JSON parsing, Python bytecode compilation, Cargo workspace metadata, shell
  syntax, package/runner Markdown lint, `cargo fmt --all --check`, and
  `git diff --check`: passed;
- workspace metadata contains no `openwepp-gate-planner` package.

The first focused Nextest invocation failed before test execution because
`.config/nextest.toml` still named retired planner packages. The selectors were
removed in scope, and the repeated focused run passed 23 of 23 tests.

Review A/B then found that closure prose had reintroduced the retired product
name into the live runner README. The focused quality workflow contract
correctly failed 1 of 5 tests. The sentence was removed, the omitted
zero-consumer Python-ledger fixture was deleted, and the repeated workflow
contract passed 5 of 5 tests.

## Exact Diff And Size

The corrected, resolvable package base is
`c5dc88fc063927f3bbb3941cab07fbdf77758aa9`. The exact staged final tree
contains 147 paths, 1,064 insertions, and 49,322 deletions. The only untracked
path is the excluded, user-owned readiness audit.

New Rust test owners contain 286 and 154 lines. No new or enlarged production
Rust file approaches the 2,000-line governance threshold; the production Rust
control plane is deleted. The pre-existing quality-observatory Python owners
remain 2,638 and 1,368 lines and were migrated in place rather than split as
part of this retirement package.

## Terminal Results

The first delegated broad run overlapped active review edits and is not terminal
evidence. Strict Clippy passed. Nextest stopped after 192 of 2,137 tests because
`cqr_quality_evidence_handoff_contract` correctly rejected its no-longer-current
exact-head fixture; 191 executed tests passed and one failed. Logs are retained
under `target/order4-heavy-closure/`. No retired planner selector or package
appeared in either log.

The package-authorized comparator-suite runner repeated both commands at clean
reviewed head `f8c4502ada673e93734d391d098961c3e8cf1e58`:

- `cargo nextest run --workspace`: 2,137 of 2,137 passed, five skipped;
  1,158.40 seconds elapsed;
- `cargo clippy --workspace --all-targets --all-features -- -D warnings`:
  passed; 11.60 seconds elapsed.

Terminal logs are retained under `target/order4-heavy-closure-final/`. Dual
review and dual verification also passed without open findings.

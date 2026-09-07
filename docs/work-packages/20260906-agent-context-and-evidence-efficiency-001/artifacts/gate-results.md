# Validation evidence

Terminal independent verification: A PASS (rules/metrics, exact paired reports,
recursive protocol/authority routing, five TOML parses and focused tests), B PASS
(23 administrative tests, complete 11-test Rust run ID
173a94ae-ecc3-4216-9e0b-8a3ce0fb9d9b, standalone CLI recovery and 19 negative
exit-2 probes). Exact commands/paths are in verification_agent_a/b.md. B's
retained independent bundle is /tmp/openwepp-verifier-b-9fWKjk/bundle, manifest
SHA-256 1c19d79daf224167902a623403d8b9c36baac9090753398f2d5742059a116687.
VA-01 CQR catalog write-set correction received both focused reviews and A's
independent verification. No unresolved accepted findings remain.
Terminal diff reconciliation: 60 paths, 35 non-package, all within declared
scope; no production Rust/canonical SC edits, only named administrative test.
Final index/status wording adds 14 bytes to the measured author locator read;
context-after.json regenerated and final figures updated. No quota inference.

Ran: parent checks below; independent checks pending. Base 033dfe30073bc22aaa30aed877cc301b6745a086; administrative intent, no production Rust/SC changes.

- .venv/bin/python -m unittest discover -s tools/agents -p 'test_*.py': 19 passed, exit 0; /tmp/openwepp-context-admin-tests.log. Includes source/index custody, removal of original fixture, execution, corruption/missing/path/destination/symlink/environment/race tests, identity mutations, role routing, Markdown links/anchors and TOML.
- Initial tests exposed missed early source mutation (fixed with whole-selection stat fence) and atime false-positive (fixed by excluding atime from mutation fingerprint). Retained session output records failures; corrected tests pass.
- git diff --check: exit 0.
- TOML parse: five configuration files PASS. codex --strict-config features list: exit 1, unsupported command option; bundled offline model metadata inspected, no service/runtime claim.
- Direct cargo nextest command: exit 127, cargo absent from ambient PATH; /tmp/openwepp-context-governance-tests.log.
- nix develop --offline --command cargo nextest run --offline --test adr0017_comparator_distrust_ratification_contract --test advisory_linter_authority_contract: 10/11 pass, exit 100; /tmp/openwepp-context-governance-nix.log. Live standard hash mismatch fixed prospectively in impact-map.json.
- Same focused command after hash fix: 10/11 pass, exit 100; /tmp/openwepp-context-governance-nix-corrected.log. Remaining failure: inherited WAT5 row-count assertion expects 22, base and current map contain 27. Static exact base comparison confirms entries identical. Changed live hash assertion passes before that inherited assertion. No science binding or Rust test changed to hide it.

The inherited attempts remain FAIL. Review rejected closing with that selected
failure. The owner's administrative-test exception was used prospectively:
tests/integration/advisory_linter_authority_contract.rs changes only its stale
inventory expectation 22 to existing 27. Base/cut map entries are identical;
all shared-path bindings remain asserted. The complete same Nextest command now
passes 11/11, exit 0, run ID dff2a2ab-3bfb-48cc-8706-02c0227782c8;
/tmp/openwepp-context-governance-final.log. No test deleted, filtered or weakened;
no production, external-authority cohort/case/atomic-binding semantics changed.
Full workspace/science experiments remain inapplicable. No heavy runner fallback.

Corrected security/context suite: .venv/bin/python -m unittest discover -s
tools/agents -p 'test_*.py': 23 PASS, exit 0;
/tmp/openwepp-context-admin-corrected-tests.log. Includes working/staged chained
symlinks, noncanonical destinations and distinct-revision context accounting.
Python AST parse: three files PASS. nix develop --offline --command rustfmt
--check tests/integration/advisory_linter_authority_contract.rs initially found
inherited formatting in the final assertion (/tmp/openwepp-context-rustfmt.log).
Applied only that formatter-equivalent layout correction; rerun exit 0,
/tmp/openwepp-context-rustfmt-corrected.log. Explicit historical revision-61 search still
resolves original disposition; no historical evidence rewritten.

Context report commands: .venv/bin/python tools/agents/context_report.py <package>/artifacts/context-inputs.json --phase before|after (exact repository package path in adjacent inputs). Results are structural byte measurements, not observed total usage.

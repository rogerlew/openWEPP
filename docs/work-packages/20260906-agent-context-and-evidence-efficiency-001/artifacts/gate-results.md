# Validation evidence

Ran: parent checks below; independent checks pending. Base 033dfe30073bc22aaa30aed877cc301b6745a086; administrative intent, no production Rust/SC changes.

- .venv/bin/python -m unittest discover -s tools/agents -p 'test_*.py': 19 passed, exit 0; /tmp/openwepp-context-admin-tests.log. Includes source/index custody, removal of original fixture, execution, corruption/missing/path/destination/symlink/environment/race tests, identity mutations, role routing, Markdown links/anchors and TOML.
- Initial tests exposed missed early source mutation (fixed with whole-selection stat fence) and atime false-positive (fixed by excluding atime from mutation fingerprint). Retained session output records failures; corrected tests pass.
- git diff --check: exit 0.
- TOML parse: five configuration files PASS. codex --strict-config features list: exit 1, unsupported command option; bundled offline model metadata inspected, no service/runtime claim.
- Direct cargo nextest command: exit 127, cargo absent from ambient PATH; /tmp/openwepp-context-governance-tests.log.
- nix develop --offline --command cargo nextest run --offline --test adr0017_comparator_distrust_ratification_contract --test advisory_linter_authority_contract: 10/11 pass, exit 100; /tmp/openwepp-context-governance-nix.log. Live standard hash mismatch fixed prospectively in impact-map.json.
- Same focused command after hash fix: 10/11 pass, exit 100; /tmp/openwepp-context-governance-nix-corrected.log. Remaining failure: inherited WAT5 row-count assertion expects 22, base and current map contain 27. Static exact base comparison confirms entries identical. Changed live hash assertion passes before that inherited assertion. No science binding or Rust test changed to hide it.

The inherited mixed test failure remains FAIL, not a full PASS claim. Independent reviewers must adjudicate its relevance to administrative closure; current modified-binding behavior is exercised, scientific row-count correction is outside protected scope. Full Rust workspace, science experiments, anti-evasion suites not selected: no production, external-authority cohort/case/binding semantics changed. Updating a schema-only nonblocking live documentation hash does not change those semantics. No heavyweight execution/comparator requirement selected; no runner fallback claimed.

Context report commands: .venv/bin/python tools/agents/context_report.py <package>/artifacts/context-inputs.json --phase before|after (exact repository package path in adjacent inputs). Results are structural byte measurements, not observed total usage.

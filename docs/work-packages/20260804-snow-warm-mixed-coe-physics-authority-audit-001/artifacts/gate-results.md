# Gate Results

Status: final pass

Evidence mode: Ran unless marked Static

Working directory for every command: `/home/workdir/openWEPP`.

| Requirement | Command / evidence | Result |
| --- | --- | --- |
| Python syntax | `.venv/bin/python -m py_compile docs/work-packages/20260804-snow-warm-mixed-coe-physics-authority-audit-001/tools/audit_coe_authority.py docs/work-packages/20260804-snow-warm-mixed-coe-physics-authority-audit-001/tools/test_audit_coe_authority.py` | `PASS` |
| Focused tests | `.venv/bin/python -m unittest docs/work-packages/20260804-snow-warm-mixed-coe-physics-authority-audit-001/tools/test_audit_coe_authority.py -v` | `PASS`, 4 tests |
| Frozen identity and quantitative rerun | `.venv/bin/python docs/work-packages/20260804-snow-warm-mixed-coe-physics-authority-audit-001/tools/audit_coe_authority.py --freeze docs/work-packages/20260804-snow-warm-mixed-coe-physics-authority-audit-001/artifacts/audit-freeze.json --output target/snow_warm_mixed_coe_physics_authority_audit/terminal-quantitative-audit.json --receipt target/snow_warm_mixed_coe_physics_authority_audit/terminal-execution-receipt.json` | `PASS`, 19 frozen inputs including pinned Git blob |
| Accepted-result reproducibility | `cmp docs/work-packages/20260804-snow-warm-mixed-coe-physics-authority-audit-001/artifacts/quantitative-audit.json target/snow_warm_mixed_coe_physics_authority_audit/terminal-quantitative-audit.json` | `PASS`, byte-identical |
| Quantitative tolerance | terminal receipt | `PASS`, `394705` hours, `17431` days, maximum `9.941202185450096e-18 m` |
| JSON parse | `jq empty docs/work-packages/20260804-snow-warm-mixed-coe-physics-authority-audit-001/artifacts/audit-freeze.json docs/work-packages/20260804-snow-warm-mixed-coe-physics-authority-audit-001/artifacts/quantitative-audit.json docs/work-packages/20260804-snow-warm-mixed-coe-physics-authority-audit-001/artifacts/execution-receipt.json target/snow_warm_mixed_coe_physics_authority_audit/terminal-quantitative-audit.json target/snow_warm_mixed_coe_physics_authority_audit/terminal-execution-receipt.json` | `PASS` |
| Markdown lint | `markdown-doc lint --path docs/work-packages/20260804-snow-warm-mixed-coe-physics-authority-audit-001 --format plain` | `PASS`, 29 files, 0 errors/warnings |
| Markdown schema | `markdown-doc validate --path docs/work-packages/20260804-snow-warm-mixed-coe-physics-authority-audit-001 --format plain` | `PASS`, 29 files, 0 errors |
| Instruction discovery | `tools/agents/find-agents --for docs/work-packages/20260804-snow-warm-mixed-coe-physics-authority-audit-001 docs/work-packages/README.md docs/ROADMAP.md docs/planning/snow-surface-energy-balance-roadmap.md` | `PASS`, root plus package-local chain reproduced |
| Prompt archive | `git show c95edb9f:docs/work-packages/20260804-snow-warm-mixed-coe-physics-authority-audit-001/prompts/active/20260804-snow-warm-mixed-coe-physics-authority-audit-001_kickoff_agent_prompt.md \| sha256sum` and `sha256sum docs/work-packages/20260804-snow-warm-mixed-coe-physics-authority-audit-001/prompts/archived/20260804-snow-warm-mixed-coe-physics-authority-audit-001_kickoff_agent_prompt.md` | `PASS`, both `7cbfc23a...ba0e` |
| Spelling preview | `uk2us` preview plus targeted British-spelling scan | `PASS_WITH_REVIEW`, only protected `CoE` acronym/path false positives; no targeted British spelling remains |
| Diff hygiene | `git diff --check` | `PASS` |
| Exact write set | `git diff --name-only ae3f49a3684b3da35a35a2250ee362e147259b09 --` plus `git ls-files --others --exclude-standard -- docs/work-packages/20260804-snow-warm-mixed-coe-physics-authority-audit-001` | `PASS`, 37-path union including the archived prompt, 0 outside declared tracked set |
| Protected paths | manual classification of the preceding exact path inventory | `PASS`, 0 production/contract/test/reference paths |
| Rust line-count exposure | preceding inventory plus `wc -l docs/work-packages/20260804-snow-warm-mixed-coe-physics-authority-audit-001/tools/audit_coe_authority.py docs/work-packages/20260804-snow-warm-mixed-coe-physics-authority-audit-001/tools/test_audit_coe_authority.py` | `PASS`, 0 `.rs` paths; package Python `477 + 142` lines |
| Dual science review | `review_agent_a.md`, `review_agent_b.md`, `review-disposition.md` | `PASS_WITH_FINDINGS`; all seven accepted and remediated |
| Dual terminal verification | `verification_agent_a.md`, `verification_agent_b.md` | `PASS`; both independent rechecks found no remaining blocker |

Not selected: cargo profiles, full workspace, comparator, release, anti-evasion,
or coverage/CRAP. Exact terminal diff changes no production, contract, test,
fixture, observation, authority-suite binding, or public behavior, so those
gates are not applicable under the frozen intent and testing strategy.

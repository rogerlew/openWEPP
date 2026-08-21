# Gate results

Status: complete / terminal dual verification PASS

Evidence mode: Ran

| Gate | Status | Evidence |
| --- | --- | --- |
| Exact base and clean tree | PASS | `1d0239f4a` release verified at intake; local Child 2C edits are now the active review tree. |
| Contract schema/profile | PASS | Markdown, five strict Binding Exposure Index checks, five JSON artifacts, and receipt fixtures pass structural checks. |
| Contract-derived vectors/tests | PASS | Independent Python oracle: 17 cases, 9 accepted/8 rejected; focused nextest: 5 passed. |
| Science review A/B | PASS / dispositioned | `review_agent_a.md` and `review_agent_b.md` completed HOLD reviews; all 17 findings are dispositioned in `review-finding-disposition.md`. |
| Verification A/B | PASS / terminal | `verification_agent_a.md` and `verification_agent_b.md` both return bounded terminal PASS after the custody correction chain. |
| Exact diff and docs integrity | PASS | `exact-diff-reconciliation.md` records base, allowlist, no production Rust, package lint, and diff checks. |
| Production Rust | OUT OF SCOPE | Later implementation package |

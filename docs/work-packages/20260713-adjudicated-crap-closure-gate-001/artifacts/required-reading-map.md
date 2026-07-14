# Required Reading Map

Evidence class: **Ran + Static**

## Instruction Discovery

`tools/agents/find-agents --for` was run on every declared write path before
edits. Root `AGENTS.md` applies to all paths. `tests/AGENTS.md` additionally
applies to `tests/python/test_adjudicated_crap_gate.py`;
`docs/standards/AGENTS.md` applies to both standards; and
`docs/work-packages/AGENTS.md` applies to package files and the package index.

## Reading Tiers

| Path | Tier | Reason |
| --- | --- | --- |
| `AGENTS.md` | Core | Repository-wide closure and truthfulness rules |
| `docs/work-packages/AGENTS.md` | Core | Package, delegation, review, and evidence governance |
| `docs/standards/AGENTS.md` | Core | Standards-editing governance |
| `docs/standards/prompt-wording-guidance.md` | Core | Required delegated-run and review wording |
| `docs/decisions/0021-module-coverage-closure-thresholds.md` | Core | Binding CRAP threshold and eligibility authority |
| `docs/standards/code-quality-refactor-authoring-guide.md` | Core | Active CQR ratchet procedure |
| `docs/standards/module-test-enhancement-authoring-guide.md` | Core | Coverage/CRAP measurement and exception rules |
| `tests/AGENTS.md` | Conditional | Applies before focused-test edits |
| `docs/work-packages/cqr-pre-integration-campaign-execution-contract.md` | On-demand | Exact production filter and deduplication rule |
| `docs/work-packages/cqr-pre-integration-campaign-evidence/low/campaign-final-assessment.md` | On-demand | Terminal empty-actionable-set authority |
| `docs/work-packages/cqr-pre-integration-campaign-evidence/low/raw-to-actionable-ledger.md` | On-demand | Exact retained row classification |

The ten measured instruction and authority files total `109321` bytes, below
the `400000`-byte `OK` threshold.

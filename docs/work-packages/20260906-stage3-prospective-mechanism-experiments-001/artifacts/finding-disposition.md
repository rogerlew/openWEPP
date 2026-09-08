# Finding disposition

Static: prospective design review, not terminal implementation review.

| Finding | Decision | Action/evidence |
|---|---|---|
| Review A authority/protocol | accepted | GO for design; runtime admission remains pending |
| Review B authority/protocol | accepted | GO for design; runtime admission remains pending |
| Review B competitive-candidate ambiguity | accepted | prospective protocol now defines scale selection before results |
| Review B mapping units | accepted | preserve kernel field names; distinguish Pss_File from RssFile |

## 2026-09-08 terminal review findings

| Finding | Disposition | Evidence |
|---|---|---|
| A/B: final raw series only in `/tmp` | ACCEPTED, FIXED | all six controlling `results.jsonl` files and timing freeze records copied under `artifacts/raw/` |
| A/B: collector suite 8/8 ERROR after fixture replacement | ACCEPTED, FIXED | fixture now consumes the current durable admission record/identity; `.venv/bin/python -m unittest test_collectors.py` PASS 8/8 |
| B: architecture handoff retained pending wording | ACCEPTED, FIXED | owner-ready text reconciled to treatment-results.md |
| A: F 10/19 scale qualification unmet | ACCEPTED LIMIT, NOT FIXED | 10-OFE execution has P=0 and violates the frozen applicability rule; 19 OFE remains NOT RUN; F is bounded to useful building block, not standalone/scaling claim |
| A: one-OFE causal language too broad | ACCEPTED, NARROWED | result wording states exact one-OFE invocation-local effect and does not extrapolate |

Focused re-review and dual terminal verification PASS for the truthful bounded
HOLD. The unmet scale obligation continues to prevent COMPLETE.

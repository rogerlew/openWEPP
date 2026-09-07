# Prospective authority gate

Ran: the exact new F and R contract tests each passed (one test each), logs
raw/authority-F-test.log and raw/authority-R-test.log. Static: dual independent
protocol-review-a.md and protocol-review-b.md approve the four-file authority
cut hashed in authority-proposal.md. Historic authority remains unchanged.

Authority author ran strict binding-exposure and unit-compliance checks on both
contracts: PASS (authority-proposal.md). No F/R behavior has been implemented.
Runtime-derived tests, forced-reference parity and implementation reviews remain
required for candidate admission; a structural/textual gate is not runtime proof.

Invalid metadata: early executor duration_ms fields are shell-clock arithmetic
errors (including a negative value). They are retained but not duration evidence;
command exits and Nextest result counts are direct evidence. No duration is
inferred and no passing workflow is rerun solely to repair duration metadata.

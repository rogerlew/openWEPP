# Finding disposition

Static: findings from independent cut 6ab4e5fb6; Ran: corrected tests below.

| Finding | Disposition | Correction and evidence |
| --- | --- | --- |
| A-01 selected Rust FAIL | accepted, fixed; focused re-review pending | Prospective exact test-path amendment; stale administrative inventory count increased 22 to 27, no bindings changed. Complete same 11-test Nextest command now PASS, /tmp/openwepp-context-governance-final.log. |
| B1 composed symlink escape | accepted, fixed; focused re-review pending | Reject every parent-traversing symlink component; working and distinct staged regressions. |
| B2 noncanonical destination overlap | accepted, fixed; focused re-review pending | Reject .. in destination before overlap checks/writes; regression proves checkout bundle never created. |
| B3 missing mandatory protocol context | accepted, fixed; focused re-review pending | Both reports now count frozen Stage-3 kickoff, seven explicit historical prerequisites and governance Core recursively, plus conditional LSE authority/instructions and parity expansion. Same before/after convention. |
| B nonblocking multi-revision accounting | accepted, fixed | Unique coverage now keys path plus content digest; distinct-revision test prevents conflating different bytes. |

No findings rejected/deferred or waived. Failed attempts remain in review/gate
records. Runtime settings and workflow-total remain UNOBSERVED, not findings
manufactured into PASS. All selected checks must pass and independent closure
remains pending until corrected-cut re-review and dual verification.

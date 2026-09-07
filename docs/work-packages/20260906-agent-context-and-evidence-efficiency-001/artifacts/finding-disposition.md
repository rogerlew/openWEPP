# Finding disposition

VA-01 MEDIUM (independent terminal verification): accepted. CQR package
template's intended write set omitted README.md despite required catalog update.
Fixed by retaining both active.md and README.md; no obligation removed.
23-test suite passes after correction; focused review/verification pending.

Static: findings from independent cut 6ab4e5fb6; Ran: corrected tests below.

| Finding | Disposition | Correction and evidence |
| --- | --- | --- |
| A-01 selected Rust FAIL | accepted, fixed; independent focused re-review PASS at 772eade86 | Prospective exact test-path amendment; stale administrative inventory count increased 22 to 27, no bindings changed. Complete same 11-test Nextest command now PASS, /tmp/openwepp-context-governance-final.log. |
| B1 composed symlink escape | accepted, fixed; independent focused re-review PASS at 772eade86 | Reject every parent-traversing symlink component; working and distinct staged regressions. |
| B2 noncanonical destination overlap | accepted, fixed; independent focused re-review PASS at 772eade86 | Reject .. in destination before overlap checks/writes; regression proves checkout bundle never created. |
| B3 missing mandatory protocol context | accepted, fixed; independent focused re-review PASS at 772eade86 | Both reports now count frozen Stage-3 kickoff, seven explicit historical prerequisites and governance Core recursively, plus conditional LSE authority/instructions and parity expansion. Same before/after convention. |
| B nonblocking multi-revision accounting | accepted, fixed | Unique coverage now keys path plus content digest; distinct-revision test prevents conflating different bytes. |
| A-02 incomplete administrative input membership | accepted, fixed; independent focused re-review PASS at 772eade86 | All live context selections, direct test documents/schema, build configuration and command/environment/Python binding included. Frozen reads bind revision plus content hash. No complete Rust executable custody or automatic reuse claim. |

No findings rejected/deferred or waived. Failed attempts remain in review/gate
records. Runtime settings and workflow-total remain UNOBSERVED, not findings
manufactured into PASS. All selected checks must pass and dual verification remains pending. Both independent reviews approve corrected cut
772eade86; initial FAILs and focused fix evidence remain in their own artifacts.

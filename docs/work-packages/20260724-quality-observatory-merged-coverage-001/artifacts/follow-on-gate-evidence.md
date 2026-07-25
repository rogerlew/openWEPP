# Follow-On Gate Evidence

Evidence class: Ran.

Current correction gates:

- Python bytecode compilation: `PASS`.
- Quality observatory behavioral self-test: `PASS`.
- Focused integration/source contract: 5 passed.
- Rustfmt: `PASS`.
- Warnings-denied Clippy for the integration contract: `PASS`.
- Diff whitespace check: `PASS`.
- Independent measurement re-review: `PASS`, no findings.
- Independent security re-review: `PASS`; one HIGH accepted, fixed, and closed.
- Fresh real execution-snapshot admission: `READY`; exact `/.venv\n` policy and
  empty Git status.
- Three attempt-5 exact-checkout identities in that snapshot: 3 passed,
  3 slow, 177 skipped; `1265.185s`.
- Runtime-artifact Python compile/self-test and focused contract: `PASS`, 5/5.
- Runtime-artifact measurement re-review: `PASS`, no findings.
- Runtime-artifact security re-review: `PASS`; one HIGH accepted, fixed, and
  closed.
- Fresh committed runtime-consumer admission: `READY`; 2,279 full, 36
  science-manual, and 2,315 workspace tests.
- Exact assurance publication consumer probe: `PASS`; 3 passed in `47.951s`.
- Admitted executable manifest: 291 rows before and after, exact equality.
- Admitted working-tree identity: exact equality before and after.
- Attempt 11 admission: `READY`; exact 2,279 / 36 / 2,315 inventories.
- Attempt 11 `full`: `PASS`; 2,279 passed, 15 slow, 31 skipped;
  `2299.240s`.
- Attempt 11 `science-manual`: `FAIL`; 35 passed, 1 failed, 1 slow;
  `480.503s`.
- Attempt 11 final read-only identity: 291 executable rows and working tree
  equal admission.
- Attempt 11 publication: not reached; 0 files and no evidence ID.
- Science confinement and TESTGATE scheduling checks: `PASS`, 2/2.
- Unlisted ordinary-path adversarial probe: expected `FAIL`, exact path
  reported.
- Nested allowed-suffix collision probe: expected `FAIL`, exit 100, exact
  unlisted path reported.
- Science-confinement Rustfmt and warnings-denied Clippy: `PASS`.
- Science-confinement implementation re-review: `PASS`, no findings.
- Science-confinement security re-review: one `HIGH` fixed and closed; final
  `PASS`, no findings.
- Attempt 12 `full`: `FAIL`; 2,278 passed, 1 failed, 15 slow, 31 skipped;
  `2840.520s`.
- Attempt 12 sole failure: gate-planner public-audit coverage test rejected
  transient shared-checkout dirt.
- Attempt 12 final identity: 291 executable rows and working tree equal
  admission; source and snapshot clean.
- Attempt 12 publication: not reached; 0 files and no evidence ID.
- Isolated public-audit consumer: three `PASS` runs, including one with
  deliberate ambient untracked dirt.
- Snapshot-isolation implementation/security reviews: both `PASS`, no
  findings.

- Attempt 13 admission: `READY`; exact 2,279 / 36 / 2,315 inventories.
- Attempt 13 `full`: `PASS`; 2,279 passed, 16 slow, 31 skipped;
  `3473.738s`.
- Attempt 13 `science-manual`: `PASS`; 36 passed, 1 slow, 2,286 skipped;
  `482.897s`.
- Merged snowbench gate: `PASS`; all 18 rows prove science-manual
  contribution and none remain false debt.
- Adjudicated CRAP: `PASS`; 11,432 production entries, 2 raw rows,
  2 adjudicated rows, and 0 actionable rows.
- Publication: 11 regular files, 1,421,222 bytes; evidence ID
  `f641feeda798047dac30ad7ef760bbadc31b71265e32415353be71b53e8b5544`.
- Terminal verifier A: `PASS` after infrastructure-only ENOSPC remediation;
  exact inventories, head, publication, and evidence ID.
- Terminal verifier B: `PASS` after the same independent remediation; exact
  inventories, head, publication, and evidence ID.
- Final findings: none.

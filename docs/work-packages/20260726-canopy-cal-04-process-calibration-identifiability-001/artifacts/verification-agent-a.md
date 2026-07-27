# Independent Verification A

Status: `COMPLETE`

Evidence class: `Ran: independent command replay, inventory reconciliation,
checksum, deterministic rebuild, validator, prompt, documentation, diff, and
write-set checks; Static: authority, stage order, and hold legitimacy`

Verdict: `PASS — EXECUTED / HOLD`

The verifier independently confirmed:

- corrected CMD-008 and CMD-013 replay exactly with exit 0;
- all CMD-001..013 are referenced by the 19-row inventory: seven intake
  (including retained failed CMD-004), five closure, and seven blocked
  scientific rows;
- both checksum manifests and all 14 manifest identities match;
- timing reconstruction is byte-identical: 1,251 rows, SHA-256
  `890a0ff09ca707b097a15cb5de7964698a9b4d5af797ed6b81d5fccf7c141b61`;
- roles are 932 Hubbard calibration and 319 Harvard holdout, disjoint, without
  Harvard fall 1992;
- candidate, failure, accepted-ensemble, and Harvard ledgers have zero rows;
- all stages retain required order and unexecuted stages use
  `accepted_range=NONE`;
- Harvard is sealed, no downstream evaluation exists, and the hold validator
  passes;
- active prompts are empty, kickoff is archived, Markdown lint and diff hygiene
  pass, and all visible status paths are authorized.

Blocked scientific gates are neither waived nor passed and correctly force the
terminal hold. No discrepancy remains.

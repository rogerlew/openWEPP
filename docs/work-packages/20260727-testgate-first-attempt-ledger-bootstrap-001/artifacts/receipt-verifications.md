# Receipt Verifications

Status: `PASS`

Retain independent A/B reports that verify without relabelling:

- package campaign receipt
  `30054b51863488b85d23c95a68b8d5ebc8f5d2d9be5b94959dfec4dab194b54f`
  proves fresh ledger, inherited FD, LIGHT, and READY audit but retains the
  unrelated Clippy failure;
- successor receipt
  `29d71a54d2cf38680190885abaf2d2967d547cdedefc0c31af5e00de669aa5d4`
  proves that external defect and every previously blocked workspace node pass.

## Verifier A

Evidence class: `Static + Ran`

Verdict: `PASS`

Verifier A independently recomputed both receipt IDs, plan/audit/LIGHT
identities, attempt-index sets, every file hash, all ledger records/links,
source identities, counts, ancestry, root baselines, and the five-file
byte-unchanged proof. It confirmed the original honest 9/1/2 result and the
successor 12/12, 2,387/2,387, 2,361/2,361 result remain distinct.

## Verifier B

Evidence class: `Static + Ran`

Verdict: `PASS`

Verifier B independently recomputed 49/49 original and 50/50 successor indexed
files, both four-entry ledgers, exact plan/audit/receipt identities, counts,
retry posture, full log, ancestry, and byte equality of all five ledger paths.
No campaign relabelling, source drift, retry misuse, or protected-state
violation exists.

Both verifiers explicitly retain the read-only Harvard fixture disclosure and
the precise no-CAL-population/no-protected-state-mutation claim.

# Terminal Verification A: Governance

Evidence class: static, provider, host, and execution receipts.

Initial verdict: HOLD. The reviewer confirmed the exact online/idle forest1
runner and accepted gate evidence, but correctly rejected static-only claims
for normal routing and rollback. It also found that rollback reuse rejected
valid 2/2/0 adjudicated closure.

Focused post-patch verdict: PASS for the rollback patch and smoke. All six
broad/reuse steps reject smoke mode; the receipt cannot claim qualification;
the corrected predicate requires authenticated global PASS, closure
eligibility, no invalid adjudications, complete raw-row adjudication, and zero
actionable rows. Provider run `29692305394` then passed with those broad steps
skipped.

Remaining terminal condition: the real normal workflow must schedule forest1,
then complete independent hosted verification and aggregate attestation on the
documentation-only activation commit.

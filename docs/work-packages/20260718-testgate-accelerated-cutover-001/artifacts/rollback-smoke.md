# Conservative Rollback Smoke

Ran: GitHub Actions run `29692305394`, 2026-07-19 UTC.

- Workflow/job: `testgate-conservative` / `openwepp/conservative-rollback`.
- Exact base/head:
  `e16801182975657994434e37e341f866ec696953` /
  `54f7d55f04bd3e73c2ff7313b9d6383a329f2628`.
- Provider result: `success`; elapsed job time: 19 seconds.
- Receipt: schema `openwepp-conservative-smoke-v1`, mode
  `ADMISSION_SMOKE`, result `PASS`, runner environment `github-hosted`, and
  `qualification_claim=false`.
- Checkout, trusted-main comparison admission, smoke proof, and receipt upload
  passed.
- Toolchain installation, spelling installation, authenticated evidence reuse,
  reuse verification, broad conservative validation, and conservative
  comparison were all visibly skipped.

This is executable consumer-path proof that the independent hosted rollback
lane can be admitted and invoked. It is deliberately not a conservative,
critical, campaign, release, or authority qualification result. The full lane
retains all broad requirements, and its reuse predicate now accepts canonical
fully adjudicated current-source closure rather than requiring zero raw rows.

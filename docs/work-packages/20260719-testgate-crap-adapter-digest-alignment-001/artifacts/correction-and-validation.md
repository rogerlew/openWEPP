# Correction And Validation

Evidence class: `Ran` and `Static`

Direct runner SHA-256 is
`b75a06fae6899a05aabb77805933b4466b072a71a58e815430eefcffa0db1a85`.
Both `affected-adjudicated-crap-v1` and `adjudicated-crap-v1` now bind that
value. The JSON diff changes exactly those two strings; commands, risk classes,
prerequisites, outputs, and all other fields are unchanged.

Focused evidence:

- direct SHA/JQ equality: PASS;
- `cargo nextest run --test testgate_align_authority_contract`: 10/10 PASS in
  0.209 seconds; and
- `git diff --check`: PASS.

Critical terminal receipt
`78f526eee1d0b8a9142afc9f3ff8f9434702d1a5409d917a1c2a22687aa7638c`:

- 11 PASS / 1 FAIL / 0 BLOCKED;
- full Nextest: 2,165/2,165 passed, 5 skipped, 1,689.064 seconds;
- global CRAP: FAIL before acquisition because its default absolute output path
  is rejected by executor-safe relative relocation; and
- no GitHub or forest1 execution.

No passing node is represented as closing the failed receipt. The distinct
runner defect is assigned to
`20260719-testgate-global-crap-output-relocation-001`.

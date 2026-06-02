# Verification Agent B

Status: complete

Evidence mode: ran

Ran:

- Full H1..H39 runtime:
  `/tmp/hphys0252_20260602T195147Z/reports/hillslope_batch_status.tsv`.
- Full H1..H39 semantic:
  `/tmp/hphys0252_20260602T195147Z/reports/semantic_status.tsv`.
- Apples-to-apples HPHYS0251 semantic rerun:
  `/tmp/hphys0252_hphys0251_semantic_rerun_20260602T200305Z`.
- Governance guards:
  `tools/release/check_authority_suite_antievasion.sh` and
  `auth11_required_suite_obligation_guards_contract`.

Result:

- Runtime success: `39/39`.
- Semantic pass: `0/39`.
- Apples-to-apples selected-symbol delta from HPHYS0251: zero.
- Governance guards passed.

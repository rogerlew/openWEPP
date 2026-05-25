# simimpl16-span-key-overlap-test-matrix-evidence

Status: complete
Evidence mode: static+ran
Date: 2026-05-25

## Static
- Added overlap/key matrix assertions:
- aligned overlap (`common=2, only_baseline=0, only_candidate=0`) -> promotable.
- span collapse (`common=1, only_baseline=1`) -> hold.
- key-domain mismatch (`common=0, only_baseline=2, only_candidate=2`) -> hold.

## Ran
- `pl14_contract_conformance_requires_nonzero_replay_span_overlap_for_promotion` passed.
- `pl14_contract_conformance_rejects_key_domain_mismatch_before_comparator_promotion` passed.

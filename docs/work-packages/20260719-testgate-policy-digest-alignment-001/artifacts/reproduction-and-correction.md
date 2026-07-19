# Reproduction And Correction

Evidence class: `Static` and `Ran`

Before correction:

- canonical strategy SHA-256:
  `02b9033ca5504cf41411695d73be0b3cbe3bbeb71daecfa94c8410911c0973b3`;
- bound `policy_sha256`:
  `e5a4341832babf04ea7ca79263e7da8c4826b047649e797d82d1e6e24f4ee063`;
- the obsolete digest occurred in executable policy only at
  `gate-policy/v1/impact-map.json`.

Correction commit:
`734a7861f25e38a6d1a37ca453905bd607cd779e`.

After correction, direct `sha256sum` reconstruction and `jq` extraction both
return
`02b9033ca5504cf41411695d73be0b3cbe3bbeb71daecfa94c8410911c0973b3`.
`jq empty` passes. The policy diff is exactly one deletion and one insertion in
the `policy_sha256` value; schema version, policy ID, generation, enforcement,
matchers, risks, and gates are unchanged.

# Terminal Verification B

Evidence class: **Static + Ran**

Disposition: **TECHNICAL PASS / HOLD-INDEPENDENT-HUMAN-REVIEW**.

At terminal generation
`0a63d3fe7d847a0b623c163ac0d83f0ca64a47807b39575d3b0b101f76d50567`,
the verifier confirmed:

- byte- and JSON-identical reproduction of exactly 188 values, SHA-256
  `90cc97ff4893cc45fd478d16358c660a86eb20db3c989088b95758d697c7c0dd`;
- all 121 identity-source digests, the review-lock digest, and all 28 manifest
  `local_content` dependencies match;
- the 98-file review rendering, archived prompt, and 21K-21N staged research
  objects match their bound digests;
- focused amendment cases pass 2/2 and the source contract passes 12/12;
- no changed status path falls outside the declared write set, diff hygiene
  passes, and `amendment.rs` remains 2,887 lines;
- the public-report directory is absent; lifecycle is `IN_REVIEW`, approval
  lock is null, and independent human approvals remain pending.

The verifier observed the README drift fail closed, then reverified identity,
the same tests, unchanged report roots, and the active event after typed rebind.
No unresolved finding remains.

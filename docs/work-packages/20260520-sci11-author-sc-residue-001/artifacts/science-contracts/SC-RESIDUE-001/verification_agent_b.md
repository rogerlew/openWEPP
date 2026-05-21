# Verification Agent B

Status: complete
Date: 2026-05-20 UTC
Evidence mode: `Static`
Verified contract snapshot: `8516dc2a445556cdf5422b72d7ca2db08f1de887a62f9fd97f8294c98ac30ae2`
Disposition source: `artifacts/science-contracts/SC-RESIDUE-001/disposition.md`

Closure check:
- `B-001`: `closed`
  - verification: `Cr` appears in Variables and Units with ET interface unit semantics.
  - refs: `docs/specifications/science-contracts/contracts/SC-RESIDUE-001.md:71`
- `B-002`: `closed`
  - verification: alias map now includes explicit mappings, including prior coverage gaps for `P` and `Wn`.
  - refs: `docs/specifications/science-contracts/contracts/SC-RESIDUE-001.md:142`, `:146`
- `B-003`: `closed`
  - verification: mixed-unit buckets are replaced with explicit unit rows on cited surfaces.
  - refs: `docs/specifications/science-contracts/contracts/SC-RESIDUE-001.md:76`, `:77`, `:78`, `:87`, `:88`

Regression check:
- No new regressions observed on the `B-001`..`B-003` closure surfaces.

Disposition consistency:
- Verified accepted action claims for `B-001`..`B-003` match current post-fix contract edits.

Verdict:
- `PASS`

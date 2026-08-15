# Hydrology, Science and Ownership Review — `413c0c32a`

Evidence class: `Static + Ran`

Exact reviewed commit:
`413c0c32a41ad609b43343a4fd461c58bdc94862`.

Verdict: `HOLD`.

## Material Finding

`B-TERMINAL-413-HIGH-001`: configuration/state declared digests are not
verified in identity-only preflight. Full validation performs numeric-domain
checks first, so stale configuration/state digest plus NaN returns E003 before
canonical E002. Required closure is isolated stale-digest-plus-NaN vectors for
configuration and state with complete hashes, zero callback execution and
unchanged owner bytes.

## Ran Evidence

- LSE real-hydrology integration: 64/64 PASS.
- LSE crate: 28/28 PASS.
- Surface-liquid authority: 10/10 PASS.
- Diff hygiene: PASS.

## Closed and Residual Surfaces

Standalone D/A/F coverage, real E003/E006 water classification, executable
owner/source/OFE/tile routing, persistent restart custody, finalized-use-only
debit, signed condensation and rollback remain coherent. The public typed-error
source break is authorized. Duplicated thermodynamic constants are bit-identical
and remain a non-blocking maintenance risk.

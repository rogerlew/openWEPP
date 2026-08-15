# Rust Correctness Review — `413c0c32a`

Evidence class: `Static + Ran`

Exact reviewed commit:
`413c0c32a41ad609b43343a4fd461c58bdc94862`.

Verdict: `HOLD`.

## Material Findings

1. `A-TERMINAL-413-HIGH-001`: configuration/state identity preflight omits
   canonical self-digest recomputation. Full validation checks numeric domains
   before declared digests, so stale digest plus NaN returns E003 before E002.
2. `A-TERMINAL-413-HIGH-002`: ingress-identity E002 contextualization hashes
   only the unified projection and does not join raw configuration/state bytes.
   Two raw-invalid attempts sharing declared digests and the same ingress defect
   can therefore alias.

## Ran Evidence

- LSE crate: 28/28 PASS.
- Unified integration contract: 64/64 PASS.
- Custody authority contract: 10/10 PASS.
- Strict all-target/all-feature Clippy for both affected crates: PASS.
- Formatting and diff hygiene: PASS.

## Closed and Residual Surfaces

Standalone sealing now rejects empty, missing and extra ground D/A/F coverage;
real water bounds retain E003/E006 classification; source/OFE/tile identity,
candidate isolation and line-count governance pass. The intentional typed-error
source break remains documented. Bit-identical duplicated thermodynamic
constants remain a non-blocking maintenance risk.

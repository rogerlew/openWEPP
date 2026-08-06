# Owned-File Manifest

Status: narrowed for Phase-1 HOLD

Evidence class: Static. Base commit: `2f423325`.

The authority `NO-GO` narrowed the executed write set before any production or
contract edit:

- `docs/work-packages/20260805-snow-stage3-terminal-meltout-soil-handoff-001/**`
- `docs/ROADMAP.md`
- `docs/work-packages/README.md`
- `docs/planning/snow-surface-energy-balance-roadmap.md`

Explicitly not edited:

- every `SC-*` contract and `docs/specifications/science-contracts/index.md`;
- all Rust production and test paths;
- fixtures, manifests, selectors, schemas, and runtime/public outputs.

The terminal diff must contain only the four documentation surfaces above.

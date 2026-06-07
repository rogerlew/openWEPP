# Contract Implementation Evidence

Status: closed-with-follow-up-postreview

Evidence mode: Static

Contracts amended:

- `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
  - version `52`
  - amended `INV-SNOWFREEZE-019`
  - added `TOL-SNOWFREEZE-006`
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
  - version `145`
  - amended `INV-WATBAL-059`
- `docs/specifications/science-contracts/index.md`
  - updated registry dates and summaries for `SC-SNOWFREEZE-001` and
    `SC-WATBAL-001`

Authority encoded:

- Runtime SWE is derived from authoritative post-hourly depth/density storage.
- Routed snowpack melt must match the storage loss implied by that same store.
- Negative raw melt remains diagnostic and cannot create an independent SWE
  debit.
